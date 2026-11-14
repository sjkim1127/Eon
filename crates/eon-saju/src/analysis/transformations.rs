//! Element Transformation (合化) Analysis
//!
//! Implements element transformation logic when combinations meet specific criteria.

use crate::analysis::relationships::{
    SeasonalCombination, SemiCombination, StemCombination, TripleCombination,
};
use crate::core::branch::EarthlyBranch;
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use crate::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

/// Transformation Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectiveElement {
    /// Original Element
    pub original: Element,
    /// Transformed Element (if transformed)
    pub effective: Element,
    /// Reason for transformation
    pub reason: Option<String>,
}

use crate::core::ganzi::GanZi;

/// Effective Element Statistics across chart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationAnalysis {
    /// Year Stem Effective Element
    pub year_stem: EffectiveElement,
    /// Month Stem Effective Element
    pub month_stem: EffectiveElement,
    /// Day Stem Effective Element
    pub day_stem: EffectiveElement,
    /// Hour Stem Effective Element (`None` when the birth hour is unknown)
    pub hour_stem: Option<EffectiveElement>,
    /// Includes 4 branches (reflecting triple combinations)
    pub year_branch: EffectiveElement,
    pub month_branch: EffectiveElement,
    pub day_branch: EffectiveElement,
    /// `None` when the birth hour is unknown
    pub hour_branch: Option<EffectiveElement>,
    /// Major Luck Stem Effective Element
    pub major_stem: Option<EffectiveElement>,
    /// Yearly Luck Stem Effective Element
    pub saewun_stem: Option<EffectiveElement>,
    /// Major Luck Branch Effective Element
    pub major_branch: Option<EffectiveElement>,
    /// Yearly Luck Branch Effective Element
    pub saewun_branch: Option<EffectiveElement>,
}

impl TransformationAnalysis {
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        let rel = pillars.relationships();
        let month_branch_element = pillars.month.branch.element();

        let mut year_stem = EffectiveElement::new(pillars.year.stem.element());
        let mut month_stem = EffectiveElement::new(pillars.month.stem.element());
        let mut day_stem = EffectiveElement::new(pillars.day.stem.element());
        let mut hour_stem = pillars
            .hour
            .map(|h| EffectiveElement::new(h.stem.element()));

        // 1. Stem transformation processing
        // Rival / Jealous combination analysis
        let mut stem_usage = [0u8; 4]; // Year, month, day, hour order
        for (_combo, p1, p2) in &rel.stem_combinations {
            let idx1 = match p1.as_str() {
                "year_stem" => 0,
                "month_stem" => 1,
                "day_stem" => 2,
                "hour_stem" => 3,
                _ => 99,
            };
            let idx2 = match p2.as_str() {
                "year_stem" => 0,
                "month_stem" => 1,
                "day_stem" => 2,
                "hour_stem" => 3,
                _ => 99,
            };
            if idx1 < 4 {
                stem_usage[idx1] += 1;
            }
            if idx2 < 4 {
                stem_usage[idx2] += 1;
            }
        }

        for (combo, p1, p2) in &rel.stem_combinations {
            let idx1 = match p1.as_str() {
                "year_stem" => 0,
                "month_stem" => 1,
                "day_stem" => 2,
                "hour_stem" => 3,
                _ => 99,
            };
            let idx2 = match p2.as_str() {
                "year_stem" => 0,
                "month_stem" => 1,
                "day_stem" => 2,
                "hour_stem" => 3,
                _ => 99,
            };

            // Transformation invalidated if rival/jealous combination occurs
            let is_competed =
                (idx1 < 4 && stem_usage[idx1] > 1) || (idx2 < 4 && stem_usage[idx2] > 1);

            if is_competed {
                let reason = format!(
                    "Transformation invalidated due to {}",
                    if stem_usage[idx1.min(3)] > 1 {
                        "Rival Combination (争合)"
                    } else {
                        "Jealous Combination (妬合)"
                    }
                );
                // Record reason as invalidated
                apply_stem_transform_failure(
                    pillars,
                    combo,
                    &reason,
                    &mut year_stem,
                    &mut month_stem,
                    &mut day_stem,
                    &mut hour_stem,
                );
                continue;
            }

            let transformed_element = combo.transformed_element();

            // Transform only if month branch supports or shares the target element
            let is_supported = month_branch_element == transformed_element
                || transformed_element.generated_by() == month_branch_element;

            if is_supported {
                let reason = format!("Transformed by {}", combo.hangul());
                apply_stem_transform(
                    pillars,
                    combo,
                    transformed_element,
                    &reason,
                    &mut year_stem,
                    &mut month_stem,
                    &mut day_stem,
                    &mut hour_stem,
                );
            }
        }

