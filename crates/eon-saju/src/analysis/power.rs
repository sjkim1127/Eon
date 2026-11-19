//! Five Elements & Ten Gods Power Scoring Analysis
//!
//! Supports 4 analysis modes based on correction options:
//! 1. Baseline (no corrections)
//! 2. Transformations applied
//! 3. Palace & climate corrections applied
//! 4. Transformations and palace/climate corrections applied

use crate::core::branch::EarthlyBranch;
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use crate::core::stem::HeavenlyStem;
use crate::core::ten_gods::TenGod;
use serde::{Deserialize, Serialize};

/// Analysis Options
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisOptions {
    /// Apply element transformations from combinations
    pub apply_transform: bool,
    /// Apply climate and palace adjustments
    pub apply_correction: bool,
}

impl Default for AnalysisOptions {
    fn default() -> Self {
        Self {
            apply_transform: true,
            apply_correction: true,
        }
    }
}

/// Integrated Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegratedAnalysis {
    pub options: AnalysisOptions,
    /// Five Elements ratios (Element, Percentage, Score)
    pub element_scores: Vec<(Element, f32, f32)>,
    /// Ten Gods ratios (TenGod, Percentage, Score)
    pub ten_god_scores: Vec<(TenGod, f32, f32)>,
    /// Dominant Element
    pub dominant_element: Element,
    /// Dominant Ten God
    pub dominant_ten_god: TenGod,
}

impl IntegratedAnalysis {
    pub fn calculate(
        pillars: &FourPillars,
        options: AnalysisOptions,
        config: &crate::core::config::AnalysisConfig,
    ) -> Self {
        let dm = pillars.day_master();
        let month_branch = pillars.month.branch;

        // 1. Configure weights
        let weights = if options.apply_correction {
            [
                config.weights.stem,
                config.weights.stem,
                config.weights.stem,
                config.weights.stem,
                config.weights.other_branch,
                config.weights.month_branch,
                config.weights.day_branch,
                config.weights.other_branch,
            ]
        } else {
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]
        };

        // Scale weights
        // 110-point method compatibility
        // Uses weights as defined in config

        let mut total_weight: f32 = 0.0;

        // 2. Determine effective element and ten god per position
        let mut el_scores = [0.0f32; 5];
        let mut tg_scores = [0.0f32; 10];

        // Effective element map (reflecting transformations)
        let eff_map = if options.apply_transform {
            pillars.effective_elements()
        } else {
            let mut map: [Option<(Element, Element)>; 8] = [None; 8];
            let stems = [
                Some(pillars.year.stem),
                Some(pillars.month.stem),
                Some(pillars.day.stem),
                pillars.hour.map(|h| h.stem),
            ];
            let branches = [
                Some(pillars.year.branch),
                Some(pillars.month.branch),
                Some(pillars.day.branch),
                pillars.hour.map(|h| h.branch),
            ];
            for i in 0..4 {
                map[i] = stems[i].map(|s| (s.element(), s.element()));
                map[i + 4] = branches[i].map(|b| (b.element(), b.element()));
            }
            map
        };

        // 2-1. Process stems (0~3); the hour slot is skipped when the birth hour is unknown
        let stems = [
            Some(pillars.year.stem),
            Some(pillars.month.stem),
            Some(pillars.day.stem),
            pillars.hour.map(|h| h.stem),
        ];
        let branches = [
            Some(pillars.year.branch),
            Some(pillars.month.branch),
            Some(pillars.day.branch),
            pillars.hour.map(|h| h.branch),
        ];

        for i in 0..4 {
            let (Some(stem), Some((_, effective_el))) = (stems[i], eff_map[i]) else {
                continue;
            };
            let mut weight = weights[i];

            if options.apply_correction {
                let mut multiplier = 1.0;
                // Additional position multipliers for year, month, day, hour
                let branch_bonus_factors = [0.05, 0.5, 0.2, 0.1];
                for j in 0..4 {
                    let Some(branch) = branches[j] else {
                        continue;
                    };
                    let has_root = branch
                        .hidden_stems()
                        .iter()
                        .any(|&hs| hs.element() == stem.element());
                    if has_root {
                        multiplier += branch_bonus_factors[j];
                    }
                }
                weight *= multiplier;
            }

            el_scores[effective_el.index() as usize] += weight;
            total_weight += weight;

            // Ten god calculation
            let god = if effective_el != stem.element() {
                // When transformed
                TenGod::from_stems(dm, get_dummy_stem(effective_el, stem.polarity()))
            } else {
                TenGod::from_stems(dm, stem)
            };
            tg_scores[god.index()] += weight;
        }

