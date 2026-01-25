//! 합화(合化, Element Transformation) 분석
//! 
//! 천간합 및 지지합이 특정 조건에서 오행이 변하는 로직을 구현합니다.

use serde::{Deserialize, Serialize};
use crate::core::stem::HeavenlyStem;
use crate::core::branch::EarthlyBranch;
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use crate::analysis::relationships::{StemCombination, TripleCombination, SeasonalCombination};

/// 합화 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectiveElement {
    /// 원래 오행
    pub original: Element,
    /// 변한 오행 (합화된 경우)
    pub effective: Element,
    /// 합화 원인 (있을 경우)
    pub reason: Option<String>,
}

/// 사주 전체의 실질적 오행 통계
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationAnalysis {
    /// 년간 실질 오행
    pub year_stem: EffectiveElement,
    /// 월간 실질 오행
    pub month_stem: EffectiveElement,
    /// 일간 실질 오행
    pub day_stem: EffectiveElement,
    /// 시간 실질 오행
    pub hour_stem: EffectiveElement,
    /// 지지 4곳 포함 (지지는 주로 삼합 반영)
    pub year_branch: EffectiveElement,
    pub month_branch: EffectiveElement,
    pub day_branch: EffectiveElement,
    pub hour_branch: EffectiveElement,
}

impl TransformationAnalysis {
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        let rel = pillars.relationships();
        let month_branch_element = pillars.month.branch.element();
        
        let mut year_stem = EffectiveElement::new(pillars.year.stem.element());
        let mut month_stem = EffectiveElement::new(pillars.month.stem.element());
        let mut day_stem = EffectiveElement::new(pillars.day.stem.element());
        let mut hour_stem = EffectiveElement::new(pillars.hour.stem.element());

        // 1. 천간 합화 처리
        for (combo, _, _) in &rel.stem_combinations {
            let transformed_element = combo.transformed_element();

            // 월지가 합화되는 오행을 돕거나 같을 때만 화(화)하는 것으로 간주
            let is_supported = month_branch_element == transformed_element || 
                             transformed_element.generated_by() == month_branch_element;

            if is_supported {
                let reason = format!("{}에 의한 합화", combo.hangul());
                apply_stem_transform(pillars, combo, transformed_element, &reason, 
                                   &mut year_stem, &mut month_stem, &mut day_stem, &mut hour_stem);
            }
        }

        // 2. 지지 합화 처리 (삼합 우선)
        let mut year_branch = EffectiveElement::new(pillars.year.branch.element());
        let mut month_branch = EffectiveElement::new(pillars.month.branch.element());
        let mut day_branch = EffectiveElement::new(pillars.day.branch.element());
        let mut hour_branch = EffectiveElement::new(pillars.hour.branch.element());

        for tri in &rel.triple_combinations {
            let transformed = tri.element();
            let reason = format!("{}에 의한 합화", tri.hangul());
            apply_triple_transform(pillars, tri, transformed, &reason,
                                 &mut year_branch, &mut month_branch, &mut day_branch, &mut hour_branch);
        }

        // 3. 지지 방합 처리
        for sea in &rel.seasonal_combinations {
            let transformed = sea.element();
            let reason = format!("{}에 의한 합화", sea.hangul());
            apply_seasonal_transform(pillars, sea, transformed, &reason,
                                   &mut year_branch, &mut month_branch, &mut day_branch, &mut hour_branch);
        }