        // 2. Branch transformation (Triple combo precedence)
        let mut year_branch = EffectiveElement::new(pillars.year.branch.element());
        let mut month_branch = EffectiveElement::new(pillars.month.branch.element());
        let mut day_branch = EffectiveElement::new(pillars.day.branch.element());
        let mut hour_branch = pillars
            .hour
            .map(|h| EffectiveElement::new(h.branch.element()));

        for tri in &rel.triple_combinations {
            let transformed = tri.element();
            let reason = format!("Transformed by {}", tri.hangul());
            apply_triple_transform(
                pillars,
                tri,
                transformed,
                &reason,
                &mut year_branch,
                &mut month_branch,
                &mut day_branch,
                &mut hour_branch,
            );
        }

        // 3. Seasonal combination transformation
        for sea in &rel.seasonal_combinations {
            let transformed = sea.element();
            let reason = format!("Transformed by {}", sea.hangul());
            apply_seasonal_transform(
                pillars,
                sea,
                transformed,
                &reason,
                &mut year_branch,
                &mut month_branch,
                &mut day_branch,
                &mut hour_branch,
            );
        }

        // 4. Semi-combination transformation
        for (semi, _p1, _p2) in &rel.dominant_semi_combinations {
            // Element transformation
            let transformed = match semi {
                SemiCombination::YinWu | SemiCombination::WuXu => Element::Fire,
                SemiCombination::ShenZi | SemiCombination::ZiChen => Element::Water,
                SemiCombination::SiYou | SemiCombination::YouChou => Element::Metal,
                SemiCombination::HaiMao | SemiCombination::MaoWei => Element::Wood,
                _ => continue, // Safety fallback
            };

            let reason = format!("Transformed by {}", semi.hangul());
            apply_semi_transform(
                pillars,
                semi,
                transformed,
                &reason,
                &mut year_branch,
                &mut month_branch,
                &mut day_branch,
                &mut hour_branch,
            );
        }

