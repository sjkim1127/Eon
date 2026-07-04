//! 용신(用神, Useful God) 정밀 분석
//!
//! 억부(抑扶), 조후(調候), 통관(通關), 병약(病藥)의 관점에서
//! 사주의 균형을 맞추는 최적의 오행을 찾습니다.

use crate::analysis::strength::{StrengthAnalysis, StrengthType};
use crate::analysis::structure::StructureType;
use crate::core::branch::EarthlyBranch;
use crate::core::config::AnalysisConfig;
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use serde::{Deserialize, Serialize};

/// 용신의 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum YongshinType {
    /// 억부용신 (일간의 강약을 조절)
    Eokbu,
    /// 조후용신 (기후와 온도를 조절)
    Johu,
    /// 통관용신 (대립하는 기운을 소통)
    Tonggwan,
    /// 병약용신 (병이 되는 기운을 치유)
    Byeongyak,
}

impl YongshinType {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Eokbu => "억부용신",
            Self::Johu => "조후용신",
            Self::Tonggwan => "통관용신",
            Self::Byeongyak => "병약용신",
        }
    }
}

/// 낱개 용신 추천 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedYongshin {
    pub yongshin_type: YongshinType,
    pub element: Element,
    pub summary: String,
    pub description: String,
    pub reasons: Vec<String>,
}

