//! Manseryuk (Ten-Thousand Year Calendar) astronomical data.
//!
//! Provides daily Ganzi lookup and 24 solar term timing calculations.

use crate::cache::GlobalCache;
use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc};

/// Retrieve the 60-Ganzi index of a specific date.
///
/// Computed using Julian Day Number (JDN).
pub fn get_day_ganzi_index(date: NaiveDate) -> u8 {
    let jdn = gregorian_to_jdn(date.year(), date.month() as i32, date.day() as i32);
    // 2000-01-01 = JDN 2451545 = Wu-Wu day (index=54)
    ((jdn + 49) % 60) as u8
}

/// Retrieve the date and time of a solar term for a given year.
///
/// Checks the binary cache first; falls back to real-time astronomical calculation via eon-astro.
pub fn get_solar_term_datetime(
    year: i32,
    term_index: u8,
) -> Result<DateTime<Utc>, crate::error::DataError> {
    // 1. Query the binary cache first
    if let Some(dt) = GlobalCache::get_solar_term(year, term_index) {
        return Ok(dt);
    }

    // 2. Fall back to real-time astronomical calculation with eon-astro
    let engine = eon_astro::AstroEngine::new();

    // Approximate solar term position (Lichun = 315 deg)
    // 24 solar terms are spaced by ~15 days starting in early February (Lichun).
    // Start search window around Feb 1.
    let approx_start = Utc
        .with_ymd_and_hms(year, 2, 1, 0, 0, 0)
        .single()
        .ok_or(crate::error::DataError::InvalidDate)?;

    match engine.find_solar_term_time(approx_start, term_index) {
        Ok(dt) => Ok(dt),
        Err(_) => {
            // Ultimate fallback approximation if astronomical engine fails
            // Lichun (index 0) is approximately Feb 4
            // Each solar term/sub-term occurs at roughly 15.22 day intervals
            let base_date = Utc
                .with_ymd_and_hms(year, 2, 4, 0, 0, 0)
                .single()
                .ok_or(crate::error::DataError::InvalidDate)?;
            let offset_days = (term_index as f64 * 15.218).round() as i64;
            Ok(base_date + chrono::Duration::days(offset_days))
        }
    }
}

/// Convert Gregorian date to Julian Day Number (JDN).
fn gregorian_to_jdn(year: i32, month: i32, day: i32) -> i64 {
    let a = (14 - month) / 12;
    let y = year + 4800 - a;
    let m = month + 12 * a - 3;

    let jdn = day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;
    jdn as i64
}

/// Lunar-Solar calendar conversion engine.
pub struct LunarCalendar;

impl LunarCalendar {
    /// Convert solar date to lunar date (KST-based calculation) with cache lookup.
    pub fn from_solar(date: NaiveDate) -> Result<(i32, u32, u32, bool), crate::error::DataError> {
        // 1. Check cache first
        if let Some(res) = GlobalCache::get_lunar_date(date) {
            return Ok(res);
        }

        // 2. Real-time astronomical computation
        Self::from_solar_internal(date)
    }

