//! 사주 팔자(四柱八字) 계산
//!
//! 생년월일시로부터 년주, 월주, 일주, 시주를 계산합니다.

use serde::{Deserialize, Serialize};
use crate::core::stem::HeavenlyStem;
use crate::core::branch::EarthlyBranch;
use crate::core::ganzi::GanZi;
use crate::core::element::Element;
use chrono::{DateTime, Utc, Datelike, NaiveDate, Duration, TimeZone, Timelike};

/// 사주 계산 입력
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SajuInput {
    /// 년도 (양력)
    pub year: i32,
    /// 월 (1-12)
    pub month: u32,
    /// 일 (1-31)
    pub day: u32,
    /// 시 (0-23)
    pub hour: u32,
    /// 분 (0-59, 선택적)
    pub minute: u32,
    /// 음력 여부
    pub is_lunar: bool,
    /// 윤달 여부 (음력인 경우에만 유효)
    pub is_leap_month: bool,
    /// 경도 시차 (분 단위, 예: 안산 -33)
    pub longitude_offset_m: i32,
}

impl SajuInput {
    /// 양력 생년월일시로 입력 생성
    pub fn new_solar(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            is_lunar: false,
            is_leap_month: false,
            longitude_offset_m: 0,
        }
    }

    /// 지역 시차를 포함한 양력 입력 생성
    pub fn new_solar_with_offset(year: i32, month: u32, day: u32, hour: u32, minute: u32, offset_m: i32) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            is_lunar: false,
            is_leap_month: false,
            longitude_offset_m: offset_m,
        }
    }

    /// 음력 생년월일시로 입력 생성
    pub fn new_lunar(year: i32, month: u32, day: u32, hour: u32, minute: u32, is_leap: bool) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            is_lunar: true,
            is_leap_month: is_leap,
            longitude_offset_m: 0,
        }
    }
}

/// 사주 팔자(四柱八字)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FourPillars {
    /// 년주(年柱)
    pub year: GanZi,
    /// 월주(月柱)
    pub month: GanZi,
    /// 일주(日柱)
    pub day: GanZi,
    /// 시주(時柱)
    pub hour: GanZi,
    /// 기준 시각 (UTC)
    pub birth_time: DateTime<Utc>,
}

impl FourPillars {
    /// 사주 입력으로부터 사주 계산
    pub fn calculate(input: &SajuInput) -> Result<Self, SajuError> {
        // 음력인 경우 양력으로 변환
        let (solar_year, solar_month, solar_day) = if input.is_lunar {
            use eon_data::LunarCalendar;
            let solar_date = LunarCalendar::to_solar(input.year, input.month, input.day, input.is_leap_month)
                .ok_or_else(|| SajuError::CalculationError("음력 변환에 실패했습니다.".to_string()))?;
            (solar_date.year(), solar_date.month(), solar_date.day())
        } else {
            (input.year, input.month, input.day)
        };

        // 입력 유효성 검사 (변환된 양력 기준)
        if solar_month < 1 || solar_month > 12 {
            return Err(SajuError::InvalidDateTime(format!("Invalid month: {}", solar_month)));
        }

        // 1. 절기 계산용 UTC (보정 없는 실제 시간)
        let dt_absolute_utc = NaiveDate::from_ymd_opt(solar_year, solar_month, solar_day)
            .and_then(|d| d.and_hms_opt(input.hour, input.minute, 0))
            .map(|dt| Utc.from_utc_datetime(&(dt - Duration::hours(9)))) // KST 9시간 차이 가정
            .ok_or_else(|| SajuError::InvalidDateTime(format!("Absolute DT error: {}-{}-{}", solar_year, solar_month, solar_day)))?;

        // 2. 지역 시차 보정 (True Solar Time 계산) - 일주/시주용
        let (adj_year, adj_month, adj_day, adj_hour, _adj_minute) = if input.longitude_offset_m != 0 {
            let dt = NaiveDate::from_ymd_opt(solar_year, solar_month, solar_day)
                .and_then(|d| d.and_hms_opt(input.hour, input.minute, 0))
                .ok_or_else(|| SajuError::InvalidDateTime(format!("{}-{}-{} {}:{}", solar_year, solar_month, solar_day, input.hour, input.minute)))?;
            
            let adjusted_dt = dt + Duration::minutes(input.longitude_offset_m as i64);
            (adjusted_dt.year(), adjusted_dt.month(), adjusted_dt.day(), adjusted_dt.hour(), adjusted_dt.minute())
        } else {
            (solar_year, solar_month, solar_day, input.hour, input.minute)
        };

        // 각 주(Pillar) 계산
        // 연주, 월주는 절대 시간(dt_absolute_utc) 사용
        let year_pillar = Self::calculate_year_pillar(dt_absolute_utc);
        let month_pillar = Self::calculate_month_pillar(dt_absolute_utc);

        // 일주, 시주는 보정된 날짜/시간(adj_...) 사용 (진태양시 기준)
        let day_pillar = Self::calculate_day_pillar(adj_year, adj_month, adj_day);
        let hour_pillar = Self::calculate_hour_pillar(&day_pillar, adj_hour);

        Ok(Self {
            year: year_pillar,
            month: month_pillar,
            day: day_pillar,
            hour: hour_pillar,
            birth_time: dt_absolute_utc,
        })
    }

