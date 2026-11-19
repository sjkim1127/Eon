//! Structure / Pattern (格局) Analysis
//!
//! Analyzes projections of month branch hidden stems into heaven to determine chart structure.

use crate::analysis::Analyzable;
use crate::core::config::AnalysisConfig;
use crate::core::element::Polarity;
use crate::core::pillars::FourPillars;
use crate::core::stem::HeavenlyStem;
use crate::core::ten_gods::TenGod;
use serde::{Deserialize, Serialize};

/// Structure Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StructureType {
    ///  (食神格)
    ShiShen,
    ///  (傷官格)
    ShangGuan,
    ///  (偏財格)
    PianCai,
    ///  (正財格)
    ZhengCai,
    ///  (偏官格)
    PianGuan,
    ///  (正官格)
    ZhengGuan,
    ///  (偏印格)
    PianYin,
    ///  (正印格)
    ZhengYin,
    ///  (建祿格) -   
    JianLu,
    ///  (陽刃格) -   ()
    YangIn,
    /// / ( )
    Special,

    // ---  (從格) ---
    ///  (眞從兒格) -   
    JongAh,
    ///  (眞從財格) -   
    JongJae,
    ///  (眞從殺格) -   
    JongSal,
    ///  (眞從强格) -   
    JongGang,
    ///  (眞從旺格) -   
    JongWang,

    ///  (假從兒格) -  /   
    GaJongAh,
    ///  (假從財格) -  /   
    GaJongJae,
    ///  (假從殺格) -  /   
    GaJongSal,
    ///  (假從强格) -  //   
    GaJongGang,
    ///  (假從旺格) -  //   
    GaJongWang,

    ///  (化氣格) -    
    HwaGi,

    ///  (/)
    Follower,
    ///  (專旺格) -    
    SpecialTransformation,

    // ---   5 ---
    ///  (曲直格 -  )
    GokJik,
    ///  (炎上格 -  )
    YeomSang,
    ///  (稼穡格 -  )
    GaSaek,
    ///  (從革格 -  )
    JongHyeok,
    ///  (潤下格 -  )
    YoonHa,

    ///  (官殺混雜格)
    GwanSalHonJab,
}

