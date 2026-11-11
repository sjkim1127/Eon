//! Day Master Strength (身强身弱) Determination
//!
//! (日干)  .
//!
//! ##  
//!
//! 1. **(得令)** -   (生)  (比)
//! 2. **(得地)** -   ()
//! 3. **(得時)** -   (生)  (比)
//! 4. **(得勢)** - (比劫)/(印星)
//!
//! ##  
//!
//! - 4  2   → (身强)
//! - 4  1   → (身弱)

use crate::analysis::relationships::RelationshipAnalysis;
use crate::core::branch::EarthlyBranch;
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use crate::core::stem::HeavenlyStem;
use crate::core::ten_gods::TenGod;
use serde::{Deserialize, Serialize};

use crate::core::config::AnalysisConfig;

/// Strength Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StrengthType {
    /// (身强) -  
    Strong,
    /// (身弱) -  
    Weak,
    /// (中和) -    ()
    Balanced,
}

impl StrengthType {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Strong => "Strong",
            Self::Weak => "Weak",
            Self::Balanced => "Balanced",
        }
    }

    pub const fn hanja(&self) -> &'static str {
        match self {
            Self::Strong => "身强",
            Self::Weak => "身弱",
            Self::Balanced => "中和",
        }
    }

    pub const fn description(&self) -> &'static str {
        match self {
            Self::Strong => "Day Master is strong; outputs, wealth, or officers are favorable.",
            Self::Weak => "Day Master is weak; resources or parallels are favorable.",
            Self::Balanced => "Day Master and surrounding elements are well balanced.",
        }
    }
}

impl std::fmt::Display for StrengthType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hangul())
    }
}

/// Obtain Season (得令) Determination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeukRyeong {
    /// Acquired season
    pub acquired: bool,
    /// Month Branch
    pub month_branch: EarthlyBranch,
    /// Month Branch
    pub month_element: Element,
    /// Day Master Element
    pub day_master_element: Element,
}

impl DeukRyeong {
    /// Determine Obtain Season
    /// Month Branch     
    pub fn check(day_master: HeavenlyStem, month_branch: EarthlyBranch) -> Self {
        let day_element = day_master.element();
        let month_element = month_branch.element();

        //   (生) (比)
        let is_generating = month_element.generates() == day_element;
        let is_same = month_element == day_element;
        let acquired = is_generating || is_same;

        Self {
            acquired,
            month_branch,
            month_element,
            day_master_element: day_element,
        }
    }

    /// Checks DeukRyeong using effective transformed element.
    pub fn check_effective(
        day_master: HeavenlyStem,
        month_branch: EarthlyBranch,
        month_eff: Element,
    ) -> Self {
        let day_element = day_master.element();

        //     (生) (比)
        let is_generating = month_eff.generates() == day_element;
        let is_same = month_eff == day_element;
        let acquired = is_generating || is_same;

        Self {
            acquired,
            month_branch,
            month_element: month_eff,
            day_master_element: day_element,
        }
    }

    /// Relationship description
    pub fn relation(&self) -> &'static str {
        let is_generating = self.month_element.generates() == self.day_master_element;
        let is_same = self.month_element == self.day_master_element;

        if is_generating {
            "Month branch generates Day Master (Generates)"
        } else if is_same {
            "Month branch shares Day Master element (Same)"
        } else if self.day_master_element.generates() == self.month_element {
            "Day Master generates month branch (Exhausts)"
        } else if self.day_master_element.controls() == self.month_element {
            "Day Master controls month branch (Controls)"
        } else {
            "Month branch controls Day Master (Controlled)"
        }
    }
}

/// Obtain Ground (得地) Determination
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeukJi {
    /// Acquired ground
    pub acquired: bool,
    /// Root count
    pub root_count: u8,
    /// Root weight score sum
    pub root_score: f32,
    /// Root positions
    pub root_positions: Vec<String>,
    /// Strong 12-stage count
    pub strong_stage_count: u8,
    /// 12-stage weight sum
    pub stage_weight_sum: f32,
}

