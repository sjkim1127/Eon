//! 대운(大運, Major Luck Cycles) 계산
//!
//! 10년 단위의 운세 흐름을 계산합니다.
//!
//! ## 대운 계산 원리
//!
//! 1. **순행/역행** 결정
//!    - 양년생 남자 or 음년생 여자 → 순행 (월주에서 앞으로)
//!    - 음년생 남자 or 양년생 여자 → 역행 (월주에서 뒤로)
//!
//! 2. **대운 시작 나이** 계산
//!    - 생일부터 가장 가까운 절기까지의 일수 ÷ 3 = 시작 나이
//!    - 순행: 다음 절기까지의 거리
//!    - 역행: 이전 절기까지의 거리
//!
//! 3. **대운 간지** 계산
//!    - 월주를 기준으로 순행 또는 역행하여 간지 결정

use serde::{Deserialize, Serialize};
use eon_core::Gender;
use crate::core::stem::HeavenlyStem;
use crate::core::element::Polarity;
use crate::core::ganzi::GanZi;
use crate::core::pillars::FourPillars;
use crate::core::ten_gods::TenGod;

/// 대운 진행 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LuckDirection {
    /// 순행 (월주에서 앞으로)
    Forward,
    /// 역행 (월주에서 뒤로)
    Reverse,
}

impl LuckDirection {
    /// 년간과 성별로 대운 방향 결정
    /// 
    /// - 양년생 남자 or 음년생 여자 → 순행
    /// - 음년생 남자 or 양년생 여자 → 역행
    pub fn from_year_and_gender(year_stem: HeavenlyStem, gender: Gender) -> Self {
        let is_yang_year = year_stem.polarity() == Polarity::Yang;
        let is_male = gender.is_male();

        if (is_yang_year && is_male) || (!is_yang_year && !is_male) {
            Self::Forward
        } else {
            Self::Reverse
        }
    }
}

impl std::fmt::Display for LuckDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Forward => write!(f, "순행"),
            Self::Reverse => write!(f, "역행"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorLuck {
    /// 대운 간지
    pub ganzi: GanZi,
    /// 시작 나이
    pub start_age: u32,
    /// 종료 나이
    pub end_age: u32,
    /// 천간 십성
    pub stem_god: TenGod,
    /// 지지 십성 (정기 기준)
    pub branch_god: TenGod,
    /// 실제 대운 시작 날짜 (초정밀 교운기)
    pub start_date: chrono::DateTime<chrono::Utc>,
}

impl MajorLuck {
    /// 새 대운 생성
    pub fn new(
        ganzi: GanZi, 
        start_age: u32, 
        day_master: HeavenlyStem,
        start_date: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            ganzi,
            start_age,
            end_age: start_age + 9, // 10년 주기
            stem_god: TenGod::from_stems(day_master, ganzi.stem),
            branch_god: TenGod::from_stem_and_branch(day_master, ganzi.branch),
            start_date,
        }
    }

    /// 특정 나이가 이 대운에 해당하는지 확인
    pub fn contains_age(&self, age: u32) -> bool {
        age >= self.start_age && age <= self.end_age
    }
}

impl std::fmt::Display for MajorLuck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:2}세~{:2}세: {} ({}/{}) | 시작: {}",
            self.start_age, self.end_age,
            self.ganzi,
            self.stem_god.hangul(),
            self.branch_god.hangul(),
            self.start_date.format("%Y-%m-%d %H:%M")
        )
    }
}

/// 대운 분석 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorLuckAnalysis {
    /// 대운 방향
    pub direction: LuckDirection,
    /// 대운 시작 나이 (세)
    pub start_age: u32,
    /// 대운 시작 개월 (0~11)
    pub start_months: u32,
    /// 대운 시작 일 (0~30)
    pub start_days: u32,
    /// 실제 대운 시작 날짜 (교운기 연월일시)
    pub start_date: chrono::DateTime<chrono::Utc>,
    /// 대운 목록 (보통 8~10개)
    pub cycles: Vec<MajorLuck>,
    /// 일간 (기준)
    pub day_master: HeavenlyStem,
}