impl StructureType {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::ShiShen => "ShiShen (Eating God)",
            Self::ShangGuan => "ShangGuan (Hurting Officer)",
            Self::PianCai => "PianCai (Indirect Wealth)",
            Self::ZhengCai => "ZhengCai (Direct Wealth)",
            Self::PianGuan => "PianGuan (Seven Killings)",
            Self::ZhengGuan => "ZhengGuan (Direct Officer)",
            Self::PianYin => "PianYin (Indirect Resource)",
            Self::ZhengYin => "ZhengYin (Direct Resource)",
            Self::JianLu => "JianLu (Prosperity)",
            Self::YangIn => "YangIn (Yang Edge)",
            Self::Special => "BiJie (Parallel)",
            Self::JongAh => "ZhenJongAh (True Follow Output)",
            Self::JongJae => "ZhenJongJae (True Follow Wealth)",
            Self::JongSal => "ZhenJongSal (True Follow Officer)",
            Self::JongGang => "ZhenJongGang (True Follow Resource)",
            Self::JongWang => "ZhenJongWang (True Follow Parallel)",
            Self::GaJongAh => "JiaJongAh (Pseudo Follow Output)",
            Self::GaJongJae => "JiaJongJae (Pseudo Follow Wealth)",
            Self::GaJongSal => "JiaJongSal (Pseudo Follow Officer)",
            Self::GaJongGang => "JiaJongGang (Pseudo Follow Resource)",
            Self::GaJongWang => "JiaJongWang (Pseudo Follow Parallel)",
            Self::HwaGi => "HwaGi (Transformed Qi)",
            Self::Follower => "JongGyeok (Follower)",
            Self::SpecialTransformation => "JeonWangGyeok (Dominant Sovereign)",
            Self::GokJik => "GokJik (Wood Sovereign)",
            Self::YeomSang => "YeomSang (Fire Sovereign)",
            Self::GaSaek => "GaSaek (Earth Sovereign)",
            Self::JongHyeok => "JongHyeok (Metal Sovereign)",
            Self::YoonHa => "YoonHa (Water Sovereign)",
            Self::GwanSalHonJab => "GwanSalHonJab (Mixed Officers)",
        }
    }

    pub const fn hanja(&self) -> &'static str {
        match self {
            Self::ShiShen => "食神格",
            Self::ShangGuan => "傷官格",
            Self::PianCai => "偏財格",
            Self::ZhengCai => "正財格",
            Self::PianGuan => "偏官格",
            Self::ZhengGuan => "正官格",
            Self::PianYin => "偏印格",
            Self::ZhengYin => "正印格",
            Self::JianLu => "建祿格",
            Self::YangIn => "陽刃格",
            Self::Special => "特殊格",
            Self::JongAh => "眞從兒格",
            Self::JongJae => "眞從財格",
            Self::JongSal => "眞從殺格",
            Self::JongGang => "眞從强格",
            Self::JongWang => "眞從旺格",
            Self::GaJongAh => "假從兒格",
            Self::GaJongJae => "假從財格",
            Self::GaJongSal => "假從殺格",
            Self::GaJongGang => "假從强格",
            Self::GaJongWang => "假從旺格",
            Self::HwaGi => "化氣格",
            Self::Follower => "從格",
            Self::SpecialTransformation => "專旺格",
            Self::GokJik => "曲直格",
            Self::YeomSang => "炎上格",
            Self::GaSaek => "稼穡格",
            Self::JongHyeok => "從革格",
            Self::YoonHa => "潤下格",
            Self::GwanSalHonJab => "官殺混雜格",
        }
    }

    /// Convert from Ten God to Structure Type (excluding Parallels)
    pub fn from_ten_god(god: TenGod) -> Option<Self> {
        match god {
            TenGod::Shishen => Some(Self::ShiShen),
            TenGod::Shangguan => Some(Self::ShangGuan),
            TenGod::Piancai => Some(Self::PianCai),
            TenGod::Zhengcai => Some(Self::ZhengCai),
            TenGod::Pianguan => Some(Self::PianGuan),
            TenGod::Zhengguan => Some(Self::ZhengGuan),
            TenGod::Pianyin => Some(Self::PianYin),
            TenGod::Zhengyin => Some(Self::ZhengYin),
            _ => None,
        }
    }
}

/// Structure Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureAnalysis {
    /// Determined Structure
    pub structure: StructureType,
    /// Projected Stem (if present)
    pub projected_stem: Option<HeavenlyStem>,
    /// Projection Position (year_stem, month_stem, hour_stem)
    pub projection_path: Option<String>,
    /// Structure Summary
    pub summary: String,
    /// Structure Description
    pub description: String,
    /// Structure Grounds / Reasons
    pub reasons: Vec<String>,
}