impl DeukJi {
    /// Determine Obtain Ground
    pub fn check(pillars: &FourPillars, config: &AnalysisConfig) -> Self {
        let relations = RelationshipAnalysis::from_pillars(pillars);
        Self::check_with_relations(pillars, &relations, config)
    }

    /// Determine Obtain Ground reflecting interactions
    pub fn check_with_relations(
        pillars: &FourPillars,
        relations: &RelationshipAnalysis,
        config: &AnalysisConfig,
    ) -> Self {
        let day_element = pillars.day_master().element();

        let branches = [
            ("year_branch", pillars.year.branch),
            ("month_branch", pillars.month.branch),
            ("day_branch", pillars.day.branch),
            ("hour_branch", pillars.hour.branch),
        ];

        let mut root_count = 0;
        let mut root_score = 0.0;
        let mut root_positions = Vec::new();
        let mut strong_stage_count = 0;
        let mut stage_weight_sum = 0.0_f32;

        //     (  )
        use std::collections::HashMap;
        let mut clash_info: HashMap<&str, &crate::analysis::relationships::BranchClash> =
            HashMap::new();
        for (clash, pos1, pos2) in &relations.branch_clashes {
            clash_info.insert(pos1.as_str(), clash);
            clash_info.insert(pos2.as_str(), clash);
        }

        for (name, branch) in &branches {
            let base_weight = match *name {
                "month_branch" => config.weights.month_branch,
                "day_branch" => config.weights.day_branch,
                _ => config.weights.other_branch, // Year and hour branches
            };

            // (Clash)
            let (weight, clash_label) = if let Some(clash) = clash_info.get(*name) {
                let damage = clash.damage_ratio();
                let adjusted = base_weight * (1.0 - damage);
                (
                    adjusted,
                    Some(format!("{} ({})", name, clash.clash_type().hangul())),
                )
            } else {
                (base_weight, None)
            };

            // (Hidden Stems)
            let hidden_stems = branch.hidden_stems();
            let mut branch_root_score = 0.0;
            let saryeong = if *name == "month_branch" {
                pillars.saryeong().ok()
            } else {
                None
            };

            for (idx, stem) in hidden_stems.iter().enumerate() {
                if stem.element() == day_element {
                    //     (: 1.0, : 0.6, : 0.3)
                    let part_weight = if idx == hidden_stems.len() - 1 {
                        config.root.main_root
                    } else if hidden_stems.len() == 3 && idx == 1 {
                        config.root.middle_root
                    } else {
                        config.root.remain_root
                    };

                    // (Commanding)   ( ,  20% )
                    let saryeong_multiplier = if let Some(ref s) = saryeong {
                        if s.commanding_stem == *stem {
                            config.root.saryeong_bonus
                        } else {
                            1.0
                        }
                    } else {
                        1.0
                    };

                    branch_root_score += (weight * part_weight * saryeong_multiplier).min(weight);
                }
            }

            if branch_root_score > 0.0 {
                root_count += 1;
                root_score += branch_root_score;
                if let Some(label) = clash_label {
                    root_positions.push(label);
                } else {
                    root_positions.push(name.to_string());
                }
            }

            // 12
            let stage =
                crate::core::twelve_stages::calculate_twelve_stage(pillars.day_master(), *branch);

            // 12   (A=1.0, B=0.5, C=0.0)
            stage_weight_sum += stage.root_weight();

            //  12  (A  B)
            if stage.is_strong() {
                strong_stage_count += 1;
            }
        }

        //  :
        // 1) 12   1.0  (A 1  B 2 )
        // 2)
        let acquired = stage_weight_sum >= 1.0 || root_score >= config.root.min_deuk_ji_score;

        Self {
            acquired,
            root_count,
            root_score,
            root_positions,
            strong_stage_count,
            stage_weight_sum,
        }
    }
}

/// Obtain Time (得時) Determination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeukSi {
    /// Acquired time
    pub acquired: bool,
    /// Hour Branch
    pub hour_branch: EarthlyBranch,
    /// Hour Branch
    pub hour_element: Element,
    /// Day Master Element
    pub day_master_element: Element,
}

