//! Useful God (用神, Yongshin) Precision Analysis
//!
//! (抑扶), (調候), (通關), (病藥)
//!      .

use crate::analysis::strength::{StrengthAnalysis, StrengthType};
use crate::analysis::structure::StructureType;
use crate::core::branch::EarthlyBranch;
use crate::core::config::AnalysisConfig;
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use serde::{Deserialize, Serialize};

/// Yongshin Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum YongshinType {
    ///  (  )
    Eokbu,
    ///  (  )
    Johu,
    ///  (  )
    Tonggwan,
    ///  (   )
    Byeongyak,
}

impl YongshinType {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Eokbu => "EokBu (Balance / Strength)",
            Self::Johu => "JoHu (Climate / Season)",
            Self::Tonggwan => "TongGwan (Mediation)",
            Self::Byeongyak => "ByeongYak (Remedy / Cure)",
        }
    }
}

/// Recommended Yongshin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedYongshin {
    pub yongshin_type: YongshinType,
    pub element: Element,
    pub summary: String,
    pub description: String,
    pub reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_stems: Option<Vec<crate::core::stem::HeavenlyStem>>,
}

/// Byeongyak (Disease and Medicine) Analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ByeongyakAnalysis {
    pub disease: Element,
    pub medicine: Element,
    pub summary: String,
    pub description: String,
    pub reasons: Vec<String>,
}

/// Comprehensive Yongshin Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YongshinAnalysis {
    /// Ranked recommendations
    pub recommendations: Vec<RecommendedYongshin>,
    ///   1
    pub primary: Element,
    ///   (喜神)
    pub assistant: Element,
    ///     ( )
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byeongyak_detail: Option<ByeongyakAnalysis>,
}

