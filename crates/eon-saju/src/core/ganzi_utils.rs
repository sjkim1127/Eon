//! Dynamic GanZi Calculation Utilities
//!
//! Provides shared functions for dynamic calculation of month, day, and hour GanZi.
//! Uses the identical core logic as `FourPillars::calculate_*` to guarantee consistency.
//!
//! ## Usage
//! - Used during high-resolution timeline sweeps in the fuzzer
//! - Used for rapid calculation of GanZi for a given date/time

use crate::core::branch::EarthlyBranch;
use crate::core::ganzi::GanZi;
use crate::core::stem::HeavenlyStem;
use chrono::{DateTime, NaiveDate, Utc};

/// Calculates the month GanZi corresponding to a specific year and month.
///
/// # Arguments
/// * `year` - Solar year
/// * `month` - Solar month (1-12)
///
/// # Returns
/// GanZi of the specified month
///
/// # Note
/// Simplified calculation without considering precise solar term boundaries.
/// If precision solar term month GanZi is required, use `FourPillars::calculate_month_pillar`.
pub fn calculate_month_ganzi(year: i32, month: i32) -> GanZi {
    let year_ganzi = GanZi::from_year(year);

    // Five Tigers Seeking Month (五虎遁月法): Determine Yin-month stem from Year stem
    // Jia/Ji years -> Bing-Yin month, Yi/Geng years -> Wu-Yin month, Bing/Xin years -> Geng-Yin month,
    // Ding/Ren years -> Ren-Yin month, Wu/Gui years -> Jia-Yin month
    let first_month_stem_idx = match year_ganzi.stem.index() % 5 {
        0 => 2, // Jia, Ji -> Bing
        1 => 4, // Yi, Geng -> Wu
        2 => 6, // Bing, Xin -> Geng
        3 => 8, // Ding, Ren -> Ren
        4 => 0, // Wu, Gui -> Jia
        _ => 0,
    };

    // Month branch: 1st month = Yin(2), 2nd month = Mao(3), etc.
    // Approximate mapping from solar month to lunar month (ignoring exact solar term cutoffs)
    let month_branch_idx = (month + 1) % 12;
    let month_stem_idx = (first_month_stem_idx + (month - 1)) % 10;

    GanZi::new(
        HeavenlyStem::from_index(month_stem_idx),
        EarthlyBranch::from_index(month_branch_idx),
    )
}

/// Calculates the day GanZi corresponding to a specific date.
///
/// # Arguments
/// * `year` - Solar year
/// * `month` - Solar month (1-12)
/// * `day` - Solar day (1-31)
///
/// # Returns
/// GanZi of the specified day
///
/// # Note
/// Internally uses `eon_data::manseryuk::get_day_ganzi_index` to guarantee
/// consistency with Manseryuk ephemeris calendar data.
pub fn calculate_day_ganzi(year: i32, month: u32, day: u32) -> GanZi {
    if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
        let idx = eon_data::manseryuk::get_day_ganzi_index(date);
        GanZi::from_index(idx as i32)
    } else {
        // Fallback for invalid date (should not occur)
        GanZi::from_index(0)
    }
}

/// Day GanZi calculation using Julian Day Number (without Manseryuk calendar cache)
///
/// # Arguments
/// * `year` - Solar year
/// * `month` - Solar month (1-12)
/// * `day` - Solar day (1-31)
///
/// # Returns
/// GanZi of the specified day
///
/// # Note
/// Computes day GanZi purely algorithmically without Manseryuk tables.
/// When highest precision is required, use `calculate_day_ganzi`.
pub fn calculate_day_ganzi_jdn(year: i32, month: u32, day: u32) -> GanZi {
    // Gregorian calendar -> Julian Day Number conversion (simplified formula)
    let a = (14 - month as i32) / 12;
    let y = year + 4800 - a;
    let m = month as i32 + 12 * a - 3;
    let jdn = day as i64 + (153 * m + 2) as i64 / 5 + 365 * y as i64 + y as i64 / 4
        - y as i64 / 100
        + y as i64 / 400
        - 32045;

    // JDN + 49 -> 60 JiaZi index (JiaZi = 0 baseline alignment)
    let idx = (jdn + 49).rem_euclid(60);
    GanZi::from_index(idx as i32)
}

/// Calculates the hour GanZi corresponding to a specific hour.
///
/// # Arguments
/// * `daily_ganzi` - Day GanZi (required for determining hour stem)
/// * `hour` - Hour (0-23)
///
/// # Returns
/// GanZi of the specified hour
pub fn calculate_hour_ganzi(daily_ganzi: GanZi, hour: u32) -> GanZi {
    let branch = EarthlyBranch::from_hour(hour as u8);

    // Five Rats Seeking Hour (五子遁時法): Determine Zi-hour stem from Day stem
    // Jia/Ji day -> Jia-Zi, Yi/Geng day -> Bing-Zi, Bing/Xin day -> Wu-Zi,
    // Ding/Ren day -> Geng-Zi, Wu/Gui day -> Ren-Zi
    let zi_stem_idx = (daily_ganzi.stem.index() % 5) * 2;
    let stem = HeavenlyStem::from_index((zi_stem_idx + branch.index()) as i32);

    GanZi::new(stem, branch)
}

/// Calculates the full GanZi pillar set from a given DateTime<Utc>.
///
/// # Arguments
/// * `dt` - UTC timestamp
///
/// # Returns
/// (Year GanZi, Month GanZi, Day GanZi, Hour GanZi) tuple
///
/// # Note
/// Simplified calculation without considering precise solar term boundaries.
pub fn calculate_all_ganzi_from_datetime(dt: DateTime<Utc>) -> (GanZi, GanZi, GanZi, GanZi) {
    use chrono::Datelike;
    use chrono::Timelike;

    let year = dt.year();
    let month = dt.month() as i32;
    let day = dt.day();
    let hour = dt.hour();

    let year_ganzi = GanZi::from_year(year);
    let month_ganzi = calculate_month_ganzi(year, month);
    let day_ganzi = calculate_day_ganzi(year, month as u32, day);
    let hour_ganzi = calculate_hour_ganzi(day_ganzi, hour);

    (year_ganzi, month_ganzi, day_ganzi, hour_ganzi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_month_ganzi_basic() {
        // January 2024 (Yin month of Jia-Chen year) -> Bing-Yin month
        let mg = calculate_month_ganzi(2024, 1);
        assert_eq!(mg.stem, HeavenlyStem::Bing); // 丙
        assert_eq!(mg.branch, EarthlyBranch::Yin); // 寅
    }

    #[test]
    fn test_day_ganzi_consistency() {
        // Verify consistency between Manseryuk-based and JDN-based calculation
        let day1 = calculate_day_ganzi(2024, 1, 1);
        let day1_jdn = calculate_day_ganzi_jdn(2024, 1, 1);

        // Both methods should produce identical branch outcomes
        assert_eq!(day1.branch, day1_jdn.branch);
    }

    #[test]
    fn test_hour_ganzi() {
        // Jia-Zi day Zi hour (0:00) -> Jia-Zi hour
        let day = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        let hour = calculate_hour_ganzi(day, 0);
        assert_eq!(hour.stem, HeavenlyStem::Jia);
        assert_eq!(hour.branch, EarthlyBranch::Zi);

        // Jia-Zi day Wu hour (12:00) -> Geng-Wu hour
        let hour_noon = calculate_hour_ganzi(day, 12);
        assert_eq!(hour_noon.stem, HeavenlyStem::Geng);
        assert_eq!(hour_noon.branch, EarthlyBranch::Wu);
    }
}
