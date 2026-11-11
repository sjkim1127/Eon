//! Yearly Luck and Monthly Luck Calculation
//!
//! - Yearly luck: Relationship between yearly GanZi and Day Master
//! - Monthly luck: Relationship between monthly GanZi and Day Master

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

/// Yearly Luck (annual cycle)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearlyLuck {
    /// Year
    pub year: i32,
    /// Yearly GanZi
    pub ganzi: GanZi,
    /// Stem Ten God
    pub stem_god: TenGod,
    /// Branch Ten God
    pub branch_god: TenGod,
    /// Interactions with natal chart
    pub influence: Option<LuckInfluence>,
    /// Special events (Heavenly & Earthly Double Clash, etc.)
    pub special_events: Vec<String>,
    /// Twelve Life Stage (optional)
    pub twelve_stage: Option<String>,
}

impl YearlyLuck {
    /// Calculate yearly luck for given year
    pub fn calculate(year: i32, pillars: &FourPillars) -> Self {
        let ganzi = Self::year_ganzi(year);
        let day_master = pillars.day_master();

        let influence = Some(DynamicLuckAnalysis::get_influence(ganzi, "Yearly", pillars));

        // Heavenly & Earthly Double Clash check
        let mut special_events = Vec::new();
        // Check stem clash and branch clash between Day pillar and Yearly luck
        let day_stem_clash = StemClash::check(ganzi.stem, pillars.day.stem).is_some();
        let day_branch_clash = BranchClash::check(ganzi.branch, pillars.day.branch).is_some();

        if day_stem_clash && day_branch_clash {
            special_events.push("Heavenly & Earthly Double Clash (天戦地沖)".to_string());
        }

        // Add spirit markers
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

    /// Year  
    ///
    /// 1984 = 甲子 ( 0)
    fn year_ganzi(year: i32) -> GanZi {
        // 1984 甲子 ( 0)
        let base_year = 1984;
        let index = (year - base_year).rem_euclid(60);
        GanZi::from_index(index)
    }
}

impl std::fmt::Display for YearlyLuck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} ({}/{})",
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
            write!(f, " | [!] {}", event)?;
        }

        Ok(())
    }
}

/// Monthly Luck
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyLuck {
    /// Year
    pub year: i32,
    /// Month (1-12)
    pub month: u32,
    /// Monthly GanZi
    pub ganzi: GanZi,
    /// Stem Ten God
    pub stem_god: TenGod,
    /// Branch Ten God
    pub branch_god: TenGod,
    /// Interactions with natal chart
    pub influence: Option<LuckInfluence>,
    /// Special events / markers
    pub special_events: Vec<String>,
    /// Twelve Life Stage
    pub twelve_stage: Option<String>,
}