impl StructureAnalysis {
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        Self::from_pillars_with_config(pillars, &AnalysisConfig::default())
    }

    pub fn from_pillars_with_config(pillars: &FourPillars, config: &AnalysisConfig) -> Self {
        let dm = pillars.day_master();
        let month_branch = pillars.month.branch;
        let hidden_stems = month_branch.hidden_stems();

        let mut other_stems = vec![
            ("year_stem", pillars.year.stem),
            ("month_stem", pillars.month.stem),
        ];
        if let Some(hour) = pillars.hour {
            other_stems.push(("hour_stem", hour.stem));
        }

        // -1. (HwaGi)   ( 破星  / )
        let mut hwagi_result = None;
        let mut adjacent_stems = vec![("month_stem", pillars.month.stem)];
        if let Some(hour) = pillars.hour {
            adjacent_stems.push(("hour_stem", hour.stem));
        }
        for (path, stem_on_top) in &adjacent_stems {
            if let Some(combo) =
                crate::analysis::relationships::StemCombination::check(dm, *stem_on_top)
            {
                let transformed_elem = combo.transformed_element();
                if month_branch.element() == transformed_elem {
                    // 1)    (破星)
                    let breaker_elem = transformed_elem.controlled_by();
                    let mut all_stems =
                        vec![pillars.year.stem, pillars.month.stem, pillars.day.stem];
                    all_stems.extend(pillars.hour.map(|h| h.stem));
                    let has_break_star = all_stems.iter().any(|s| s.element() == breaker_elem);

                    // 2) /
                    let same_dm_count = all_stems.iter().filter(|&&s| s == dm).count();
                    let same_partner_count =
                        all_stems.iter().filter(|&&s| s == *stem_on_top).count();
                    let is_competing = same_dm_count > 1 || same_partner_count > 1;

                    if !has_break_star && !is_competing {
                        hwagi_result = Some((path, combo, transformed_elem, *stem_on_top));
                        break;
                    }
                }
            }
        }

        if let Some((path, combo, transformed_elem, matched_stem)) = hwagi_result {
            return Self {
                structure: StructureType::HwaGi,
                projected_stem: Some(matched_stem),
                projection_path: Some(path.to_string()),
                summary: format!("HwaGi structure where Day Master combines with {} via {} transforming to {}", path, combo.hangul(), transformed_elem.hangul()),
                description: "Special chart where Day Master combines with adjacent stem and month branch seasonal Qi solidly supports complete transformation.".to_string(),
                reasons: vec![
                    format!("Day Master {} and {} {} form {}", dm.hanja(), path, matched_stem.hanja(), combo.hanja()),
                    format!("Transformed element ({}) matches month branch {}", transformed_elem.hanja(), month_branch.hanja()),
                    "Verified absence of breaker star and rival/jealous combinations".to_string(),
                ],
            };
        }

        // 0.  (/)
        let strength = pillars.strength_with_config(config);
        let is_polarized = strength.deuk_se.support_ratio >= config.strength.polarized_high
            || strength.deuk_se.support_ratio <= config.strength.polarized_low;

        if is_polarized {
            let mut all_branches = vec![
                pillars.year.branch,
                pillars.month.branch,
                pillars.day.branch,
            ];
            all_branches.extend(pillars.hour.map(|h| h.branch));

            //    ()   ( 眞從 vs  假從 )
            let dm_el = dm.element();
            let yin_el = dm_el.generated_by();
            let has_dm_root = all_branches.iter().any(|b| {
                b.hidden_stems()
                    .iter()
                    .any(|hs| hs.element() == dm_el || hs.element() == yin_el)
            });

            if strength.deuk_se.support_ratio >= config.strength.polarized_high {
                // /   5 (, , , , )
                use crate::core::branch::EarthlyBranch::*;
                use crate::core::element::Element;

                let (structure, name, desc) = match dm_el {
                    Element::Wood if matches!(month_branch, Yin | Mao | Chen | Hai) => (
                        StructureType::GokJik,
                        "GokJik (Wood Sovereign)",
                        "Wood vitality forms a dominant sovereign structure, bestowing benevolence, scholarly excellence, and artistic vision.",
                    ),
                    Element::Fire if matches!(month_branch, Si | Wu | Wei | Yin) => (
                        StructureType::YeomSang,
                        "YeomSang (Fire Sovereign)",
                        "Fire vitality forms a dominant sovereign structure, bestowing charisma, honor, and brilliant strategic expression.",
                    ),
                    Element::Earth if matches!(month_branch, Chen | Xu | Chou | Wei) => (
                        StructureType::GaSaek,
                        "GaSaek (Earth Sovereign)",
                        "Earth vitality forms a dominant sovereign structure, bestowing steady trustworthiness, immense tolerance, and solid capital resilience.",
                    ),
                    Element::Metal if matches!(month_branch, Shen | You | Xu | Si) => (
                        StructureType::JongHyeok,
                        "JongHyeok (Metal Sovereign)",
                        "Metal vitality forms a dominant sovereign structure, bestowing razor-sharp decisiveness, executive fortitude, and supreme authority.",
                    ),
                    Element::Water if matches!(month_branch, Hai | Zi | Chou | Shen) => (
                        StructureType::YoonHa,
                        "YoonHa (Water Sovereign)",
                        "Water vitality forms a dominant sovereign structure, bestowing profound philosophical wisdom and boundless adaptability.",
                    ),
                    _ => {
                        let yinxing = strength.deuk_se.yinxing_count;
                        let bijie = strength.deuk_se.bijie_count;
                        if bijie >= yinxing {
                            (
                                StructureType::JongWang,
                                "ZhenJongWang (True Follow Parallel)",
                                "Parallel vitality is exceedingly strong, forming a dominant sovereign pattern that preserves its own momentum.",
                            )
                        } else {
                            (
                                StructureType::JongGang,
                                "ZhenJongGang (True Follow Resource)",
                                "Resource vitality is exceedingly strong, forming a dominant sovereign pattern that thrives on following resource momentum.",
                            )
                        }
                    }
                };

                return Self {
                    structure,
                    projected_stem: None,
                    projection_path: None,
                    summary: format!(
                        "Dominant Sovereign structure following supreme momentum: {}",
                        name
                    ),
                    description: desc.to_string(),
                    reasons: vec![
                        format!(
                            "Day Master power ratio: {:.1}%",
                            strength.deuk_se.support_ratio
                        ),
                        format!(
                            "Sovereign threshold: >= {:.1}%",
                            config.strength.polarized_high
                        ),
                    ],
                };
            } else {
                use crate::analysis::power::{AnalysisOptions, IntegratedAnalysis};
                let options = AnalysisOptions {
                    apply_transform: false,
                    apply_correction: true,
                };
                let integrated = IntegratedAnalysis::calculate(pillars, options, config);

                let mut shishang_power = 0.0f32;
                let mut cai_power = 0.0f32;
                let mut guan_power = 0.0f32;

                for (tg, pct, _) in integrated.ten_god_scores {
                    match tg {
                        TenGod::Shishen | TenGod::Shangguan => shishang_power += pct,
                        TenGod::Zhengcai | TenGod::Piancai => cai_power += pct,
                        TenGod::Zhengguan | TenGod::Pianguan => guan_power += pct,
                        _ => {}
                    }
                }

                let (structure, name, desc) = if shishang_power >= cai_power
                    && shishang_power >= guan_power
                {
                    if has_dm_root {
                        (
                            StructureType::GaJongAh,
                            "JiaJongAh (Pseudo Follow Output)",
                            "Pseudo follow structure following output momentum despite residual faint roots.",
                        )
                    } else {
                        (
                            StructureType::JongAh,
                            "ZhenJongAh (True Follow Output)",
                            "True follow structure purely following output momentum with zero day master roots.",
                        )
                    }
                } else if cai_power >= shishang_power && cai_power >= guan_power {
                    if has_dm_root {
                        (
                            StructureType::GaJongJae,
                            "JiaJongJae (Pseudo Follow Wealth)",
                            "Pseudo follow structure following wealth momentum despite residual faint roots.",
                        )
                    } else {
                        (
                            StructureType::JongJae,
                            "ZhenJongJae (True Follow Wealth)",
                            "True follow structure purely following wealth momentum with zero day master roots.",
                        )
                    }
                } else {
                    if has_dm_root {
                        (
                            StructureType::GaJongSal,
                            "JiaJongSal (Pseudo Follow Officer)",
                            "Pseudo follow structure following officer momentum despite residual faint roots.",
                        )
                    } else {
                        (
                            StructureType::JongSal,
                            "ZhenJongSal (True Follow Officer)",
                            "True follow structure purely following officer momentum with zero day master roots.",
                        )
                    }
                };

                return Self {
                    structure,
                    projected_stem: None,
                    projection_path: None,
                    summary: format!(
                        "Follower structure following overwhelming momentum: {}",
                        name
                    ),
                    description: desc.to_string(),
                    reasons: vec![
                        format!(
                            "Day Master power ratio: {:.1}%",
                            strength.deuk_se.support_ratio
                        ),
                        format!(
                            "Follower threshold: <= {:.1}%",
                            config.strength.polarized_low
                        ),
                        format!(
                            "Day Master / Resource Root: {}",
                            if has_dm_root {
                                "Present (Pseudo Follow)"
                            } else {
                                "None (True Follow)"
                            }
                        ),
                    ],
                };
            }
        }

        // 0.5. (官殺混雜格)
        // //     (/)
        let mut exposed_stems = vec![pillars.year.stem, pillars.month.stem];
        exposed_stems.extend(pillars.hour.map(|h| h.stem));
        let exposed_tgs: Vec<TenGod> = exposed_stems
            .iter()
            .map(|&s| TenGod::from_stems(dm, s))
            .collect();
        let has_zhengguan = exposed_tgs.contains(&TenGod::Zhengguan);
        let has_pianguan = exposed_tgs.contains(&TenGod::Pianguan);
        if has_zhengguan && has_pianguan {
            return Self {
                structure: StructureType::GwanSalHonJab,
                projected_stem: None,
                projection_path: None,
                summary: "Mixed Officers (GwanSalHonJab) with both Direct and Seven Killings projected".to_string(),
                description: "Both Direct Officer and Seven Killings appear in heaven simultaneously, requiring clarity in professional focus.".to_string(),
                reasons: vec![
                    "Both Direct Officer and Seven Killings projected into heaven".to_string(),
                    "Mixed Officer configuration formed".to_string(),
                ],
            };
        }

        // 1. /
        let stage = crate::core::twelve_stages::calculate_twelve_stage(dm, month_branch);
        if stage == crate::core::twelve_stages::TwelveStage::Jianlu {
            return Self {
                structure: StructureType::JianLu,
                projected_stem: None,
                projection_path: None,
                summary: "JianLu structure deriving prime vitality from month branch".to_string(),
                description: "Born during the Day Master robust prosperity stage, granting strong self-determination and self-made capacity.".to_string(),
                reasons: vec![format!("Day Master {} in month branch {} is at JianLu prosperity stage", dm.hanja(), month_branch.hanja())],
            };
        }
        if stage == crate::core::twelve_stages::TwelveStage::Diwang
            && dm.polarity() == Polarity::Yang
        {
            return Self {
                structure: StructureType::YangIn,
                projected_stem: None,
                projection_path: None,
                summary: "YangIn structure possessing supreme peak intensity".to_string(),
                description:
                    "Peak energy resembles wielding a keen blade; channeled effectively, it commands high authority."
                        .to_string(),
                reasons: vec![format!(
                    "Yang stem {} in month branch {} is at Diwang peak stage",
                    dm.hanja(),
                    month_branch.hanja()
                )],
            };
        }

        // 2. (透出)  - ()
        for stem_in_branch in hidden_stems.iter().rev() {
            for (path, stem_on_top) in &other_stems {
                if stem_in_branch == stem_on_top {
                    let god = TenGod::from_stems(dm, *stem_on_top);
                    if let Some(structure) = StructureType::from_ten_god(god) {
                        return Self {
                            structure,
                            projected_stem: Some(*stem_on_top),
                            projection_path: Some(path.to_string()),
                            summary: format!("{} structure projected from hidden stem to {}", path, structure.hangul()),
                            description: "Hidden stem of the month branch emerges into heaven, defining core structure.".to_string(),
                            reasons: vec![format!("Month branch {} hidden stem {} projected to {} {}", month_branch.hanja(), stem_in_branch.hanja(), path, stem_on_top.hanja())],
                        };
                    }
                }
            }
        }

        // 3.     ()  ( )
        let primary_stem = month_branch.primary_stem();
        let god = TenGod::from_stems(dm, primary_stem);
        let structure = StructureType::from_ten_god(god).unwrap_or(StructureType::Special);

        Self {
            structure,
            projected_stem: None,
            projection_path: None,
            summary: format!(
                "{} structure anchored on month branch primary stem",
                structure.hangul()
            ),
            description:
                "No projected stems into heaven; primary seasonal Qi forms core structure."
                    .to_string(),
            reasons: vec![format!(
                "Determined by month branch primary stem {} due to absence of heaven projection",
                primary_stem.hanja()
            )],
        }
    }
}

impl std::fmt::Display for StructureAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【Structure (格局) Analysis】")?;
        writeln!(f, "─────────────────────────────────")?;
        writeln!(
            f,
            "▶ {} ({})",
            self.structure.hangul(),
            self.structure.hanja()
        )?;
        writeln!(f, "  Summary: {}", self.summary)?;
        writeln!(f, "  Description: {}", self.description)?;
        if !self.reasons.is_empty() {
            writeln!(f, "  Basis: {}", self.reasons.join(", "))?;
        }
        Ok(())
    }
}

impl FourPillars {
    /// Structure analysis
    pub fn structure(&self) -> StructureAnalysis {
        StructureAnalysis::from_pillars(self)
    }

    /// Structure analysis with config
    pub fn structure_with_config(&self, config: &AnalysisConfig) -> StructureAnalysis {
        StructureAnalysis::from_pillars_with_config(self, config)
    }
}

impl Analyzable for StructureAnalysis {
    type Output = StructureAnalysis;
    fn analyze(pillars: &FourPillars, config: &AnalysisConfig) -> Self::Output {
        StructureAnalysis::from_pillars_with_config(pillars, config)
    }
}
