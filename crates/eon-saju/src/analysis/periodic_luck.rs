//! 연운(年運) 및 월운(月運) 계산
//!
//! - 연운: 해당 연도의 간지와 일간의 관계
//! - 월운: 해당 월의 간지와 일간의 관계

use crate::analysis::dynamic_luck::{DynamicLuckAnalysis, LuckInfluence};
use crate::analysis::major_luck::{MajorLuck, MajorLuckAnalysis};
use crate::analysis::relationships::{BranchClash, StemClash};
use crate::core::ganzi::GanZi;
use crate::core::pillars::FourPillars;
use crate::core::pillars::SajuError;
use crate::core::stem::HeavenlyStem;
use crate::core::ten_gods::TenGod;
use chrono::Datelike;
use eon_core::Gender;
use serde::{Deserialize, Serialize};

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
    /// 원국과의 상호작용 (합충형해)
    pub influence: Option<LuckInfluence>,
    /// 특수 사건 (천전지충 등)
    pub special_events: Vec<String>,
    /// 12운성 (선택적)
    pub twelve_stage: Option<String>,
}

impl YearlyLuck {
    /// 특정 연도의 연운 계산
    pub fn calculate(year: i32, pillars: &FourPillars) -> Self {
        let ganzi = Self::year_ganzi(year);
        let day_master = pillars.day_master();

        let influence = Some(DynamicLuckAnalysis::get_influence(ganzi, "세운", pillars));

        // 천전지충(天戦地沖) 체크
        let mut special_events = Vec::new();
        // 일주와 세운 간의 천충지충 확인
        let day_stem_clash = StemClash::check(ganzi.stem, pillars.day.stem).is_some();
        let day_branch_clash = BranchClash::check(ganzi.branch, pillars.day.branch).is_some();

        if day_stem_clash && day_branch_clash {
            special_events.push("천전지충(天戦地沖)".to_string());
        }

        // 신살 분석 추가
        let shinsal_events =
            crate::analysis::shinsal::ShinsalAnalysis::calculate_for_luck(ganzi, pillars);
        special_events.extend(shinsal_events);

        Self {
            year,
            ganzi,
            stem_god: TenGod::from_stems(day_master, ganzi.stem),
            branch_god: TenGod::from_stem_and_branch(day_master, ganzi.branch),
            influence,
            special_events,
            twelve_stage: Some(
                crate::core::twelve_stages::calculate_twelve_stage(day_master, ganzi.branch)
                    .hangul()
                    .to_string(),
            ),
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
        write!(
            f,
            "{}년 {} ({}/{})",
            self.year,
            self.ganzi,
            self.stem_god.hangul(),
            self.branch_god.hangul()
        )?;

        if let Some(inf) = &self.influence {
            if !inf.relations_with_natal.is_empty() {
                write!(f, " | {}", inf.relations_with_natal.join(", "))?;
            }
        }
        for event in &self.special_events {
            write!(f, " | ⚠️ {}", event)?;
        }

        Ok(())
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
    /// 원국과의 상호작용
    pub influence: Option<LuckInfluence>,
    /// 신살 등의 특수 정보
    pub special_events: Vec<String>,
    /// 12운성
    pub twelve_stage: Option<String>,
}

impl MonthlyLuck {
    /// 특정 연월의 월운 계산
    pub fn calculate(year: i32, month: u32, pillars: &FourPillars) -> Self {
        let ganzi = Self::month_ganzi(year, month);
        let day_master = pillars.day_master();

        let influence = Some(DynamicLuckAnalysis::get_influence(ganzi, "월운", pillars));
        let special_events =
            crate::analysis::shinsal::ShinsalAnalysis::calculate_for_luck(ganzi, pillars);

        Self {
            year,
            month,
            ganzi,
            stem_god: TenGod::from_stems(day_master, ganzi.stem),
            branch_god: TenGod::from_stem_and_branch(day_master, ganzi.branch),
            influence,
            special_events,
            twelve_stage: Some(
                crate::core::twelve_stages::calculate_twelve_stage(day_master, ganzi.branch)
                    .hangul()
                    .to_string(),
            ),
        }
    }

    fn month_ganzi(year: i32, month: u32) -> GanZi {
        let saju_month = if month == 1 { 12 } else { month - 1 };
        crate::core::ganzi_utils::calculate_month_ganzi(year, saju_month as i32)
    }
}

impl std::fmt::Display for MonthlyLuck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}년 {}월 {} ({}/{})",
            self.year,
            self.month,
            self.ganzi,
            self.stem_god.hangul(),
            self.branch_god.hangul()
        )?;

        if let Some(inf) = &self.influence {
            if !inf.relations_with_natal.is_empty() {
                write!(f, " | {}", inf.relations_with_natal.join(", "))?;
            }
        }
        Ok(())
    }
}