/// 용신 분석 종합 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YongshinAnalysis {
    /// 추천된 용신 목록
    pub recommendations: Vec<RecommendedYongshin>,
    /// 가장 우선시되는 제1용신
    pub primary: Element,
    /// 이를 돕는 희신(喜神)
    pub assistant: Element,
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

        let eokbu_element = match structure_analysis.structure {
            StructureType::JongAh => day_master_el.generates(),
            StructureType::JongJae => day_master_el.generates().generates(), // 재성
            StructureType::JongSal => day_master_el.controlled_by(),
            StructureType::JongGang => day_master_el.generated_by(),
            StructureType::JongWang => day_master_el,
            StructureType::Follower => pillars.month.branch.element(),
            StructureType::SpecialTransformation => day_master_el,
            _ => {
                match strength.strength_type {
                    StrengthType::Weak => {
                        // 신약(身弱)은 항상 인성(印星)이 억부용신
                        // - 인성이 원국에 없으면: 인성 보충 필요
                        // - 인성이 원국에 있어도: 그 힘을 더 강화해야 함
                        day_master_el.generated_by() // 인성
                    }
                    StrengthType::Strong => {
                        // 신강의 원인을 분석하여 용신 세분화
                        let yinxing = strength.deuk_se.yinxing_count as f32;
                        let bijie = strength.deuk_se.bijie_count as f32;

                        if yinxing > bijie * 1.5 {
                            // 인성 과다로 신강: 재성으로 인성 극복 (용재파인)
                            day_master_el.generates() // 재성
                        } else if bijie > yinxing * 1.5 {
                            // 비겁 과다로 신강: 관성으로 비겁 제어 (관살제겁)
                            day_master_el.controlled_by() // 관성
                        } else {
                            // 인성/비겁 균형: 식상으로 설기 (설기생재)
                            day_master_el.generates() // 식상
                        }
                    }
                    StrengthType::Balanced => day_master_el,
                }
            }
        };

        // 종격/전왕격인 경우 억부(격국) 용신을 최우선으로 배치하고 조후는 참고로만 제시
        let is_polarized = matches!(
            structure_analysis.structure,
            StructureType::JongAh
                | StructureType::JongJae
                | StructureType::JongSal
                | StructureType::JongGang
                | StructureType::JongWang
                | StructureType::Follower
                | StructureType::SpecialTransformation
        );

        if is_polarized {
            let reasons = vec![
                format!("격국: {}", structure_analysis.structure.hangul()),
                format!("세력비율: {:.1}%", strength.deuk_se.support_ratio),
                format!(
                    "판정기준: {}",
                    if strength.deuk_se.support_ratio >= config.strength.polarized_high {
                        "전왕(專旺)"
                    } else {
                        "종(從)"
                    }
                ),
            ];
            recommendations.push(RecommendedYongshin {
                yongshin_type: YongshinType::Eokbu,
                element: eokbu_element,
                summary: format!("격국의 기세를 따르는 {} 용신", eokbu_element.hangul()),
                description:
                    "강한 세력에 대항하기보다 그 흐름을 따르는 것이 운의 흐름을 원활하게 합니다."
                        .to_string(),
                reasons,
            });

            // 조후(調候) 판단 (종격에서는 조후보다 격국이 우선임)
            let thermal_index = calculate_thermal_index(pillars, config);
            let humidity_index = calculate_humidity_index(pillars, config);
            if let Some(mut johu) =
                get_johu_analysis(pillars, thermal_index, humidity_index, config)
            {
                johu.description = format!(
                    "{} 단, 종격 사주이므로 조후보다 격국의 기세를 따르는 것이 안전함",
                    johu.description
                );
                recommendations.push(johu);
            }
        } else {
            // 일반적인 경우 조후가 급하면 조후 우선
            let thermal_index = calculate_thermal_index(pillars, config);
            let humidity_index = calculate_humidity_index(pillars, config);
            if let Some(johu) = get_johu_analysis(pillars, thermal_index, humidity_index, config) {
                recommendations.push(johu);
            }

            let (summary, description, reasons) =
                get_eokbu_explainability(&strength, eokbu_element);
            recommendations.push(RecommendedYongshin {
                yongshin_type: YongshinType::Eokbu,
                element: eokbu_element,
                summary,
                description,
                reasons,
            });
        }

        // 3. 통관용신(通關) 판단 (대립 해소)
        if let Some(tonggwan) = get_tonggwan_analysis(pillars, config) {
            recommendations.push(tonggwan);
        }

        // 4. 병약용신(病藥) 판단 (최악의 기운 제어)
        if let Some(byeongyak) = get_byeongyak_analysis(pillars, &strength) {
            recommendations.push(byeongyak);
        }

        // 제1용신 결정 로직
        // 조후가 극단적이거나(절기 영향) 억부 균형보다 시급할 때 조후 우선
        let thermal_index_for_primary = calculate_thermal_index(pillars, config);
        let humidity_index_for_primary = calculate_humidity_index(pillars, config);
        let is_extreme_thermal = thermal_index_for_primary.abs() >= config.thermal.extreme
            || humidity_index_for_primary.abs() >= config.thermal.extreme
            || (thermal_index_for_primary.abs() >= config.thermal.moderate
                && strength.strength_score.abs() < 10.0)
            || (humidity_index_for_primary.abs() >= config.thermal.moderate
                && strength.strength_score.abs() < 10.0);
        let primary = if is_extreme_thermal
            && recommendations
                .iter()
                .any(|r| r.yongshin_type == YongshinType::Johu)
        {
            recommendations
                .iter()
                .find(|r| r.yongshin_type == YongshinType::Johu)
                .map(|r| r.element)
                .unwrap_or(recommendations[0].element)
        } else if recommendations
            .iter()
            .any(|r| r.yongshin_type == YongshinType::Byeongyak)
        {
            // 병약용신이 있으면 약을 우선으로 쓰는 경우가 많음
            recommendations
                .iter()
                .find(|r| r.yongshin_type == YongshinType::Byeongyak)
                .map(|r| r.element)
                .unwrap_or(recommendations[0].element)
        } else {
            recommendations
                .iter()
                .find(|r| r.yongshin_type == YongshinType::Eokbu)
                .map(|r| r.element)
                .unwrap_or(recommendations[0].element)
        };

        // 희신은 용신을 생하거나 돕는 오행
        let assistant = primary.generated_by();

        Self {
            recommendations,
            primary,
            assistant,
        }
    }
}

