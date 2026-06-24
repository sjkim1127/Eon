use crate::planets::VedicPlanet;
use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Panchanga {
    pub vara: String,
    pub tithi: u8, // 1~30
    pub tithi_name: String,
    pub nakshatra: u8, // 1~27
    pub yoga: u8,      // 1~27 (Nitya Yoga)
    pub karana: u8,    // 1~60 (Sequential) or 1~11 (Types)
    pub karana_name: String,

    // New Fields for Kala Bala
    pub current_time: DateTime<Utc>,
    pub sunrise: DateTime<Utc>,
    pub sunset: DateTime<Utc>,
    pub next_sunrise: DateTime<Utc>,
    pub is_day_birth: bool,
    pub day_lord: VedicPlanet,
    pub hour_lord: VedicPlanet,
    pub daily_parts: [VedicPlanet; 8], // Tribhaga lords (Day: 3, Night: 3, Total usually handled as 8 yamas or parts)
    pub is_night_birth: bool,
    
    // Yogi, Avayogi, Dagdha Rashi
    pub yogi_point: f64,
    pub yogi_planet: VedicPlanet,
    pub avayogi_planet: VedicPlanet,
    pub dagdha_rashis: Vec<u8>,

    // Rahu Kalam, Yamaganda, Gulika
    pub rahu_kalam: (DateTime<Utc>, DateTime<Utc>),
    pub yamaganda: (DateTime<Utc>, DateTime<Utc>),
    pub gulika: (DateTime<Utc>, DateTime<Utc>),
}

fn get_nakshatra_lord(nakshatra: u8) -> VedicPlanet {
    let lords = [
        VedicPlanet::Ketu,
        VedicPlanet::Venus,
        VedicPlanet::Sun,
        VedicPlanet::Moon,
        VedicPlanet::Mars,
        VedicPlanet::Rahu,
        VedicPlanet::Jupiter,
        VedicPlanet::Saturn,
        VedicPlanet::Mercury,
    ];
    lords[((nakshatra - 1) % 9) as usize]
}

fn get_dagdha_rashis(tithi: u8) -> Vec<u8> {
    let t = if tithi > 15 { tithi - 15 } else { tithi };
    match t {
        1 => vec![1, 7],   // Aries, Libra
        2 => vec![2, 5],   // Taurus, Leo
        3 => vec![3, 6],   // Gemini, Virgo
        4 => vec![2, 11],  // Taurus, Aquarius
        5 => vec![3, 9],   // Gemini, Sagittarius
        6 => vec![1, 8],   // Aries, Scorpio
        7 => vec![3, 12],  // Gemini, Pisces
        8 => vec![3, 6],   // Gemini, Virgo
        9 => vec![5, 8],   // Leo, Scorpio
        10 => vec![5, 9],  // Leo, Sagittarius
        11 => vec![9, 12], // Sagittarius, Pisces
        12 => vec![7, 10], // Libra, Capricorn
        13 => vec![2, 3],  // Taurus, Gemini
        14 => vec![2, 3, 6, 9], // Taurus, Gemini, Virgo, Sagittarius
        _ => vec![],
    }
}


pub struct PanchangaEngine;