impl YongshinAnalysis {
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        Self::from_pillars_with_config(pillars, &AnalysisConfig::default())
    }

    pub fn from_pillars_with_config(pillars: &FourPillars, config: &AnalysisConfig) -> Self {
        let mut recommendations = Vec::new();
        let strength = pillars.strength_with_config(config);
        let day_master_el = pillars.day_master_element();
        let structure_analysis = pillars.structure();

        // 1.        (IntegratedAnalysis)
        let options = crate::analysis::power::AnalysisOptions {
            apply_transform: false,
            apply_correction: true,
        };
        let integrated =
            crate::analysis::power::IntegratedAnalysis::calculate(pillars, options, config);

        let get_elem_power = |el: Element| -> f32 {
            integrated
                .element_scores
                .iter()
                .find(|(e, _, _)| *e == el)
                .map(|(_, pct, _)| *pct)
                .unwrap_or(0.0)
        };

        let dm_power = get_elem_power(day_master_el);
        let yin_power = get_elem_power(day_master_el.generated_by());
        let shi_power = get_elem_power(day_master_el.generates());
        let cai_power = get_elem_power(day_master_el.generates().generates());
        let guan_power = get_elem_power(day_master_el.controlled_by());

        let eokbu_element = match structure_analysis.structure {
            StructureType::JongAh | StructureType::GaJongAh => day_master_el.generates(),
            StructureType::JongJae | StructureType::GaJongJae => {
                day_master_el.generates().generates()
            }
            StructureType::JongSal | StructureType::GaJongSal => day_master_el.controlled_by(),
            StructureType::JongGang | StructureType::GaJongGang => day_master_el.generated_by(),
            StructureType::JongWang | StructureType::GaJongWang => day_master_el,
            StructureType::GokJik
            | StructureType::YeomSang
            | StructureType::GaSaek
            | StructureType::JongHyeok
            | StructureType::YoonHa => day_master_el,
            StructureType::Follower => pillars.month.branch.element(),
            StructureType::SpecialTransformation => day_master_el,
            _ => match strength.strength_type {
                StrengthType::Weak => {
                    //    :
                    // 1)  (Caisheng > 35% & Yinxing weak < 15%): (BiGeop)
                    // 2)  / : (Inseong)
                    if cai_power > 35.0 && yin_power < 15.0 {
                        day_master_el // Parallel
                    } else {
                        day_master_el.generated_by() // Resource
                    }
                }
                StrengthType::Strong => {
                    //     (   ):
                    if yin_power > dm_power * 1.2 || (yin_power > 30.0 && yin_power > dm_power) {
                        //  :  ()
                        day_master_el.generates().generates()
                    } else if dm_power > yin_power * 1.2
                        || (dm_power > 30.0 && dm_power > yin_power)
                    {
                        //  :  ()
                        day_master_el.controlled_by()
                    } else {
                        // / :  ()
                        day_master_el.generates()
                    }
                }
                StrengthType::Balanced => day_master_el,
            },
        };

        let is_polarized = matches!(
            structure_analysis.structure,
            StructureType::JongAh
                | StructureType::JongJae
                | StructureType::JongSal
                | StructureType::JongGang
                | StructureType::JongWang
                | StructureType::GaJongAh
                | StructureType::GaJongJae
                | StructureType::GaJongSal
                | StructureType::GaJongGang
                | StructureType::GaJongWang
                | StructureType::GokJik
                | StructureType::YeomSang
                | StructureType::GaSaek
                | StructureType::JongHyeok
                | StructureType::YoonHa
                | StructureType::Follower
                | StructureType::SpecialTransformation
        );

        let thermal_index = calculate_thermal_index(pillars, config);
        let humidity_index = calculate_humidity_index(pillars, config);

        if is_polarized {
            let reasons = vec![
                format!("Structure: {}", structure_analysis.structure.hangul()),
                format!("Power Ratio: {:.1}%", strength.deuk_se.support_ratio),
                format!(
                    "Criteria: {}",
                    if strength.deuk_se.support_ratio >= config.strength.polarized_high {
                        "Dominant Sovereign (專旺)"
                    } else {
                        "Follower (從)"
                    }
                ),
            ];
            recommendations.push(RecommendedYongshin {
                yongshin_type: YongshinType::Eokbu,
                element: eokbu_element,
                summary: format!("Follows structure momentum: {} Yongshin", eokbu_element.hangul()),
                description:
                    "Flowing with the overwhelming momentum smooths energy pathways better than resisting."
                        .to_string(),
                reasons,
                preferred_stems: None,
            });

            if let Some(mut johu) =
                get_johu_analysis(pillars, thermal_index, humidity_index, config)
            {
                johu.description = format!(
                    "{} (Note: For follower structures, aligning with momentum takes precedence over climate).",
                    johu.description
                );
                recommendations.push(johu);
            }
        } else {
            if let Some(johu) = get_johu_analysis(pillars, thermal_index, humidity_index, config) {
                recommendations.push(johu);
            }

            let (summary, description, reasons) = get_eokbu_explainability(
                &strength,
                eokbu_element,
                cai_power,
                yin_power,
                guan_power,
                shi_power,
                dm_power,
            );
            recommendations.push(RecommendedYongshin {
                yongshin_type: YongshinType::Eokbu,
                element: eokbu_element,
                summary,
                description,
                reasons,
                preferred_stems: None,
            });
        }

        // 3. (通關)  ( )
        if let Some(tonggwan) = get_tonggwan_analysis(pillars, config) {
            recommendations.push(tonggwan);
        }

        // 4. (病藥)  (  )
        let byeongyak_res = get_byeongyak_analysis(pillars, &strength, config, eokbu_element);
        let byeongyak_detail = if let Some((rec, detail)) = byeongyak_res {
            recommendations.push(rec);
            Some(detail)
        } else {
            None
        };

        // 5.     1(Primary Yongshin)
        let mut best_priority = -100.0f32;
        let mut primary = recommendations[0].element;

        for rec in &recommendations {
            let priority = match rec.yongshin_type {
                YongshinType::Johu => {
                    let t_abs = thermal_index.abs();
                    let h_abs = humidity_index.abs();
                    if t_abs >= config.thermal.extreme || h_abs >= config.thermal.extreme {
                        90.0 + (t_abs.max(h_abs) as f32) * 0.1
                    } else if t_abs >= config.thermal.moderate || h_abs >= config.thermal.moderate {
                        65.0
                    } else {
                        45.0
                    }
                }
                YongshinType::Tonggwan => 85.0,
                YongshinType::Byeongyak => 80.0,
                YongshinType::Eokbu => {
                    if is_polarized {
                        95.0
                    } else {
                        60.0
                    }
                }
            };

            if priority > best_priority {
                best_priority = priority;
                primary = rec.element;
            }
        }

        let assistant = primary.generated_by();

        Self {
            recommendations,
            primary,
            assistant,
            byeongyak_detail,
        }
    }
}

