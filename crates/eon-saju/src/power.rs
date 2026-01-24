//! 오행 및 십성 점수 정밀 분석 (Power & Ten Gods Scoring)
//!
//! 보정 옵션에 따른 4가지 분석 모드를 지원합니다.
//! 1. 기본 (보정 X)
//! 2. 합화 적용
//! 3. 궁성/조후 보정 적용
//! 4. 합화 + 궁성/조후 보정 모두 적용

use serde::{Deserialize, Serialize};
use crate::stem::HeavenlyStem;
use crate::branch::EarthlyBranch;
use crate::element::Element;
use crate::pillars::FourPillars;
use crate::ten_gods::TenGod;

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
    pub fn calculate(pillars: &FourPillars, options: AnalysisOptions) -> Self {
        let dm = pillars.day_master();
        let rel = pillars.relationships();
        let month_branch = pillars.month.branch;

        // 1. 가중치 설정
        // 궁성 보정 적용 시: 천간(10), 월지(35), 일지(15), 년/시지(10) = 총 110점
        // 보정 미적용 시: 모든 기둥 동일 가중치 (각 1) = 총 8점
        let weights = if options.apply_correction {
            [10.0, 10.0, 10.0, 10.0, 10.0, 35.0, 15.0, 10.0] // [년간, 월간, 일간, 시간, 년지, 월지, 일지, 시지]
        } else {
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]
        };

        let total_weight: f32 = weights.iter().sum();

        // 2. 각 위치별 실질 오행 및 십성 결정
        let mut el_scores = [0.0f32; 5];
        let mut tg_scores = [0.0f32; 10];

        // 기둥 데이터 배열화
        let stems = [pillars.year.stem, pillars.month.stem, pillars.day.stem, pillars.hour.stem];
        let branches = [pillars.year.branch, pillars.month.branch, pillars.day.branch, pillars.hour.branch];

        // 2-1. 천간 처리
        for i in 0..4 {
            let stem = stems[i];
            let mut element = stem.element();
            let weight = weights[i];

            // 합화 적용
            if options.apply_transform {
                for (combo, p1, p2) in &rel.stem_combinations {
                    if (i == 0 && (p1 == "년간" || p2 == "년간")) ||
                       (i == 1 && (p1 == "월간" || p2 == "월간")) ||
                       (i == 2 && (p1 == "일간" || p2 == "일간")) ||
                       (i == 3 && (p1 == "시간" || p2 == "시간")) {
                        // 합화 조건 (월령 득함) 확인은 생략하거나 이미 transformations에서 계산된 결과 사용
                        // 여기서는 단순 합화 오행을 바로 적용
                        element = combo.transformed_element();
                    }
                }
            }

            el_scores[element.index() as usize] += weight;
            
            // 십성 계산
            let god = if options.apply_transform && element != stem.element() {
                // 합화된 경우, 변한 오행과 일간의 관계 (음양은 원본 유지 또는 합화 오행 특징 적용)
                // 만세력 로직에 따라 합화된 오행을 직접 십성으로 변환
                TenGod::from_stems(dm, get_dummy_stem(element, stem.polarity()))
            } else {
                TenGod::from_stems(dm, stem)
            };
            tg_scores[god.index()] += weight;
        }

        // 2-2. 지지 처리
        for i in 0..4 {
            let branch = branches[i];
            let mut element = branch.element();
            let weight = weights[i + 4];

            // 조후 보정 (Climate) - 예: 여름의 미토는 화로 취급
            if options.apply_correction {
                element = apply_climate_correction(branch, month_branch);
            }

            // 지지 합화 (삼합 및 방합)
            if options.apply_transform {
                // 삼합 체크
                for tri in &rel.triple_combinations {
                    let combo_branches = match tri {
                        crate::relationships::TripleCombination::YinWuXu => vec![EarthlyBranch::Yin, EarthlyBranch::Wu, EarthlyBranch::Xu],
                        crate::relationships::TripleCombination::ShenZiChen => vec![EarthlyBranch::Shen, EarthlyBranch::Zi, EarthlyBranch::Chen],
                        crate::relationships::TripleCombination::SiYouChou => vec![EarthlyBranch::Si, EarthlyBranch::You, EarthlyBranch::Chou],
                        crate::relationships::TripleCombination::HaiMaoWei => vec![EarthlyBranch::Hai, EarthlyBranch::Mao, EarthlyBranch::Wei],
                    };
                    if combo_branches.contains(&branch) {
                        element = tri.element();
                    }
                }
                // 방합 체크
                for sea in &rel.seasonal_combinations {
                    let combo_branches = match sea {
                        crate::relationships::SeasonalCombination::YinMaoChen => vec![EarthlyBranch::Yin, EarthlyBranch::Mao, EarthlyBranch::Chen],
                        crate::relationships::SeasonalCombination::SiWuWei => vec![EarthlyBranch::Si, EarthlyBranch::Wu, EarthlyBranch::Wei],
                        crate::relationships::SeasonalCombination::ShenYouXu => vec![EarthlyBranch::Shen, EarthlyBranch::You, EarthlyBranch::Xu],
                        crate::relationships::SeasonalCombination::HaiZiChou => vec![EarthlyBranch::Hai, EarthlyBranch::Zi, EarthlyBranch::Chou],
                    };
                    if combo_branches.contains(&branch) {
                        element = sea.element();
                    }
                }
            }

            el_scores[element.index() as usize] += weight;
            
            // 지지 십성
            let god = TenGod::from_stems(dm, branch.primary_stem()); // 정기 기준
            // 만약 합화되었다면 십성도 변경
            let final_tg = if element != branch.element() {
                TenGod::from_stems(dm, get_dummy_stem(element, branch.primary_stem().polarity()))
            } else {
                god
            };
            tg_scores[final_tg.index()] += weight;
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
fn get_dummy_stem(element: Element, polarity: crate::element::Polarity) -> HeavenlyStem {
    use crate::element::Polarity::*;
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
    pub fn integrated_analysis(&self, options: AnalysisOptions) -> IntegratedAnalysis {
        IntegratedAnalysis::calculate(self, options)
    }

    /// 기본 옵션으로 분석 수행
    pub fn analyze(&self) -> IntegratedAnalysis {
        self.integrated_analysis(AnalysisOptions::default())
    }
}
