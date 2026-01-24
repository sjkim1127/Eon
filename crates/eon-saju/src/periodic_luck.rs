//! 연운(年運) 및 월운(月運) 계산
//!
//! - 연운: 해당 연도의 간지와 일간의 관계
//! - 월운: 해당 월의 간지와 일간의 관계

use serde::{Deserialize, Serialize};
use eon_core::Gender;
use crate::stem::HeavenlyStem;
use crate::branch::EarthlyBranch;
use crate::ganzi::GanZi;
use crate::pillars::FourPillars;
use crate::ten_gods::TenGod;
use crate::major_luck::{MajorLuck, MajorLuckAnalysis};

/// 연운 (해당 연도의 운세)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearlyLuck {
    /// 연도
    pub year: i32,
    /// 해당 연도의 간지
    pub ganzi: GanZi,
    /// 천간 십성
    pub stem_god: TenGod,
    /// 지지 십성
    pub branch_god: TenGod,
    /// 12운성 (선택적)
    pub twelve_stage: Option<String>,
}

impl YearlyLuck {
    /// 특정 연도의 연운 계산
    pub fn calculate(year: i32, day_master: HeavenlyStem) -> Self {
        let ganzi = Self::year_ganzi(year);
        
        Self {
            year,
            ganzi,
            stem_god: TenGod::from_stems(day_master, ganzi.stem),
            branch_god: TenGod::from_stem_and_branch(day_master, ganzi.branch),
            twelve_stage: None, // TODO: 12운성 계산
        }
    }

    /// 연도의 간지 계산
    /// 
    /// 1984년 = 甲子년 (인덱스 0)
    fn year_ganzi(year: i32) -> GanZi {
        // 1984년이 甲子년 (인덱스 0)
        let base_year = 1984;
        let index = (year - base_year).rem_euclid(60);
        GanZi::from_index(index)
    }
}

impl std::fmt::Display for YearlyLuck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}년 {} ({}/{})",
            self.year,
            self.ganzi,
            self.stem_god.hangul(),
            self.branch_god.hangul()
        )
    }
}

/// 월운 (해당 월의 운세)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyLuck {
    /// 연도
    pub year: i32,
    /// 월 (1-12)
    pub month: u32,
    /// 해당 월의 간지
    pub ganzi: GanZi,
    /// 천간 십성
    pub stem_god: TenGod,
    /// 지지 십성
    pub branch_god: TenGod,
}

impl MonthlyLuck {
    /// 특정 연월의 월운 계산
    pub fn calculate(year: i32, month: u32, day_master: HeavenlyStem) -> Self {
        let ganzi = Self::month_ganzi(year, month);
        
        Self {
            year,
            month,
            ganzi,
            stem_god: TenGod::from_stems(day_master, ganzi.stem),
            branch_god: TenGod::from_stem_and_branch(day_master, ganzi.branch),
        }
    }

    /// 월의 간지 계산
    /// 
    /// 년간에 따른 월간 결정 (年上起月法)
    fn month_ganzi(year: i32, month: u32) -> GanZi {
        let year_ganzi = YearlyLuck::year_ganzi(year);
        let year_stem = year_ganzi.stem;
        
        // 년간에 따른 1월(인월) 천간 결정
        // 甲己년 → 丙寅월 시작
        // 乙庚년 → 戊寅월 시작
        // 丙辛년 → 庚寅월 시작
        // 丁壬년 → 壬寅월 시작
        // 戊癸년 → 甲寅월 시작
        let first_month_stem_index = match year_stem {
            HeavenlyStem::Jia | HeavenlyStem::Ji => 2,   // 丙
            HeavenlyStem::Yi | HeavenlyStem::Geng => 4,  // 戊
            HeavenlyStem::Bing | HeavenlyStem::Xin => 6, // 庚
            HeavenlyStem::Ding | HeavenlyStem::Ren => 8, // 壬
            HeavenlyStem::Wu | HeavenlyStem::Gui => 0,   // 甲
        };

        // 월지는 고정 (1월=寅, 2월=卯, ...)
        // 절기 기준 월: 1월(입춘~경칩)=寅, 2월(경칩~청명)=卯, ...
        let month_branch_index = ((month as i32 + 1) % 12) as i32; // 1월=寅(2), 2월=卯(3)...
        
        // 월간 계산: 첫 월 천간에서 월수만큼 진행
        let month_stem_index = ((first_month_stem_index + (month as i32 - 1)) % 10) as i32;

        GanZi {
            stem: HeavenlyStem::from_index(month_stem_index),
            branch: EarthlyBranch::from_index(month_branch_index),
        }
    }
}

impl std::fmt::Display for MonthlyLuck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}년 {}월 {} ({}/{})",
            self.year, self.month,
            self.ganzi,
            self.stem_god.hangul(),
            self.branch_god.hangul()
        )
    }
}

/// 전체 운세 분석 (대운 + 연운 + 월운)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuckAnalysis {
    /// 일간 (기준)
    pub day_master: HeavenlyStem,
    /// 대운
    pub major_luck: MajorLuckAnalysis,
    /// 연운 목록
    pub yearly_lucks: Vec<YearlyLuck>,
    /// 월운 목록 (현재 연도)
    pub monthly_lucks: Vec<MonthlyLuck>,
}