impl MajorLuckAnalysis {
    /// 천문 엔진을 사용한 대운 분석 생성
    pub fn calculate_astro(
        pillars: &FourPillars,
        gender: Gender,
        birth_year: i32,
        birth_month: u32,
        birth_day: u32,
        birth_hour: u32,
        birth_min: u32,
    ) -> Self {
        use eon_astro::AstroEngine;
        use chrono::{TimeZone, Utc};

        let day_master = pillars.day_master();
        let direction = LuckDirection::from_year_and_gender(pillars.year.stem, gender);
        let astro = AstroEngine::new();
        
        let birth_time = Utc.with_ymd_and_hms(birth_year, birth_month, birth_day, birth_hour, birth_min, 0).unwrap();
        
        // 현재 24절기 인덱스 확인 (0: 입춘 ~ 23: 대한)
        let term_idx = astro.get_solar_term_index(birth_time);
        
        // 대운은 '절기(Jieqi, 짝수 인덱스)'를 기준으로 함
        let target_term_idx = match direction {
            LuckDirection::Forward => {
                // 순행: 다음 절입 시각 (현재보다 큰 가장 가까운 짝수 인덱스)
                if term_idx % 2 == 0 { (term_idx + 2) % 24 } else { (term_idx + 1) % 24 }
            },
            LuckDirection::Reverse => {
                // 역행: 이전 절입 시각 (현재 또는 이전의 가장 가까운 짝수 인덱스)
                if term_idx % 2 == 0 { term_idx } else { term_idx - 1 }
            }
        };

        let target_term_time = astro.find_solar_term_time(birth_time, target_term_idx).unwrap();

        // 대운 시작 정밀 계산 (년, 월, 일 단위)
        let (start_age, start_months, start_days) = Self::calculate_precise_start_with_times(
            birth_time, target_term_time, direction
        );
        
        // 실제 대운 시작 날짜 (교운기 확정)
        // 3일 = 1년 법칙: (초)로 환산하면 (10일 diff = 1217일 실제 시간)
        // diff_seconds * (365.2425 / 3)
        let diff = if direction == LuckDirection::Forward {
            target_term_time - birth_time
        } else {
            birth_time - target_term_time
        };
        let offset_seconds = (diff.num_seconds() as f64 * (365.2425 / 3.0)) as i64;
        let start_date = birth_time + chrono::Duration::seconds(offset_seconds);

        // 대운 간지 계산 (월주 기준으로 순행/역행)
        let mut cycles = Vec::new();
        let mut current_ganzi = pillars.month;
        
        for i in 0..10 {
            let age = start_age + (i * 10);
            // 각 대운은 이전 대운 + 10년 (대략 365.2425 * 10일)
            let cycle_start_date = start_date + chrono::Duration::seconds((i as f64 * 10.0 * 365.2425 * 86400.0) as i64);
            
            current_ganzi = match direction {
                LuckDirection::Forward => current_ganzi.next(),
                LuckDirection::Reverse => current_ganzi.prev(),
            };
            cycles.push(MajorLuck::new(current_ganzi, age, day_master, cycle_start_date));
        }

        Self {
            direction,
            start_age,
            start_months,
            start_days,
            start_date,
            cycles,
            day_master,
        }
    }

    pub fn calculate(
        pillars: &FourPillars,
        gender: Gender,
        birth_year: i32,
        birth_month: u32,
        birth_day: u32,
        birth_hour: u32,
        birth_min: u32,
        term_year: i32,
        term_month: u32,
        term_day: u32,
        term_hour: u32,
        term_min: u32,
    ) -> Self {
        let day_master = pillars.day_master();
        let direction = LuckDirection::from_year_and_gender(pillars.year.stem, gender);
        
        use chrono::{TimeZone, Utc};
        let birth_time = Utc.with_ymd_and_hms(birth_year, birth_month, birth_day, birth_hour, birth_min, 0).unwrap();
        let term_time = Utc.with_ymd_and_hms(term_year, term_month, term_day, term_hour, term_min, 0).unwrap();

        // 대운 시작 정밀 계산 (년, 월, 일 단위)
        let (start_age, start_months, start_days) = Self::calculate_precise_start_with_times(
            birth_time, term_time, direction
        );
        
        let diff = if direction == LuckDirection::Forward {
            term_time - birth_time
        } else {
            birth_time - term_time
        };
        let offset_seconds = (diff.num_seconds() as f64 * (365.2425 / 3.0)) as i64;
        let start_date = birth_time + chrono::Duration::seconds(offset_seconds);

        // 대운 간지 계산 (월주 기준으로 순행/역행)
        let mut cycles = Vec::new();
        let mut current_ganzi = pillars.month;
        
        for i in 0..10 {
            let age = start_age + (i * 10);
            let cycle_start_date = start_date + chrono::Duration::seconds((i as f64 * 10.0 * 365.2425 * 86400.0) as i64);

            current_ganzi = match direction {
                LuckDirection::Forward => current_ganzi.next(),
                LuckDirection::Reverse => current_ganzi.prev(),
            };
            
            cycles.push(MajorLuck::new(current_ganzi, age, day_master, cycle_start_date));
        }

        Self {
            direction,
            start_age,
            start_months,
            start_days,
            start_date,
            cycles,
            day_master,
        }
    }

    /// DateTime을 인자로 받는 대운 시작 정밀 계산
    fn calculate_precise_start_with_times(
        birth_time: chrono::DateTime<chrono::Utc>,
        term_time: chrono::DateTime<chrono::Utc>,
        direction: LuckDirection
    ) -> (u32, u32, u32) {
        let diff = if direction == LuckDirection::Forward {
            term_time - birth_time
        } else {
            birth_time - term_time
        };

        let total_minutes = diff.num_minutes().abs();

        // 3일(4320분) = 1년
        let years = total_minutes / 4320;
        let rem_years = total_minutes % 4320;

        // 6시간(360분) = 1개월
        let months = rem_years / 360;
        let rem_months = rem_years % 360;

        // 12분 = 1일
        let days = rem_months / 12;

        (years as u32, months as u32, days as u32)
    }

