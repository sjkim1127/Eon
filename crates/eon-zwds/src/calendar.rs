//! Lunar calendar conversion wrapper.
//!
//! Reuses Manseryuk lunar calendar calculation
//! to extract lunar parameters for ZWDS.

use crate::error::ZwdsError;
use crate::types::LunarBirthInfo;
use chrono::NaiveDate;

// Reuse eon-data calendar module
use eon_data::LunarCalendar;

/// ZWDS coordinate system branch index (Yin=0)
///   (子=0)
///
/// : 子=0, 丑=1, 寅=2, 卯=3, 辰=4, 巳=5, 午=6, 未=7, 申=8, 酉=9, 戌=10, 亥=11
/// ZWDS: 寅=0, 卯=1, 辰=2, 巳=3, 午=4, 未=5, 申=6, 酉=7, 戌=8, 亥=9, 子=10, 丑=11
pub fn branch_to_zwds_idx(standard_branch_idx: usize) -> usize {
    //   寅(2)  shift
    (standard_branch_idx + 10) % 12
}

///   (子=0) (0-23) (時支)
/// (子時): 23:00-00:59
pub fn hour_to_time_branch_idx(hour: u32) -> usize {
    // : 23  0 → 子(0)
    let adjusted = (hour + 1) % 24;
    (adjusted / 2) as usize
}

///   →    
pub fn solar_to_lunar_birth(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
) -> Result<LunarBirthInfo, ZwdsError> {
    let date = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| ZwdsError::InvalidBirthDate(format!("{}-{}-{}", year, month, day)))?;

    let (lunar_year, lunar_month, lunar_day, is_leap) =
        LunarCalendar::from_solar(date).map_err(|e| ZwdsError::LunarConversion(e.to_string()))?;

    //  : 60
    // (甲子): 4,  (year - 4) % 10
    let year_stem_idx = ((lunar_year - 4).rem_euclid(10)) as usize;

    // : (year - 4) % 12 → 子=0
    let year_branch_std_idx = ((lunar_year - 4).rem_euclid(12)) as usize;

    //  (子=0  )
    let time_branch_std_idx = hour_to_time_branch_idx(hour);

    Ok(LunarBirthInfo {
        year: lunar_year,
        month: lunar_month,
        day: lunar_day,
        is_leap_month: is_leap,
        year_stem_idx,
        year_branch_idx: year_branch_std_idx,
        time_branch_idx: time_branch_std_idx,
    })
}

///       ( )
pub fn lunar_to_birth_info(
    lunar_year: i32,
    lunar_month: u32,
    lunar_day: u32,
    is_leap: bool,
    hour: u32,
) -> LunarBirthInfo {
    let year_stem_idx = ((lunar_year - 4).rem_euclid(10)) as usize;
    let year_branch_std_idx = ((lunar_year - 4).rem_euclid(12)) as usize;
    let time_branch_std_idx = hour_to_time_branch_idx(hour);

    LunarBirthInfo {
        year: lunar_year,
        month: lunar_month,
        day: lunar_day,
        is_leap_month: is_leap,
        year_stem_idx,
        year_branch_idx: year_branch_std_idx,
        time_branch_idx: time_branch_std_idx,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solar_to_lunar_2004_11_27() {
        // 2004-11-27 →  2004-10-16
        let info = solar_to_lunar_birth(2004, 11, 27, 22).unwrap();
        assert_eq!(info.year, 2004);
        assert_eq!(info.month, 10);
        assert_eq!(info.day, 16);
        assert!(!info.is_leap_month);
    }

    #[test]
    fn test_hour_to_time_branch() {
        // 23 → 子時(0)
        assert_eq!(hour_to_time_branch_idx(23), 0);
        // 0 → 子時(0)
        assert_eq!(hour_to_time_branch_idx(0), 0);
        // 22 → 亥時(11)
        assert_eq!(hour_to_time_branch_idx(22), 11);
        // 1 → 丑時(1)
        assert_eq!(hour_to_time_branch_idx(1), 1);
    }

    #[test]
    fn test_branch_to_zwds_idx() {
        // 子(0) → ZWDS 10
        assert_eq!(branch_to_zwds_idx(0), 10);
        // 寅(2) → ZWDS 0
        assert_eq!(branch_to_zwds_idx(2), 0);
        // 丑(1) → ZWDS 11
        assert_eq!(branch_to_zwds_idx(1), 11);
    }
}