impl DeukSi {
    /// Determine Obtain Time
    /// Hour Branch     
    pub fn check(day_master: HeavenlyStem, hour_branch: EarthlyBranch) -> Self {
        let day_element = day_master.element();
        let hour_element = hour_branch.element();

        //   (生) (比)
        let is_generating = hour_element.generates() == day_element;
        let is_same = hour_element == day_element;
        let acquired = is_generating || is_same;

        Self {
            acquired,
            hour_branch,
            hour_element,
            day_master_element: day_element,
        }
    }

    /// Checks DeukSi using effective transformed element.
    pub fn check_effective(
        day_master: HeavenlyStem,
        hour_branch: EarthlyBranch,
        hour_eff: Element,
    ) -> Self {
        let day_element = day_master.element();

        //     (生) (比)
        let is_generating = hour_eff.generates() == day_element;
        let is_same = hour_eff == day_element;
        let acquired = is_generating || is_same;

        Self {
            acquired,
            hour_branch,
            hour_element: hour_eff,
            day_master_element: day_element,
        }
    }

    /// Relationship description
    pub fn relation(&self) -> &'static str {
        let is_generating = self.hour_element.generates() == self.day_master_element;
        let is_same = self.hour_element == self.day_master_element;

        if is_generating {
            "Hour branch generates Day Master (Generates)"
        } else if is_same {
            "Hour branch shares Day Master element (Same)"
        } else if self.day_master_element.generates() == self.hour_element {
            "Day Master generates hour branch (Exhausts)"
        } else if self.day_master_element.controls() == self.hour_element {
            "Day Master controls hour branch (Controls)"
        } else {
            "Hour branch controls Day Master (Controlled)"
        }
    }
}

/// Obtain Power (得勢) Determination
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeukSe {
    /// Acquired power
    pub acquired: bool,
    /// Parallel count
    pub bijie_count: u8,
    /// Resource count
    pub yinxing_count: u8,
    /// Output count
    pub shishang_count: u8,
    /// Wealth count
    pub caisheng_count: u8,
    /// Officer count
    pub guanxing_count: u8,
    /// Parallels + Resources ratio (%)
    pub support_ratio: f32,
}

impl DeukSe {
    /// Determine Obtain Power
    /// + ++  
    pub fn check(pillars: &FourPillars, config: &AnalysisConfig) -> Self {
        Self::check_with_options(pillars, false, config)
    }

    ///     (Input vs Output   )
    pub fn check_with_options(
        pillars: &FourPillars,
        apply_transform: bool,
        config: &AnalysisConfig,
    ) -> Self {
        use crate::analysis::power::{AnalysisOptions, IntegratedAnalysis};
        let options = AnalysisOptions {
            apply_transform,
            apply_correction: true,
        };
        let integrated = IntegratedAnalysis::calculate(pillars, options, config);

        let mut bijie_count = 0;
        let mut yinxing_count = 0;
        let mut shishang_count = 0;
        let mut caisheng_count = 0;
        let mut guanxing_count = 0;

        let mut support_ratio = 0.0;

        for (ten_god, percentage, _) in integrated.ten_god_scores {
            if percentage > 0.0 {
                match ten_god {
                    TenGod::Bijian | TenGod::Jiecai => {
                        bijie_count += 1;
                        support_ratio += percentage;
                    }
                    TenGod::Zhengyin | TenGod::Pianyin => {
                        yinxing_count += 1;
                        support_ratio += percentage;
                    }
                    TenGod::Shishen | TenGod::Shangguan => shishang_count += 1,
                    TenGod::Zhengcai | TenGod::Piancai => caisheng_count += 1,
                    TenGod::Zhengguan | TenGod::Pianguan => guanxing_count += 1,
                }
            }
        }

        let acquired = support_ratio >= 50.0;

        Self {
            acquired,
            bijie_count,
            yinxing_count,
            shishang_count,
            caisheng_count,
            guanxing_count,
            support_ratio,
        }
    }
}

/// Strength Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrengthAnalysis {
    /// Day Master
    pub day_master: HeavenlyStem,
    /// Strength Type
    pub strength_type: StrengthType,
    /// Determine Obtain Season
    pub deuk_ryeong: DeukRyeong,
    /// Obtain Ground
    pub deuk_ji: DeukJi,
    /// Determine Obtain Time
    pub deuk_si: DeukSi,
    /// Determine Obtain Power
    pub deuk_se: DeukSe,
    /// Acquired count (0-4)
    pub acquired_count: u8,
    /// Strength score (0-100, 50 baseline)
    pub strength_score: f32,
}