impl PanchangaEngine {
    pub fn calculate(
        sun_deg: f64,
        moon_deg: f64,
        time: DateTime<Utc>,
        latitude: f64,
        longitude: f64,
    ) -> Panchanga {
        let sun = sun_deg;
        let moon = moon_deg;

        // 1. Fetch sunrise/sunset for the current LOCAL date
        let local_offset_mins = (longitude * 4.0) as i64;
        let local_time = time + chrono::Duration::minutes(local_offset_mins);
        
        let (mut sunrise, mut sunset) = Self::calculate_sunrise_sunset(local_time, latitude, longitude);
        
        // If current time is before the sunrise of its local date, 
        // the astrological day started at yesterday's sunrise
        if time < sunrise {
            let (prev_rise, prev_set) = Self::calculate_sunrise_sunset(local_time - chrono::Duration::days(1), latitude, longitude);
            sunrise = prev_rise;
            sunset = prev_set;
        }
        
        let (next_sunrise, _) = Self::calculate_sunrise_sunset(sunrise + chrono::Duration::days(1), latitude, longitude);

        // 2. Vara (Weekday) - Vedic Day starts at Sunrise
        let is_day_birth = time >= sunrise && time < sunset;
        let is_night_birth = !is_day_birth;
        
        // Use the actual sunrise time to determine the local weekday
        let local_offset_mins = (longitude * 4.0) as i64;
        let sunrise_local = sunrise + chrono::Duration::minutes(local_offset_mins);
        let vara_date = sunrise_local.date_naive();
        let vara = match vara_date.weekday() {
            chrono::Weekday::Sun => "Sunday",
            chrono::Weekday::Mon => "Monday",
            chrono::Weekday::Tue => "Tuesday",
            chrono::Weekday::Wed => "Wednesday",
            chrono::Weekday::Thu => "Thursday",
            chrono::Weekday::Fri => "Friday",
            chrono::Weekday::Sat => "Saturday",
        }.to_string();

        let day_lord = match vara_date.weekday() {
            chrono::Weekday::Sun => VedicPlanet::Sun,
            chrono::Weekday::Mon => VedicPlanet::Moon,
            chrono::Weekday::Tue => VedicPlanet::Mars,
            chrono::Weekday::Wed => VedicPlanet::Mercury,
            chrono::Weekday::Thu => VedicPlanet::Jupiter,
            chrono::Weekday::Fri => VedicPlanet::Venus,
            chrono::Weekday::Sat => VedicPlanet::Saturn,
        };

        // 3. Hourly Lord (Hora)
        let hour_lord = Self::calculate_hora(time, sunrise, sunset, next_sunrise, day_lord);
        let daily_parts = Self::calculate_tribhaga_lords(day_lord); // Simplified for Tribhaga

        // 4. Tithi (Moon - Sun) / 12
        let tithi_deg = (moon - sun + 360.0) % 360.0;
        let tithi = (tithi_deg / 12.0).floor() as u8 + 1;
        let tithi_name = Self::get_tithi_name(tithi);

        // 5. Nakshatra (Moon / 13.333)
        let nakshatra = (moon / (360.0 / 27.0)).floor() as u8 + 1;

        // 6. Yoga (Sun + Moon) / 13.333
        let yoga_deg = (sun + moon) % 360.0;
        let yoga = (yoga_deg / (360.0 / 27.0)).floor() as u8 + 1;

        // 7. Karana (Tithi_deg / 6)
        // Karana is half of a Tithi.
        let karana_idx = (tithi_deg / 6.0).floor() as u16 + 1;
        let karana_name = Self::get_karana_name(karana_idx);

        // Yogi, Avayogi & Dagdha Rashi
        let yogi_point = (sun + moon + 93.3333333) % 360.0;
        let yogi_nak = (yogi_point / (360.0 / 27.0)).floor() as u8 + 1;
        let yogi_planet = get_nakshatra_lord(yogi_nak);

        let avayogi_point = (yogi_point + 186.6666667) % 360.0;
        let avayogi_nak = (avayogi_point / (360.0 / 27.0)).floor() as u8 + 1;
        let avayogi_planet = get_nakshatra_lord(avayogi_nak);

        let dagdha_rashis = get_dagdha_rashis(tithi);

        // Rahu Kalam, Yamaganda, Gulika calculations (daytime divided into 8 parts)
        let rahu_part = match vara_date.weekday() {
            chrono::Weekday::Sun => 8,
            chrono::Weekday::Mon => 2,
            chrono::Weekday::Tue => 7,
            chrono::Weekday::Wed => 5,
            chrono::Weekday::Thu => 6,
            chrono::Weekday::Fri => 4,
            chrono::Weekday::Sat => 3,
        };

        let yama_part = match vara_date.weekday() {
            chrono::Weekday::Sun => 5,
            chrono::Weekday::Mon => 4,
            chrono::Weekday::Tue => 3,
            chrono::Weekday::Wed => 2,
            chrono::Weekday::Thu => 1,
            chrono::Weekday::Fri => 7,
            chrono::Weekday::Sat => 6,
        };

        let guli_part = match vara_date.weekday() {
            chrono::Weekday::Sun => 7,
            chrono::Weekday::Mon => 6,
            chrono::Weekday::Tue => 5,
            chrono::Weekday::Wed => 4,
            chrono::Weekday::Thu => 3,
            chrono::Weekday::Fri => 2,
            chrono::Weekday::Sat => 1,
        };

        let day_duration_ms = sunset.signed_duration_since(sunrise).num_milliseconds() as f64;
        let part_ms = day_duration_ms / 8.0;

        let get_time_range = |part_idx: u8| -> (DateTime<Utc>, DateTime<Utc>) {
            let start_offset_ms = (part_idx - 1) as f64 * part_ms;
            let end_offset_ms = part_idx as f64 * part_ms;
            
            let start_time = sunrise + chrono::Duration::milliseconds(start_offset_ms as i64);
            let end_time = sunrise + chrono::Duration::milliseconds(end_offset_ms as i64);
            (start_time, end_time)
        };

        let rahu_kalam = get_time_range(rahu_part);
        let yamaganda = get_time_range(yama_part);
        let gulika = get_time_range(guli_part);

        Panchanga {
            vara,
            tithi,
            tithi_name,
            nakshatra,
            yoga,
            karana: Self::get_karana_type(karana_idx),
            karana_name,
            current_time: time,
            sunrise,
            sunset,
            next_sunrise,
            is_day_birth,
            is_night_birth,
            day_lord,
            hour_lord,
            daily_parts,
            yogi_point,
            yogi_planet,
            avayogi_planet,
            dagdha_rashis,
            rahu_kalam,
            yamaganda,
            gulika,
        }
    }

