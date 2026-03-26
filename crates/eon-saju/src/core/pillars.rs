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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    /// 경도 (Longitude, 진태양시 계산용, 예: 서울 127.0)
    pub longitude: Option<f64>,
    /// 위도 (Latitude, 조후 보정용)
    pub latitude: Option<f64>,
    /// 시간대 오프셋 (시간 단위, 예: KST = 9.0)
    pub timezone_offset_h: f32,
    /// 야자시(夜子時) 적용 여부
    /// - true: 23:00~24:00를 당일의 야자시로 인정 (일주 유지)
    /// - false: 23:00~24:00를 다음 날의 자시(명자시)로 간주 (일주 변경) - 기본값
    pub use_night_rat_hour: bool,
    /// 성별 (대운 순역 방향 결정용)
    pub gender: eon_core::Gender,
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
            longitude: None,
            latitude: None,
            timezone_offset_h: 9.0, // 기본값 KST
            use_night_rat_hour: false, // 기본적으로 자시=새날 적용
            gender: eon_core::Gender::Male, // 기본값 남성
        }
    }

    /// 경도와 위도를 포함한 입력 생성 (진태양시 보정용)
    pub fn new_solar_at(year: i32, month: u32, day: u32, hour: u32, minute: u32, lon: f64, lat: f64) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            is_lunar: false,
            is_leap_month: false,
            longitude_offset_m: 0,
            longitude: Some(lon),
            latitude: Some(lat),
            timezone_offset_h: 9.0,
            use_night_rat_hour: false,
            gender: eon_core::Gender::Male,
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
            longitude: None,
            latitude: None,
            timezone_offset_h: 9.0, // 기본값 KST
            use_night_rat_hour: false,
            gender: eon_core::Gender::Male,
        }
    }

    /// 인성적인 경도 기반 시차(분) 반환
    /// 한국 표준시(135도) 기준, 서울(127도)은 -8도 차이 -> -32분
    pub fn get_longitude_correction_minutes(&self) -> i32 {
        if let Some(lon) = self.longitude {
            let std_meridian = self.timezone_offset_h as f64 * 15.0;
            ((lon - std_meridian) * 4.0).round() as i32
        } else {
            self.longitude_offset_m
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
            longitude: None,
            latitude: None,
            timezone_offset_h: 9.0, // 기본값 KST
            use_night_rat_hour: false,
            gender: eon_core::Gender::Male,
        }
    }
    
    /// 성별 설정
    pub fn with_gender(mut self, gender: eon_core::Gender) -> Self {
        self.gender = gender;
        self
    }
    
    /// 야자시 옵션 설정
    pub fn with_night_rat_hour(mut self, use_night_rat: bool) -> Self {
        self.use_night_rat_hour = use_night_rat;
        self
    }
}

/// 사주 팔자(四柱八字)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    /// 성별
    pub gender: eon_core::Gender,
    /// 원시 입력 데이터 (운세 재계산용)
    pub raw_input: SajuInput,
    /// 보조 기둥 분석 결과
    pub supplementary_pillars: crate::analysis::supplementary_pillars::SupplementaryPillars,
}