        Self {
            year_stem,
            month_stem,
            day_stem,
            hour_stem,
            year_branch,
            month_branch,
            day_branch,
            hour_branch,
            major_stem: None,
            saewun_stem: None,
            major_branch: None,
            saewun_branch: None,
        }
    }

    /// Extended 5/6 pillar transformation analysis
    pub fn from_expanded(
        pillars: &FourPillars,
        major: Option<GanZi>,
        yearly: Option<GanZi>,
    ) -> Self {
        use crate::analysis::dynamic_luck::DynamicLuckAnalysis;
        let dyn_luck = DynamicLuckAnalysis::analyze(pillars, major, yearly, None, None, None);
        let rel = &dyn_luck.combined_relations;
        let month_branch_element = pillars.month.branch.element();

        let mut year_stem = EffectiveElement::new(pillars.year.stem.element());
        let mut month_stem = EffectiveElement::new(pillars.month.stem.element());
        let mut day_stem = EffectiveElement::new(pillars.day.stem.element());
        let mut hour_stem = pillars
            .hour
            .map(|h| EffectiveElement::new(h.stem.element()));
        let mut major_stem = major.map(|m| EffectiveElement::new(m.stem.element()));
        let mut saewun_stem = yearly.map(|y| EffectiveElement::new(y.stem.element()));

        // 1. Stem transformation processing
        for (combo, _p1, _p2) in &rel.stem_combinations {
            let transformed_element = combo.transformed_element();
            let is_supported = month_branch_element == transformed_element
                || transformed_element.generated_by() == month_branch_element;

            if is_supported {
                let reason = format!("Transformed by {}", combo.hangul());
                apply_stem_transform(
                    pillars,
                    combo,
                    transformed_element,
                    &reason,
                    &mut year_stem,
                    &mut month_stem,
                    &mut day_stem,
                    &mut hour_stem,
                );
                let (s1, s2) = combo.stems();
                if let (Some(ref mut ms), Some(m)) = (&mut major_stem, major) {
                    if m.stem == s1 || m.stem == s2 {
                        ms.effective = transformed_element;
                        ms.reason = Some(reason.clone());
                    }
                }
                if let (Some(ref mut ss), Some(y)) = (&mut saewun_stem, yearly) {
                    if y.stem == s1 || y.stem == s2 {
                        ss.effective = transformed_element;
                        ss.reason = Some(reason.clone());
                    }
                }
            }
        }

        // 2.    ()
        let mut year_branch = EffectiveElement::new(pillars.year.branch.element());
        let mut month_branch = EffectiveElement::new(pillars.month.branch.element());
        let mut day_branch = EffectiveElement::new(pillars.day.branch.element());
        let mut hour_branch = pillars
            .hour
            .map(|h| EffectiveElement::new(h.branch.element()));
        let mut major_branch = major.map(|m| EffectiveElement::new(m.branch.element()));
        let mut saewun_branch = yearly.map(|y| EffectiveElement::new(y.branch.element()));

        for tri in &rel.triple_combinations {
            let transformed = tri.element();
            let reason = format!("Transformed by {}", tri.hangul());
            apply_triple_transform(
                pillars,
                tri,
                transformed,
                &reason,
                &mut year_branch,
                &mut month_branch,
                &mut day_branch,
                &mut hour_branch,
            );
            let tri_bs = tri.branches();
            if let (Some(ref mut mb), Some(m)) = (&mut major_branch, major) {
                if tri_bs.contains(&m.branch) {
                    mb.effective = transformed;
                    mb.reason = Some(reason.clone());
                }
            }
            if let (Some(ref mut sb), Some(y)) = (&mut saewun_branch, yearly) {
                if tri_bs.contains(&y.branch) {
                    sb.effective = transformed;
                    sb.reason = Some(reason.clone());
                }
            }
        }

        // 3. Seasonal combination transformation
        for sea in &rel.seasonal_combinations {
            let transformed = sea.element();
            let reason = format!("Transformed by {}", sea.hangul());
            apply_seasonal_transform(
                pillars,
                sea,
                transformed,
                &reason,
                &mut year_branch,
                &mut month_branch,
                &mut day_branch,
                &mut hour_branch,
            );
            let sea_bs = sea.branches();
            if let (Some(ref mut mb), Some(m)) = (&mut major_branch, major) {
                if sea_bs.contains(&m.branch) {
                    mb.effective = transformed;
                    mb.reason = Some(reason.clone());
                }
            }
            if let (Some(ref mut sb), Some(y)) = (&mut saewun_branch, yearly) {
                if sea_bs.contains(&y.branch) {
                    sb.effective = transformed;
                    sb.reason = Some(reason.clone());
                }
            }
        }

        // 4. Semi-combination transformation
        for (semi, _p1, _p2) in &rel.dominant_semi_combinations {
            let transformed = match semi {
                SemiCombination::YinWu | SemiCombination::WuXu => Element::Fire,
                SemiCombination::ShenZi | SemiCombination::ZiChen => Element::Water,
                SemiCombination::SiYou | SemiCombination::YouChou => Element::Metal,
                SemiCombination::HaiMao | SemiCombination::MaoWei => Element::Wood,
                _ => continue,
            };

            let reason = format!("Transformed by {}", semi.hangul());
            apply_semi_transform(
                pillars,
                semi,
                transformed,
                &reason,
                &mut year_branch,
                &mut month_branch,
                &mut day_branch,
                &mut hour_branch,
            );
        }

        Self {
            year_stem,
            month_stem,
            day_stem,
            hour_stem,
            year_branch,
            month_branch,
            day_branch,
            hour_branch,
            major_stem,
            saewun_stem,
            major_branch,
            saewun_branch,
        }
    }

    /// Aggregate effective element counts
    pub fn counts(&self) -> [(Element, f32); 5] {
        let mut counts = [
            (Element::Wood, 0.0),
            (Element::Fire, 0.0),
            (Element::Earth, 0.0),
            (Element::Metal, 0.0),
            (Element::Water, 0.0),
        ];

        let mut elements = vec![
            &self.year_stem,
            &self.month_stem,
            &self.day_stem,
            &self.year_branch,
            &self.month_branch,
            &self.day_branch,
        ];
        if let Some(ref h) = self.hour_stem {
            elements.push(h);
        }
        if let Some(ref h) = self.hour_branch {
            elements.push(h);
        }
        if let Some(ref s) = self.major_stem {
            elements.push(s);
        }
        if let Some(ref s) = self.saewun_stem {
            elements.push(s);
        }
        if let Some(ref b) = self.major_branch {
            elements.push(b);
        }
        if let Some(ref b) = self.saewun_branch {
            elements.push(b);
        }

        for eff in elements {
            let idx = eff.effective.index() as usize;
            counts[idx].1 += 1.0;
        }

        counts
    }
}