    /// Convert solar date to lunar date using real-time astronomical calculation.
    pub fn from_solar_internal(
        date: NaiveDate,
    ) -> Result<(i32, u32, u32, bool), crate::error::DataError> {
        let engine = eon_astro::AstroEngine::new();
        let kst_offset = chrono::Duration::hours(9);

        // 1. Convert end-of-day in KST to UTC
        let dt_utc = Utc
            .with_ymd_and_hms(date.year(), date.month(), date.day(), 23, 59, 59)
            .single()
            .ok_or(crate::error::DataError::InvalidDate)?
            - kst_offset;

        // 2. Find preceding New Moon time
        let nm_t = engine.find_new_moon_before(dt_utc)?;
        let nm_date_kst = (nm_t + kst_offset).date_naive();
        let lunar_day = (date - nm_date_kst).num_days() as u32 + 1;

        // 3. Find reference Winter Solstice to determine month and year alignment
        // Find Winter Solstice before or at the current new moon
        let ws_t = engine.find_winter_solstice_before(nm_t + chrono::Duration::days(1))?;
        // The month containing the Winter Solstice is decided by calendar date (KST), not by
        // timestamp: a solstice on the same KST date as a later new moon belongs to the month
        // that begins on that date.
        let ws_nm_t = Self::new_moon_on_or_before_kst_date(&engine, ws_t)?;

        // Determine if this sui (cycle between Winter Solstices) has 13 lunar months (contains a leap month)
        let next_ws_t = engine.find_winter_solstice_before(ws_t + chrono::Duration::days(380))?;
        let next_ws_nm_t = Self::new_moon_on_or_before_kst_date(&engine, next_ws_t)?;
        let sui_months = ((next_ws_nm_t - ws_nm_t).num_days() as f64 / 29.53).round() as i32;
        let has_leap_year = sui_months >= 13;

        // The month containing the Winter Solstice is always lunar Month 11
        let mut curr_nm = ws_nm_t;
        let mut month_num = 11;
        let mut is_leap = false;
        let mut leap_placed = false;
        let mut lunar_year = (ws_t + kst_offset).year();

        // 4. Step forward month by month from Month 11 to current New Moon
        while (nm_t - curr_nm).num_minutes() > 1440 {
            let next_nm = engine.find_new_moon_before(curr_nm + chrono::Duration::days(45))?;
            let next_next_nm = engine.find_new_moon_before(next_nm + chrono::Duration::days(45))?;

            // Evaluate the upcoming month [next_nm, next_next_nm)
            let mut has_zq = false;
            let month_start_kst = (next_nm + kst_offset).date_naive();
            let month_end_kst = (next_next_nm + kst_offset).date_naive();

            if let Some(start_dt) = Utc
                .with_ymd_and_hms(
                    month_start_kst.year(),
                    month_start_kst.month(),
                    month_start_kst.day(),
                    0,
                    0,
                    0,
                )
                .single()
            {
                let search_start_utc = start_dt - kst_offset;
                for i in 1..=23 {
                    if i % 2 == 1 {
                        // Zhong-qi odd indices (1, 3, 5, ..., 23)
                        if let Ok(zq_t) = engine.find_solar_term_time(search_start_utc, i) {
                            let zq_date = (zq_t + kst_offset).date_naive();
                            if zq_date >= month_start_kst && zq_date < month_end_kst {
                                has_zq = true;
                                break;
                            }
                        }
                    }
                }
            }

            if has_leap_year && !leap_placed && !has_zq {
                // Month without Zhong-qi in a leap sui becomes the leap month
                is_leap = true;
                leap_placed = true;
            } else {
                month_num += 1;
                if month_num > 12 {
                    month_num = 1;
                    lunar_year += 1;
                }
                is_leap = false;
            }
            curr_nm = next_nm;
        }

        Ok((lunar_year, month_num as u32, lunar_day, is_leap))
    }

    /// Latest new moon whose KST calendar date is on or before the KST date of `t`.
    fn new_moon_on_or_before_kst_date(
        engine: &eon_astro::AstroEngine,
        t: DateTime<Utc>,
    ) -> Result<DateTime<Utc>, crate::error::DataError> {
        let kst_offset = chrono::Duration::hours(9);
        let d = (t + kst_offset).date_naive();
        let end_of_day_utc = Utc
            .with_ymd_and_hms(d.year(), d.month(), d.day(), 23, 59, 59)
            .single()
            .ok_or(crate::error::DataError::InvalidDate)?
            - kst_offset;
        Ok(engine.find_new_moon_before(end_of_day_utc)?)
    }