/// 년주 계산용 기준 연도 (4년 = 甲子년)
const GANZI_BASE_YEAR: i32 = 4;

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
        if !(1..=12).contains(&solar_month) {
            return Err(SajuError::InvalidDateTime(format!("Invalid month: {}", solar_month)));
        }

        // 1. 절기 계산용 UTC (보정 없는 실제 시간)
        let dt_absolute_utc = NaiveDate::from_ymd_opt(solar_year, solar_month, solar_day)
            .and_then(|d| d.and_hms_opt(input.hour, input.minute, 0))
            .map(|dt| {
                let offset_ms = (input.timezone_offset_h * 3600.0 * 1000.0) as i64;
                Utc.from_utc_datetime(&(dt - Duration::milliseconds(offset_ms)))
            })
            .ok_or_else(|| SajuError::InvalidDateTime(format!("Absolute DT error: {}-{}-{}", solar_year, solar_month, solar_day)))?;

        // 2. 지역 시차 보정 (True Solar Time 계산) - 일주/시주용
        let lon_correction = input.get_longitude_correction_minutes();
        let (adj_year, adj_month, adj_day, adj_hour, _adj_minute) = if lon_correction != 0 {
            let dt = NaiveDate::from_ymd_opt(solar_year, solar_month, solar_day)
                .and_then(|d| d.and_hms_opt(input.hour, input.minute, 0))
                .ok_or_else(|| SajuError::InvalidDateTime(format!("{}-{}-{} {}:{}", solar_year, solar_month, solar_day, input.hour, input.minute)))?;
            
            let adjusted_dt = dt + Duration::minutes(lon_correction as i64);
            (adjusted_dt.year(), adjusted_dt.month(), adjusted_dt.day(), adjusted_dt.hour(), adjusted_dt.minute())
        } else {
            (solar_year, solar_month, solar_day, input.hour, input.minute)
        };

        // 각 주(Pillar) 계산
        // 연주, 월주는 절대 시간(dt_absolute_utc) 사용
        let year_pillar = Self::calculate_year_pillar(dt_absolute_utc)?;
        let month_pillar = Self::calculate_month_pillar(dt_absolute_utc)?;

        // 일주, 시주는 보정된 날짜/시간(adj_...) 사용 (진태양시 기준)
        // [야자시 처리]
        // use_night_rat_hour == false 이고 시간이 23시 이상이면 다음날로 간주하여 일주 계산
        let (calc_year, calc_month, calc_day) = if !input.use_night_rat_hour && adj_hour >= 23 {
            // 안전한 날짜 생성: 유효하지 않으면 월초로 fallback
            let base_date = NaiveDate::from_ymd_opt(adj_year, adj_month, adj_day)
                .ok_or_else(|| SajuError::InvalidDateTime(format!("Cannot create date: {}-{}-{}", adj_year, adj_month, adj_day)))?;
            let next_day = base_date + Duration::days(1);
            (next_day.year(), next_day.month(), next_day.day())
        } else {
            (adj_year, adj_month, adj_day)
        };

        let day_pillar = Self::calculate_day_pillar(calc_year, calc_month, calc_day)?;
        let hour_pillar = Self::calculate_hour_pillar(&day_pillar, adj_hour);

        Ok(Self {
            year: year_pillar,
            month: month_pillar,
            day: day_pillar,
            hour: hour_pillar,
            birth_time: dt_absolute_utc,
            gender: input.gender,
            raw_input: input.clone(),
            supplementary_pillars: crate::analysis::supplementary_pillars::SupplementaryPillars::calculate_partial(&year_pillar, &month_pillar, &day_pillar, &hour_pillar),
        })
    }

    /// 년주는 입춘(立春)을 기준으로 바뀝니다.
    pub(crate) fn calculate_year_pillar(dt: DateTime<Utc>) -> Result<GanZi, SajuError> {
        use eon_astro::AstroEngine;
        let engine = AstroEngine::new();
        let year = dt.year();
        let month = dt.month();
        
        let sun_long = engine.get_sun_longitude(dt)
            .map_err(|e| SajuError::CalculationError(format!("Sun longitude error: {}", e)))?;
        
        let effective_year = if month <= 2 {
            if month == 1 || sun_long < 315.0 {
                year - 1
            } else {
                year
            }
        } else {
            year
        };

        let idx = (effective_year - GANZI_BASE_YEAR).rem_euclid(60);
        Ok(GanZi::from_index(idx))
    }

    /// 월주 계산
    pub(crate) fn calculate_month_pillar(dt: DateTime<Utc>) -> Result<GanZi, SajuError> {
        use crate::core::calendar::get_month_branch_index_from_dt;
        
        let branch_idx = get_month_branch_index_from_dt(dt);
        let branch = EarthlyBranch::from_index(branch_idx as i32);

        let year_pillar = Self::calculate_year_pillar(dt)?;
        let year_stem = year_pillar.stem;

        let zhi_idx = branch.index();
        let yi_stem_idx = (year_stem.index() as i32 % 5) * 2 + 2;
        let month_stem_idx = (yi_stem_idx + (zhi_idx as i32 - 2).rem_euclid(12)) % 10;
        
        Ok(GanZi::new(
            HeavenlyStem::from_index(month_stem_idx),
            branch
        ))
    }

    /// 일주 계산
    pub(crate) fn calculate_day_pillar(year: i32, month: u32, day: u32) -> Result<GanZi, SajuError> {
        let date = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| SajuError::InvalidDateTime(format!("{}-{}-{}", year, month, day)))?;
        
        let idx = eon_data::manseryuk::get_day_ganzi_index(date);
        Ok(GanZi::from_index(idx as i32))
    }

    /// 시주 계산
    pub(crate) fn calculate_hour_pillar(day_pillar: &GanZi, hour: u32) -> GanZi {
        let branch_idx = hour.div_ceil(2) % 12;
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

#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum SajuError {
    #[error("잘못된 날짜/시간: {0}")]
    InvalidDateTime(String),
    #[error("음력 변환 미지원")]
    LunarNotSupported,
    #[error("계산 오류: {0}")]
    CalculationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year_pillar_basic() {
        let dt = Utc.with_ymd_and_hms(2024, 3, 20, 10, 0, 0).unwrap();
        let pillar = FourPillars::calculate_year_pillar(dt).unwrap();
        assert_eq!(pillar.stem, HeavenlyStem::Jia);
        assert_eq!(pillar.branch, EarthlyBranch::Chen);
    }

    #[test]
    fn test_rat_hour_options() {
        // 2024-03-20 23:30 (야자시/조자시 경계)
        let input_base = SajuInput::new_solar(2024, 3, 20, 23, 30);
        
        // 1. 야자시 미사용 (기본값) -> 다음날 일주 사용
        let pillars_no_night = FourPillars::calculate(&input_base).unwrap();
        // 3월 20일은 癸卯, 21일은 甲辰. 23시 이후이므로 甲辰이 되어야 함.
        assert_eq!(pillars_no_night.day.stem.index(), 0); // 甲
        assert_eq!(pillars_no_night.hour.branch.index(), 0); // 子
        
        // 2. 야자시 사용 -> 오늘 일주 유지
        let input_night = input_base.with_night_rat_hour(true);
        let pillars_night = FourPillars::calculate(&input_night).unwrap();
        assert_eq!(pillars_night.day.stem.index(), 9); // 癸
        assert_eq!(pillars_night.hour.branch.index(), 0); // 子
    }

    #[test]
    fn test_month_pillar_zi_chou() {
        // 2014년 (甲午年) 12월 (丙子月) - 대설(12/7) 이후
        let input = SajuInput::new_solar(2014, 12, 10, 12, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        
        assert_eq!(pillars.year.stem.index(), 0); // 甲
        assert_eq!(pillars.month.stem.index(), 2); // 丙 (Zi month of Jia year is Bing Zi)
        assert_eq!(pillars.month.branch.index(), 0); // 子
        
        // 2015년 새해 ( still 甲午年 until Li Chun) 1월 (丁丑月) - 소한(1/6) 이후
        let input2 = SajuInput::new_solar(2015, 1, 10, 12, 0);
        let pillars2 = FourPillars::calculate(&input2).unwrap();
        
        assert_eq!(pillars2.year.stem.index(), 0); // 甲 (still Jia Wu until Feb)
        assert_eq!(pillars2.month.stem.index(), 3); // 丁 (Chou month of Jia year is Ding Chou)
        assert_eq!(pillars2.month.branch.index(), 1); // 丑
    }
}