        // 2-2. Process branches (4~7)
        for i in 0..4 {
            let (Some(branch), Some((_, mut effective_el))) = (branches[i], eff_map[i + 4]) else {
                continue;
            };
            let weight = weights[i + 4];

            // Climate correction
            // Climate correction takes precedence
            if options.apply_correction
                && effective_el == branch.element()
                && matches!(
                    branch,
                    EarthlyBranch::Chen
                        | EarthlyBranch::Xu
                        | EarthlyBranch::Chou
                        | EarthlyBranch::Wei
                )
            {
                effective_el = apply_climate_correction(branch, month_branch);
            }

            el_scores[effective_el.index() as usize] += weight;
            total_weight += weight;

            // Branch ten gods
            let god = if effective_el != branch.element() {
                TenGod::from_stems(
                    dm,
                    get_dummy_stem(effective_el, branch.primary_stem().polarity()),
                )
            } else {
                TenGod::from_stems(dm, branch.primary_stem())
            };
            tg_scores[god.index()] += weight;
        }

        // 3. Format results
        let mut final_el = Vec::new();
        for i in 0..5 {
            let el = Element::from_index(i as i32);
            let score = el_scores[i];
            final_el.push((el, (score / total_weight) * 100.0, score));
        }

        let mut final_tg = Vec::new();
        for i in 0..10 {
            let god = TenGod::ALL[i];
            let score = tg_scores[i];
            final_tg.push((god, (score / total_weight) * 100.0, score));
        }

        let dominant_element = final_el
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(&final_el[0])
            .0;
        let dominant_ten_god = final_tg
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(&final_tg[0])
            .0;

