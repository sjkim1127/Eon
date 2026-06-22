//! 음력 변환 — eon-data의 만세력 캐시 래퍼
//!
//! `eon-saju`의 `LunarCalendar::from_solar()`를 재사용하여
//! 자미두수 계산에 필요한 음력 데이터를 추출합니다.

use chrono::NaiveDate;
use crate::error::ZwdsError;
use crate::types::LunarBirthInfo;

// eon-data의 만세력 모듈 재사용
use eon_data::LunarCalendar;

/// 자미두수 좌표계 기준 지지 인덱스 (寅=0)
/// 표준 지지 순서(子=0)에서 변환
///
/// 표준: 子=0, 丑=1, 寅=2, 卯=3, 辰=4, 巳=5, 午=6, 未=7, 申=8, 酉=9, 戌=10, 亥=11
/// ZWDS: 寅=0, 卯=1, 辰=2, 巳=3, 午=4, 未=5, 申=6, 酉=7, 戌=8, 亥=9, 子=10, 丑=11
pub fn branch_to_zwds_idx(standard_branch_idx: usize) -> usize {
    // 표준 지지에서 寅(2)을 기준으로 shift
    (standard_branch_idx + 10) % 12
}

/// 표준 지지 인덱스(子=0)에서 시간(0-23)으로 시지(時支) 계산
/// 자시(子時): 23:00-00:59
pub fn hour_to_time_branch_idx(hour: u32) -> usize {
    // 자시: 23시 또는 0시 → 子(0)
    let adjusted = (hour + 1) % 24;
    (adjusted / 2) as usize
}

/// 양력 생년월일시 → 자미두수용 음력 정보 변환
pub fn solar_to_lunar_birth(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
) -> Result<LunarBirthInfo, ZwdsError> {
    let date = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| ZwdsError::InvalidBirthDate(format!("{}-{}-{}", year, month, day)))?;

    let (lunar_year, lunar_month, lunar_day, is_leap) = LunarCalendar::from_solar(date)
        .map_err(|e| ZwdsError::LunarConversion(e.to_string()))?;

    // 연간 계산: 60갑자 기반
    // 갑자년(甲子): 4, 즉 (year - 4) % 10 으로 천간 인덱스 산출
    let year_stem_idx = ((lunar_year - 4).rem_euclid(10)) as usize;

    // 연지: (year - 4) % 12 → 子=0 기준
    let year_branch_std_idx = ((lunar_year - 4).rem_euclid(12)) as usize;

    // 시지 (子=0 표준 기준)
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

/// 음력 직접 입력 시 정보 조합 (윤달 지원)
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
        // 2004-11-27 → 음력 2004-10-16
        let info = solar_to_lunar_birth(2004, 11, 27, 22).unwrap();
        assert_eq!(info.year, 2004);
        assert_eq!(info.month, 10);
        assert_eq!(info.day, 16);
        assert!(!info.is_leap_month);
    }

    #[test]
    fn test_hour_to_time_branch() {
        // 23시 → 子時(0)
        assert_eq!(hour_to_time_branch_idx(23), 0);
        // 0시 → 子時(0)
        assert_eq!(hour_to_time_branch_idx(0), 0);
        // 22시 → 亥時(11)
        assert_eq!(hour_to_time_branch_idx(22), 11);
        // 1시 → 丑時(1)
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