impl EffectiveElement {
    fn new(element: Element) -> Self {
        Self {
            original: element,
            effective: element,
            reason: None,
        }
    }
}

// Helper functions
fn apply_stem_transform(
    pillars: &FourPillars,
    combo: &StemCombination,
    target: Element,
    reason: &str,
    y: &mut EffectiveElement,
    m: &mut EffectiveElement,
    d: &mut EffectiveElement,
    h: &mut Option<EffectiveElement>,
) {
    let (s1, s2) = match combo {
        StemCombination::JiaJi => (HeavenlyStem::Jia, HeavenlyStem::Ji),
        StemCombination::YiGeng => (HeavenlyStem::Yi, HeavenlyStem::Geng),
        StemCombination::BingXin => (HeavenlyStem::Bing, HeavenlyStem::Xin),
        StemCombination::DingRen => (HeavenlyStem::Ding, HeavenlyStem::Ren),
        StemCombination::WuGui => (HeavenlyStem::Wu, HeavenlyStem::Gui),
    };
    if pillars.year.stem == s1 || pillars.year.stem == s2 {
        y.effective = target;
        y.reason = Some(reason.to_string());
    }
    if pillars.month.stem == s1 || pillars.month.stem == s2 {
        m.effective = target;
        m.reason = Some(reason.to_string());
    }
    if pillars.day.stem == s1 || pillars.day.stem == s2 {
        d.effective = target;
        d.reason = Some(reason.to_string());
    }
    if let (Some(h), Some(hp)) = (h.as_mut(), pillars.hour) {
        if hp.stem == s1 || hp.stem == s2 {
            h.effective = target;
            h.reason = Some(reason.to_string());
        }
    }
}

fn apply_stem_transform_failure(
    pillars: &FourPillars,
    combo: &StemCombination,
    reason: &str,
    y: &mut EffectiveElement,
    m: &mut EffectiveElement,
    d: &mut EffectiveElement,
    h: &mut Option<EffectiveElement>,
) {
    let (s1, s2) = match combo {
        StemCombination::JiaJi => (HeavenlyStem::Jia, HeavenlyStem::Ji),
        StemCombination::YiGeng => (HeavenlyStem::Yi, HeavenlyStem::Geng),
        StemCombination::BingXin => (HeavenlyStem::Bing, HeavenlyStem::Xin),
        StemCombination::DingRen => (HeavenlyStem::Ding, HeavenlyStem::Ren),
        StemCombination::WuGui => (HeavenlyStem::Wu, HeavenlyStem::Gui),
    };
    // Retain reason while keeping original element
    if (pillars.year.stem == s1 || pillars.year.stem == s2) && y.reason.is_none() {
        y.reason = Some(reason.to_string());
    }
    if (pillars.month.stem == s1 || pillars.month.stem == s2) && m.reason.is_none() {
        m.reason = Some(reason.to_string());
    }
    if (pillars.day.stem == s1 || pillars.day.stem == s2) && d.reason.is_none() {
        d.reason = Some(reason.to_string());
    }
    if let (Some(h), Some(hp)) = (h.as_mut(), pillars.hour) {
        if (hp.stem == s1 || hp.stem == s2) && h.reason.is_none() {
            h.reason = Some(reason.to_string());
        }
    }
}

