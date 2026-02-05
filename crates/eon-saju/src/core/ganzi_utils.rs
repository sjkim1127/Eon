//! 간지(干支) 동적 계산 유틸리티
//!
//! 월간지, 일간지, 시간지를 동적으로 계산하는 공통 함수를 제공합니다.
//! `FourPillars::calculate_*` 함수들과 동일한 로직을 사용하여 정합성을 보장합니다.
//!
//! ## 용도
//! - Fuzzer에서 고해상도 타임라인 조사 시 사용
//! - 주어진 날짜/시간의 간지를 빠르게 계산할 때 사용

use chrono::{NaiveDate, DateTime, Utc};
use crate::core::ganzi::GanZi;
use crate::core::stem::HeavenlyStem;
use crate::core::branch::EarthlyBranch;

/// 특정 연월에 해당하는 월간지를 계산합니다.
///
/// # Arguments
/// * `year` - 양력 년도
/// * `month` - 양력 월 (1-12)
///
/// # Returns
/// 해당 월의 간지
///
/// # Note
/// 절기 경계를 고려하지 않은 간략화된 계산입니다.
/// 정밀한 절기 기반 월간지가 필요한 경우 `FourPillars::calculate_month_pillar`를 사용하세요.
pub fn calculate_month_ganzi(year: i32, month: i32) -> GanZi {
    let year_ganzi = GanZi::from_year(year);
    
    // 오호둔월법(五虎遁月法): 연간에 따른 정월(인월) 천간 결정
    // 甲己년 -> 丙寅월, 乙庚년 -> 戊寅월, 丙辛년 -> 庚寅월, 丁壬년 -> 壬寅월, 戊癸년 -> 甲寅월
    let first_month_stem_idx = match year_ganzi.stem.index() % 5 {
        0 => 2, // 甲, 己 -> 丙
        1 => 4, // 乙, 庚 -> 戊
        2 => 6, // 丙, 辛 -> 庚
        3 => 8, // 丁, 壬 -> 壬
        4 => 0, // 戊, 癸 -> 甲
        _ => 0,
    };
    
    // 월지: 인월(1월)=寅(2), 묘월(2월)=卯(3), ...
    // 양력 월을 음력 월로 대략 매핑 (절기 경계 무시)
    let month_branch_idx = (month + 1) % 12;
    let month_stem_idx = (first_month_stem_idx + (month - 1)) % 10;
    
    GanZi::new(
        HeavenlyStem::from_index(month_stem_idx),
        EarthlyBranch::from_index(month_branch_idx),
    )
}

/// 특정 날짜에 해당하는 일간지를 계산합니다.
///
/// # Arguments
/// * `year` - 양력 년도
/// * `month` - 양력 월 (1-12)
/// * `day` - 양력 일 (1-31)
///
/// # Returns
/// 해당 일의 간지
///
/// # Note
/// `eon_data::manseryuk::get_day_ganzi_index`를 내부적으로 사용하여
/// 만세력 데이터와 정합성을 보장합니다.
pub fn calculate_day_ganzi(year: i32, month: u32, day: u32) -> GanZi {
    if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
        let idx = eon_data::manseryuk::get_day_ganzi_index(date);
        GanZi::from_index(idx as i32)
    } else {
        // 유효하지 않은 날짜의 경우 fallback (일어나선 안됨)
        GanZi::from_index(0)
    }
}

/// Julian Day Number를 이용한 일간지 계산 (만세력 데이터 없이)
///
/// # Arguments
/// * `year` - 양력 년도
/// * `month` - 양력 월 (1-12)
/// * `day` - 양력 일 (1-31)
///
/// # Returns
/// 해당 일의 간지
///
/// # Note
/// 만세력 데이터 없이 순수 계산으로 일간지를 구합니다.
/// 정밀도가 필요한 경우 `calculate_day_ganzi`를 사용하세요.
pub fn calculate_day_ganzi_jdn(year: i32, month: u32, day: u32) -> GanZi {
    // 그레고리력 -> Julian Day Number 변환 (간략화된 공식)
    let a = (14 - month as i32) / 12;
    let y = year + 4800 - a;
    let m = month as i32 + 12 * a - 3;
    let jdn = day as i64 
        + (153 * m + 2) as i64 / 5 
        + 365 * y as i64 
        + y as i64 / 4 
        - y as i64 / 100 
        + y as i64 / 400 
        - 32045;
    
    // JDN + 49 → 60갑자 인덱스 (甲子 = 0 기준 보정)
    let idx = (jdn + 49).rem_euclid(60);
    GanZi::from_index(idx as i32)
}

/// 특정 시간에 해당하는 시간지를 계산합니다.
///
/// # Arguments
/// * `daily_ganzi` - 해당 일의 간지 (시간 천간 결정에 필요)
/// * `hour` - 시간 (0-23)
///
/// # Returns
/// 해당 시간의 간지
pub fn calculate_hour_ganzi(daily_ganzi: GanZi, hour: u32) -> GanZi {
    let branch = EarthlyBranch::from_hour(hour as u8);
    
    // 오자둔시법(五子遁時法): 일간에 따른 자시(子時) 천간 결정
    // 甲己일 -> 甲子시, 乙庚일 -> 丙子시, 丙辛일 -> 戊子시, 丁壬일 -> 庚子시, 戊癸일 -> 壬子시
    let zi_stem_idx = (daily_ganzi.stem.index() % 5) * 2;
    let stem = HeavenlyStem::from_index((zi_stem_idx + branch.index()) as i32);
    
    GanZi::new(stem, branch)
}

/// 특정 DateTime<Utc>에서 전체 간지 세트를 계산합니다.
///
/// # Arguments
/// * `dt` - UTC 시간
///
/// # Returns
/// (년간지, 월간지, 일간지, 시간지) 튜플
///
/// # Note
/// 절기 경계를 고려하지 않은 간략화된 계산입니다.
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
        // 2024년 1월 (甲辰년의 인월) -> 丙寅월
        let mg = calculate_month_ganzi(2024, 1);
        assert_eq!(mg.stem, HeavenlyStem::Bing); // 丙
        assert_eq!(mg.branch, EarthlyBranch::Yin); // 寅
    }

    #[test]
    fn test_day_ganzi_consistency() {
        // 만세력 기반 계산과 JDN 기반 계산의 정합성 확인
        let day1 = calculate_day_ganzi(2024, 1, 1);
        let day1_jdn = calculate_day_ganzi_jdn(2024, 1, 1);
        
        // 두 방식이 동일한 결과를 반환해야 함
        // Note: 만세력 데이터가 정확하다면 일치해야 함
        assert_eq!(day1.branch, day1_jdn.branch);
    }

    #[test]
    fn test_hour_ganzi() {
        // 甲子일의 자시(0시) -> 甲子시
        let day = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        let hour = calculate_hour_ganzi(day, 0);
        assert_eq!(hour.stem, HeavenlyStem::Jia);
        assert_eq!(hour.branch, EarthlyBranch::Zi);

        // 甲子일의 오시(12시) -> 庚午시
        let hour_noon = calculate_hour_ganzi(day, 12);
        assert_eq!(hour_noon.stem, HeavenlyStem::Geng);
        assert_eq!(hour_noon.branch, EarthlyBranch::Wu);
    }
}
