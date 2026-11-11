//! eon-astro: High-precision pure-Rust astronomical calculation engine.
//!
//! Based on `swisseph-rs` (pure-Rust port of Swiss Ephemeris), running Moshier analytical
//! astronomical algorithms in 100% Rust without external C/FFI toolchains.

use chrono::{DateTime, Datelike, Timelike, Utc};
use std::sync::LazyLock;
use swisseph::{
    date::julday, Body, CalcFlags, CalendarType, Ephemeris, EphemerisConfig, HouseSystem,
};

/// Astronomical calculation errors
#[derive(Debug, thiserror::Error, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "type", content = "message")]
pub enum AstroError {
    #[error("Mutex lock poisoned")]
    LockPoisoned,

    #[error("Swiss Ephemeris error: {0}")]
    FfiError(String),

    #[error("Failed to calculate houses")]
    HouseCalculationError,

    #[error(
        "Invalid geographic coordinates: latitude must be [-90, 90] and longitude [-180, 180]"
    )]
    InvalidCoordinates,

    #[error("Invalid Swiss Ephemeris house system code: {0}")]
    InvalidHouseSystem(i32),
}

static GLOBAL_EPH: LazyLock<Ephemeris> = LazyLock::new(|| {
    Ephemeris::new(EphemerisConfig::default()).expect("failed to initialize swisseph ephemeris")
});