/// 조후 지수(한난) 계산 (-100 ~ 100)
/// - 음수: 춥다 (Water, Metal, Yin-Earth)
/// - 양수: 덥다 (Fire, Wood, Yang-Earth)
pub fn calculate_thermal_index(pillars: &FourPillars, _config: &AnalysisConfig) -> i32 {
    let mut score = 0;

    // 사령(당령) 분석을 통한 동적 스케일링
    if let Ok(saryeong) = crate::core::branch_days::SaryeongAnalysis::from_pillars(pillars) {
        let base_weight = match pillars.month.branch {
            EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou => -40,
            EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei => 40,
            EarthlyBranch::Yin | EarthlyBranch::Mao | EarthlyBranch::Chen => 10,
            EarthlyBranch::Shen | EarthlyBranch::You | EarthlyBranch::Xu => -10,
        };
        let multiplier = if saryeong.part == "정기(正氣)" {
            1.2
        } else if saryeong.part == "중기(中氣)" {
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

    let stems = [
        pillars.year.stem,
        pillars.month.stem,
        pillars.day.stem,
        pillars.hour.stem,
    ];
    let branches = [
        pillars.year.branch,
        pillars.month.branch,
        pillars.day.branch,
        pillars.hour.branch,
    ];

    for s in stems {
        use crate::core::stem::HeavenlyStem as S;
        match s {
            S::Bing => score += 15, // 태양 (가장 뜨거움)
            S::Ding => score += 10, // 등촉 (따뜻함)
            S::Ren => score -= 15,  // 강수 (가장 차가움)
            S::Gui => score -= 10,  // 우로 (차갑고 습함)
            S::Jia | S::Yi => score += 5,
            S::Geng | S::Xin => score -= 5,
            _ => {}
        }
    }

    for b in branches {
        use EarthlyBranch as EB;
        match b {
            EB::Si | EB::Wu => score += 15,
            EB::Wei | EB::Xu => score += 10, // 마른 흙
            EB::Hai | EB::Zi => score -= 15,
            EB::Chou | EB::Chen => score -= 10, // 습한 흙
            EB::Yin | EB::Mao => score += 5,
            EB::Shen | EB::You => score -= 5,
        }
    }

    score.clamp(-100, 100)
}

/// 조습 지수 계산 (-100 ~ 100)
/// - 음수: 습하다 (Water, Wet-Earth, Wood)
/// - 양수: 건조하다 (Fire, Dry-Earth, Metal)
pub fn calculate_humidity_index(pillars: &FourPillars, _config: &AnalysisConfig) -> i32 {
    let mut score = 0;

    if let Ok(saryeong) = crate::core::branch_days::SaryeongAnalysis::from_pillars(pillars) {
        let base_weight = match pillars.month.branch {
            EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou | EarthlyBranch::Chen => {
                -40
            }
            EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei | EarthlyBranch::Xu => 40,
            EarthlyBranch::Yin | EarthlyBranch::Mao => -10, // 목은 생명체라 기본적으로 습기를 머금음
            EarthlyBranch::Shen | EarthlyBranch::You => 10, // 금은 건조함
        };
        let multiplier = if saryeong.part == "정기(正氣)" {
            1.2
        } else if saryeong.part == "중기(中氣)" {
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

    let stems = [
        pillars.year.stem,
        pillars.month.stem,
        pillars.day.stem,
        pillars.hour.stem,
    ];
    let branches = [
        pillars.year.branch,
        pillars.month.branch,
        pillars.day.branch,
        pillars.hour.branch,
    ];

    for s in stems {
        use crate::core::stem::HeavenlyStem as S;
        match s {
            S::Bing | S::Ding => score += 15,
            S::Geng | S::Xin => score += 10, // 금도 건조함에 기여
            S::Ren | S::Gui => score -= 15,
            S::Jia | S::Yi => score -= 5,
            S::Wu => score += 10, // 무토는 조토
            S::Ji => score -= 10, // 기토는 습토
        }
    }

    for b in branches {
        use EarthlyBranch as EB;
        match b {
            EB::Si | EB::Wu => score += 15,
            EB::Wei | EB::Xu => score += 15, // 마른 흙
            EB::Shen | EB::You => score += 10,
            EB::Hai | EB::Zi => score -= 15,
            EB::Chou | EB::Chen => score -= 15, // 습한 흙
            EB::Yin | EB::Mao => score -= 5,
        }
    }

    score.clamp(-100, 100)
}

/// 조후 분석 (한난 & 조습 2D 매트릭스)
fn get_johu_analysis(
    _pillars: &FourPillars,
    thermal: i32,
    humidity: i32,
    _config: &AnalysisConfig,
) -> Option<RecommendedYongshin> {
    let is_cold = thermal <= -30;
    let is_hot = thermal >= 30;
    let is_wet = humidity <= -30;
    let is_dry = humidity >= 30;

    let reasons = vec![
        format!("한난 지수: {}", thermal),
        format!("조습 지수: {}", humidity),
    ];

    if is_cold && is_wet {
        Some(RecommendedYongshin {
            yongshin_type: YongshinType::Johu,
            element: Element::Fire,
            summary: "한습(寒濕)한 사주를 덥히고 말리는 火 용신".to_string(),
            description: "사주가 매우 차갑고 습하므로, 불(火)과 마른 흙(燥土)의 기운이 절실합니다."
                .to_string(),
            reasons,
        })
    } else if is_hot && is_dry {
        Some(RecommendedYongshin {
            yongshin_type: YongshinType::Johu,
            element: Element::Water,
            summary: "조열(燥熱)한 사주를 식히고 적시는 水 용신".to_string(),
            description:
                "사주가 매우 뜨겁고 건조하므로, 물(水)과 습한 흙(濕土)의 기운이 절실합니다."
                    .to_string(),
            reasons,
        })
    } else if is_hot && is_wet {
        Some(RecommendedYongshin {
            yongshin_type: YongshinType::Johu,
            element: Element::Metal, // 금이나 수
            summary: "습열(濕熱)한 사주를 씻어내리는 金 용신".to_string(),
            description: "사주가 뜨겁고 습하여 불쾌지수가 높으므로, 금(金)이나 수(水)의 기운으로 씻어내려야 합니다.".to_string(),
            reasons,
        })
    } else if is_cold && is_dry {
        Some(RecommendedYongshin {
            yongshin_type: YongshinType::Johu,
            element: Element::Wood, // 목이나 화
            summary: "한조(寒燥)한 사주에 생기를 부여하는 木 용신".to_string(),
            description:
                "사주가 춥고 메말라 있으므로, 생기를 돋우는 목(木)이나 화(火)의 기운이 필요합니다."
                    .to_string(),
            reasons,
        })
    } else if is_cold {
        Some(RecommendedYongshin {
            yongshin_type: YongshinType::Johu,
            element: Element::Fire,
            summary: "한랭한 사주를 따뜻하게 하는 火 용신".to_string(),
            description: "사주의 기운이 차가우므로 불(火)의 기운으로 온도를 조절해야 발복합니다."
                .to_string(),
            reasons,
        })
    } else if is_hot {
        Some(RecommendedYongshin {
            yongshin_type: YongshinType::Johu,
            element: Element::Water,
            summary: "조열한 사주를 시원하게 하는 水 용신".to_string(),
            description: "사주의 기운이 뜨거우므로 물(水)의 기운으로 온도를 낮추어야 발복합니다."
                .to_string(),
            reasons,
        })
    } else if is_wet {
        Some(RecommendedYongshin {
            yongshin_type: YongshinType::Johu,
            element: Element::Fire, // 습하면 마르게
            summary: "습한 사주를 뽀송하게 말리는 火 용신".to_string(),
            description:
                "사주에 습기가 과다하므로 이를 말려주는 볕이나 마른 흙(燥土)이 필요합니다."
                    .to_string(),
            reasons,
        })
    } else if is_dry {
        Some(RecommendedYongshin {
            yongshin_type: YongshinType::Johu,
            element: Element::Water, // 건조하면 적시게
            summary: "건조한 사주를 촉촉하게 적시는 水 용신".to_string(),
            description: "사주가 메말라 있으므로 물기를 공급해주어야 생기가 돕니다.".to_string(),
            reasons,
        })
    } else {
        None
    }
}

/// 통관 분석 (서로 싸우는 강한 두 기운 중재)
///
/// 개수가 아닌 **세력 점수**를 기준으로 판단합니다.
/// 두 상충 오행의 세력이 비등할 때(차이 20% 이내)만 통관용신을 적용합니다.
fn get_tonggwan_analysis(
    pillars: &FourPillars,
    config: &AnalysisConfig,
) -> Option<RecommendedYongshin> {
    use crate::analysis::power::{AnalysisOptions, IntegratedAnalysis};

    // 세력 점수 계산 (기본 보정 적용)
    let options = AnalysisOptions {
        apply_transform: false, // 합화는 적용하지 않음 (원국 기준)
        apply_correction: false,
    };
    let analysis = IntegratedAnalysis::calculate(pillars, options, config);

    // 오행별 세력 점수 추출 (0~100)
    let mut scores: [f32; 5] = [0.0; 5];
    for (elem, pct, _) in &analysis.element_scores {
        scores[elem.index() as usize] = *pct;
    }

    // 상충 관계 쌍과 통관 오행 정의
    // (오행1, 오행2, 통관 오행, 설명)
    let conflicts: [(Element, Element, Element, &str); 5] = [
        (
            Element::Metal,
            Element::Wood,
            Element::Water,
            "금(金)과 목(木)이 대립하고 있어 이를 유통시키는 수(水)가 필요함",
        ),
        (
            Element::Water,
            Element::Fire,
            Element::Wood,
            "수(水)와 화(火)가 대립하고 있어 이를 유통시키는 목(木)이 필요함",
        ),
        (
            Element::Wood,
            Element::Earth,
            Element::Fire,
            "목(木)과 토(土)가 대립하고 있어 이를 유통시키는 화(火)가 필요함",
        ),
        (
            Element::Fire,
            Element::Metal,
            Element::Earth,
            "화(火)와 금(金)이 대립하고 있어 이를 유통시키는 토(土)가 필요함",
        ),
        (
            Element::Earth,
            Element::Water,
            Element::Metal,
            "토(土)와 수(水)가 대립하고 있어 이를 유통시키는 금(金)이 필요함",
        ),
    ];

    for (elem1, elem2, mediator, reason) in conflicts {
        let score1 = scores[elem1.index() as usize];
        let score2 = scores[elem2.index() as usize];

        // 두 오행의 세력이 모두 15% 이상이고, 차이가 20% 이내일 때 통관 적용
        let min_threshold = 15.0; // 최소 세력
        let max_diff = 20.0; // 세력 차이 허용 범위

        if score1 >= min_threshold && score2 >= min_threshold {
            let diff = (score1 - score2).abs();
            if diff <= max_diff {
                return Some(RecommendedYongshin {
                    yongshin_type: YongshinType::Tonggwan,
                    element: mediator,
                    summary: format!("{}와 {}의 대립을 해소하는 {} 용신", elem1.hangul(), elem2.hangul(), mediator.hangul()),
                    description: "상극하는 두 기운이 팽팽하게 대립할 때는 이를 연결해 주는 오행이 행운의 열쇠가 됩니다.".to_string(),
                    reasons: vec![
                        reason.to_string(),
                        format!("{}: {:.1}%", elem1.hangul(), score1),
                        format!("{}: {:.1}%", elem2.hangul(), score2),
                    ],
                });
            }
        }
    }

    None
}

/// 병약 분석 (사주의 가장 큰 문제점 제어)
fn get_byeongyak_analysis(
    pillars: &FourPillars,
    strength: &crate::analysis::strength::StrengthAnalysis,
) -> Option<RecommendedYongshin> {
    // 일간을 극하거나 설기하는 기운이 너무 강할 때 (병)
    // 병을 제어하는 기운 (약)
    if strength.strength_type == StrengthType::Weak {
        if strength.deuk_se.guanxing_count >= 3 {
            return Some(RecommendedYongshin {
                 yongshin_type: YongshinType::Byeongyak,
                 element: pillars.day_master_element().generated_by(),
                 summary: "과도한 관성을 제어하는 병약용신".to_string(),
                 description: "일간을 극하는 관성이 너무 강해 병이 되었으므로, 이를 화(化)해주는 인성이 약이 됩니다.".to_string(),
                 reasons: vec![format!("관성 개수: {}개", strength.deuk_se.guanxing_count)],
             });
        }
        if strength.deuk_se.shishang_count >= 3 {
            return Some(RecommendedYongshin {
                yongshin_type: YongshinType::Byeongyak,
                element: pillars.day_master_element().generated_by(),
                summary: "과도한 식상을 제어하는 병약용신".to_string(),
                description: "일간의 기운을 빼앗는 식상이 너무 강해 병이 되었으므로, 이를 제어하는 인성이 약이 됩니다.".to_string(),
                reasons: vec![format!("식상 개수: {}개", strength.deuk_se.shishang_count)],
            });
        }
    }
    None
}

impl std::fmt::Display for YongshinAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【용신(用神) 정밀 판정】")?;
        writeln!(f, "─────────────────────────────────")?;

        for rec in &self.recommendations {
            writeln!(
                f,
                "● {} : {} ({})",
                rec.yongshin_type.hangul(),
                rec.element.hangul(),
                rec.element.hanja()
            )?;
            writeln!(f, "   - 요약: {}", rec.summary)?;
            writeln!(f, "   - 설명: {}", rec.description)?;
            if !rec.reasons.is_empty() {
                writeln!(f, "   - 근거: {}", rec.reasons.join(", "))?;
            }
        }

        writeln!(f)?;
        writeln!(
            f,
            "▶ 제1용신: {} ({})",
            self.primary.hangul(),
            self.primary.hanja()
        )?;
        writeln!(
            f,
            "▶ 희신(喜神): {} ({})",
            self.assistant.hangul(),
            self.assistant.hanja()
        )?;

        Ok(())
    }
}

impl FourPillars {
    /// 용신 분석
    pub fn yongshin(&self) -> YongshinAnalysis {
        YongshinAnalysis::from_pillars(self)
    }

    /// 설정을 포함한 용신 분석
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

/// 억부용신 결정 이유 상세화
fn get_eokbu_explainability(
    strength: &StrengthAnalysis,
    eokbu_element: Element,
) -> (String, String, Vec<String>) {
    let mut reasons = vec![
        format!("신강약점수: {:.1}", strength.strength_score),
        format!("현재상태: {}", strength.strength_type.hangul()),
    ];

    match strength.strength_type {
        StrengthType::Weak => {
            (
                format!("일간을 돕는 {} 억부용신", eokbu_element.hangul()),
                "일간이 신약하여 기운이 부족하므로 이를 비추거나 생조해주는 오행이 행운을 가져옵니다.".to_string(),
                reasons
            )
        },
        StrengthType::Strong => {
            let yinxing = strength.deuk_se.yinxing_count as f32;
            let bijie = strength.deuk_se.bijie_count as f32;

            if yinxing > bijie * 1.5 {
                reasons.push(format!("인성({}) 과다", yinxing));
                (
                    format!("인성을 제어하는 {} 용재파인 용신", eokbu_element.hangul()),
                    "인성이 너무 많아 신강해진 경우, 부작용을 막기 위해 재성으로 인성을 적절히 견제해야 합니다.".to_string(),
                    reasons
                )
            } else if bijie > yinxing * 1.5 {
                reasons.push(format!("비겁({}) 과다", bijie));
                (
                    format!("비겁을 제어하는 {} 관살제겁 용신", eokbu_element.hangul()),
                    "자아가 너무 강해져 주변과 충돌하기 쉬운 경우, 관성으로 스스로를 다스리는 힘을 길러야 합니다.".to_string(),
                    reasons
                )
            } else {
                (
                    format!("기운을 유통시키는 {} 설기생재 용신", eokbu_element.hangul()),
                    "일간이 신강하여 기운이 옹색해지기 쉬우므로 식상으로 기운을 빼서 재성으로 연결해야 합니다.".to_string(),
                    reasons
                )
            }
        },
        StrengthType::Balanced => {
            (
                format!("균형을 유지하는 {} 중화용신", eokbu_element.hangul()),
                "이미 기운이 중화되어 안정적이므로, 현재의 균형을 유지해 주는 오행을 용신으로 삼습니다.".to_string(),
                reasons
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pillars::SajuInput;

    #[test]
    fn test_johu_thermal_and_humidity() {
        let input = SajuInput::new_solar(2024, 1, 1, 12, 0); // 한겨울 한낮
        let pillars = FourPillars::calculate(&input).unwrap();
        let thermal = calculate_thermal_index(&pillars, &AnalysisConfig::default());
        let humidity = calculate_humidity_index(&pillars, &AnalysisConfig::default());

        // 겨울 자월이므로 춥고, 수는 기본적으로 습함.
        assert!(thermal < 0);
        assert!(humidity < 0);

        let johu = get_johu_analysis(&pillars, thermal, humidity, &AnalysisConfig::default());
        assert!(johu.is_some());
        let johu = johu.unwrap();
        assert_eq!(johu.element, Element::Fire);
    }
}