/// 일운 (해당 일의 운세)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyLuck {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub ganzi: GanZi,
    pub stem_god: TenGod,
    pub branch_god: TenGod,
    pub influence: Option<LuckInfluence>,
    pub special_events: Vec<String>,
    pub twelve_stage: Option<String>,
}

impl DailyLuck {
    pub fn calculate(year: i32, month: u32, day: u32, pillars: &FourPillars) -> Self {
        let ganzi = crate::core::ganzi_utils::calculate_day_ganzi(year, month, day);
        let day_master = pillars.day_master();

        let influence = Some(DynamicLuckAnalysis::get_influence(ganzi, "일운", pillars));
        let special_events =
            crate::analysis::shinsal::ShinsalAnalysis::calculate_for_luck(ganzi, pillars);

        Self {
            year,
            month,
            day,
            ganzi,
            stem_god: TenGod::from_stems(day_master, ganzi.stem),
            branch_god: TenGod::from_stem_and_branch(day_master, ganzi.branch),
            influence,
            special_events,
            twelve_stage: Some(
                crate::core::twelve_stages::calculate_twelve_stage(day_master, ganzi.branch)
                    .hangul()
                    .to_string(),
            ),
        }
    }
}

/// 시운 (해당 시간의 운세)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyLuck {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub ganzi: GanZi,
    pub stem_god: TenGod,
    pub branch_god: TenGod,
    pub influence: Option<LuckInfluence>,
    pub special_events: Vec<String>,
    pub twelve_stage: Option<String>,
}

impl HourlyLuck {
    pub fn calculate(year: i32, month: u32, day: u32, hour: u32, pillars: &FourPillars) -> Self {
        let day_ganzi = crate::core::ganzi_utils::calculate_day_ganzi(year, month, day);
        let ganzi = crate::core::pillars::FourPillars::calculate_hour_pillar(&day_ganzi, hour);
        let day_master = pillars.day_master();

        let influence = Some(DynamicLuckAnalysis::get_influence(ganzi, "시운", pillars));
        let special_events =
            crate::analysis::shinsal::ShinsalAnalysis::calculate_for_luck(ganzi, pillars);

        Self {
            year,
            month,
            day,
            hour,
            ganzi,
            stem_god: TenGod::from_stems(day_master, ganzi.stem),
            branch_god: TenGod::from_stem_and_branch(day_master, ganzi.branch),
            influence,
            special_events,
            twelve_stage: Some(
                crate::core::twelve_stages::calculate_twelve_stage(day_master, ganzi.branch)
                    .hangul()
                    .to_string(),
            ),
        }
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
    /// 일운 목록
    pub daily_lucks: Vec<DailyLuck>,
    /// 시운 목록
    pub hourly_lucks: Vec<HourlyLuck>,
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
        birth_hour: u32,
        birth_min: u32,
        term_year: i32,
        term_month: u32,
        term_day: u32,
        term_hour: u32,
        term_min: u32,
        current_year: i32,
    ) -> Result<Self, SajuError> {
        let day_master = pillars.day_master();

        // 대운 계산
        let major_luck = MajorLuckAnalysis::calculate(
            pillars,
            gender,
            birth_year,
            birth_month,
            birth_day,
            birth_hour,
            birth_min,
            term_year,
            term_month,
            term_day,
            term_hour,
            term_min,
        )?;

        // 연운 계산 (현재 년도 기준 ±5년)
        let yearly_lucks: Vec<YearlyLuck> = (current_year - 5..=current_year + 5)
            .map(|y| YearlyLuck::calculate(y, pillars))
            .collect();

        // 월운 계산 (현재 연도 12개월)
        let monthly_lucks: Vec<MonthlyLuck> = (1..=12)
            .map(|m| MonthlyLuck::calculate(current_year, m, pillars))
            .collect();

        // 일운 계산 (기준 연도/월의 모든 날짜 계산 - 윤년 및 해당 월의 실제 말일까지)
        let days_in_month = if term_month == 12 {
            chrono::NaiveDate::from_ymd_opt(current_year + 1, 1, 1)
        } else {
            chrono::NaiveDate::from_ymd_opt(current_year, term_month + 1, 1)
        }
        .and_then(|next_1st| next_1st.pred_opt())
        .map(|last_day| last_day.day())
        .unwrap_or(30);

        let daily_lucks: Vec<DailyLuck> = (1..=days_in_month)
            .map(|d| DailyLuck::calculate(current_year, term_month, d, pillars))
            .collect();

        // 시운 계산 (기준 일자의 24시간 또는 12시진)
        let mut hourly_lucks = Vec::new();
        for h in 0..24 {
            hourly_lucks.push(HourlyLuck::calculate(
                current_year,
                term_month,
                term_day,
                h,
                pillars,
            ));
        }

        Ok(Self {
            day_master,
            major_luck,
            yearly_lucks,
            monthly_lucks,
            daily_lucks,
            hourly_lucks,
        })
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
        YearlyLuck::calculate(year, self)
    }