    /// Convert lunar date to solar date.
    pub fn to_solar(
        year: i32,
        month: u32,
        day: u32,
        is_leap: bool,
    ) -> Result<NaiveDate, crate::error::DataError> {
        let test_date =
            NaiveDate::from_ymd_opt(year, month, 1).ok_or(crate::error::DataError::InvalidDate)?;

        // Scan across a +/- 6 month window (Lunar month 1 corresponds to ~Feb)
        for i in -60..360 {
            let date = test_date + chrono::Duration::days(i);
            if let Ok((ly, lm, ld, il)) = Self::from_solar(date) {
                if ly == year && lm == month && ld == day && il == is_leap {
                    return Ok(date);
                }
            }
        }
        Err(crate::error::DataError::InvalidDate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_day_ganzi_index() {
        // 2000-01-01 = Wu-Wu day (index=54)
        let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        assert_eq!(get_day_ganzi_index(date), 54);

        // 2000-01-02 = Ji-Wei day (index=55)
        let date = NaiveDate::from_ymd_opt(2000, 1, 2).unwrap();
        assert_eq!(get_day_ganzi_index(date), 55);
    }

    #[test]
    fn test_sixty_day_cycle() {
        // Same Ganzi cycle repeats after 60 days
        let date1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(); // 60 days later
        assert_eq!(get_day_ganzi_index(date1), get_day_ganzi_index(date2));
    }

    #[test]
    fn test_lunar_conversion() {
        // Reference birthdate: Solar 2004-11-27 -> Lunar 2004-10-16
        let solar = NaiveDate::from_ymd_opt(2004, 11, 27).unwrap();
        let lunar = LunarCalendar::from_solar(solar).unwrap();
        println!("DEBUG: Lunar result for 2004-11-27: {:?}", lunar);

        assert_eq!(lunar.0, 2004, "Year should be 2004");
        assert_eq!(lunar.1, 10, "Month should be 10");
        assert_eq!(lunar.2, 16, "Day should be 16");
        assert!(!lunar.3, "Should not be a leap month");

        // Verify inverse conversion
        let solar_back = LunarCalendar::to_solar(2004, 10, 16, false).unwrap();
        assert_eq!(solar_back, solar);

        // KASI reference: 2023-03-22 is Lunar 2023 Leap Month 2 Day 1 (윤2월 1일)
        let leap_date = NaiveDate::from_ymd_opt(2023, 3, 22).unwrap();
        let lunar_leap = LunarCalendar::from_solar_internal(leap_date).unwrap();
        println!("DEBUG: Lunar result for 2023-03-22: {:?}", lunar_leap);
        assert_eq!(lunar_leap, (2023, 2, 1, true));
    }

    #[test]
    fn test_leap_month_when_solstice_and_new_moon_share_kst_date() {
        // Winter solstice precedes the new moon on the same KST date (12-22):
        // the month beginning that day contains the solstice and is month 11.
        let d = |y, m, dd| NaiveDate::from_ymd_opt(y, m, dd).unwrap();
        // 1984 leap month 10 (윤10월) starts 1984-11-23
        assert_eq!(
            LunarCalendar::from_solar_internal(d(1984, 11, 25)).unwrap(),
            (1984, 10, 3, true)
        );
        // 2014 leap month 9 (윤9월) starts 2014-10-24
        assert_eq!(
            LunarCalendar::from_solar_internal(d(2014, 10, 24)).unwrap(),
            (2014, 9, 1, true)
        );
        // 1985 has no leap month: 1985-02-20 is Lunar New Year (1월 1일)
        assert_eq!(
            LunarCalendar::from_solar_internal(d(1985, 2, 20)).unwrap(),
            (1985, 1, 1, false)
        );
    }

    #[test]
    fn test_embedded_cache_matches_live_calculation() {
        // Guards against a stale manseryuk.bin: regenerate with
        // `cargo run --release -p eon-data --example generate_cache`.
        let cache = crate::cache::GlobalCache::get();
        for year in [1984, 1985, 2014, 2017, 2023, 2033, 2034] {
            let records = cache
                .lunar_months
                .get(&year)
                .unwrap_or_else(|| panic!("no lunar records for {year}"));
            assert!(!records.is_empty());
            for r in records {
                let live = LunarCalendar::from_solar_internal(r.new_moon_date).unwrap();
                assert_eq!(
                    (r.lunar_year, r.lunar_month, 1, r.is_leap),
                    live,
                    "stale cache at {}",
                    r.new_moon_date
                );
            }
        }
    }

    #[test]
    fn test_to_solar_leap_month_roundtrip() {
        let solar = LunarCalendar::to_solar(2023, 2, 1, true).unwrap();
        assert_eq!(solar, NaiveDate::from_ymd_opt(2023, 3, 22).unwrap());
        assert_eq!(
            LunarCalendar::from_solar(solar).unwrap(),
            (2023, 2, 1, true)
        );
    }
}