impl MonthlyLuck {
    /// Calculate monthly luck for given year/month
    pub fn calculate(year: i32, month: u32, pillars: &FourPillars) -> Self {
        let ganzi = Self::month_ganzi(year, month);
        let day_master = pillars.day_master();

        let influence = Some(DynamicLuckAnalysis::get_influence(
            ganzi, "Monthly", pillars,
        ));
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

    /// Precise monthly luck calculation for DateTime<Utc>
    pub fn calculate_at_datetime(
        dt: chrono::DateTime<chrono::Utc>,
        pillars: &FourPillars,
    ) -> Result<Self, eon_astro::AstroError> {
        use chrono::Datelike;
        let ganzi = Self::try_month_ganzi_at(dt)?;
        let day_master = pillars.day_master();

        let influence = Some(DynamicLuckAnalysis::get_influence(
            ganzi, "Monthly", pillars,
        ));
        let special_events =
            crate::analysis::shinsal::ShinsalAnalysis::calculate_for_luck(ganzi, pillars);

        Ok(Self {
            year: dt.year(),
            month: dt.month(),
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
        })
    }

    /// Precise monthly GanZi calculation based on astronomical solar terms
    pub fn try_month_ganzi_at(
        dt: chrono::DateTime<chrono::Utc>,
    ) -> Result<GanZi, eon_astro::AstroError> {
        use chrono::Datelike;
        use eon_astro::AstroEngine;

        let engine = AstroEngine::new();
        let term_24_idx = engine.get_solar_term_index(dt)?;
        let term_12_idx = (term_24_idx / 2) as u32;
        let saju_month_ordinal = term_12_idx + 1;

        let dt_year = dt.year();
        let year_start = chrono::NaiveDate::from_ymd_opt(dt_year, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();

        // Pre-XiaoHan year shift: when dt is in January (including DongZhi term_24_idx 21,
        // XiaoHan entry, and before LiChun), saju_year must be evaluated as dt_year - 1.
        let saju_year = if dt.month() == 1 {
            dt_year - 1
        } else {
            let lichun = engine.find_solar_term_time(year_start, 0)?;
            if dt < lichun {
                dt_year - 1
            } else {
                dt_year
            }
        };

        let saju_year_stem_idx = GanZi::from_year(saju_year).stem.index();
        let first_month_stem_idx = match saju_year_stem_idx % 5 {
            0 => 2,
            1 => 4,
            2 => 6,
            3 => 8,
            4 => 0,
            _ => 0,
        };

        let month_stem_idx = (first_month_stem_idx + (saju_month_ordinal - 1)) % 10;
        let month_branch_idx = (saju_month_ordinal + 1) % 12;

        Ok(GanZi::new(
            HeavenlyStem::from_index(month_stem_idx as i32),
            crate::core::branch::EarthlyBranch::from_index(month_branch_idx as i32),
        ))
    }

    /// Legacy infallible wrapper. New callers should prefer `try_month_ganzi_at`.
    pub fn month_ganzi_at(dt: chrono::DateTime<chrono::Utc>) -> GanZi {
        Self::try_month_ganzi_at(dt).unwrap_or_else(|_| GanZi::from_year(dt.year()))
    }

    /// Monthly GanZi based on solar terms
    pub fn month_ganzi(year: i32, month: u32) -> GanZi {
        let dt = chrono::NaiveDate::from_ymd_opt(year, month, 15)
            .unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap())
            .and_hms_opt(12, 0, 0)
            .unwrap()
            .and_utc();
        Self::month_ganzi_at(dt)
    }
}

impl std::fmt::Display for MonthlyLuck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} Month {} {} ({}/{})",
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

/// Daily Luck
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

        let influence = Some(DynamicLuckAnalysis::get_influence(ganzi, "Daily", pillars));
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

/// Hourly Luck
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

        let influence = Some(DynamicLuckAnalysis::get_influence(ganzi, "Hourly", pillars));
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

/// Comprehensive Luck Analysis (Major + Yearly + Monthly)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuckAnalysis {
    /// Day Master
    pub day_master: HeavenlyStem,
    /// Major Luck
    pub major_luck: MajorLuckAnalysis,
    /// Yearly Luck list
    pub yearly_lucks: Vec<YearlyLuck>,
    /// Monthly Luck list
    pub monthly_lucks: Vec<MonthlyLuck>,
    /// Daily Luck list
    pub daily_lucks: Vec<DailyLuck>,
    /// Hourly Luck list
    pub hourly_lucks: Vec<HourlyLuck>,
}

impl LuckAnalysis {
    /// Comprehensive Luck Analysis
    ///
    /// # Arguments
    /// * `pillars` -  
    /// * `gender` -
    /// * `birth_year` -  
    /// * `birth_month` -  
    /// * `birth_day` -  
    /// * `current_year` -   (/ )
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

        // Calculate Major Luck
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

        // Calculate Yearly Luck (+-5 years)
        let yearly_lucks: Vec<YearlyLuck> = (current_year - 5..=current_year + 5)
            .map(|y| YearlyLuck::calculate(y, pillars))
            .collect();

        // Calculate Monthly Luck (12 months)
        let monthly_lucks: Vec<MonthlyLuck> = (1..=12)
            .map(|m| MonthlyLuck::calculate(current_year, m, pillars))
            .collect();

        // Calculate Daily Luck
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

        // Calculate Hourly Luck
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

    /// Query yearly luck for year
    pub fn yearly_at(&self, year: i32) -> Option<&YearlyLuck> {
        self.yearly_lucks.iter().find(|l| l.year == year)
    }

    /// Query monthly luck for year/month
    pub fn monthly_at(&self, month: u32) -> Option<&MonthlyLuck> {
        self.monthly_lucks.iter().find(|l| l.month == month)
    }

    /// Query current major luck by age
    pub fn current_major_luck(&self, age: u32) -> Option<&MajorLuck> {
        self.major_luck.at_age(age)
    }
}