        Self {
            year_stem, month_stem, day_stem, hour_stem,
            year_branch, month_branch, day_branch, hour_branch,
        }
    }

    /// 실질 오행 개수 집계
    pub fn counts(&self) -> [(Element, f32); 5] {
        let mut counts = [
            (Element::Wood, 0.0), (Element::Fire, 0.0), (Element::Earth, 0.0),
            (Element::Metal, 0.0), (Element::Water, 0.0),
        ];

        let elements = [
            &self.year_stem, &self.month_stem, &self.day_stem, &self.hour_stem,
            &self.year_branch, &self.month_branch, &self.day_branch, &self.hour_branch,
        ];

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

// 헬퍼 함수들
fn apply_stem_transform(
    pillars: &FourPillars, 
    combo: &StemCombination, 
    target: Element, 
    reason: &str,
    y: &mut EffectiveElement, m: &mut EffectiveElement, d: &mut EffectiveElement, h: &mut EffectiveElement
) {
    let (s1, s2) = match combo {
        StemCombination::JiaJi => (HeavenlyStem::Jia, HeavenlyStem::Ji),
        StemCombination::YiGeng => (HeavenlyStem::Yi, HeavenlyStem::Geng),
        StemCombination::BingXin => (HeavenlyStem::Bing, HeavenlyStem::Xin),
        StemCombination::DingRen => (HeavenlyStem::Ding, HeavenlyStem::Ren),
        StemCombination::WuGui => (HeavenlyStem::Wu, HeavenlyStem::Gui),
    };
    if pillars.year.stem == s1 || pillars.year.stem == s2 { y.effective = target; y.reason = Some(reason.to_string()); }
    if pillars.month.stem == s1 || pillars.month.stem == s2 { m.effective = target; m.reason = Some(reason.to_string()); }
    if pillars.day.stem == s1 || pillars.day.stem == s2 { d.effective = target; d.reason = Some(reason.to_string()); }
    if pillars.hour.stem == s1 || pillars.hour.stem == s2 { h.effective = target; h.reason = Some(reason.to_string()); }
}

fn apply_triple_transform(
    pillars: &FourPillars,
    tri: &TripleCombination,
    target: Element,
    reason: &str,
    y: &mut EffectiveElement, m: &mut EffectiveElement, d: &mut EffectiveElement, h: &mut EffectiveElement
) {
    let branches = match tri {
        TripleCombination::YinWuXu => vec![EarthlyBranch::Yin, EarthlyBranch::Wu, EarthlyBranch::Xu],
        TripleCombination::ShenZiChen => vec![EarthlyBranch::Shen, EarthlyBranch::Zi, EarthlyBranch::Chen],
        TripleCombination::SiYouChou => vec![EarthlyBranch::Si, EarthlyBranch::You, EarthlyBranch::Chou],
        TripleCombination::HaiMaoWei => vec![EarthlyBranch::Hai, EarthlyBranch::Mao, EarthlyBranch::Wei],
    };
    if branches.contains(&pillars.year.branch) { y.effective = target; y.reason = Some(reason.to_string()); }
    if branches.contains(&pillars.month.branch) { m.effective = target; m.reason = Some(reason.to_string()); }
    if branches.contains(&pillars.day.branch) { d.effective = target; d.reason = Some(reason.to_string()); }
    if branches.contains(&pillars.hour.branch) { h.effective = target; h.reason = Some(reason.to_string()); }
}

fn apply_seasonal_transform(
    pillars: &FourPillars,
    sea: &SeasonalCombination,
    target: Element,
    reason: &str,
    y: &mut EffectiveElement, m: &mut EffectiveElement, d: &mut EffectiveElement, h: &mut EffectiveElement
) {
    let branches = match sea {
        SeasonalCombination::YinMaoChen => vec![EarthlyBranch::Yin, EarthlyBranch::Mao, EarthlyBranch::Chen],
        SeasonalCombination::SiWuWei => vec![EarthlyBranch::Si, EarthlyBranch::Wu, EarthlyBranch::Wei],
        SeasonalCombination::ShenYouXu => vec![EarthlyBranch::Shen, EarthlyBranch::You, EarthlyBranch::Xu],
        SeasonalCombination::HaiZiChou => vec![EarthlyBranch::Hai, EarthlyBranch::Zi, EarthlyBranch::Chou],
    };
    if branches.contains(&pillars.year.branch) { y.effective = target; y.reason = Some(reason.to_string()); }
    if branches.contains(&pillars.month.branch) { m.effective = target; m.reason = Some(reason.to_string()); }
    if branches.contains(&pillars.day.branch) { d.effective = target; d.reason = Some(reason.to_string()); }
    if branches.contains(&pillars.hour.branch) { h.effective = target; h.reason = Some(reason.to_string()); }
}

impl std::fmt::Display for TransformationAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【오행 합화(合化) 분석】")?;
        writeln!(f, "─────────────────────────────────")?;
        
        let items = [
            ("년간", &self.year_stem), ("월간", &self.month_stem), ("일간", &self.day_stem), ("시간", &self.hour_stem),
            ("년지", &self.year_branch), ("월지", &self.month_branch), ("일지", &self.day_branch), ("시지", &self.hour_branch),
        ];

        let mut has_transform = false;
        for (name, eff) in items {
            if let Some(reason) = &eff.reason {
                writeln!(f, "  {} ({} → {}): {}", name, eff.original.hangul(), eff.effective.hangul(), reason)?;
                has_transform = true;
            }
        }

        if !has_transform {
            writeln!(f, "  합화된 오행이 없습니다.")?;
        }

        writeln!(f, "\n[실질 오행 통계]")?;
        for (el, count) in self.counts() {
            if count > 0.0 {
                writeln!(f, "  {}: {:.1}개", el.hangul(), count)?;
            }
        }
        Ok(())
    }
}

impl FourPillars {
    /// 오행 합화 분석
    pub fn transformations(&self) -> TransformationAnalysis {
        TransformationAnalysis::from_pillars(self)
    }
}