        Self {
            options,
            element_scores: final_el,
            ten_god_scores: final_tg,
            dominant_element,
            dominant_ten_god,
        }
    }

    /// Compute extended 5/6 pillar scores (Natal + Major Luck + Yearly Luck)
    pub fn calculate_expanded(
        pillars: &FourPillars,
        major: Option<crate::core::ganzi::GanZi>,
        yearly: Option<crate::core::ganzi::GanZi>,
        options: AnalysisOptions,
        config: &crate::core::config::AnalysisConfig,
    ) -> Self {
        let dm = pillars.day_master();
        let month_branch = pillars.month.branch;

        let trans = crate::analysis::transformations::TransformationAnalysis::from_expanded(
            pillars, major, yearly,
        );

        let mut total_weight: f32 = 0.0;
        let mut el_scores = [0.0f32; 5];
        let mut tg_scores = [0.0f32; 10];

        let base_weights = if options.apply_correction {
            [
                config.weights.stem,
                config.weights.stem,
                config.weights.stem,
                config.weights.stem,
                config.weights.other_branch,
                config.weights.month_branch,
                config.weights.day_branch,
                config.weights.other_branch,
            ]
        } else {
            [1.0; 8]
        };

        let mut items = vec![
            (
                pillars.year.stem.element(),
                trans.year_stem.effective,
                base_weights[0],
                true,
                pillars.year.stem.polarity(),
                None,
            ),
            (
                pillars.month.stem.element(),
                trans.month_stem.effective,
                base_weights[1],
                true,
                pillars.month.stem.polarity(),
                None,
            ),
            (
                pillars.day.stem.element(),
                trans.day_stem.effective,
                base_weights[2],
                true,
                pillars.day.stem.polarity(),
                None,
            ),
            (
                pillars.year.branch.element(),
                trans.year_branch.effective,
                base_weights[4],
                false,
                pillars.year.branch.primary_stem().polarity(),
                Some(pillars.year.branch),
            ),
            (
                pillars.month.branch.element(),
                trans.month_branch.effective,
                base_weights[5],
                false,
                pillars.month.branch.primary_stem().polarity(),
                Some(pillars.month.branch),
            ),
            (
                pillars.day.branch.element(),
                trans.day_branch.effective,
                base_weights[6],
                false,
                pillars.day.branch.primary_stem().polarity(),
                Some(pillars.day.branch),
            ),
        ];

        // Hour pillar is absent when the birth hour is unknown
        if let (Some(h), Some(hs), Some(hb)) = (pillars.hour, &trans.hour_stem, &trans.hour_branch)
        {
            items.push((
                h.stem.element(),
                hs.effective,
                base_weights[3],
                true,
                h.stem.polarity(),
                None,
            ));
            items.push((
                h.branch.element(),
                hb.effective,
                base_weights[7],
                false,
                h.branch.primary_stem().polarity(),
                Some(h.branch),
            ));
        }

        if let (Some(m_ganzi), Some(ref ms)) = (major, &trans.major_stem) {
            items.push((
                m_ganzi.stem.element(),
                ms.effective,
                1.0,
                true,
                m_ganzi.stem.polarity(),
                None,
            ));
        }
        if let (Some(m_ganzi), Some(ref mb)) = (major, &trans.major_branch) {
            items.push((
                m_ganzi.branch.element(),
                mb.effective,
                1.5,
                false,
                m_ganzi.branch.primary_stem().polarity(),
                Some(m_ganzi.branch),
            ));
        }
        if let (Some(y_ganzi), Some(ref ss)) = (yearly, &trans.saewun_stem) {
            items.push((
                y_ganzi.stem.element(),
                ss.effective,
                1.0,
                true,
                y_ganzi.stem.polarity(),
                None,
            ));
        }
        if let (Some(y_ganzi), Some(ref sb)) = (yearly, &trans.saewun_branch) {
            items.push((
                y_ganzi.branch.element(),
                sb.effective,
                1.5,
                false,
                y_ganzi.branch.primary_stem().polarity(),
                Some(y_ganzi.branch),
            ));
        }

        for (orig_el, eff_el, weight, is_stem, polarity, branch_opt) in items {
            let actual_el =
                if let (false, true, Some(br)) = (is_stem, options.apply_correction, branch_opt) {
                    if eff_el != orig_el {
                        eff_el
                    } else if matches!(
                        br,
                        EarthlyBranch::Chen
                            | EarthlyBranch::Xu
                            | EarthlyBranch::Chou
                            | EarthlyBranch::Wei
                    ) {
                        apply_climate_correction(br, month_branch)
                    } else {
                        eff_el
                    }
                } else {
                    eff_el
                };

            el_scores[actual_el.index() as usize] += weight;
            total_weight += weight;

            let god = if actual_el != orig_el {
                TenGod::from_stems(dm, get_dummy_stem(actual_el, polarity))
            } else {
                TenGod::from_stems(dm, get_dummy_stem(orig_el, polarity))
            };
            tg_scores[god.index()] += weight;
        }

        let mut final_el = Vec::new();
        for i in 0..5 {
            let el = Element::from_index(i as i32);
            let score = el_scores[i];
            final_el.push((
                el,
                if total_weight > 0.0 {
                    (score / total_weight) * 100.0
                } else {
                    0.0
                },
                score,
            ));
        }

        let mut final_tg = Vec::new();
        for i in 0..10 {
            let god = TenGod::ALL[i];
            let score = tg_scores[i];
            final_tg.push((
                god,
                if total_weight > 0.0 {
                    (score / total_weight) * 100.0
                } else {
                    0.0
                },
                score,
            ));
        }

        let dominant_element = final_el
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(&final_el[0])
            .0;
        let dominant_ten_god = final_tg
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(&final_tg[0])
            .0;

        Self {
            options,
            element_scores: final_el,
            ten_god_scores: final_tg,
            dominant_element,
            dominant_ten_god,
        }
    }
}

/// Climate adjustment logic for the four earth branches (Chen, Xu, Chou, Wei)
fn apply_climate_correction(branch: EarthlyBranch, month: EarthlyBranch) -> Element {
    match branch {
        EarthlyBranch::Wei => {
            // Wei in summer strengthens Fire
            if matches!(month, EarthlyBranch::Si | EarthlyBranch::Wu) {
                Element::Fire
            } else {
                Element::Earth
            }
        }
        EarthlyBranch::Chou => {
            // Chou in winter strengthens Water
            if matches!(month, EarthlyBranch::Hai | EarthlyBranch::Zi) {
                Element::Water
            } else {
                Element::Earth
            }
        }
        EarthlyBranch::Chen => {
            // Chen in spring strengthens Wood
            if matches!(month, EarthlyBranch::Yin | EarthlyBranch::Mao) {
                Element::Wood
            } else {
                Element::Earth
            }
        }
        EarthlyBranch::Xu => {
            // Xu in autumn strengthens Metal
            if matches!(month, EarthlyBranch::Shen | EarthlyBranch::You) {
                Element::Metal
            } else {
                Element::Earth
            }
        }
        _ => branch.element(),
    }
}

