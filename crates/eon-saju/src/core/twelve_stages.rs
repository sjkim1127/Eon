//! Twelve Life Stages (十二運星) Analysis
//!
//! Calculates energy states of earthly branches relative to the Day Master.
//!
//! ## Twelve Stages
//!
//! 1. Changsheng (長生) - Birth / Emerging Life
//! 2. Muyu (沐浴) - Bath / Cleansing & Growth
//! 3. Guandai (冠帶) - Crown / Coming of Age
//! 4. Jianlu (建祿) - Prosperity / Steady Foundation
//! 5. Diwang (帝旺) - Peak / Maximum Zenith
//! 6. Shuai (衰) - Decline / Transitioning
//! 7. Bing (病) - Illness / Softening Vigor
//! 8. Si (死) - Death / Complete Standstill
//! 9. Mu (墓) - Grave / Storage & Consolidation
//! 10. Jue (絶) - Extinction / Severance & Reset
//! 11. Tai (胎) - Embryo / Gestation & Incubation
//! 12. Yang (養) - Nurture / Formative Growth

use crate::core::branch::EarthlyBranch;
use crate::core::pillars::FourPillars;
use crate::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

/// Twelve Life Stages (十二運星)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TwelveStage {
    /// Changsheng (長生) - Birth
    Changsheng = 0,
    /// Muyu (沐浴) - Bath
    Muyu = 1,
    /// Guandai (冠帶) - Crown
    Guandai = 2,
    /// Jianlu (建祿) - Prosperity
    Jianlu = 3,
    /// Diwang (帝旺) - Peak
    Diwang = 4,
    /// Shuai (衰) - Decline
    Shuai = 5,
    /// Bing (病) - Illness
    Bing = 6,
    /// Si (死) - Death
    Si = 7,
    /// Mu (墓) - Grave
    Mu = 8,
    /// Jue (絶) - Extinction
    Jue = 9,
    /// Tai (胎) - Embryo
    Tai = 10,
    /// Yang (養) - Nurture
    Yang = 11,
}

impl TwelveStage {
    /// All 12 stages array
    pub const ALL: [TwelveStage; 12] = [
        Self::Changsheng,
        Self::Muyu,
        Self::Guandai,
        Self::Jianlu,
        Self::Diwang,
        Self::Shuai,
        Self::Bing,
        Self::Si,
        Self::Mu,
        Self::Jue,
        Self::Tai,
        Self::Yang,
    ];

    /// English name notation
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Changsheng => "Birth",
            Self::Muyu => "Bath",
            Self::Guandai => "Crown",
            Self::Jianlu => "Prosperity",
            Self::Diwang => "Peak",
            Self::Shuai => "Decline",
            Self::Bing => "Illness",
            Self::Si => "Death",
            Self::Mu => "Grave",
            Self::Jue => "Extinction",
            Self::Tai => "Embryo",
            Self::Yang => "Nurture",
        }
    }

    /// English name alias
    pub const fn english(&self) -> &'static str {
        self.hangul()
    }

    /// Hanja notation
    pub const fn hanja(&self) -> &'static str {
        match self {
            Self::Changsheng => "長生",
            Self::Muyu => "沐浴",
            Self::Guandai => "冠帶",
            Self::Jianlu => "建祿",
            Self::Diwang => "帝旺",
            Self::Shuai => "衰",
            Self::Bing => "病",
            Self::Si => "死",
            Self::Mu => "墓",
            Self::Jue => "絶",
            Self::Tai => "胎",
            Self::Yang => "養",
        }
    }

    /// Energy level (0-100)
    pub const fn energy_level(&self) -> u8 {
        match self {
            Self::Diwang => 100,
            Self::Jianlu => 90,
            Self::Guandai => 70,
            Self::Changsheng => 60,
            Self::Muyu => 50,
            Self::Yang => 40,
            Self::Tai => 30,
            Self::Shuai => 20,
            Self::Bing => 15,
            Self::Si => 10,
            Self::Mu => 5,
            Self::Jue => 0,
        }
    }

    /// Root weight for grounding (通根)
    pub const fn root_weight(&self) -> f32 {
        match self {
            Self::Jianlu | Self::Diwang | Self::Guandai => 1.0,
            Self::Changsheng | Self::Muyu => 0.5,
            Self::Shuai | Self::Bing | Self::Si | Self::Mu | Self::Jue | Self::Tai | Self::Yang => {
                0.0
            }
        }
    }

    /// Check if strong stage (Grade A or B)
    pub const fn is_strong(&self) -> bool {
        matches!(
            self,
            Self::Jianlu | Self::Diwang | Self::Guandai | Self::Changsheng | Self::Muyu
        )
    }

    /// Auspicious evaluation
    pub const fn is_auspicious(&self) -> Option<bool> {
        match self {
            Self::Changsheng | Self::Guandai | Self::Jianlu | Self::Diwang => Some(true),
            Self::Bing | Self::Si | Self::Jue => Some(false),
            Self::Muyu | Self::Shuai | Self::Mu | Self::Tai | Self::Yang => None,
        }
    }

    /// Description
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Changsheng => "Emerging life, new beginnings, rising vitality",
            Self::Muyu => "Formative growth, cleansing, minor instability",
            Self::Guandai => "Coming of age, self-reliance, social entry",
            Self::Jianlu => "Established foundation, professional standing, steady prosperity",
            Self::Diwang => "Zenith of power, maximum energy and achievement",
            Self::Shuai => "Beginning of decline, wisdom and moderation",
            Self::Bing => "Softening strength, reflective and cautious",
            Self::Si => "Complete stillness, deep rest, consolidation",
            Self::Mu => "Storage, preserved latent potential, treasury",
            Self::Jue => "Severance, clean break, reset before renewal",
            Self::Tai => "Conception, gestation of fresh ideas",
            Self::Yang => "Nurturing, steady preparation under care",
        }
    }

    /// Construct from index
    pub const fn from_index(idx: i32) -> Self {
        Self::ALL[idx.rem_euclid(12) as usize]
    }
}