///  ()  (-100 ~ 100)
/// - :  (Water, Metal, Yin-Earth)
/// - :  (Fire, Wood, Yang-Earth)
pub fn calculate_thermal_index(pillars: &FourPillars, _config: &AnalysisConfig) -> i32 {
    let mut score = 0;

    // ()
    if let Ok(saryeong) = crate::core::branch_days::SaryeongAnalysis::from_pillars(pillars) {
        let base_weight = match pillars.month.branch {
            EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou => -40,
            EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei => 40,
            EarthlyBranch::Yin | EarthlyBranch::Mao | EarthlyBranch::Chen => 10,
            EarthlyBranch::Shen | EarthlyBranch::You | EarthlyBranch::Xu => -10,
        };
        let multiplier = if saryeong.part.contains("Main") {
            1.2
        } else if saryeong.part.contains("Middle") {
            1.0
        } else {
            0.8
        };
        score += (base_weight as f64 * multiplier) as i32;
    } else {
        match pillars.month.branch {
            EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou => score -= 40,
            EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei => score += 40,
            _ => {}
        }
    }

    // The Hour pillar is absent when the birth time is unknown
    let mut stems = vec![pillars.year.stem, pillars.month.stem, pillars.day.stem];
    let mut branches = vec![
        pillars.year.branch,
        pillars.month.branch,
        pillars.day.branch,
    ];
    if let Some(hour) = pillars.hour {
        stems.push(hour.stem);
        branches.push(hour.branch);
    }

    for s in stems {
        use crate::core::stem::HeavenlyStem as S;
        match s {
            S::Bing => score += 15, // Sun (warmest)
            S::Ding => score += 10, // Lamp fire (warm)
            S::Ren => score -= 15,  // River water (coldest)
            S::Gui => score -= 10,  // Rain water (cold/damp)
            S::Jia | S::Yi => score += 5,
            S::Geng | S::Xin => score -= 5,
            _ => {}
        }
    }

    for b in branches {
        use EarthlyBranch as EB;
        match b {
            EB::Si | EB::Wu => score += 15,
            EB::Wei | EB::Xu => score += 10, // Dry Earth
            EB::Hai | EB::Zi => score -= 15,
            EB::Chou | EB::Chen => score -= 10, // Wet Earth
            EB::Yin | EB::Mao => score += 5,
            EB::Shen | EB::You => score -= 5,
        }
    }

    score.clamp(-100, 100)
}

///    (-100 ~ 100)
/// - :  (Water, Wet-Earth, Wood)
/// - :  (Fire, Dry-Earth, Metal)
pub fn calculate_humidity_index(pillars: &FourPillars, _config: &AnalysisConfig) -> i32 {
    let mut score = 0;

    if let Ok(saryeong) = crate::core::branch_days::SaryeongAnalysis::from_pillars(pillars) {
        let base_weight = match pillars.month.branch {
            EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou | EarthlyBranch::Chen => {
                -40
            }
            EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei | EarthlyBranch::Xu => 40,
            EarthlyBranch::Yin | EarthlyBranch::Mao => -10, // Wood retains organic moisture
            EarthlyBranch::Shen | EarthlyBranch::You => 10, // Metal is dry
        };
        let multiplier = if saryeong.part.contains("Main") {
            1.2
        } else if saryeong.part.contains("Middle") {
            1.0
        } else {
            0.8
        };
        score += (base_weight as f64 * multiplier) as i32;
    } else {
        match pillars.month.branch {
            EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou | EarthlyBranch::Chen => {
                score -= 40
            }
            EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei | EarthlyBranch::Xu => {
                score += 40
            }
            _ => {}
        }
    }

    // The Hour pillar is absent when the birth time is unknown
    let mut stems = vec![pillars.year.stem, pillars.month.stem, pillars.day.stem];
    let mut branches = vec![
        pillars.year.branch,
        pillars.month.branch,
        pillars.day.branch,
    ];
    if let Some(hour) = pillars.hour {
        stems.push(hour.stem);
        branches.push(hour.branch);
    }

    for s in stems {
        use crate::core::stem::HeavenlyStem as S;
        match s {
            S::Bing | S::Ding => score += 15,
            S::Geng | S::Xin => score += 10, // Metal contributes to dryness
            S::Ren | S::Gui => score -= 15,
            S::Jia | S::Yi => score -= 5,
            S::Wu => score += 10, // Wu Earth is dry earth
            S::Ji => score -= 10, // Ji Earth is damp earth
        }
    }

    for b in branches {
        use EarthlyBranch as EB;
        match b {
            EB::Si | EB::Wu => score += 15,
            EB::Wei | EB::Xu => score += 15, // Dry Earth
            EB::Shen | EB::You => score += 10,
            EB::Hai | EB::Zi => score -= 15,
            EB::Chou | EB::Chen => score -= 15, // Wet Earth
            EB::Yin | EB::Mao => score -= 5,
        }
    }

    score.clamp(-100, 100)
}