/// Return provisional stem from element and polarity
fn get_dummy_stem(element: Element, polarity: crate::core::element::Polarity) -> HeavenlyStem {
    use crate::core::element::Polarity::*;
    match (element, polarity) {
        (Element::Wood, Yang) => HeavenlyStem::Jia,
        (Element::Wood, Yin) => HeavenlyStem::Yi,
        (Element::Fire, Yang) => HeavenlyStem::Bing,
        (Element::Fire, Yin) => HeavenlyStem::Ding,
        (Element::Earth, Yang) => HeavenlyStem::Wu,
        (Element::Earth, Yin) => HeavenlyStem::Ji,
        (Element::Metal, Yang) => HeavenlyStem::Geng,
        (Element::Metal, Yin) => HeavenlyStem::Xin,
        (Element::Water, Yang) => HeavenlyStem::Ren,
        (Element::Water, Yin) => HeavenlyStem::Gui,
    }
}

impl std::fmt::Display for IntegratedAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Transformations applied: {}",
            if self.options.apply_transform {
                "o"
            } else {
                "x"
            }
        )?;
        writeln!(
            f,
            "Climate & palace corrections applied: {}",
            if self.options.apply_correction {
                "o"
            } else {
                "x"
            }
        )?;
        writeln!(
            f,
            "{} {}",
            self.dominant_element.hanja(),
            self.dominant_ten_god.hanja()
        )?;

        writeln!(f, "Five Elements")?;
        for (el, percent, _) in &self.element_scores {
            if *percent > 0.0 {
                let state = if *percent < 10.0 {
                    "Deficient"
                } else if *percent <= 20.0 {
                    "Balanced"
                } else if *percent <= 35.0 {
                    "Developed"
                } else {
                    "Excessive"
                };
                writeln!(
                    f,
                    "  {}({})  {:.1}% {}",
                    el.hangul(),
                    el.hanja(),
                    percent,
                    state
                )?;
            }
        }

        writeln!(f, "Ten Gods")?;
        for (god, percent, _) in &self.ten_god_scores {
            if *percent > 0.0 {
                writeln!(f, "  {}({})  {:.1}%", god.hangul(), god.hanja(), percent)?;
            } else {
                writeln!(f, "  {}({})  -", god.hangul(), god.hanja())?;
            }
        }
        Ok(())
    }
}

impl FourPillars {
    /// Integrated Power Analysis
    pub fn integrated_analysis(
        &self,
        options: AnalysisOptions,
        config: &crate::core::config::AnalysisConfig,
    ) -> IntegratedAnalysis {
        IntegratedAnalysis::calculate(self, options, config)
    }

    /// Extended 5/6 pillar integrated analysis
    pub fn integrated_analysis_expanded(
        &self,
        major: Option<crate::core::ganzi::GanZi>,
        yearly: Option<crate::core::ganzi::GanZi>,
        options: AnalysisOptions,
        config: &crate::core::config::AnalysisConfig,
    ) -> IntegratedAnalysis {
        IntegratedAnalysis::calculate_expanded(self, major, yearly, options, config)
    }

    /// Perform analysis with default options and config
    pub fn analyze(&self) -> IntegratedAnalysis {
        self.integrated_analysis(
            AnalysisOptions::default(),
            &crate::core::config::AnalysisConfig::default(),
        )
    }
}

use crate::analysis::Analyzable;

impl Analyzable for IntegratedAnalysis {
    type Output = IntegratedAnalysis;
    fn analyze(
        pillars: &FourPillars,
        config: &crate::core::config::AnalysisConfig,
    ) -> Self::Output {
        IntegratedAnalysis::calculate(pillars, AnalysisOptions::default(), config)
    }
}