impl std::fmt::Display for TwelveStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hangul())
    }
}

const YANG_STEM_CHANGSHENG: [EarthlyBranch; 5] = [
    EarthlyBranch::Hai,
    EarthlyBranch::Yin,
    EarthlyBranch::Yin,
    EarthlyBranch::Si,
    EarthlyBranch::Shen,
];

const YIN_STEM_CHANGSHENG: [EarthlyBranch; 5] = [
    EarthlyBranch::Wu,
    EarthlyBranch::You,
    EarthlyBranch::You,
    EarthlyBranch::Zi,
    EarthlyBranch::Mao,
];

pub fn calculate_twelve_stage(day_stem: HeavenlyStem, branch: EarthlyBranch) -> TwelveStage {
    calculate_twelve_stage_with_config(
        day_stem,
        branch,
        &crate::core::config::AnalysisConfig::default(),
    )
}

pub fn calculate_twelve_stage_with_config(
    day_stem: HeavenlyStem,
    branch: EarthlyBranch,
    config: &crate::core::config::AnalysisConfig,
) -> TwelveStage {
    let is_yang = day_stem.index().is_multiple_of(2);
    let element_idx = (day_stem.index() / 2) as usize;

    let changsheng_branch = if is_yang {
        YANG_STEM_CHANGSHENG[element_idx]
    } else {
        YIN_STEM_CHANGSHENG[element_idx]
    };

    let branch_idx = branch.index() as i32;
    let changsheng_idx = changsheng_branch.index() as i32;

    let distance = if is_yang || !config.yin_stem_reverse {
        (branch_idx - changsheng_idx).rem_euclid(12)
    } else {
        (changsheng_idx - branch_idx).rem_euclid(12)
    };

    TwelveStage::from_index(distance)
}

/// Four Pillars Twelve Stages Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwelveStageAnalysis {
    pub day_master: HeavenlyStem,
    pub year_stage: TwelveStage,
    pub month_stage: TwelveStage,
    pub day_stage: TwelveStage,
    pub hour_stage: TwelveStage,
    pub total_energy: u8,
    pub year_self: TwelveStage,
    pub month_self: TwelveStage,
    pub day_self: TwelveStage,
    pub hour_self: TwelveStage,
}

impl TwelveStageAnalysis {
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        Self::from_pillars_with_config(pillars, &crate::core::config::AnalysisConfig::default())
    }

    pub fn from_pillars_with_config(
        pillars: &FourPillars,
        config: &crate::core::config::AnalysisConfig,
    ) -> Self {
        let day_master = pillars.day_master();

        let year_stage =
            calculate_twelve_stage_with_config(day_master, pillars.year.branch, config);
        let month_stage =
            calculate_twelve_stage_with_config(day_master, pillars.month.branch, config);
        let day_stage = calculate_twelve_stage_with_config(day_master, pillars.day.branch, config);
        let hour_stage =
            calculate_twelve_stage_with_config(day_master, pillars.hour.branch, config);

        let year_self =
            calculate_twelve_stage_with_config(pillars.year.stem, pillars.year.branch, config);
        let month_self =
            calculate_twelve_stage_with_config(pillars.month.stem, pillars.month.branch, config);
        let day_self =
            calculate_twelve_stage_with_config(pillars.day.stem, pillars.day.branch, config);
        let hour_self =
            calculate_twelve_stage_with_config(pillars.hour.stem, pillars.hour.branch, config);

        let total_energy = (year_stage.energy_level() as u32
            + month_stage.energy_level() as u32
            + day_stage.energy_level() as u32
            + hour_stage.energy_level() as u32)
            / 4;

        Self {
            day_master,
            year_stage,
            month_stage,
            day_stage,
            hour_stage,
            year_self,
            month_self,
            day_self,
            hour_self,
            total_energy: total_energy as u8,
        }
    }

    pub fn strongest_position(&self) -> (&'static str, TwelveStage) {
        let positions = [
            ("year_branch", self.year_stage),
            ("month_branch", self.month_stage),
            ("day_branch", self.day_stage),
            ("hour_branch", self.hour_stage),
        ];

        positions
            .into_iter()
            .max_by_key(|(_, stage)| stage.energy_level())
            .unwrap()
    }

    pub fn weakest_position(&self) -> (&'static str, TwelveStage) {
        let positions = [
            ("year_branch", self.year_stage),
            ("month_branch", self.month_stage),
            ("day_branch", self.day_stage),
            ("hour_branch", self.hour_stage),
        ];

        positions
            .into_iter()
            .min_by_key(|(_, stage)| stage.energy_level())
            .unwrap()
    }

    pub fn has_stage(&self, stage: TwelveStage) -> bool {
        self.year_stage == stage
            || self.month_stage == stage
            || self.day_stage == stage
            || self.hour_stage == stage
    }
}

