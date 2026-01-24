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
use eon_core::{BirthInfo, Gender};
use crate::stem::HeavenlyStem;
use crate::branch::EarthlyBranch;
use crate::element::Polarity;
use crate::ganzi::GanZi;
use crate::pillars::FourPillars;
use crate::ten_gods::{TenGod, TenGodAnalysis};
use crate::calendar::SolarTerm;

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

/// 단일 대운 (10년 주기)
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
}

impl MajorLuck {
    /// 새 대운 생성
    pub fn new(ganzi: GanZi, start_age: u32, day_master: HeavenlyStem) -> Self {
        Self {
            ganzi,
            start_age,
            end_age: start_age + 9, // 10년 주기
            stem_god: TenGod::from_stems(day_master, ganzi.stem),
            branch_god: TenGod::from_stem_and_branch(day_master, ganzi.branch),
        }
    }

    /// 특정 나이가 이 대운에 해당하는지 확인
    pub fn contains_age(&self, age: u32) -> bool {
        age >= self.start_age && age <= self.end_age
    }
}

impl std::fmt::Display for MajorLuck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}세~{}세: {} ({}/{})",
            self.start_age, self.end_age,
            self.ganzi,
            self.stem_god.hangul(),
            self.branch_god.hangul()
        )
    }
}

/// 대운 분석 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorLuckAnalysis {
    /// 대운 방향
    pub direction: LuckDirection,
    /// 대운 시작 나이 (첫 번째 대운)
    pub start_age: u32,
    /// 대운 목록 (보통 8~10개)
    pub cycles: Vec<MajorLuck>,
    /// 일간 (기준)
    pub day_master: HeavenlyStem,
}

impl MajorLuckAnalysis {
    /// 대운 분석 생성
    /// 
    /// # Arguments
    /// * `pillars` - 사주 팔자
    /// * `gender` - 성별
    /// * `birth_year` - 출생 년도
    /// * `birth_month` - 출생 월
    /// * `birth_day` - 출생 일
    pub fn calculate(
        pillars: &FourPillars,
        gender: Gender,
        birth_year: i32,
        birth_month: u32,
        birth_day: u32,
    ) -> Self {
        let day_master = pillars.day_master();
        let direction = LuckDirection::from_year_and_gender(pillars.year.stem, gender);
        
        // 대운 시작 나이 계산 (간략화된 버전)
        // 실제로는 절기까지의 거리를 계산해야 함
        let start_age = Self::calculate_start_age(birth_year, birth_month, birth_day, direction);
        
        // 대운 간지 계산 (월주 기준으로 순행/역행)
        let mut cycles = Vec::new();
        let mut current_ganzi = pillars.month;
        
        for i in 0..10 {
            let age = start_age + (i * 10);
            
            // 대운 간지 결정
            current_ganzi = match direction {
                LuckDirection::Forward => current_ganzi.next(),
                LuckDirection::Reverse => current_ganzi.prev(),
            };
            
            cycles.push(MajorLuck::new(current_ganzi, age, day_master));
        }

        Self {
            direction,
            start_age,
            cycles,
            day_master,
        }
    }

    /// 대운 시작 나이 계산
    /// 
    /// 간략화된 버전: 절기 대신 월의 절기 일자를 사용
    fn calculate_start_age(year: i32, month: u32, day: u32, direction: LuckDirection) -> u32 {
        // 각 월의 대략적인 절기 시작일
        let term_day = match month {
            1 => 6,   // 소한
            2 => 4,   // 입춘
            3 => 6,   // 경칩
            4 => 5,   // 청명
            5 => 6,   // 입하
            6 => 6,   // 망종
            7 => 7,   // 소서
            8 => 8,   // 입추
            9 => 8,   // 백로
            10 => 8,  // 한로
            11 => 7,  // 입동
            12 => 7,  // 대설
            _ => 5,
        };

        // 절기까지의 일수 계산
        let days_to_term = match direction {
            LuckDirection::Forward => {
                // 순행: 다음 절기까지
                if day < term_day {
                    term_day - day
                } else {
                    // 다음 달 절기까지
                    let next_term_day = match month {
                        12 => 6, // 1월 소한
                        _ => match month + 1 {
                            2 => 4, 3 => 6, 4 => 5, 5 => 6, 6 => 6,
                            7 => 7, 8 => 8, 9 => 8, 10 => 8, 11 => 7, 12 => 7,
                            _ => 5,
                        }
                    };
                    (30 - day) + next_term_day
                }
            }
            LuckDirection::Reverse => {
                // 역행: 이전 절기까지
                if day >= term_day {
                    day - term_day
                } else {
                    // 이전 달 절기까지
                    day + (30 - term_day)
                }
            }
        };

        // 3일 = 1년, 반올림
        let start_age = ((days_to_term as f32) / 3.0).round() as u32;
        
        // 최소 1세, 최대 10세
        start_age.clamp(1, 10)
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
        writeln!(f, "대운 ({}, {}세 시작)", self.direction, self.start_age)?;
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
    /// 대운 분석
    pub fn major_luck(&self, gender: Gender, birth_year: i32, birth_month: u32, birth_day: u32) -> MajorLuckAnalysis {
        MajorLuckAnalysis::calculate(self, gender, birth_year, birth_month, birth_day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pillars::SajuInput;

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
        
        // 甲(양목) + 남자 = 순행
        let luck = pillars.major_luck(Gender::Male, 2004, 11, 27);
        
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
        let luck = pillars.major_luck(Gender::Male, 2004, 11, 27);

        // 현재 나이 (2026년 기준)로 대운 조회
        let current = luck.at_age(22);
        assert!(current.is_some());
        println!("22세 대운: {}", current.unwrap());
    }
}