fn apply_triple_transform(
    pillars: &FourPillars,
    tri: &TripleCombination,
    target: Element,
    reason: &str,
    y: &mut EffectiveElement,
    m: &mut EffectiveElement,
    d: &mut EffectiveElement,
    h: &mut Option<EffectiveElement>,
) {
    let branches = match tri {
        TripleCombination::YinWuXu => {
            vec![EarthlyBranch::Yin, EarthlyBranch::Wu, EarthlyBranch::Xu]
        }
        TripleCombination::ShenZiChen => {
            vec![EarthlyBranch::Shen, EarthlyBranch::Zi, EarthlyBranch::Chen]
        }
        TripleCombination::SiYouChou => {
            vec![EarthlyBranch::Si, EarthlyBranch::You, EarthlyBranch::Chou]
        }
        TripleCombination::HaiMaoWei => {
            vec![EarthlyBranch::Hai, EarthlyBranch::Mao, EarthlyBranch::Wei]
        }
    };
    if branches.contains(&pillars.year.branch) {
        y.effective = target;
        y.reason = Some(reason.to_string());
    }
    if branches.contains(&pillars.month.branch) {
        m.effective = target;
        m.reason = Some(reason.to_string());
    }
    if branches.contains(&pillars.day.branch) {
        d.effective = target;
        d.reason = Some(reason.to_string());
    }
    if let (Some(h), Some(hp)) = (h.as_mut(), pillars.hour) {
        if branches.contains(&hp.branch) {
            h.effective = target;
            h.reason = Some(reason.to_string());
        }
    }
}

fn apply_seasonal_transform(
    pillars: &FourPillars,
    sea: &SeasonalCombination,
    target: Element,
    reason: &str,
    y: &mut EffectiveElement,
    m: &mut EffectiveElement,
    d: &mut EffectiveElement,
    h: &mut Option<EffectiveElement>,
) {
    let branches = match sea {
        SeasonalCombination::YinMaoChen => {
            vec![EarthlyBranch::Yin, EarthlyBranch::Mao, EarthlyBranch::Chen]
        }
        SeasonalCombination::SiWuWei => {
            vec![EarthlyBranch::Si, EarthlyBranch::Wu, EarthlyBranch::Wei]
        }
        SeasonalCombination::ShenYouXu => {
            vec![EarthlyBranch::Shen, EarthlyBranch::You, EarthlyBranch::Xu]
        }
        SeasonalCombination::HaiZiChou => {
            vec![EarthlyBranch::Hai, EarthlyBranch::Zi, EarthlyBranch::Chou]
        }
    };
    if branches.contains(&pillars.year.branch) {
        y.effective = target;
        y.reason = Some(reason.to_string());
    }
    if branches.contains(&pillars.month.branch) {
        m.effective = target;
        m.reason = Some(reason.to_string());
    }
    if branches.contains(&pillars.day.branch) {
        d.effective = target;
        d.reason = Some(reason.to_string());
    }
    if let (Some(h), Some(hp)) = (h.as_mut(), pillars.hour) {
        if branches.contains(&hp.branch) {
            h.effective = target;
            h.reason = Some(reason.to_string());
        }
    }
}

