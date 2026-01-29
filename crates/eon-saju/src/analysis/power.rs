//! 오행 및 십성 점수 정밀 분석 (Power & Ten Gods Scoring)
//!
//! 보정 옵션에 따른 4가지 분석 모드를 지원합니다.
//! 1. 기본 (보정 X)
//! 2. 합화 적용
//! 3. 궁성/조후 보정 적용
//! 4. 합화 + 궁성/조후 보정 모두 적용

use serde::{Deserialize, Serialize};
use crate::core::stem::HeavenlyStem;
use crate::core::branch::EarthlyBranch;
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use crate::core::ten_gods::TenGod;

/// 분석 옵션
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AnalysisOptions {
    /// 합에 따른 오행 변화 적용 (Transformations)
    pub apply_transform: bool,
    /// 조후와 궁성 보정값 적용 (Climate & Palace Corrections)
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

/// 분석 엔진 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedAnalysis {
    pub options: AnalysisOptions,
    /// 오행 비율 (Element, Percentage, Score)
    pub element_scores: Vec<(Element, f32, f32)>,
    /// 십성 비율 (TenGod, Percentage, Score)
    pub ten_god_scores: Vec<(TenGod, f32, f32)>,
    /// 대표 오행
    pub dominant_element: Element,
    /// 대표 십성
    pub dominant_ten_god: TenGod,
}

impl IntegratedAnalysis {
    pub fn calculate(pillars: &FourPillars, options: AnalysisOptions, config: &crate::core::config::AnalysisConfig) -> Self {
        let dm = pillars.day_master();
        let month_branch = pillars.month.branch;

        // 1. 가중치 설정
        let weights = if options.apply_correction {
            [
                config.weights.stem, 
                config.weights.stem, 
                config.weights.stem, 
                config.weights.stem, 
                config.weights.other_branch, 
                config.weights.month_branch, 
                config.weights.day_branch, 
                config.weights.other_branch
            ]
        } else {
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]
        };

        // 가중치 스케일링 (전체 합이 가중치 합이 되도록 함)
        // 기존 110점법 호환을 위해, 만약 stem=1.0이면 10.0으로 취급하고 싶을 경우를 고려할 수 있으나
        // 여기서는 config에 정의된 대로 사용함. (보통 stem 1.0, month 3.5 등)
        
        let total_weight: f32 = weights.iter().sum();

        // 2. 각 위치별 실질 오행 및 십성 결정
        let mut el_scores = [0.0f32; 5];
        let mut tg_scores = [0.0f32; 10];

        // 실질 오행 맵 (합화 반영 여부에 따라 결정)
        let eff_map = if options.apply_transform {
            pillars.effective_elements()
        } else {
            let mut map = [(Element::Wood, Element::Wood); 8];
            let stems = [pillars.year.stem, pillars.month.stem, pillars.day.stem, pillars.hour.stem];
            let branches = [pillars.year.branch, pillars.month.branch, pillars.day.branch, pillars.hour.branch];
            for i in 0..4 {
                map[i] = (stems[i].element(), stems[i].element());
                map[i+4] = (branches[i].element(), branches[i].element());
            }
            map
        };

        // 2-1. 천간 처리 (0~3)
        let stems = [pillars.year.stem, pillars.month.stem, pillars.day.stem, pillars.hour.stem];
        for i in 0..4 {
            let stem = stems[i];
            let (_, effective_el) = eff_map[i];
            let weight = weights[i];

            el_scores[effective_el.index() as usize] += weight;
            
            // 십성 계산
            let god = if effective_el != stem.element() {
                // 합화된 경우
                TenGod::from_stems(dm, get_dummy_stem(effective_el, stem.polarity()))
            } else {
                TenGod::from_stems(dm, stem)
            };
            tg_scores[god.index()] += weight;
        }

        // 2-2. 지지 처리 (4~7)
        let branches = [pillars.year.branch, pillars.month.branch, pillars.day.branch, pillars.hour.branch];
        for i in 0..4 {
            let branch = branches[i];
            let (_, mut effective_el) = eff_map[i + 4];
            let weight = weights[i + 4];

            // 조후 보정 (Climate) - 예: 여름의 미토는 화로 취급 (합화되지 않은 경우에만 또는 보정이 우선일 수 있음)
            // 여기서는 보정이 합화보다 시급한 물리적 변동으로 간주함
            if options.apply_correction {
                effective_el = apply_climate_correction(branch, month_branch);
            }

            el_scores[effective_el.index() as usize] += weight;
            
            // 지지 십성
            let god = if effective_el != branch.element() {
                TenGod::from_stems(dm, get_dummy_stem(effective_el, branch.primary_stem().polarity()))
            } else {
                TenGod::from_stems(dm, branch.primary_stem())
            };
            tg_scores[god.index()] += weight;
        }

        // 3. 결과 포맷팅
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

        let dominant_element = final_el.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0;
        let dominant_ten_god = final_tg.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0;

        Self {
            options,
            element_scores: final_el,
            ten_god_scores: final_tg,
            dominant_element,
            dominant_ten_god,
        }
    }
}

/// 조후 보정 로직
fn apply_climate_correction(branch: EarthlyBranch, month: EarthlyBranch) -> Element {
    match branch {
        EarthlyBranch::Wei => {
            // 여름(巳, 午)의 미토는 화(火)의 기운이 강함
            if matches!(month, EarthlyBranch::Si | EarthlyBranch::Wu) { Element::Fire } 
            else { Element::Earth }
        },
        EarthlyBranch::Chou => {
            // 겨울(亥, 子)의 축토는 수(水)의 기운이 강함
            if matches!(month, EarthlyBranch::Hai | EarthlyBranch::Zi) { Element::Water }
            else { Element::Earth }
        },
        _ => branch.element()
    }
}

/// 오행과 음양을 받아 임시 천간을 반환 (십성 계산용)
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
        writeln!(f, "합에 따른 오행 변화 적용 {}", if self.options.apply_transform { "o" } else { "x" })?;
        writeln!(f, "조후와 궁성 보정값 적용 {}", if self.options.apply_correction { "o" } else { "x" })?;
        writeln!(f, "{} {}", self.dominant_element.hanja(), self.dominant_ten_god.hanja())?;
        
        writeln!(f, "오행")?;
        for (el, percent, _) in &self.element_scores {
            if *percent > 0.0 {
                let state = if *percent < 10.0 { "부족" } else if *percent <= 20.0 { "적정" } else if *percent <= 35.0 { "발달" } else { "과다" };
                writeln!(f, "  {}({})  {:.1}% {}", el.hangul(), el.hanja(), percent, state)?;
            }
        }
        
        writeln!(f, "십성")?;
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
    /// 통합 정밀 분석
    pub fn integrated_analysis(&self, options: AnalysisOptions, config: &crate::core::config::AnalysisConfig) -> IntegratedAnalysis {
        IntegratedAnalysis::calculate(self, options, config)
    }

    /// 기본 옵션 및 기본 설정으로 분석 수행
    pub fn analyze(&self) -> IntegratedAnalysis {
        self.integrated_analysis(AnalysisOptions::default(), &crate::core::config::AnalysisConfig::default())
    }
}

use crate::analysis::Analyzable;

impl Analyzable for IntegratedAnalysis {
    type Output = IntegratedAnalysis;
    fn analyze(pillars: &FourPillars, config: &crate::core::config::AnalysisConfig) -> Self::Output {
        IntegratedAnalysis::calculate(pillars, AnalysisOptions::default(), config)
    }
}