    /// 년주는 입춘(立春)을 기준으로 바뀝니다.
    pub(crate) fn calculate_year_pillar(dt: DateTime<Utc>) -> GanZi {
        use eon_astro::AstroEngine;
        let engine = AstroEngine::new();
        let year = dt.year();
        let month = dt.month();
        
        let sun_long = engine.get_sun_longitude(dt).unwrap_or(0.0);
        
        let effective_year = if month <= 2 {
            if month == 1 || sun_long < 315.0 {
                year - 1
            } else {
                year
            }
        } else {
            year
        };

        let idx = (effective_year - 4).rem_euclid(60);
        GanZi::from_index(idx)
    }

    /// 월주 계산
    pub(crate) fn calculate_month_pillar(dt: DateTime<Utc>) -> GanZi {
        use crate::core::calendar::get_month_branch_index_from_dt;
        
        let branch_idx = get_month_branch_index_from_dt(dt);
        let branch = EarthlyBranch::from_index(branch_idx as i32);

        let year_pillar = Self::calculate_year_pillar(dt);
        let year_stem = year_pillar.stem;

        let zhi_idx = branch.index();
        let yi_stem_idx = (year_stem.index() as i32 % 5) * 2 + 2;
        let month_stem_idx = (yi_stem_idx + (zhi_idx as i32 - 2)) % 10;
        
        GanZi::new(
            HeavenlyStem::from_index(month_stem_idx),
            branch
        )
    }

    /// 일주 계산
    pub(crate) fn calculate_day_pillar(year: i32, month: u32, day: u32) -> GanZi {
        let y = if month <= 2 { year - 1 } else { year };
        let m = if month <= 2 { month + 12 } else { month };
        
        let d = day as i32;
        
        let jd = (365.25 * (y + 4716) as f64) as i32 
               + (30.6001 * (m + 1) as f64) as i32 
               + d + 2 - (y / 100) + (y / 400) - 1524;
        
        let idx = (jd - 11) % 60;
        let final_idx = if idx < 0 { idx + 60 } else { idx };
        
        GanZi::from_index(final_idx)
    }

    /// 시주 계산
    pub(crate) fn calculate_hour_pillar(day_pillar: &GanZi, hour: u32) -> GanZi {
        let branch_idx = ((hour + 1) / 2) % 12;
        let branch = EarthlyBranch::from_index(branch_idx as i32);

        let day_stem = day_pillar.stem;
        let zi_stem_idx = (day_stem.index() as i32 % 5) * 2;
        let hour_stem_idx = (zi_stem_idx + branch.index() as i32) % 10;

        GanZi::new(
            HeavenlyStem::from_index(hour_stem_idx),
            branch
        )
    }

    /// 오행 분포 계산
    pub fn element_distribution(&self) -> [(Element, u32); 5] {
        let elements = [
            self.year.stem.element(), self.year.branch.element(),
            self.month.stem.element(), self.month.branch.element(),
            self.day.stem.element(), self.day.branch.element(),
            self.hour.stem.element(), self.hour.branch.element(),
        ];
        
        let mut counts = [0u32; 5];
        for el in elements {
            counts[el as usize] += 1;
        }
        
        [
            (Element::Wood, counts[0]),
            (Element::Fire, counts[1]),
            (Element::Earth, counts[2]),
            (Element::Metal, counts[3]),
            (Element::Water, counts[4]),
        ]
    }

    /// 일간(日干) 반환
    pub fn day_master(&self) -> HeavenlyStem {
        self.day.stem
    }

    /// 일간(日干) 오행 반환
    pub fn day_master_element(&self) -> Element {
        self.day.stem.element()
    }

    /// 오행 개수 (Alias of element_distribution)
    pub fn element_counts(&self) -> [(Element, u32); 5] {
        self.element_distribution()
    }

    /// 한자 표기
    pub fn hanja(&self) -> String {
        format!("{} {} {} {}", 
            self.hour.hanja(), self.day.hanja(), self.month.hanja(), self.year.hanja())
    }

    /// 한글 표기
    pub fn hangul(&self) -> String {
        format!("{} {} {} {}", 
            self.hour.hangul(), self.day.hangul(), self.month.hangul(), self.year.hangul())
    }
}

impl std::fmt::Display for FourPillars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    時    日    月    年")?;
        writeln!(f, "  {:>2}   {:>2}   {:>2}   {:>2}", 
            self.hour.stem.hanja(), self.day.stem.hanja(), self.month.stem.hanja(), self.year.hanja())?;
        writeln!(f, "  ---   ---   ---   ---")?;
        writeln!(f, "  {}   {}   {}   {}", 
            self.hour.hangul(), self.day.hangul(), self.month.hangul(), self.year.hangul())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum SajuError {
    InvalidDateTime(String),
    LunarNotSupported,
    CalculationError(String),
}

impl std::fmt::Display for SajuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDateTime(msg) => write!(f, "잘못된 날짜/시간: {}", msg),
            Self::LunarNotSupported => write!(f, "음력 변환 미지원"),
            Self::CalculationError(msg) => write!(f, "계산 오류: {}", msg),
        }
    }
}

impl std::error::Error for SajuError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year_pillar_basic() {
        let dt = Utc.with_ymd_and_hms(2024, 3, 20, 10, 0, 0).unwrap();
        let pillar = FourPillars::calculate_year_pillar(dt);
        assert_eq!(pillar.stem, HeavenlyStem::Jia);
        assert_eq!(pillar.branch, EarthlyBranch::Chen);
    }
}