fn apply_semi_transform(
    pillars: &FourPillars,
    semi: &SemiCombination,
    target: Element,
    reason: &str,
    y: &mut EffectiveElement,
    m: &mut EffectiveElement,
    d: &mut EffectiveElement,
    h: &mut Option<EffectiveElement>,
) {
    let branches = match semi {
        SemiCombination::YinWu => vec![EarthlyBranch::Yin, EarthlyBranch::Wu],
        SemiCombination::WuXu => vec![EarthlyBranch::Wu, EarthlyBranch::Xu],
        SemiCombination::ShenZi => vec![EarthlyBranch::Shen, EarthlyBranch::Zi],
        SemiCombination::ZiChen => vec![EarthlyBranch::Zi, EarthlyBranch::Chen],
        SemiCombination::SiYou => vec![EarthlyBranch::Si, EarthlyBranch::You],
        SemiCombination::YouChou => vec![EarthlyBranch::You, EarthlyBranch::Chou],
        SemiCombination::HaiMao => vec![EarthlyBranch::Hai, EarthlyBranch::Mao],
        SemiCombination::MaoWei => vec![EarthlyBranch::Mao, EarthlyBranch::Wei],
        _ => return,
    };
    if branches.contains(&pillars.year.branch) {
        y.effective = target;
        y.reason = Some(reason.to_string());
    }
    if branches.contains(&pillars.month.branch) {
        m.effective = target;
        m.reason = Some(reason.to_string());
    }
    if branches.contains(&pillars.day.branch) {
        d.effective = target;
        d.reason = Some(reason.to_string());
    }
    if let (Some(h), Some(hp)) = (h.as_mut(), pillars.hour) {
        if branches.contains(&hp.branch) {
            h.effective = target;
            h.reason = Some(reason.to_string());
        }
    }
}

impl std::fmt::Display for TransformationAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【Element Transformation (合化) Analysis】")?;
        writeln!(f, "─────────────────────────────────")?;

        let items = [
            ("year_stem", Some(&self.year_stem)),
            ("month_stem", Some(&self.month_stem)),
            ("day_stem", Some(&self.day_stem)),
            ("hour_stem", self.hour_stem.as_ref()),
            ("year_branch", Some(&self.year_branch)),
            ("month_branch", Some(&self.month_branch)),
            ("day_branch", Some(&self.day_branch)),
            ("hour_branch", self.hour_branch.as_ref()),
        ];

        let mut has_transform = false;
        for (name, eff) in items.into_iter().filter_map(|(n, e)| e.map(|e| (n, e))) {
            if let Some(reason) = &eff.reason {
                writeln!(
                    f,
                    "  {} ({} → {}): {}",
                    name,
                    eff.original.hangul(),
                    eff.effective.hangul(),
                    reason
                )?;
                has_transform = true;
            }
        }

        if !has_transform {
            writeln!(f, "  No transformed elements.")?;
        }

        writeln!(f, "\n[Effective Element Statistics]")?;
        for (el, count) in self.counts() {
            if count > 0.0 {
                writeln!(f, "  {}: {:.1}", el.hangul(), count)?;
            }
        }
        Ok(())
    }
}

impl FourPillars {
    /// Element transformation analysis
    pub fn transformations(&self) -> TransformationAnalysis {
        TransformationAnalysis::from_pillars(self)
    }

    /// Extended 5/6 pillar transformation analysis
    pub fn transformations_expanded(
        &self,
        major: Option<GanZi>,
        yearly: Option<GanZi>,
    ) -> TransformationAnalysis {
        TransformationAnalysis::from_expanded(self, major, yearly)
    }

    /// Extract effective element map
    /// Returns: [(original, effective); 8] in order year/month/day/hour stem then branch;
    /// the two hour slots (indices 3 and 7) are `None` when the birth hour is unknown.
    pub fn effective_elements(&self) -> [Option<(Element, Element)>; 8] {
        let trans = self.transformations();
        let pair = |e: &EffectiveElement| (e.original, e.effective);
        [
            Some(pair(&trans.year_stem)),
            Some(pair(&trans.month_stem)),
            Some(pair(&trans.day_stem)),
            trans.hour_stem.as_ref().map(pair),
            Some(pair(&trans.year_branch)),
            Some(pair(&trans.month_branch)),
            Some(pair(&trans.day_branch)),
            trans.hour_branch.as_ref().map(pair),
        ]
    }
}