impl StrengthAnalysis {
    /// Strength analysis from Four Pillars
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        Self::from_pillars_with_options(pillars, true, &AnalysisConfig::default())
    }

    /// Strength analysis from Four Pillars with options
    pub fn from_pillars_with_options(
        pillars: &FourPillars,
        apply_transform: bool,
        config: &AnalysisConfig,
    ) -> Self {
        let day_master = pillars.day_master();
        let relations = RelationshipAnalysis::from_pillars(pillars);
        let eff_map = if apply_transform {
            Some(pillars.effective_elements())
        } else {
            None
        };

        let deuk_ryeong = if let Some(map) = eff_map {
            DeukRyeong::check_effective(day_master, pillars.month.branch, map[5].1)
        } else {
            DeukRyeong::check(day_master, pillars.month.branch)
        };

        let deuk_ji = DeukJi::check_with_relations(pillars, &relations, config);

        let deuk_si = if let Some(map) = eff_map {
            DeukSi::check_effective(day_master, pillars.hour.branch, map[7].1)
        } else {
            DeukSi::check(day_master, pillars.hour.branch)
        };

        let deuk_se = DeukSe::check_with_options(pillars, apply_transform, config);

        //    (4)
        let acquired_count = (deuk_ryeong.acquired as u8)
            + (deuk_ji.acquired as u8)
            + (deuk_si.acquired as u8)
            + (deuk_se.acquired as u8);

        let strength_type = match acquired_count {
            2..=4 => StrengthType::Strong,
            1 | 0 => StrengthType::Weak,
            _ => StrengthType::Balanced,
        };

        let score_ryeong = if deuk_ryeong.acquired {
            config.strength.criteria_score
        } else {
            0.0
        };
        let score_ji = if deuk_ji.acquired {
            config.strength.criteria_score
        } else {
            0.0
        };
        let score_si = if deuk_si.acquired {
            config.strength.criteria_score
        } else {
            0.0
        };
        let score_se = deuk_se.support_ratio * config.strength.deuk_se_weight;
        let strength_score = score_ryeong + score_ji + score_si + score_se;

        Self {
            day_master,
            strength_type,
            deuk_ryeong,
            deuk_ji,
            deuk_si,
            deuk_se,
            acquired_count,
            strength_score,
        }
    }

    /// Recommended Yongshin element groups
    pub fn recommend_yongshin(&self) -> Vec<Element> {
        let dm_element = self.day_master.element();
        match self.strength_type {
            StrengthType::Strong => {
                //  /  /
                vec![
                    dm_element.generates(),     // Output
                    dm_element.controls(),      // Wealth
                    dm_element.controlled_by(), // Officer
                ]
            }
            StrengthType::Weak => {
                //  /
                vec![
                    dm_element.generated_by(), // Resource
                    dm_element,                // Parallel
                ]
            }
            StrengthType::Balanced => {
                vec![]
            }
        }
    }
}