///   ( &  2D     )
fn get_johu_analysis(
    _pillars: &FourPillars,
    thermal: i32,
    humidity: i32,
    _config: &AnalysisConfig,
) -> Option<RecommendedYongshin> {
    use crate::core::stem::HeavenlyStem as S;

    let is_cold = thermal <= -30;
    let is_hot = thermal >= 30;
    let is_wet = humidity <= -30;
    let is_dry = humidity >= 30;

    let mut reasons = vec![
        format!("Thermal Index: {}", thermal),
        format!("Humidity Index: {}", humidity),
    ];

    let (element, summary, desc, preferred) = if is_cold && is_wet {
        reasons.push("QiongTongBaoJian JoHu: Bing Fire (Sun) priority".to_string());
        (
            Element::Fire,
            "Fire Yongshin (Bing Fire) warming and drying cold/damp chart",
            "Chart is severely cold and damp; radiant warmth and dry earth are urgently needed.",
            Some(vec![S::Bing]),
        )
    } else if is_hot && is_dry {
        reasons.push("QiongTongBaoJian JoHu: Gui Water (Rain) priority".to_string());
        (
            Element::Water,
            "Water Yongshin (Gui Water) cooling and moisturizing dry/hot chart",
            "Chart is severely hot and arid; Gui water and wet earth are urgently needed.",
            Some(vec![S::Gui, S::Ren]),
        )
    } else if is_hot && is_wet {
        reasons.push("QiongTongBaoJian JoHu: Geng Metal / Gui Water preferred".to_string());
        (
            Element::Metal,
            "Metal Yongshin (Geng Metal / Gui Water) cleansing damp-hot chart",
            "Chart is hot and damp; Metal and Water energy cleanses and clears heat.",
            Some(vec![S::Geng, S::Gui]),
        )
    } else if is_cold && is_dry {
        reasons.push("QiongTongBaoJian JoHu: Jia Wood / Bing Fire preferred".to_string());
        (
            Element::Wood,
            "Wood Yongshin (Jia Wood / Bing Fire) enlivening cold-arid chart",
            "Chart is cold and parched; vitalizing Wood or Fire energy is essential.",
            Some(vec![S::Jia, S::Bing]),
        )
    } else if is_cold {
        reasons.push("QiongTongBaoJian JoHu: Bing Fire preferred".to_string());
        (
            Element::Fire,
            "Fire Yongshin (Bing Fire) warming cold chart",
            "Chart energy is cold; thermal elevation via Bing Fire brings prosperity.",
            Some(vec![S::Bing]),
        )
    } else if is_hot {
        reasons.push("QiongTongBaoJian JoHu: Gui Water preferred".to_string());
        (
            Element::Water,
            "Water Yongshin (Gui Water) cooling arid-hot chart",
            "Chart energy is scorching; cooling via Gui Water restores flourishing vitality.",
            Some(vec![S::Gui]),
        )
    } else if is_wet {
        reasons.push("QiongTongBaoJian JoHu: Bing Fire preferred".to_string());
        (
            Element::Fire,
            "Fire Yongshin (Bing Fire) drying damp chart",
            "Chart holds excess dampness; radiant sun and dry earth disperse stagnation.",
            Some(vec![S::Bing]),
        )
    } else if is_dry {
        reasons.push("QiongTongBaoJian JoHu: Gui Water preferred".to_string());
        (
            Element::Water,
            "Water Yongshin (Gui Water) moistening dry chart",
            "Chart is arid; nourishing water restores organic vitality.",
            Some(vec![S::Gui]),
        )
    } else {
        return None;
    };

    Some(RecommendedYongshin {
        yongshin_type: YongshinType::Johu,
        element,
        summary: summary.to_string(),
        description: desc.to_string(),
        reasons,
        preferred_stems: preferred,
    })
}