    /// Calculate Hora Lord
    fn calculate_hora(
        time: DateTime<Utc>,
        sunrise: DateTime<Utc>,
        sunset: DateTime<Utc>,
        next_sunrise: DateTime<Utc>,
        day_lord: VedicPlanet,
    ) -> VedicPlanet {
        // Hora order: Sun, Venus, Mercury, Moon, Saturn, Jupiter, Mars sequence repeats
        // Standard Sequence starting from Day Lord is:
        // 1st Hora = Day Lord
        // 2nd Hora = 6th weekday from current...

        // Correct Sequence for Hora starting from Sunday: Sun, Venus, Mercury, Moon, Saturn, Jupiter, Mars.
        let hora_seq = [
            VedicPlanet::Sun,
            VedicPlanet::Venus,
            VedicPlanet::Mercury,
            VedicPlanet::Moon,
            VedicPlanet::Saturn,
            VedicPlanet::Jupiter,
            VedicPlanet::Mars,
        ];

        // Find index of day lord in the sequence
        let start_idx = hora_seq.iter().position(|&p| p == day_lord).unwrap_or(0);

        let is_day = time >= sunrise && time < sunset;

        let (duration, start_time) = if is_day {
            let diff = sunset.signed_duration_since(sunrise).num_seconds() as f64;
            (diff / 12.0, sunrise)
        } else {
            let diff = next_sunrise.signed_duration_since(sunset).num_seconds() as f64;
            (diff / 12.0, sunset)
        };

        if duration == 0.0 {
            return day_lord;
        } // Safety

        // Calculate offset from start_time
        let elapsed = if time >= start_time {
            time.signed_duration_since(start_time).num_seconds() as f64
        } else {
            0.0
        };

        let hora_idx = (elapsed / duration).floor() as usize;
        let offset = if is_day { hora_idx } else { hora_idx + 12 };

        let final_idx = (start_idx + offset) % 7;
        hora_seq[final_idx]
    }