impl std::fmt::Display for StrengthAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【Day Master Strength (身强身弱) Analysis】")?;
        writeln!(f, "─────────────────────────────────")?;
        writeln!(
            f,
            "Day Master: {} ({})",
            self.day_master.hanja(),
            self.day_master.element().hangul()
        )?;
        writeln!(f)?;

        let ryeong_mark = if self.deuk_ryeong.acquired {
            "○"
        } else {
            "✗"
        };
        writeln!(
            f,
            "Obtain Season (得令): {} - {}",
            ryeong_mark,
            self.deuk_ryeong.relation()
        )?;

        let ji_mark = if self.deuk_ji.acquired { "○" } else { "✗" };
        writeln!(
            f,
            "Obtain Ground (得地): {} - Roots {}, Strong stages {}",
            ji_mark, self.deuk_ji.root_count, self.deuk_ji.strong_stage_count
        )?;

        let si_mark = if self.deuk_si.acquired { "○" } else { "✗" };
        writeln!(
            f,
            "Obtain Time (得時): {} - {}",
            si_mark,
            self.deuk_si.relation()
        )?;

        let se_mark = if self.deuk_se.acquired { "○" } else { "✗" };
        writeln!(
            f,
            "Obtain Power (得勢): {} - Parallels+Resources {:.1}%",
            se_mark, self.deuk_se.support_ratio
        )?;

        writeln!(f)?;
        writeln!(f, "【Determination Result】")?;
        writeln!(f, "Conditions Met: {}/4", self.acquired_count)?;
        writeln!(f, "Overall Score: {:.1}%", self.strength_score)?;
        writeln!(f)?;
        writeln!(
            f,
            "▶ {} ({})",
            self.strength_type.hangul(),
            self.strength_type.hanja()
        )?;
        writeln!(f, "  {}", self.strength_type.description())?;

        writeln!(f)?;
        let yongshin_names: Vec<String> = self
            .recommend_yongshin()
            .iter()
            .map(|e| e.hangul().to_string())
            .collect();
        writeln!(f, "Recommended Elements: {}", yongshin_names.join(", "))?;

        Ok(())
    }
}

// ============================================
// FourPillars
// ============================================

impl FourPillars {
    /// Day Master Strength analysis
    pub fn strength(&self) -> StrengthAnalysis {
        StrengthAnalysis::from_pillars(self)
    }

    /// Strength analysis with config
    pub fn strength_with_config(&self, config: &AnalysisConfig) -> StrengthAnalysis {
        StrengthAnalysis::from_pillars_with_options(self, true, config)
    }

    /// Whether Strong
    pub fn is_strong(&self) -> bool {
        self.strength().strength_type == StrengthType::Strong
    }

    /// Whether Weak
    pub fn is_weak(&self) -> bool {
        self.strength().strength_type == StrengthType::Weak
    }
}

use crate::analysis::Analyzable;

impl Analyzable for StrengthAnalysis {
    type Output = StrengthAnalysis;
    fn analyze(pillars: &FourPillars, config: &AnalysisConfig) -> Self::Output {
        StrengthAnalysis::from_pillars_with_options(pillars, true, config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pillars::SajuInput;

    #[test]
    fn test_user_strength() {
        //  : 甲申年 乙亥月 庚戌日 丁亥時
        //  庚(),  亥(),  亥()
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        let analysis = pillars.strength();

        println!("{}", analysis);

        //   :
        // : ✗ (亥水 庚金 )
        // : ○ (申 )
        // : ✗ (亥水 庚金 )
        // : ✗
        assert!(!analysis.deuk_ryeong.acquired);
        assert!(analysis.deuk_ji.acquired);
        assert!(!analysis.deuk_si.acquired);
        assert_eq!(analysis.strength_type, StrengthType::Weak);
    }

    #[test]
    fn test_deuk_ryeong() {
        // 庚, 申 →  ( )
        let dr = DeukRyeong::check(HeavenlyStem::Geng, EarthlyBranch::Shen);
        assert!(dr.acquired);

        // 庚, 亥 →  ( →  )
        let dr2 = DeukRyeong::check(HeavenlyStem::Geng, EarthlyBranch::Hai);
        assert!(!dr2.acquired);
    }

    #[test]
    fn test_deuk_si() {
        // 庚, 申 →  ( )
        let ds = DeukSi::check(HeavenlyStem::Geng, EarthlyBranch::Shen);
        assert!(ds.acquired);

        // 庚, 亥 →  ( →  )
        let ds2 = DeukSi::check(HeavenlyStem::Geng, EarthlyBranch::Hai);
        assert!(!ds2.acquired);
    }

    #[test]
    fn test_four_criteria() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        let analysis = pillars.strength();

        // 4
        println!("Obtain Season: {}", analysis.deuk_ryeong.acquired);
        println!("Obtain Ground: {}", analysis.deuk_ji.acquired);
        println!("Obtain Time: {}", analysis.deuk_si.acquired);
        println!("Obtain Power: {}", analysis.deuk_se.acquired);
        println!("Total Acquired: {}/4", analysis.acquired_count);
    }
}