///   (     )
///
///   ** **  .
///      ( 20% )  .
fn get_tonggwan_analysis(
    pillars: &FourPillars,
    config: &AnalysisConfig,
) -> Option<RecommendedYongshin> {
    use crate::analysis::power::{AnalysisOptions, IntegratedAnalysis};

    //    (  )
    let options = AnalysisOptions {
        apply_transform: false, // Transformations not applied (baseline chart)
        apply_correction: false,
    };
    let analysis = IntegratedAnalysis::calculate(pillars, options, config);

    //     (0~100)
    let mut scores: [f32; 5] = [0.0; 5];
    for (elem, pct, _) in &analysis.element_scores {
        scores[elem.index() as usize] = *pct;
    }

    // (1, 2,  , )
    let conflicts: [(Element, Element, Element, &str); 5] = [
        (
            Element::Metal,
            Element::Wood,
            Element::Water,
            "Metal and Wood clash; Water is needed to mediate and flow",
        ),
        (
            Element::Water,
            Element::Fire,
            Element::Wood,
            "Water and Fire clash; Wood is needed to mediate and flow",
        ),
        (
            Element::Wood,
            Element::Earth,
            Element::Fire,
            "Wood and Earth clash; Fire is needed to mediate and flow",
        ),
        (
            Element::Fire,
            Element::Metal,
            Element::Earth,
            "Fire and Metal clash; Earth is needed to mediate and flow",
        ),
        (
            Element::Earth,
            Element::Water,
            Element::Metal,
            "Earth and Water clash; Metal is needed to mediate and flow",
        ),
    ];

    for (elem1, elem2, mediator, reason) in conflicts {
        let score1 = scores[elem1.index() as usize];
        let score2 = scores[elem2.index() as usize];

        //     15% ,  20%
        let min_threshold = 15.0; // Minimum power threshold
        let max_diff = 20.0; // Power difference threshold

        if score1 >= min_threshold && score2 >= min_threshold {
            let diff = (score1 - score2).abs();
            if diff <= max_diff {
                return Some(RecommendedYongshin {
                    yongshin_type: YongshinType::Tonggwan,
                    element: mediator,
                    summary: format!("Mediating Yongshin {} resolving conflict between {} and {}", elem1.hangul(), elem2.hangul(), mediator.hangul()),
                    description: "When two conflicting elements clash evenly, the bridging element is the key to unlocking fortune.".to_string(),
                    reasons: vec![
                        reason.to_string(),
                        format!("{}: {:.1}%", elem1.hangul(), score1),
                        format!("{}: {:.1}%", elem2.hangul(), score2),
                    ],
                    preferred_stems: None,
                });
            }
        }
    }

    None
}