    /// Calculate Tribhaga Lords (BPHS Standard)
    /// Day is divided into 3 parts: Mercury (1st), Sun (2nd), Saturn (3rd)
    /// Night is divided into 3 parts: Moon (1st), Venus (2nd), Mars (3rd)
    /// Returns array of 8 lords (3 day + 3 night + 2 padding for Kala Bala compatibility)
    fn calculate_tribhaga_lords(day_lord: VedicPlanet) -> [VedicPlanet; 8] {
        // BPHS Standard Tribhaga Lords:
        // Day Parts (from sunrise to sunset): Mercury, Sun, Saturn
        // Night Parts (from sunset to next sunrise): Moon, Venus, Mars

        // Return standard pattern (first 3 for day, next 3 for night)
        [
            // Day lords (used when is_day_birth == true)
            VedicPlanet::Mercury, // 1st third of day
            VedicPlanet::Sun,     // 2nd third of day
            VedicPlanet::Saturn,  // 3rd third of day
            // Night lords (used when is_day_birth == false)
            VedicPlanet::Moon,  // 1st third of night
            VedicPlanet::Venus, // 2nd third of night
            VedicPlanet::Mars,  // 3rd third of night
            // Padding for compatibility
            day_lord,
            day_lord,
        ]
    }

    /// NOAA Sunrise/Sunset Algorithm (Simplified)
    fn calculate_sunrise_sunset(
        date: DateTime<Utc>,
        lat: f64,
        lon: f64,
    ) -> (DateTime<Utc>, DateTime<Utc>) {
        // Convert date to Julian Day
        let timestamp = date.timestamp();
        let julian_day = (timestamp as f64 / 86400.0) + 2440587.5;
        let julian_century = (julian_day - 2451545.0) / 36525.0;

        // Geom Mean Long Sun (deg)
        let geom_mean_long_sun =
            (280.46646 + julian_century * (36000.76983 + julian_century * 0.0003032)) % 360.0;

        // Geom Mean Anom Sun (deg)
        let geom_mean_anom_sun =
            357.52911 + julian_century * (35999.05029 - 0.0001537 * julian_century);

        // Eccent Earth Orbit
        let eccent_earth_orbit =
            0.016708634 - julian_century * (0.000042037 + 0.0000001267 * julian_century);

        // Sun Eq of Ctr
        let sun_eq_of_ctr = geom_mean_anom_sun.to_radians().sin()
            * (1.914602 - julian_century * (0.004817 + 0.000014 * julian_century))
            + (2.0 * geom_mean_anom_sun.to_radians()).sin()
                * (0.019993 - 0.000101 * julian_century)
            + (3.0 * geom_mean_anom_sun.to_radians()).sin() * 0.000289;

        let sun_true_long = geom_mean_long_sun + sun_eq_of_ctr;

        // Sun App Long
        let sun_app_long = sun_true_long
            - 0.00569
            - 0.00478 * (125.04 - 1934.136 * julian_century).to_radians().sin();

        // Mean Obliq Ecliptic
        let mean_obliq_ecliptic = 23.0
            + (26.0
                + (21.448
                    - julian_century
                        * (46.815 + julian_century * (0.00059 - julian_century * 0.001813)))
                    / 60.0)
                / 60.0;
        let obliq_corr =
            mean_obliq_ecliptic + 0.00256 * (125.04 - 1934.136 * julian_century).to_radians().cos();

        // Sun Declin
        let sun_declin = (obliq_corr.to_radians().sin() * sun_app_long.to_radians().sin()).asin();

        // Var y
        let var_y = (obliq_corr.to_radians() / 2.0).tan().powi(2);

        // Eq of Time (minutes)
        let eq_of_time = 4.0
            * (var_y * (2.0 * geom_mean_long_sun.to_radians()).sin()
                - 2.0 * eccent_earth_orbit * geom_mean_anom_sun.to_radians().sin()
                + 4.0
                    * eccent_earth_orbit
                    * var_y
                    * geom_mean_anom_sun.to_radians().sin()
                    * (2.0 * geom_mean_long_sun.to_radians()).cos()
                - 0.5 * var_y * var_y * (4.0 * geom_mean_long_sun.to_radians()).sin()
                - 1.25
                    * eccent_earth_orbit
                    * eccent_earth_orbit
                    * (2.0 * geom_mean_anom_sun.to_radians()).sin())
            .to_degrees();

        // HA Sunrise (deg)
        // cos(HA) = cos(90.833) / (cos(lat)*cos(declin)) - tan(lat)*tan(declin)
        let zenith = 90.833f64.to_radians(); // Official sunrise zenith
        let cos_ha = (zenith.cos() / (lat.to_radians().cos() * sun_declin.cos()))
            - (lat.to_radians().tan() * sun_declin.tan());

        // Check for polar day/night (simplification: clamp)
        let ha_deg = if cos_ha > 1.0 {
            0.0
        } else if cos_ha < -1.0 {
            180.0
        } else {
            cos_ha.acos().to_degrees()
        };

        let solar_noon = 720.0 - 4.0 * lon - eq_of_time;

        let sunrise_min = solar_noon - 4.0 * ha_deg;
        let sunset_min = solar_noon + 4.0 * ha_deg;

        // Use UTC midnight of the given date reference
        let midnight = date.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();

        let rise_secs = (sunrise_min * 60.0) as i64;
        let set_secs = (sunset_min * 60.0) as i64;

        (
            midnight + chrono::Duration::seconds(rise_secs),
            midnight + chrono::Duration::seconds(set_secs),
        )
    }