impl std::fmt::Display for TwelveStageAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【Twelve Life Stages Analysis】")?;
        writeln!(f, "─────────────────────────────────")?;
        writeln!(
            f,
            "Day Master: {} ({})",
            self.day_master.hanja(),
            self.day_master.hangul()
        )?;
        writeln!(f)?;
        writeln!(
            f,
            "  Year Branch: {} ({}) - {}",
            self.year_stage.hangul(),
            self.year_stage.hanja(),
            self.year_stage.description()
        )?;
        writeln!(
            f,
            "  Month Branch: {} ({}) - {}",
            self.month_stage.hangul(),
            self.month_stage.hanja(),
            self.month_stage.description()
        )?;
        writeln!(
            f,
            "  Day Branch: {} ({}) - {}",
            self.day_stage.hangul(),
            self.day_stage.hanja(),
            self.day_stage.description()
        )?;
        writeln!(
            f,
            "  Hour Branch: {} ({}) - {}",
            self.hour_stage.hangul(),
            self.hour_stage.hanja(),
            self.hour_stage.description()
        )?;
        writeln!(f)?;
        writeln!(f, "【Internal Pillar Self Stages】")?;
        writeln!(
            f,
            "  Year: {} | Month: {} | Day: {} | Hour: {}",
            self.year_self.hangul(),
            self.month_self.hangul(),
            self.day_self.hangul(),
            self.hour_self.hangul()
        )?;
        writeln!(f)?;
        writeln!(f, "Total Energy: {}%", self.total_energy)?;

        let (strongest_pos, strongest_stage) = self.strongest_position();
        let (weakest_pos, weakest_stage) = self.weakest_position();
        writeln!(
            f,
            "Strongest Position: {} ({})",
            strongest_pos,
            strongest_stage.hangul()
        )?;
        writeln!(
            f,
            "Weakest Position: {} ({})",
            weakest_pos,
            weakest_stage.hangul()
        )?;

        Ok(())
    }
}

impl FourPillars {
    pub fn twelve_stages(&self) -> TwelveStageAnalysis {
        TwelveStageAnalysis::from_pillars(self)
    }

    pub fn stage_of(&self, branch: EarthlyBranch) -> TwelveStage {
        calculate_twelve_stage(self.day_master(), branch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pillars::SajuInput;

    #[test]
    fn test_user_twelve_stages() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        let analysis = pillars.twelve_stages();

        println!("{}", analysis);

        assert_eq!(analysis.year_stage, TwelveStage::Jianlu);
        assert_eq!(analysis.day_stage, TwelveStage::Shuai);
    }

    #[test]
    fn test_jia_stem() {
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Jia, EarthlyBranch::Hai),
            TwelveStage::Changsheng
        );
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Jia, EarthlyBranch::Zi),
            TwelveStage::Muyu
        );
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Jia, EarthlyBranch::Yin),
            TwelveStage::Jianlu
        );
    }

    #[test]
    fn test_geng_stem() {
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Geng, EarthlyBranch::Si),
            TwelveStage::Changsheng
        );
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Geng, EarthlyBranch::Shen),
            TwelveStage::Jianlu
        );
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Geng, EarthlyBranch::Xu),
            TwelveStage::Shuai
        );
    }

    #[test]
    fn test_yin_stem() {
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Yi, EarthlyBranch::Wu),
            TwelveStage::Changsheng
        );
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Yi, EarthlyBranch::Mao),
            TwelveStage::Jianlu
        );
    }

    #[test]
    fn test_energy_level() {
        assert_eq!(TwelveStage::Diwang.energy_level(), 100);
        assert_eq!(TwelveStage::Jue.energy_level(), 0);
    }
}