    /// 특정 연월의 월운 계산
    pub fn monthly_luck(&self, year: i32, month: u32) -> MonthlyLuck {
        MonthlyLuck::calculate(year, month, self)
    }

    /// 전체 운세 분석 (정밀 대운 포함)
    pub fn analyze_luck_precise(
        &self,
        gender: Gender,
        b_year: i32,
        b_month: u32,
        b_day: u32,
        b_hour: u32,
        b_min: u32,
        t_year: i32,
        t_month: u32,
        t_day: u32,
        t_hour: u32,
        t_min: u32,
        current_year: i32,
    ) -> Result<LuckAnalysis, SajuError> {
        LuckAnalysis::calculate(
            self,
            gender,
            b_year,
            b_month,
            b_day,
            b_hour,
            b_min,
            t_year,
            t_month,
            t_day,
            t_hour,
            t_min,
            current_year,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::branch::EarthlyBranch;
    use crate::core::pillars::SajuInput;

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
        let luck_2026 = YearlyLuck::calculate(2026, &pillars);
        println!("2026년 연운: {}", luck_2026);

        assert_eq!(luck_2026.ganzi.stem, HeavenlyStem::Bing); // 丙
        assert_eq!(luck_2026.ganzi.branch, EarthlyBranch::Wu); // 午
        assert_eq!(luck_2026.stem_god, TenGod::Pianguan); // 편관
    }

    #[test]
    fn test_monthly_luck() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        // 2026년 = 丙午년
        // 1월 (Jan) -> Chou (1)
        let luck1 = MonthlyLuck::calculate(2026, 1, &pillars);
        assert_eq!(luck1.ganzi.branch, EarthlyBranch::Chou);

        // 2월 (Feb) -> Yin (2)
        let luck2 = MonthlyLuck::calculate(2026, 2, &pillars);
        assert_eq!(luck2.ganzi.branch, EarthlyBranch::Yin);
        // Bing year Yin month is Geng Yin?
        // Jia: Bing, Yi: Wu, Bing: Geng. YES.
        assert_eq!(luck2.ganzi.stem, HeavenlyStem::Geng);

        // 12월 (Dec) -> Zi (0)
        let luck12 = MonthlyLuck::calculate(2026, 12, &pillars);
        assert_eq!(luck12.ganzi.branch, EarthlyBranch::Zi);
    }

    #[test]
    fn test_yearly_luck_interactions() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        // 2026년 병오(丙午)년 vs 병자(丙子)일주 (가정)
        // 실제 김성주님 일주는 경인(庚寅)이나, 충 충돌 테스트를 위해 로직 검증형으로 호출

        // 2026년(병오) -> 지지 오화(午)
        // 만약 일지가 자수(子)라면 자오충 발생

        let luck = pillars.yearly_luck(2026);
        println!("2026년 연운 상호작용: {:?}", luck.influence);

        // 김성주(2004)는 경인일주. 2026년 병오년.
        // 인오(寅午) 반합 화국 발생 예상

        if let Some(inf) = luck.influence {
            assert!(inf
                .relations_with_natal
                .iter()
                .any(|r| r.contains("합") || r.contains("충")));
        }
    }

    #[test]
    fn test_full_luck_analysis() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        let analysis = pillars
            .analyze_luck_precise(
                Gender::Male,
                2004,
                11,
                27,
                22,
                0, // 출생
                2004,
                12,
                7,
                3,
                48,   // 대설
                2026, // 기준년도
            )
            .unwrap();

        println!("{}", analysis);

        assert!(!analysis.yearly_lucks.is_empty());
        assert_eq!(analysis.monthly_lucks.len(), 12);
    }

    #[test]
    fn test_clash_detection() {
        // 경인(庚寅)일주 생성 (김성주님 사주)
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        println!("Test Subject Day Pillar: {}", pillars.day);

        // 동적으로 천전지충이 발생하는 연도를 탐색 (200년 범위)
        // 60갑자 주기 내에 반드시 최소 1번은 천전지충(천충지충)이 돌아옴
        let mut found_year = None;

        for year in 2020..2100 {
            let luck = YearlyLuck::calculate(year, &pillars);
            if luck.special_events.iter().any(|e| e.contains("천전지충")) {
                println!("천전지충 감지됨: {}년 {}", year, luck.ganzi);
                found_year = Some(year);
                break;
            }
        }

        assert!(
            found_year.is_some(),
            "천전지충이 감지되는 연도를 찾을 수 없습니다. (로직 확인 필요)"
        );
    }
}