    /// 특정 나이의 대운 조회
    pub fn at_age(&self, age: u32) -> Option<&MajorLuck> {
        self.cycles.iter().find(|luck| luck.contains_age(age))
    }

    /// 현재 나이 기준 대운 조회
    pub fn current(&self, current_age: u32) -> Option<&MajorLuck> {
        self.at_age(current_age)
    }
}

impl std::fmt::Display for MajorLuckAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "대운 ({}, {}년 {}개월 {}일 뒤 시작)", 
            self.direction, self.start_age, self.start_months, self.start_days)?;
        writeln!(f, "──────────────────────────────────────")?;
        
        for (i, luck) in self.cycles.iter().enumerate() {
            write!(f, "{:2}. ", i + 1)?;
            writeln!(f, "{}", luck)?;
        }
        
        Ok(())
    }
}

// ============================================
// 편의 함수
// ============================================

impl FourPillars {
    /// 대운 분석 (천문 엔진 사용)
    pub fn major_luck(
        &self, 
        gender: Gender, 
        b_year: i32, b_month: u32, b_day: u32, b_hour: u32, b_min: u32,
    ) -> MajorLuckAnalysis {
        MajorLuckAnalysis::calculate_astro(self, gender, b_year, b_month, b_day, b_hour, b_min)
    }

    /// 대운 분석 (특정 절입 시각 지정)
    pub fn major_luck_precise(
        &self, 
        gender: Gender, 
        b_year: i32, b_month: u32, b_day: u32, b_hour: u32, b_min: u32,
        t_year: i32, t_month: u32, t_day: u32, t_hour: u32, t_min: u32
    ) -> MajorLuckAnalysis {
        MajorLuckAnalysis::calculate(self, gender, b_year, b_month, b_day, b_hour, b_min, t_year, t_month, t_day, t_hour, t_min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pillars::SajuInput;

    #[test]
    fn test_luck_direction_yang_male() {
        // 甲(양목) + 남자 = 순행
        let direction = LuckDirection::from_year_and_gender(HeavenlyStem::Jia, Gender::Male);
        assert_eq!(direction, LuckDirection::Forward);
    }

    #[test]
    fn test_luck_direction_yang_female() {
        // 甲(양목) + 여자 = 역행
        let direction = LuckDirection::from_year_and_gender(HeavenlyStem::Jia, Gender::Female);
        assert_eq!(direction, LuckDirection::Reverse);
    }

    #[test]
    fn test_luck_direction_yin_male() {
        // 乙(음목) + 남자 = 역행
        let direction = LuckDirection::from_year_and_gender(HeavenlyStem::Yi, Gender::Male);
        assert_eq!(direction, LuckDirection::Reverse);
    }

    #[test]
    fn test_luck_direction_yin_female() {
        // 乙(음목) + 여자 = 순행
        let direction = LuckDirection::from_year_and_gender(HeavenlyStem::Yi, Gender::Female);
        assert_eq!(direction, LuckDirection::Forward);
    }

    #[test]
    fn test_user_major_luck() {
        // 김성주님 사주: 甲申年 乙亥月 庚戌日 丁亥時 (남자)
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        
        // 甲(양목) + 남자 = 순행 (대설까지의 거리 계산)
        let luck = pillars.major_luck_precise(
            Gender::Male, 
            2004, 11, 27, 22, 0, // 출생
            2004, 12, 7, 3, 48   // 대설
        );
        
        assert_eq!(luck.direction, LuckDirection::Forward);
        
        println!("=== 김성주님 대운 ===");
        println!("{}", luck);
        
        // 첫 번째 대운 확인
        assert!(luck.cycles.len() >= 8);
    }

    #[test]
    fn test_major_luck_at_age() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        let luck = pillars.major_luck_precise(
            Gender::Male, 
            2004, 11, 27, 22, 0, // 출생
            2004, 12, 7, 3, 48   // 대설
        );

        // 현재 나이 (2026년 기준)로 대운 조회
        let current = luck.at_age(22);
        assert!(current.is_some());
        println!("22세 대운: {}", current.unwrap());
    }

    #[test]
    fn test_major_luck_astro() {
        // 김성주님 사주를 천문 엔진(AstroEngine)으로 자동 계산
        // 2004-11-27 22:00 KST = 2004-11-27 13:00 UTC
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        
        // Astro 엔진은 내부적으로 UTC를 기대하므로, KST 22:00 대신 UTC 13:00을 전달하거나
        // 여기서는 엔진의 정밀도를 확인하기 위해 수동 보정된 시각을 넣음
        let luck = pillars.major_luck(Gender::Male, 2004, 11, 27, 13, 0);
        
        println!("=== 천문 엔진(Swiss Ephemeris) 기반 대운 ===");
        println!("{}", luck);
        
        // 11월 27일 13:00 UTC ~ 12월 6일 18:48 UTC (대설)
        // 약 9일 5시간 -> 3년
        assert_eq!(luck.start_age, 3);
    }
}
