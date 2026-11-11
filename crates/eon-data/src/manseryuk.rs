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
        let ws_nm_t = engine.find_new_moon_before(ws_t + chrono::Duration::minutes(1))?;

        // The month containing the Winter Solstice is always lunar Month 11
        let mut curr_nm = ws_nm_t;
        let mut month_num = 11;
        let mut is_leap = false;
        let mut lunar_year = (ws_t + kst_offset).year();
        // Since Winter Solstice is in late December, Month 1 of this lunar year usually falls in the next solar year
        if (ws_t + kst_offset).month() == 12 {
            // Winter Solstice in Dec: Month 11 belongs to this lunar cycle's year
        }

        // 4. Step forward month by month from Month 11 to current New Moon
        while (nm_t - curr_nm).num_minutes() > 4320 {
            let next_nm = engine.find_new_moon_before(curr_nm + chrono::Duration::days(45))?;

            // Check if this month contains a Zhong-qi (major solar term)
            let mut has_zq = false;
            for i in 1..=23 {
                if i % 2 == 1 {
                    // Zhong-qi odd indices (1, 3, 5, ...)
                    if let Ok(zq_t) = engine.find_solar_term_time(curr_nm, i) {
                        // Consider included if Zhong-qi occurs on the day of new moon (in KST)
                        let zq_date = (zq_t + kst_offset).date_naive();
                        let curr_nm_date = (curr_nm + kst_offset).date_naive();
                        let next_nm_date = (next_nm + kst_offset).date_naive();

                        if zq_date >= curr_nm_date && zq_date < next_nm_date {
                            has_zq = true;
                            break;
                        }
                    }
                }
            }

            if !has_zq && !is_leap {
                // If a month has no Zhong-qi and no leap month has been set yet, assign leap month
                is_leap = true;
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
    }
}