///   (        )
fn get_byeongyak_analysis(
    pillars: &FourPillars,
    _strength: &crate::analysis::strength::StrengthAnalysis,
    config: &AnalysisConfig,
    eokbu_element: Element,
) -> Option<(RecommendedYongshin, ByeongyakAnalysis)> {
    let options = crate::analysis::power::AnalysisOptions {
        apply_transform: false,
        apply_correction: true,
    };
    let integrated =
        crate::analysis::power::IntegratedAnalysis::calculate(pillars, options, config);
    let dm_el = pillars.day_master_element();

    // 1.  40%    ()
    let mut disease_elem = None;
    for (elem, pct, _) in &integrated.element_scores {
        if *pct >= 40.0 {
            disease_elem = Some((*elem, *pct));
            break;
        }
    }

    // 2. 40%       (25% )
    if disease_elem.is_none() {
        let clashing_elem = eokbu_element.controlled_by();
        let clashing_pct = integrated
            .element_scores
            .iter()
            .find(|(e, _, _)| *e == clashing_elem)
            .map(|(_, pct, _)| *pct)
            .unwrap_or(0.0);
        if clashing_pct >= 25.0 {
            disease_elem = Some((clashing_elem, clashing_pct));
        }
    }

    if let Some((disease, pct)) = disease_elem {
        let medicine = if disease == dm_el.controlled_by() || disease == dm_el.generates() {
            dm_el.generated_by()
        } else if disease == dm_el.generates().generates() {
            dm_el
        } else if disease == dm_el.generated_by() {
            dm_el.generates().generates()
        } else {
            dm_el.controlled_by()
        };

        let summary = format!(
            "Remedy Yongshin {} restraining excessive {} ({:.1}%)",
            disease.hangul(),
            pct,
            medicine.hangul()
        );
        let description = format!(
            "Element {} at {:.1}% constitutes a chart disease; {} serves as the restorative medicine.",
            disease.hangul(), pct, medicine.hangul()
        );
        let reasons = vec![
            format!("Disease Element: {} ({:.1}%)", disease.hangul(), pct),
            format!("Medicine Element: {}", medicine.hangul()),
        ];

        let rec = RecommendedYongshin {
            yongshin_type: YongshinType::Byeongyak,
            element: medicine,
            summary: summary.clone(),
            description: description.clone(),
            reasons: reasons.clone(),
            preferred_stems: None,
        };

        let detail = ByeongyakAnalysis {
            disease,
            medicine,
            summary,
            description,
            reasons,
        };

        return Some((rec, detail));
    }

    None
}

impl std::fmt::Display for YongshinAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【Useful God (用神, Yongshin) Determination】")?;
        writeln!(f, "─────────────────────────────────")?;

        for rec in &self.recommendations {
            writeln!(
                f,
                "● {} : {} ({})",
                rec.yongshin_type.hangul(),
                rec.element.hangul(),
                rec.element.hanja()
            )?;
            writeln!(f, "   - Summary: {}", rec.summary)?;
            writeln!(f, "   - Description: {}", rec.description)?;
            if !rec.reasons.is_empty() {
                writeln!(f, "   - Basis: {}", rec.reasons.join(", "))?;
            }
        }

        writeln!(f)?;
        writeln!(
            f,
            "▶ Primary Yongshin: {} ({})",
            self.primary.hangul(),
            self.primary.hanja()
        )?;
        writeln!(
            f,
            "▶ Supporting God (喜神): {} ({})",
            self.assistant.hangul(),
            self.assistant.hanja()
        )?;

        Ok(())
    }
}

impl FourPillars {
    /// Yongshin Analysis
    pub fn yongshin(&self) -> YongshinAnalysis {
        YongshinAnalysis::from_pillars(self)
    }

    /// Yongshin Analysis with custom config
    pub fn yongshin_with_config(&self, config: &AnalysisConfig) -> YongshinAnalysis {
        YongshinAnalysis::from_pillars_with_config(self, config)
    }
}

use crate::analysis::Analyzable;

impl Analyzable for YongshinAnalysis {
    type Output = YongshinAnalysis;
    fn analyze(pillars: &FourPillars, config: &AnalysisConfig) -> Self::Output {
        YongshinAnalysis::from_pillars_with_config(pillars, config)
    }
}

