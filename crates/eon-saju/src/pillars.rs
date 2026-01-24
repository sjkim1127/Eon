//! 사주 팔자(四柱八字) 계산
//!
//! 생년월일시로부터 년주, 월주, 일주, 시주를 계산합니다.

use chrono::{Datelike, Timelike};
use serde::{Deserialize, Serialize};
use crate::stem::HeavenlyStem;
use crate::branch::EarthlyBranch;
use crate::ganzi::GanZi;
use crate::element::Element;
use crate::calendar::get_month_branch_index;

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
}

impl FourPillars {
    /// 사주 입력으로부터 사주 계산
    pub fn calculate(input: &SajuInput) -> Result<Self, SajuError> {
        // 음력인 경우 양력으로 변환 필요 (현재는 양력만 지원)
        if input.is_lunar {
            return Err(SajuError::LunarNotSupported);
        }

        // 입력 유효성 검사
        if input.month < 1 || input.month > 12 {
            return Err(SajuError::InvalidDateTime(format!("Invalid month: {}", input.month)));
        }
        if input.day < 1 || input.day > 31 {
            return Err(SajuError::InvalidDateTime(format!("Invalid day: {}", input.day)));
        }
        if input.hour > 23 {
            return Err(SajuError::InvalidDateTime(format!("Invalid hour: {}", input.hour)));
        }

        // 년주 계산
        let year_pillar = Self::calculate_year_pillar(input.year, input.month, input.day);

        // 월주 계산
        let month_pillar = Self::calculate_month_pillar(input.year, input.month, input.day);

        // 일주 계산
        let day_pillar = Self::calculate_day_pillar(input.year, input.month, input.day);

        // 시주 계산
        let hour_pillar = Self::calculate_hour_pillar(&day_pillar, input.hour);

        Ok(Self {
            year: year_pillar,
            month: month_pillar,
            day: day_pillar,
            hour: hour_pillar,
        })
    }

    /// 년주 계산
    /// 
    /// 년주는 입춘을 기준으로 바뀝니다.
    /// 입춘 이전 = 전년도의 년주
    fn calculate_year_pillar(year: i32, month: u32, day: u32) -> GanZi {
        // 입춘 이전인지 확인 (대략 2월 4일)
        let effective_year = if month == 1 || (month == 2 && day < 4) {
            year - 1
        } else {
            year
        };

        // 년주 공식: (연도 - 4) mod 60 = 60갑자 인덱스
        // 서기 4년이 갑자년
        let idx = (effective_year - 4).rem_euclid(60);
        GanZi::from_index(idx)
    }

    /// 월주 계산
    /// 
    /// 월주의 지지는 절기로 결정되고, 천간은 년간에 따라 결정됩니다.
    fn calculate_month_pillar(year: i32, month: u32, day: u32) -> GanZi {
        // 월지: 절기 기준
        let branch_idx = get_month_branch_index(year, month, day);
        let branch = EarthlyBranch::from_index(branch_idx as i32);

        // 년주의 천간 구하기 (월간 결정에 필요)
        let year_pillar = Self::calculate_year_pillar(year, month, day);
        let year_stem = year_pillar.stem;

        // 월간 결정 (년간에 따른 월간 시작점)
        // 갑기년 → 병寅월 시작 (丙=2)
        // 을경년 → 무寅월 시작 (戊=4)
        // 병신년 → 경寅월 시작 (庚=6)
        // 정임년 → 임寅월 시작 (壬=8)
        // 무계년 → 갑寅월 시작 (甲=0)
        let yin_stem_idx = match year_stem.index() % 5 {
            0 => 2, // 甲, 己 → 丙
            1 => 4, // 乙, 庚 → 戊
            2 => 6, // 丙, 辛 → 庚
            3 => 8, // 丁, 壬 → 壬
            4 => 0, // 戊, 癸 → 甲
            _ => unreachable!(),
        };

        // 월간 계산: 寅월(2)부터의 거리 + 寅월 천간
        let month_offset = (branch_idx as i32 - 2).rem_euclid(12);
        let stem = HeavenlyStem::from_index(yin_stem_idx + month_offset);

        GanZi::new(stem, branch)
    }