impl LuckAnalysis {
    /// 전체 운세 분석
    /// 
    /// # Arguments
    /// * `pillars` - 사주 팔자
    /// * `gender` - 성별
    /// * `birth_year` - 출생 년도
    /// * `birth_month` - 출생 월
    /// * `birth_day` - 출생 일
    /// * `current_year` - 현재 연도 (연운/월운 계산용)
    pub fn calculate(
        pillars: &FourPillars,
        gender: Gender,
        birth_year: i32,
        birth_month: u32,
        birth_day: u32,
        current_year: i32,
    ) -> Self {
        let day_master = pillars.day_master();
        
        // 대운 계산
        let major_luck = MajorLuckAnalysis::calculate(
            pillars, gender, birth_year, birth_month, birth_day
        );

        // 연운 계산 (현재 년도 기준 ±5년)
        let yearly_lucks: Vec<YearlyLuck> = (current_year - 5..=current_year + 5)
            .map(|y| YearlyLuck::calculate(y, day_master))
            .collect();

        // 월운 계산 (현재 연도 12개월)
        let monthly_lucks: Vec<MonthlyLuck> = (1..=12)
            .map(|m| MonthlyLuck::calculate(current_year, m, day_master))
            .collect();

        Self {
            day_master,
            major_luck,
            yearly_lucks,
            monthly_lucks,
        }
    }

    /// 특정 연도의 연운 조회
    pub fn yearly_at(&self, year: i32) -> Option<&YearlyLuck> {
        self.yearly_lucks.iter().find(|l| l.year == year)
    }

    /// 특정 연월의 월운 조회
    pub fn monthly_at(&self, month: u32) -> Option<&MonthlyLuck> {
        self.monthly_lucks.iter().find(|l| l.month == month)
    }

    /// 현재 대운 조회 (나이 기준)
    pub fn current_major_luck(&self, age: u32) -> Option<&MajorLuck> {
        self.major_luck.at_age(age)
    }
}

impl std::fmt::Display for LuckAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【대운】")?;
        writeln!(f, "{}", self.major_luck)?;
        
        writeln!(f, "【연운】")?;
        for luck in &self.yearly_lucks {
            writeln!(f, "  {}", luck)?;
        }
        
        writeln!(f, "\n【월운】")?;
        for luck in &self.monthly_lucks {
            writeln!(f, "  {}", luck)?;
        }
        
        Ok(())
    }
}

// ============================================
// FourPillars 편의 메서드
// ============================================

impl FourPillars {
    /// 특정 연도의 연운 계산
    pub fn yearly_luck(&self, year: i32) -> YearlyLuck {
        YearlyLuck::calculate(year, self.day_master())
    }

    /// 특정 연월의 월운 계산
    pub fn monthly_luck(&self, year: i32, month: u32) -> MonthlyLuck {
        MonthlyLuck::calculate(year, month, self.day_master())
    }

    /// 전체 운세 분석
    pub fn full_luck_analysis(
        &self,
        gender: Gender,
        birth_year: i32,
        birth_month: u32,
        birth_day: u32,
        current_year: i32,
    ) -> LuckAnalysis {
        LuckAnalysis::calculate(
            self, gender, birth_year, birth_month, birth_day, current_year
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pillars::SajuInput;

    #[test]
    fn test_year_ganzi() {
        // 2024년 = 甲辰년
        let ganzi_2024 = YearlyLuck::year_ganzi(2024);
        assert_eq!(ganzi_2024.stem, HeavenlyStem::Jia);
        assert_eq!(ganzi_2024.branch, EarthlyBranch::Chen);

        // 2026년 = 丙午년
        let ganzi_2026 = YearlyLuck::year_ganzi(2026);
        assert_eq!(ganzi_2026.stem, HeavenlyStem::Bing);
        assert_eq!(ganzi_2026.branch, EarthlyBranch::Wu);
    }

    #[test]
    fn test_yearly_luck() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        // 2026년 연운
        let luck_2026 = pillars.yearly_luck(2026);
        println!("2026년 연운: {}", luck_2026);
        
        assert_eq!(luck_2026.ganzi.stem, HeavenlyStem::Bing);  // 丙
        assert_eq!(luck_2026.ganzi.branch, EarthlyBranch::Wu); // 午
        assert_eq!(luck_2026.stem_god, TenGod::Pianguan);      // 편관
    }

    #[test]
    fn test_monthly_luck() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        // 2026년 1월 월운
        let luck = pillars.monthly_luck(2026, 1);
        println!("2026년 1월 월운: {}", luck);
    }

    #[test]
    fn test_full_luck_analysis() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        let analysis = pillars.full_luck_analysis(
            Gender::Male, 2004, 11, 27, 2026
        );

        println!("{}", analysis);
        
        assert!(!analysis.yearly_lucks.is_empty());
        assert_eq!(analysis.monthly_lucks.len(), 12);
    }
}