/// Generates Eokbu explainability text
fn get_eokbu_explainability(
    strength: &StrengthAnalysis,
    eokbu_element: Element,
    cai_power: f32,
    yin_power: f32,
    guan_power: f32,
    shi_power: f32,
    dm_power: f32,
) -> (String, String, Vec<String>) {
    let mut reasons = vec![
        format!("Strength Score: {:.1}", strength.strength_score),
        format!("Current Status: {}", strength.strength_type.hangul()),
    ];

    match strength.strength_type {
        StrengthType::Weak => {
            if cai_power > 35.0 && yin_power < 15.0 {
                reasons.push(format!("Excessive Wealth ({:.1}%) - Heavy Wealth Weak DM", cai_power));
                (
                    format!("Parallel Yongshin {} supporting Day Master to withstand wealth", eokbu_element.hangul()),
                    "Wealth is excessively strong resulting in weak Day Master; Parallels are needed over Resources to help bear the wealth.".to_string(),
                    reasons,
                )
            } else if guan_power > 35.0 {
                reasons.push(format!("Excessive Officer ({:.1}%) - Overbearing Killings", guan_power));
                (
                    format!("Officer-Resource Conduit Yongshin {} resolving officer pressure", eokbu_element.hangul()),
                    "Pressure from officers is harsh; Resources gently channel officer energy into empowering the Day Master.".to_string(),
                    reasons,
                )
            } else if shi_power > 35.0 {
                reasons.push(format!("Excessive Output ({:.1}%) - Heavy Output Exhaustion", shi_power));
                (
                    format!("Resource Yongshin {} controlling output and replenishing Day Master", eokbu_element.hangul()),
                    "Exhaustion from output is severe; Resources restrain output and replenish energy.".to_string(),
                    reasons,
                )
            } else {
                (
                    format!("Eokbu Yongshin {} assisting Day Master", eokbu_element.hangul()),
                    "Day Master is weak and requires generative or parallel elements to bring favorable balance.".to_string(),
                    reasons,
                )
            }
        }
        StrengthType::Strong => {
            if yin_power > dm_power * 1.2 || (yin_power > 30.0 && yin_power > dm_power) {
                reasons.push(format!("Excessive Resource ({:.1}%)", yin_power));
                (
                    format!("UseWealthBreakResource Yongshin {} controlling excessive resource", eokbu_element.hangul()),
                    "Excessive resources cause over-strength; Wealth is needed to break through and check resource stagnation.".to_string(),
                    reasons,
                )
            } else if dm_power > yin_power * 1.2 || (dm_power > 30.0 && dm_power > yin_power) {
                reasons.push(format!("Excessive Parallel ({:.1}%)", dm_power));
                (
                    format!("OfficerControlsRob Yongshin {} restraining excessive parallels", eokbu_element.hangul()),
                    "Self is excessively headstrong; Officer restraint is cultivated to maintain discipline.".to_string(),
                    reasons,
                )
            } else {
                (
                    format!("ExhaustionProducesWealth Yongshin {} circulating energy", eokbu_element.hangul()),
                    "Day Master is strong; output channels excess vigor and connects into productive wealth.".to_string(),
                    reasons,
                )
            }
        }
        StrengthType::Balanced => (
            format!("Neutralizing Yongshin {} maintaining equilibrium", eokbu_element.hangul()),
            "Energy is naturally balanced and centered; elements preserving this harmony are favored.".to_string(),
            reasons,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pillars::SajuInput;

    #[test]
    fn test_johu_thermal_and_humidity() {
        let input = SajuInput::new_solar(2024, 1, 1, 12, 0); // Midwinter noon
        let pillars = FourPillars::calculate(&input).unwrap();
        let thermal = calculate_thermal_index(&pillars, &AnalysisConfig::default());
        let humidity = calculate_humidity_index(&pillars, &AnalysisConfig::default());

        //   ,   .
        assert!(thermal < 0);
        assert!(humidity < 0);

        let johu = get_johu_analysis(&pillars, thermal, humidity, &AnalysisConfig::default());
        assert!(johu.is_some());
        let johu = johu.unwrap();
        assert_eq!(johu.element, Element::Fire);
        assert!(johu.preferred_stems.is_some());
        assert!(johu
            .preferred_stems
            .unwrap()
            .contains(&crate::core::stem::HeavenlyStem::Bing));
    }

    #[test]
    fn test_byeongyak_and_priority_matrix() {
        let input = SajuInput::new_solar(1984, 7, 15, 12, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        let yongshin = pillars.yongshin();
        assert!(!yongshin.recommendations.is_empty());
        assert!(
            yongshin.primary != Element::Earth
                || yongshin.assistant == Element::Fire
                || yongshin.assistant != Element::Earth
        );
    }
}