    fn get_tithi_name(tithi: u8) -> String {
        let names = [
            "Prathama",
            "Dwitiya",
            "Tritiya",
            "Chaturthi",
            "Panchami",
            "Shashti",
            "Saptami",
            "Ashtami",
            "Navami",
            "Dashami",
            "Ekadashi",
            "Dwadashi",
            "Trayodashi",
            "Chaturdashi",
            "Purnima",
            "Prathama (K)",
            "Dwitiya (K)",
            "Tritiya (K)",
            "Chaturthi (K)",
            "Panchami (K)",
            "Shashti (K)",
            "Saptami (K)",
            "Ashtami (K)",
            "Navami (K)",
            "Dashami (K)",
            "Ekadashi (K)",
            "Dwadashi (K)",
            "Trayodashi (K)",
            "Chaturdashi (K)",
            "Amavasya",
        ];
        names
            .get(tithi as usize - 1)
            .unwrap_or(&"Unknown")
            .to_string()
    }

    /// Karana type mapping (BPHS rules)
    /// Sequential index 1-60 → Type 1-11
    /// 1: Kimstughna (fixed, only first karana)
    /// 2-8: Bava, Balava, Kaulava, Taitila, Gara, Vanija, Vishti (rotating, 8 cycles)
    /// 9-11: Shakuni, Chatushpada, Naga (fixed, last 3 karanas)
    fn get_karana_type(karana_idx: u16) -> u8 {
        if karana_idx == 1 {
            1 // Kimstughna (fixed)
        } else if karana_idx >= 58 {
            // Last 3: Shakuni(9), Chatushpada(10), Naga(11)
            (karana_idx - 58 + 9) as u8
        } else {
            // Rotating 7 movable karanas: Bava(2)..Vishti(8)
            (((karana_idx - 2) % 7) + 2) as u8
        }
    }

    fn get_karana_name(idx: u16) -> String {
        if idx == 1 {
            return "Kimstughna".to_string();
        }
        if idx >= 58 {
            match idx {
                58 => return "Shakuni".to_string(),
                59 => return "Chatushpada".to_string(),
                60 => return "Naga".to_string(),
                _ => {}
            }
        }
        let movables = [
            "Bava", "Balava", "Kaulava", "Taitila", "Gara", "Vanija", "Vishti",
        ];
        let m_idx = ((idx.saturating_sub(2)) % 7) as usize;
        movables[m_idx].to_string()
    }
}