impl std::fmt::Display for LuckAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【Major Luck】")?;
        writeln!(f, "{}", self.major_luck)?;

        writeln!(f, "【Yearly Luck】")?;
        for luck in &self.yearly_lucks {
            writeln!(f, "  {}", luck)?;
        }

        writeln!(f, "\n【Monthly Luck】")?;
        for luck in &self.monthly_lucks {
            writeln!(f, "  {}", luck)?;
        }

        Ok(())
    }
}

// ============================================
// FourPillars
// ============================================

impl FourPillars {
    /// Calculate yearly luck for given year
    pub fn yearly_luck(&self, year: i32) -> YearlyLuck {
        YearlyLuck::calculate(year, self)
    }

    /// Calculate monthly luck
    pub fn monthly_luck(&self, year: i32, month: u32) -> MonthlyLuck {
        MonthlyLuck::calculate(year, month, self)
    }

    /// Comprehensive Luck Analysis (  )
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
        // 2024 = 甲辰
        let ganzi_2024 = YearlyLuck::year_ganzi(2024);
        assert_eq!(ganzi_2024.stem, HeavenlyStem::Jia);
        assert_eq!(ganzi_2024.branch, EarthlyBranch::Chen);

        // 2026 = 丙午
        let ganzi_2026 = YearlyLuck::year_ganzi(2026);
        assert_eq!(ganzi_2026.stem, HeavenlyStem::Bing);
        assert_eq!(ganzi_2026.branch, EarthlyBranch::Wu);
    }

    #[test]
    fn test_yearly_luck() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        // 2026
        let luck_2026 = YearlyLuck::calculate(2026, &pillars);
        println!("2026 yearly luck: {}", luck_2026);

        assert_eq!(luck_2026.ganzi.stem, HeavenlyStem::Bing); // 丙
        assert_eq!(luck_2026.ganzi.branch, EarthlyBranch::Wu); // 午
        assert_eq!(luck_2026.stem_god, TenGod::Pianguan); // Seven Killings
    }

    #[test]
    fn test_monthly_luck() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        // 2026 = 丙午
        // 1 (Jan) -> Chou (1)
        let luck1 = MonthlyLuck::calculate(2026, 1, &pillars);
        assert_eq!(luck1.ganzi.branch, EarthlyBranch::Chou);

        // 2 (Feb) -> Yin (2)
        let luck2 = MonthlyLuck::calculate(2026, 2, &pillars);
        assert_eq!(luck2.ganzi.branch, EarthlyBranch::Yin);
        // Bing year Yin month is Geng Yin?
        // Jia: Bing, Yi: Wu, Bing: Geng. YES.
        assert_eq!(luck2.ganzi.stem, HeavenlyStem::Geng);

        // 12 (Dec) -> Zi (0)
        let luck12 = MonthlyLuck::calculate(2026, 12, &pillars);
        assert_eq!(luck12.ganzi.branch, EarthlyBranch::Zi);
    }

    #[test]
    fn test_yearly_luck_interactions() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        // 2026 (丙午) vs (丙子) ()
        //    (庚寅),

        // 2026() ->  (午)
        //   (子)

        let luck = pillars.yearly_luck(2026);
        println!("2026 yearly luck interaction: {:?}", luck.influence);

        // (2004) . 2026 .
        // (寅午)

        if let Some(inf) = luck.influence {
            assert!(inf.relations_with_natal.iter().any(|r| r.contains("Combo")
                || r.contains("Clash")
                || r.contains("Combination")
                || r.contains("Alliance")));
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
                0, // Birth
                2004,
                12,
                7,
                3,
                48,   // DaXue
                2026, // Reference Year
            )
            .unwrap();

        println!("{}", analysis);

        assert!(!analysis.yearly_lucks.is_empty());
        assert_eq!(analysis.monthly_lucks.len(), 12);
    }

    #[test]
    fn test_clash_detection() {
        // (庚寅)  ( )
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        println!("Test Subject Day Pillar: {}", pillars.day);

        //      (200 )
        // 60     1 ()
        let mut found_year = None;

        for year in 2020..2100 {
            let luck = YearlyLuck::calculate(year, &pillars);
            if luck
                .special_events
                .iter()
                .any(|e| e.contains("Heavenly & Earthly Double Clash"))
            {
                println!("Double Clash detected: {} {}", year, luck.ganzi);
                found_year = Some(year);
                break;
            }
        }

        assert!(found_year.is_some(), "Double clash year not found");
    }
}