    /// 일주 계산
    /// 
    /// 율리우스 적일(Julian Day Number)을 사용하여 계산합니다.
    fn calculate_day_pillar(year: i32, month: u32, day: u32) -> GanZi {
        // 율리우스 적일 계산
        let jdn = Self::gregorian_to_jdn(year, month as i32, day as i32);

        // 기준점: 1900년 1월 1일 = 甲辰일 (JDN: 2415021)
        // 또는: 서기 1년 1월 1일의 간지를 기준으로
        // 2000년 1월 1일 = 戊午일 (JDN: 2451545)
        
        // 60갑자 인덱스 계산
        // JDN 0일 = 기원전 4713년 1월 1일 (율리우스력) = 甲寅일(index=50)이라는 설이 있음
        // 하지만 일반적으로 사용하는 공식: (JDN + 10) mod 60 또는 (JDN + 49) mod 60
        
        // 2000년 1월 1일 = JDN 2451545 = 戊午일(index=54)
        // (2451545 + 49) mod 60 = 54 ✓
        let idx = (jdn + 49).rem_euclid(60);
        GanZi::from_index(idx as i32)
    }

    /// 시주 계산
    /// 
    /// 시지는 시간으로, 시간은 일간에 따라 결정됩니다.
    fn calculate_hour_pillar(day_pillar: &GanZi, hour: u32) -> GanZi {
        // 시지: 시간으로 결정
        let branch = EarthlyBranch::from_hour(hour as u8);

        // 시간 결정 (일간에 따른 시간 시작점)
        // 갑기일 → 갑子시 시작 (甲=0)
        // 을경일 → 병子시 시작 (丙=2)
        // 병신일 → 무子시 시작 (戊=4)
        // 정임일 → 경子시 시작 (庚=6)
        // 무계일 → 임子시 시작 (壬=8)
        let zi_stem_idx = (day_pillar.stem.index() % 5) * 2;
        
        // 시간 계산: 子시(0)부터의 거리 + 子시 천간
        let hour_offset = branch.index();
        let stem = HeavenlyStem::from_index((zi_stem_idx + hour_offset) as i32);

        GanZi::new(stem, branch)
    }

    /// 그레고리력 날짜를 율리우스 적일(JDN)로 변환
    fn gregorian_to_jdn(year: i32, month: i32, day: i32) -> i64 {
        let a = (14 - month) / 12;
        let y = year + 4800 - a;
        let m = month + 12 * a - 3;

        // 그레고리력 JDN 공식
        let jdn = day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;
        jdn as i64
    }

    /// 오행 분석: 각 오행별 개수 반환
    pub fn element_counts(&self) -> [(Element, u32); 5] {
        let mut counts = [
            (Element::Wood, 0),
            (Element::Fire, 0),
            (Element::Earth, 0),
            (Element::Metal, 0),
            (Element::Water, 0),
        ];

        // 천간 오행 카운트
        for pillar in [&self.year, &self.month, &self.day, &self.hour] {
            let idx = pillar.stem.element().index() as usize;
            counts[idx].1 += 1;

            // 지지 오행도 카운트 (정기 기준)
            let branch_idx = pillar.branch.element().index() as usize;
            counts[branch_idx].1 += 1;
        }

        counts
    }

    /// 일간(日干) 반환 - 사주 분석의 중심
    #[inline]
    pub fn day_master(&self) -> HeavenlyStem {
        self.day.stem
    }

    /// 일주의 오행
    #[inline]
    pub fn day_master_element(&self) -> Element {
        self.day.stem.element()
    }

    /// 한자 표기
    pub fn hanja(&self) -> String {
        format!(
            "{}年 {}月 {}日 {}時",
            self.year, self.month, self.day, self.hour
        )
    }