fn planet_id_to_body(planet_id: i32) -> Result<Body, AstroError> {
    match planet_id {
        0 => Ok(Body::Sun),
        1 => Ok(Body::Moon),
        2 => Ok(Body::Mercury),
        3 => Ok(Body::Venus),
        4 => Ok(Body::Mars),
        5 => Ok(Body::Jupiter),
        6 => Ok(Body::Saturn),
        7 => Ok(Body::Uranus),
        8 => Ok(Body::Neptune),
        9 => Ok(Body::Pluto),
        10 => Ok(Body::MeanNode),
        11 => Ok(Body::TrueNode),
        12 => Ok(Body::MeanApogee), // Black Moon Lilith (mean apogee)
        13 => Ok(Body::OscuApogee),
        14 => Ok(Body::Earth),
        15 => Ok(Body::Chiron),
        16 => Ok(Body::Pholus),
        17 => Ok(Body::Ceres),
        18 => Ok(Body::Pallas),
        19 => Ok(Body::Juno),
        20 => Ok(Body::Vesta),
        _ => Err(AstroError::FfiError(format!(
            "Unsupported planet ID: {planet_id}"
        ))),
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AstroEngine;

impl AstroEngine {
    pub fn new() -> Self {
        Self
    }

    /// Convert Chrono DateTime to Julian Day.
    fn to_julian_day(self, dt: DateTime<Utc>) -> f64 {
        let year = dt.year();
        let month = dt.month() as i32;
        let day = dt.day() as i32;
        let hour = dt.hour() as f64
            + (dt.minute() as f64 / 60.0)
            + (dt.second() as f64 / 3600.0)
            + (dt.timestamp_subsec_nanos() as f64 / 3_600_000_000_000.0);

        julday(year, month, day, hour, CalendarType::Gregorian)
    }

    /// Calculate tropical longitude of the Sun at a given datetime.
    pub fn get_sun_longitude(&self, datetime: DateTime<Utc>) -> Result<f64, AstroError> {
        let julian_day = self.to_julian_day(datetime);
        let res = GLOBAL_EPH
            .calc_ut(julian_day, Body::Sun, CalcFlags::SPEED | CalcFlags::MOSEPH)
            .map_err(|e| AstroError::FfiError(e.to_string()))?;
        Ok(res.data[0])
    }

    /// Calculate the 24 solar term index (0~23) for a given datetime.
    /// 0: Lichun, 1: Yushui, 2: Jingzhe ... 23: Dahan
    pub fn get_solar_term_index(&self, datetime: DateTime<Utc>) -> Result<u8, AstroError> {
        let sun_long = self.get_sun_longitude(datetime)?;

        // Align Lichun (315 degrees) to index 0
        let adjusted = (sun_long - 315.0 + 360.0) % 360.0;

        // Solar term changes every 15 degrees (24 terms total)
        let index = (adjusted / 15.0).floor() as u8;
        Ok(index % 24)
    }

    /// Track exact time when the Sun reaches a target tropical longitude (Root Finding).
    pub fn find_time_for_longitude(
        &self,
        start_time: DateTime<Utc>,
        target_long: f64,
    ) -> Result<DateTime<Utc>, AstroError> {
        use chrono::Duration;

        let mut current_time = start_time;

        // Iterate up to 10 times for convergence
        for _ in 0..10 {
            let current_long = self.get_sun_longitude(current_time)?;

            // Calculate angular difference (-180 to +180 deg)
            let mut diff = target_long - current_long;
            while diff > 180.0 {
                diff -= 360.0;
            }
            while diff < -180.0 {
                diff += 360.0;
            }

            if diff.abs() < 0.00001 {
                // Precision within ~1 second
                break;
            }

            // The Sun moves ~0.9856 deg/day
            // Convert degree offset to seconds
            let seconds_to_move = (diff / 0.9856) * 86400.0;
            current_time += Duration::seconds(seconds_to_move as i64);
        }

        Ok(current_time)
    }

    /// Track exact time when a specific planet reaches a target tropical longitude.
    /// Uses dynamic speed to handle retrograde motion and converges via Newton's method.
    pub fn find_time_for_planet_longitude(
        &self,
        start_time: DateTime<Utc>,
        target_long: f64,
        planet_id: i32,
    ) -> Result<DateTime<Utc>, AstroError> {
        use chrono::Duration;

        let mut current_time = start_time;

        // Iterate up to 20 times
        for _ in 0..20 {
            let (current_long, speed) = self.get_planet_full(current_time, planet_id, 256)?;

            // Angular difference (-180 to +180 deg)
            let mut diff = target_long - current_long;
            while diff > 180.0 {
                diff -= 360.0;
            }
            while diff < -180.0 {
                diff += 360.0;
            }

            if diff.abs() < 0.00001 {
                // Converged to high precision
                break;
            }

            // Guard against division by zero if planet speed is stationary
            let mut safe_speed = speed;
            if safe_speed.abs() < 0.001 {
                safe_speed = if safe_speed >= 0.0 { 0.001 } else { -0.001 };
            }

            // Speed in deg/day converted to second adjustment
            let seconds_to_move = (diff / safe_speed) * 86400.0;

            // Clamp max single-step jump (e.g. max 30 days)
            let max_jump = 30.0 * 86400.0;
            let clamped_seconds = if seconds_to_move > max_jump {
                max_jump
            } else if seconds_to_move < -max_jump {
                -max_jump
            } else {
                seconds_to_move
            };

            current_time += Duration::seconds(clamped_seconds as i64);
        }

        Ok(current_time)
    }

    /// Find the exact start time of a given 24 Solar Term.
    pub fn find_solar_term_time(
        &self,
        birth_time: DateTime<Utc>,
        term_idx: u8, // 0: Lichun, 1: Yushui ... 23: Dahan
    ) -> Result<DateTime<Utc>, AstroError> {
        let target_long = (315.0 + (term_idx as f64) * 15.0) % 360.0;
        let current_long = self.get_sun_longitude(birth_time)?;

        let diff = (target_long - current_long).rem_euclid(360.0);

        use chrono::Duration;
        let approx_time = birth_time + Duration::seconds((diff * 86400.0 / 0.9856) as i64);

        self.find_time_for_longitude(approx_time, target_long)
    }

    /// Calculate detailed planet/moon/node data (longitude, speed).
    pub fn get_planet_full(
        &self,
        datetime: DateTime<Utc>,
        planet_id: i32,
        flag: i32,
    ) -> Result<(f64, f64), AstroError> {
        let julian_day = self.to_julian_day(datetime);
        let body = planet_id_to_body(planet_id)?;

        let mut flags = CalcFlags::MOSEPH;
        if flag & 256 != 0 {
            flags |= CalcFlags::SPEED;
        }
        if flag & 2048 != 0 {
            flags |= CalcFlags::EQUATORIAL;
        }
        if flag & 16 != 0 {
            flags |= CalcFlags::TRUEPOS;
        }
        if flag & 8 != 0 {
            flags |= CalcFlags::HELCTR;
        }
        if flag & 65536 != 0 {
            flags |= CalcFlags::SIDEREAL;
        }

        let res = GLOBAL_EPH
            .calc_ut(julian_day, body, flags)
            .map_err(|e| AstroError::FfiError(e.to_string()))?;

        Ok((res.data[0], res.data[3])) // Longitude, Speed
    }

    /// Calculate position of planet, moon, or node (for Vedic/Western astrology).
    pub fn get_planet_position(
        &self,
        datetime: DateTime<Utc>,
        planet_id: i32,
        flag: i32,
    ) -> Result<f64, AstroError> {
        self.get_planet_full(datetime, planet_id, flag)
            .map(|(long, _)| long)
    }

    /// Finds the next rasi (30-degree sign) boundary crossed by a planet.
    /// The scan is direction-agnostic, so retrograde planets are handled too.
    pub fn find_next_planet_sign_entry(
        &self,
        start_time: DateTime<Utc>,
        planet_id: i32,
    ) -> Result<DateTime<Utc>, AstroError> {
        use chrono::Duration;

        let sign = |longitude: f64| (longitude.rem_euclid(360.0) / 30.0).floor() as i32;
        let mut before_time = start_time;
        let mut before_sign = sign(self.get_planet_position(before_time, planet_id, 256)?);

        for _ in 0..1100 {
            let after_time = before_time + Duration::days(1);
            let after_sign = sign(self.get_planet_position(after_time, planet_id, 256)?);
            if after_sign != before_sign {
                let mut low = before_time;
                let mut high = after_time;
                while (high - low).num_seconds() > 1 {
                    let mid = low + (high - low) / 2;
                    if sign(self.get_planet_position(mid, planet_id, 256)?) == before_sign {
                        low = mid;
                    } else {
                        high = mid;
                    }
                }
                return Ok(high);
            }
            before_time = after_time;
            before_sign = after_sign;
        }

        Err(AstroError::FfiError(
            "No planetary sign entry found within 1100 days".to_string(),
        ))
    }

    /// Finds the most recent rasi boundary crossed by a planet before the
    /// supplied time. This is useful for sign-entry-based transit readings.
    pub fn find_previous_planet_sign_entry(
        &self,
        end_time: DateTime<Utc>,
        planet_id: i32,
    ) -> Result<DateTime<Utc>, AstroError> {
        use chrono::Duration;

        let sign = |longitude: f64| (longitude.rem_euclid(360.0) / 30.0).floor() as i32;
        let mut after_time = end_time;
        let mut after_sign = sign(self.get_planet_position(after_time, planet_id, 256)?);

        for _ in 0..1100 {
            let before_time = after_time - Duration::days(1);
            let before_sign = sign(self.get_planet_position(before_time, planet_id, 256)?);
            if before_sign != after_sign {
                let mut low = before_time;
                let mut high = after_time;
                while (high - low).num_seconds() > 1 {
                    let mid = low + (high - low) / 2;
                    if sign(self.get_planet_position(mid, planet_id, 256)?) == after_sign {
                        high = mid;
                    } else {
                        low = mid;
                    }
                }
                return Ok(high);
            }
            after_time = before_time;
            after_sign = before_sign;
        }

        Err(AstroError::FfiError(
            "No planetary sign entry found within 1100 days".to_string(),
        ))
    }

    /// Finds the most recent sidereal rasi boundary, using the supplied
    /// ayanamsa. Vedic sign-entry calculations must use this method.
    pub fn find_previous_planet_sidereal_sign_entry(
        &self,
        end_time: DateTime<Utc>,
        planet_id: i32,
        ayanamsa: f64,
    ) -> Result<DateTime<Utc>, AstroError> {
        use chrono::Duration;

        if !ayanamsa.is_finite() {
            return Err(AstroError::FfiError("Non-finite ayanamsa".to_string()));
        }
        let sign =
            |longitude: f64| ((longitude - ayanamsa).rem_euclid(360.0) / 30.0).floor() as i32;
        let mut after_time = end_time;
        let mut after_sign = sign(self.get_planet_position(after_time, planet_id, 256)?);

        for _ in 0..1100 {
            let before_time = after_time - Duration::days(1);
            let before_sign = sign(self.get_planet_position(before_time, planet_id, 256)?);
            if before_sign != after_sign {
                let mut low = before_time;
                let mut high = after_time;
                while (high - low).num_seconds() > 1 {
                    let mid = low + (high - low) / 2;
                    if sign(self.get_planet_position(mid, planet_id, 256)?) == after_sign {
                        high = mid;
                    } else {
                        low = mid;
                    }
                }
                return Ok(high);
            }
            after_time = before_time;
            after_sign = before_sign;
        }

        Err(AstroError::FfiError(
            "No sidereal planetary sign entry found within 1100 days".to_string(),
        ))
    }

    /// Calculate equatorial coordinates (RA, Declination).
    pub fn get_planet_equatorial(
        &self,
        datetime: DateTime<Utc>,
        planet_id: i32,
    ) -> Result<(f64, f64), AstroError> {
        let julian_day = self.to_julian_day(datetime);
        let body = planet_id_to_body(planet_id)?;

        let res = GLOBAL_EPH
            .calc_ut(julian_day, body, CalcFlags::EQUATORIAL | CalcFlags::MOSEPH)
            .map_err(|e| AstroError::FfiError(e.to_string()))?;

        Ok((res.data[0], res.data[1])) // RA, Declination
    }

    /// Find the nearest New Moon occurring before or at the given datetime.
    pub fn find_new_moon_before(
        &self,
        datetime: DateTime<Utc>,
    ) -> Result<DateTime<Utc>, AstroError> {
        self.find_relative_conjunction_backward(datetime, 0.0)
    }

    /// Find the nearest Winter Solstice (Sun tropical longitude 270 deg) before the given datetime.
    /// Used for aligning the lunar calendar year to the Winter Solstice.
    pub fn find_winter_solstice_before(
        &self,
        datetime: DateTime<Utc>,
    ) -> Result<DateTime<Utc>, AstroError> {
        self.find_longitude_time_backward(datetime, 270.0)
    }

    /// Find the point in time backwards where the Moon-Sun angular separation equals target_diff.
    fn find_relative_conjunction_backward(
        &self,
        start_time: DateTime<Utc>,
        target_diff: f64,
    ) -> Result<DateTime<Utc>, AstroError> {
        use chrono::Duration;

        let mut t = start_time;

        // 1. Initial angular difference check (Newton-Raphson approximation seed)
        let sun_l = self.get_sun_longitude(t)?;
        let moon_l = self.get_planet_position(t, 1, 256)?;
        let diff = (moon_l - sun_l + 360.0) % 360.0;

        // Relative speed of Moon vs Sun is ~12.19 deg/day
        let days_back = diff / 12.19;
        t -= Duration::seconds((days_back * 86400.0) as i64);

        // 2. Convergence loop
        for _ in 0..10 {
            let s = self.get_sun_longitude(t)?;
            let m = self.get_planet_position(t, 1, 256)?;
            let d = (m - s + 360.0) % 360.0;

            let mut err = target_diff - d;
            while err > 180.0 {
                err -= 360.0;
            }
            while err < -180.0 {
                err += 360.0;
            }

            if err.abs() < 0.000001 {
                break;
            }

            let days_adj = err / 12.19;
            t += Duration::seconds((days_adj * 86400.0) as i64);
        }

        // 3. If result is ahead of start_time, step back one month and re-evaluate
        if t > start_time + Duration::seconds(1) {
            t -= Duration::days(29);
            return self.find_relative_conjunction_backward(t, target_diff);
        }

        Ok(t)
    }

    /// Find the point in time backwards where the Sun reaches a target tropical longitude.
    fn find_longitude_time_backward(
        &self,
        start_time: DateTime<Utc>,
        target_long: f64,
    ) -> Result<DateTime<Utc>, AstroError> {
        use chrono::Duration;
        let mut t = start_time;

        let current_long = self.get_sun_longitude(t)?;
        let mut diff = current_long - target_long;
        while diff < 0.0 {
            diff += 360.0;
        }

        // Sun speed is ~0.9856 deg/day
        let days_back = diff / 0.9856;
        t -= Duration::seconds((days_back * 86400.0) as i64);

        for _ in 0..10 {
            let c = self.get_sun_longitude(t)?;
            let mut d = target_long - c;
            while d > 180.0 {
                d -= 360.0;
            }
            while d < -180.0 {
                d += 360.0;
            }

            if d.abs() < 0.000001 {
                break;
            }

            let days_adj = d / 0.9856;
            t += Duration::seconds((days_adj * 86400.0) as i64);
        }

        if t > start_time + Duration::seconds(1) {
            t -= Duration::days(360);
            return self.find_longitude_time_backward(t, target_long);
        }
        Ok(t)
    }

    /// Calculate houses (1~12) and ASC/MC at a given datetime and coordinates.
    pub fn get_houses(
        &self,
        datetime: DateTime<Utc>,
        latitude: f64,
        longitude: f64,
        house_system: i32, // 'P' as char as i32, etc.
    ) -> Result<(Vec<f64>, [f64; 10]), AstroError> {
        if !latitude.is_finite()
            || !longitude.is_finite()
            || !(-90.0..=90.0).contains(&latitude)
            || !(-180.0..=180.0).contains(&longitude)
        {
            return Err(AstroError::InvalidCoordinates);
        }
        if !(0..=u8::MAX as i32).contains(&house_system) {
            return Err(AstroError::InvalidHouseSystem(house_system));
        }

        let Ok(hsys) = HouseSystem::try_from(house_system as u8) else {
            return Err(AstroError::InvalidHouseSystem(house_system));
        };

        let julian_day = self.to_julian_day(datetime);
        let res = GLOBAL_EPH
            .houses_ex(julian_day, CalcFlags::MOSEPH, latitude, longitude, hsys)
            .map_err(|_| AstroError::HouseCalculationError)?;

        let cusps_vec = res.cusps[1..=12].to_vec();
        let mut ascmc = [0.0; 10];
        ascmc[0] = res.ascmc.ascendant;
        ascmc[1] = res.ascmc.mc;
        ascmc[2] = res.ascmc.armc;
        ascmc[3] = res.ascmc.vertex;
        ascmc[4] = res.ascmc.equatorial_ascendant;
        ascmc[5] = res.ascmc.coascendant_koch;
        ascmc[6] = res.ascmc.coascendant_munkasey;
        ascmc[7] = res.ascmc.polar_ascendant;

        Ok((cusps_vec, ascmc))
    }

    /// Calculate Ayanamsa value at a given datetime (default: Lahiri).
    pub fn get_ayanamsa_ut(&self, datetime: DateTime<Utc>) -> f64 {
        self.get_ayanamsa_ut_for_mode(datetime, 1, 0.0, 0.0)
    }

    /// Sidereal mode configuration (in stateless engine, get_ayanamsa_ut_for_mode is recommended).
    pub fn set_sidereal_mode(&self, _method_id: i32, _t0: f64, _ayan_t0: f64) {}

    /// Set a sidereal mode and read its ayanamsa statelessly (thread-safe, zero locks).
    pub fn get_ayanamsa_ut_for_mode(
        &self,
        datetime: DateTime<Utc>,
        method_id: i32,
        t0: f64,
        ayan_t0: f64,
    ) -> f64 {
        let julian_day = self.to_julian_day(datetime);
        let mut config = GLOBAL_EPH.config().clone();
        config.set_sidereal_mode(method_id, t0, ayan_t0);

        let dt = swisseph::deltat::calc_deltat(julian_day, &config);
        GLOBAL_EPH
            .get_ayanamsa_ex_with_config(julian_day + dt, CalcFlags::empty(), &config)
            .unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{AstroEngine, AstroError};
    use chrono::{TimeZone, Utc};
    use std::thread;

    #[test]
    fn sidereal_mode_and_ayanamsa_are_atomic_across_threads() {
        let time = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        let expected_lahiri = AstroEngine::new().get_ayanamsa_ut_for_mode(time, 1, 0.0, 0.0);
        let expected_raman = AstroEngine::new().get_ayanamsa_ut_for_mode(time, 3, 0.0, 0.0);

        let handles = (0..16)
            .map(|i| {
                thread::spawn(move || {
                    let engine = AstroEngine::new();
                    let method = if i % 2 == 0 { 1 } else { 3 };
                    let actual = engine.get_ayanamsa_ut_for_mode(time, method, 0.0, 0.0);
                    let expected = if method == 1 {
                        expected_lahiri
                    } else {
                        expected_raman
                    };
                    assert!((actual - expected).abs() < 1e-12);
                })
            })
            .collect::<Vec<_>>();

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn houses_reject_non_finite_or_out_of_range_coordinates() {
        let engine = AstroEngine::new();
        let time = Utc.with_ymd_and_hms(2024, 4, 15, 12, 0, 0).unwrap();

        for (lat, lon) in [
            (90.0001, 0.0),
            (-90.0001, 0.0),
            (0.0, 180.0001),
            (0.0, f64::NAN),
        ] {
            assert_eq!(
                engine.get_houses(time, lat, lon, b'W' as i32),
                Err(AstroError::InvalidCoordinates)
            );
        }

        assert!(engine.get_houses(time, 90.0, 180.0, b'W' as i32).is_ok());
        assert!(engine.get_houses(time, 0.0, 0.0, b'V' as i32).is_ok());
        assert_eq!(
            engine.get_houses(time, 0.0, 0.0, b'Z' as i32),
            Err(AstroError::InvalidHouseSystem(b'Z' as i32))
        );
    }

    #[test]
    fn finds_next_solar_rasi_entry() {
        let engine = AstroEngine::new();
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let entry = engine.find_next_planet_sign_entry(start, 0).unwrap();

        assert!(entry >= Utc.with_ymd_and_hms(2024, 1, 19, 0, 0, 0).unwrap());
        assert!(entry <= Utc.with_ymd_and_hms(2024, 1, 22, 0, 0, 0).unwrap());
        let before = engine
            .get_planet_position(entry - chrono::Duration::seconds(2), 0, 256)
            .unwrap();
        let after = engine
            .get_planet_position(entry + chrono::Duration::seconds(2), 0, 256)
            .unwrap();
        assert_ne!((before / 30.0).floor(), (after / 30.0).floor());
    }

    #[test]
    fn finds_next_lunar_rasi_entry() {
        let engine = AstroEngine::new();
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let entry = engine.find_next_planet_sign_entry(start, 1).unwrap();

        assert!(entry > start);
        assert!(entry <= start + chrono::Duration::days(4));
        let before = engine
            .get_planet_position(entry - chrono::Duration::seconds(2), 1, 256)
            .unwrap();
        let after = engine
            .get_planet_position(entry + chrono::Duration::seconds(2), 1, 256)
            .unwrap();
        assert_ne!((before / 30.0).floor(), (after / 30.0).floor());
    }

    #[test]
    fn finds_previous_solar_rasi_entry() {
        let engine = AstroEngine::new();
        let end = Utc.with_ymd_and_hms(2024, 2, 1, 0, 0, 0).unwrap();
        let entry = engine.find_previous_planet_sign_entry(end, 0).unwrap();

        assert!(entry >= Utc.with_ymd_and_hms(2024, 1, 19, 0, 0, 0).unwrap());
        assert!(entry <= Utc.with_ymd_and_hms(2024, 1, 22, 0, 0, 0).unwrap());
        assert!(entry < end);
    }

    #[test]
    fn finds_previous_sidereal_solar_rasi_entry() {
        let engine = AstroEngine::new();
        let end = Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap();
        let entry = engine
            .find_previous_planet_sidereal_sign_entry(end, 0, 24.0)
            .unwrap();

        assert!(entry >= Utc.with_ymd_and_hms(2024, 2, 12, 0, 0, 0).unwrap());
        assert!(entry <= Utc.with_ymd_and_hms(2024, 2, 16, 0, 0, 0).unwrap());
    }

    #[test]
    fn rejects_non_finite_sidereal_ayanamsa() {
        let engine = AstroEngine::new();
        let time = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

        for ayanamsa in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(engine
                .find_previous_planet_sidereal_sign_entry(time, 0, ayanamsa)
                .is_err());
        }
    }

    #[test]
    fn finds_previous_sidereal_saturn_rasi_entry() {
        let engine = AstroEngine::new();
        let end = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let entry = engine
            .find_previous_planet_sidereal_sign_entry(end, 6, 24.0)
            .unwrap();

        assert!(entry >= Utc.with_ymd_and_hms(2023, 1, 15, 0, 0, 0).unwrap());
        assert!(entry <= Utc.with_ymd_and_hms(2023, 1, 20, 0, 0, 0).unwrap());
    }

    #[test]
    fn solar_term_crossings_match_swiss_ephemeris_oracle() {
        let engine = AstroEngine::new();
        let year_start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let expected = [
            Utc.with_ymd_and_hms(2024, 2, 4, 8, 27, 8).unwrap(),
            Utc.with_ymd_and_hms(2024, 5, 5, 0, 10, 4).unwrap(),
            Utc.with_ymd_and_hms(2024, 8, 7, 0, 9, 15).unwrap(),
            Utc.with_ymd_and_hms(2024, 11, 6, 22, 20, 5).unwrap(),
        ];

        for (term_idx, expected_time) in [0u8, 6, 12, 18].into_iter().zip(expected) {
            let actual = engine.find_solar_term_time(year_start, term_idx).unwrap();
            assert!(
                (actual - expected_time).num_seconds().abs() <= 2,
                "term {term_idx} differs by {} seconds",
                (actual - expected_time).num_seconds()
            );
        }
    }
}