    /// 한글 표기
    pub fn hangul(&self) -> String {
        format!(
            "{}년 {}월 {}일 {}시",
            self.year.hangul(),
            self.month.hangul(),
            self.day.hangul(),
            self.hour.hangul()
        )
    }
}

impl std::fmt::Display for FourPillars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┌────┬────┬────┬────┐")?;
        writeln!(f, "│ 時 │ 日 │ 月 │ 年 │")?;
        writeln!(f, "├────┼────┼────┼────┤")?;
        writeln!(f, "│ {} │ {} │ {} │ {} │",
            self.hour.stem.hanja(),
            self.day.stem.hanja(),
            self.month.stem.hanja(),
            self.year.stem.hanja()
        )?;
        writeln!(f, "│ {} │ {} │ {} │ {} │",
            self.hour.branch.hanja(),
            self.day.branch.hanja(),
            self.month.branch.hanja(),
            self.year.branch.hanja()
        )?;
        writeln!(f, "└────┴────┴────┴────┘")
    }
}

/// 사주 계산 에러
#[derive(Debug, Clone)]
pub enum SajuError {
    /// 잘못된 날짜/시간
    InvalidDateTime(String),
    /// 음력 변환 미지원
    LunarNotSupported,
    /// 계산 오류
    CalculationError(String),
}

impl std::fmt::Display for SajuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SajuError::InvalidDateTime(msg) => write!(f, "Invalid datetime: {}", msg),
            SajuError::LunarNotSupported => write!(f, "Lunar calendar conversion not yet supported"),
            SajuError::CalculationError(msg) => write!(f, "Calculation error: {}", msg),
        }
    }
}

impl std::error::Error for SajuError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year_pillar_basic() {
        // 2024년 3월 1일 (입춘 이후) = 갑진년
        let pillar = FourPillars::calculate_year_pillar(2024, 3, 1);
        assert_eq!(pillar.stem, HeavenlyStem::Jia);
        assert_eq!(pillar.branch, EarthlyBranch::Chen);
    }

    #[test]
    fn test_year_pillar_before_lichun() {
        // 2024년 1월 15일 (입춘 이전) = 계묘년 (2023년의 년주)
        let pillar = FourPillars::calculate_year_pillar(2024, 1, 15);
        assert_eq!(pillar.stem, HeavenlyStem::Gui);
        assert_eq!(pillar.branch, EarthlyBranch::Mao);
    }

    #[test]
    fn test_day_pillar_reference() {
        // 2000년 1월 1일 = 戊午일
        let pillar = FourPillars::calculate_day_pillar(2000, 1, 1);
        assert_eq!(pillar.stem, HeavenlyStem::Wu);
        assert_eq!(pillar.branch, EarthlyBranch::Wu);
    }

    #[test]
    fn test_hour_pillar() {
        let day = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        
        // 갑일 자시 = 갑자시
        let hour = FourPillars::calculate_hour_pillar(&day, 0);
        assert_eq!(hour.stem, HeavenlyStem::Jia);
        assert_eq!(hour.branch, EarthlyBranch::Zi);

        // 갑일 오시(12시) = 경오시
        let hour = FourPillars::calculate_hour_pillar(&day, 12);
        assert_eq!(hour.stem, HeavenlyStem::Geng);
        assert_eq!(hour.branch, EarthlyBranch::Wu);
    }

    #[test]
    fn test_full_calculation() {
        let input = SajuInput::new_solar(1990, 5, 15, 14, 30);
        let pillars = FourPillars::calculate(&input).unwrap();
        
        // 결과 출력 (디버그용)
        println!("{}", pillars);
        println!("일간: {}", pillars.day_master());
    }

    #[test]
    fn test_four_pillars_display() {
        let input = SajuInput::new_solar(2024, 3, 20, 10, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        
        let display = format!("{}", pillars);
        assert!(display.contains("時"));
        assert!(display.contains("日"));
        assert!(display.contains("月"));
        assert!(display.contains("年"));
    }
}
