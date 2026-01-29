//! 용신(用神, Useful God) 정밀 분석
//! 
//! 억부(抑扶), 조후(調候), 통관(通關), 병약(病藥)의 관점에서
//! 사주의 균형을 맞추는 최적의 오행을 찾습니다.

use serde::{Deserialize, Serialize};
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use crate::analysis::strength::{StrengthAnalysis, StrengthType};
use crate::analysis::structure::StructureType;
use crate::core::branch::EarthlyBranch;
use crate::core::config::thresholds::*;


/// 용신의 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
pub struct RecommendedYongshin {
    pub yongshin_type: YongshinType,
    pub element: Element,
    pub reason: String,
}

/// 용신 분석 종합 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        let mut recommendations = Vec::new();
        let strength = pillars.strength();
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
                        if strength.deuk_se.yinxing_count == 0 {
                            day_master_el.generated_by() // 인성
                        } else {
                            day_master_el // 비겁
                        }
                    },
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
                    },
                    StrengthType::Balanced => day_master_el,
                }
            }
        };

        // 종격/전왕격인 경우 억부(격국) 용신을 최우선으로 배치하고 조후는 참고로만 제시
        let is_polarized = matches!(structure_analysis.structure, 
            StructureType::JongAh | StructureType::JongJae | StructureType::JongSal | 
            StructureType::JongGang | StructureType::JongWang | StructureType::Follower | 
            StructureType::SpecialTransformation);

        if is_polarized {
            recommendations.push(RecommendedYongshin {
                yongshin_type: YongshinType::Eokbu,
                element: eokbu_element,
                reason: format!("{} 사주로, 강한 세력을 따르는 {}가 최우선 용신임 (격국: {})", 
                    if strength.deuk_se.support_ratio >= POLARIZED_RATIO_HIGH { "전왕" } else { "종" },
                    eokbu_element.hangul(),
                    structure_analysis.structure.hangul()
                ),
            });

            // 조후(調候) 판단 (종격에서는 조후보다 격국이 우선임)
            let thermal_index = calculate_thermal_index(pillars);
            if let Some(mut johu) = get_johu_analysis(pillars, thermal_index) {
                johu.reason = format!("{} 단, 종격 사주이므로 조후보다 격국의 기세를 따르는 것이 안전함", johu.reason);
                recommendations.push(johu);
            }
        } else {
            // 일반적인 경우 조후가 급하면 조후 우선
            let thermal_index = calculate_thermal_index(pillars);
            if let Some(johu) = get_johu_analysis(pillars, thermal_index) {
                recommendations.push(johu);
            }

            recommendations.push(RecommendedYongshin {
                yongshin_type: YongshinType::Eokbu,
                element: eokbu_element,
                reason: get_eokbu_reason(&strength, eokbu_element),
            });
        }

        // 3. 통관용신(通關) 판단 (대립 해소)
        if let Some(tonggwan) = get_tonggwan_analysis(pillars) {
            recommendations.push(tonggwan);
        }

        // 4. 병약용신(病藥) 판단 (최악의 기운 제어)
        if let Some(byeongyak) = get_byeongyak_analysis(pillars, &strength) {
            recommendations.push(byeongyak);
        }

        // 제1용신 결정 로직
        // 조후가 극단적이거나(절기 영향) 억부 균형보다 시급할 때 조후 우선
        let thermal_index_for_primary = calculate_thermal_index(pillars);
        let is_extreme_thermal = thermal_index_for_primary.abs() >= THERMAL_EXTREME || (thermal_index_for_primary.abs() >= THERMAL_MODERATE && strength.strength_score.abs() < 10.0);
        let primary = if is_extreme_thermal && recommendations.iter().any(|r| r.yongshin_type == YongshinType::Johu) {
            recommendations.iter()
                .find(|r| r.yongshin_type == YongshinType::Johu)
                .map(|r| r.element)
                .unwrap_or(recommendations[0].element)
        } else if recommendations.iter().any(|r| r.yongshin_type == YongshinType::Byeongyak) {
            // 병약용신이 있으면 약을 우선으로 쓰는 경우가 많음
            recommendations.iter()
                .find(|r| r.yongshin_type == YongshinType::Byeongyak)
                .map(|r| r.element)
                .unwrap_or(recommendations[0].element)
        } else {
            recommendations.iter()
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

/// 조후 지수 계산 (-100 ~ 100)
/// - 음수: 춥고 습함 (Water, Metal, Yin-Earth)
/// - 양수: 덥고 건조함 (Fire, Wood, Yang-Earth)
pub fn calculate_thermal_index(pillars: &FourPillars) -> i32 {
    let mut score = 0;
    
    // 월지의 영향력이 가장 큼 (40%)
    match pillars.month.branch {
        EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou => score -= 40,
        EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei => score += 40,
        _ => {}
    }
    
    // 전체 오행 분포 확인
    let stems = [pillars.year.stem, pillars.month.stem, pillars.day.stem, pillars.hour.stem];
    let branches = [pillars.year.branch, pillars.month.branch, pillars.day.branch, pillars.hour.branch];
    
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

/// 조후 분석 
fn get_johu_analysis(pillars: &FourPillars, index: i32) -> Option<RecommendedYongshin> {
    let month = pillars.month.branch;
    
    if index <= -30 || matches!(month, EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou) {
        Some(RecommendedYongshin {
            yongshin_type: YongshinType::Johu,
            element: Element::Fire,
            reason: format!("사주가 {}하여(지수: {}) 따뜻하게 해주는 火가 시급함", 
                if index <= -50 { "매우 한랭" } else { "한랭" }, index),
        })
    } else if index >= 30 || matches!(month, EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei) {
        Some(RecommendedYongshin {
            yongshin_type: YongshinType::Johu,
            element: Element::Water,
            reason: format!("사주가 {}하여(지수: {}) 시원하게 해주는 水가 시급함", 
                if index >= 50 { "매우 조열" } else { "조열" }, index),
        })
    } else {
        None
    }
}

/// 통관 분석 (서로 싸우는 강한 두 기운 중재)
/// 
/// 개수가 아닌 **세력 점수**를 기준으로 판단합니다.
/// 두 상충 오행의 세력이 비등할 때(차이 20% 이내)만 통관용신을 적용합니다.
fn get_tonggwan_analysis(pillars: &FourPillars) -> Option<RecommendedYongshin> {
    use crate::analysis::power::{IntegratedAnalysis, AnalysisOptions};
    
    // 세력 점수 계산 (기본 보정 적용)
    let options = AnalysisOptions {
        apply_transform: false,  // 합화는 적용하지 않음 (원국 기준)
        apply_correction: false,
    };
    let analysis = IntegratedAnalysis::calculate(pillars, options);
    
    // 오행별 세력 점수 추출 (0~100)
    let mut scores: [f32; 5] = [0.0; 5];
    for (elem, pct, _) in &analysis.element_scores {
        scores[elem.index() as usize] = *pct;
    }
    
    // 상충 관계 쌍과 통관 오행 정의
    // (오행1, 오행2, 통관 오행, 설명)
    let conflicts: [(Element, Element, Element, &str); 5] = [
        (Element::Metal, Element::Wood, Element::Water, 
         "금(金)과 목(木)이 대립하고 있어 이를 유통시키는 수(水)가 필요함"),
        (Element::Water, Element::Fire, Element::Wood,
         "수(水)와 화(火)가 대립하고 있어 이를 유통시키는 목(木)이 필요함"),
        (Element::Wood, Element::Earth, Element::Fire,
         "목(木)과 토(土)가 대립하고 있어 이를 유통시키는 화(火)가 필요함"),
        (Element::Fire, Element::Metal, Element::Earth,
         "화(火)와 금(金)이 대립하고 있어 이를 유통시키는 토(土)가 필요함"),
        (Element::Earth, Element::Water, Element::Metal,
         "토(土)와 수(水)가 대립하고 있어 이를 유통시키는 금(金)이 필요함"),
    ];
    
    for (elem1, elem2, mediator, reason) in conflicts {
        let score1 = scores[elem1.index() as usize];
        let score2 = scores[elem2.index() as usize];
        
        // 두 오행의 세력이 모두 15% 이상이고, 차이가 20% 이내일 때 통관 적용
        let min_threshold = 15.0;  // 최소 세력
        let max_diff = 20.0;       // 세력 차이 허용 범위
        
        if score1 >= min_threshold && score2 >= min_threshold {
            let diff = (score1 - score2).abs();
            if diff <= max_diff {
                return Some(RecommendedYongshin {
                    yongshin_type: YongshinType::Tonggwan,
                    element: mediator,
                    reason: format!("{} ({}:{:.1}% vs {}:{:.1}%)", 
                        reason, 
                        elem1.hangul(), score1,
                        elem2.hangul(), score2),
                });
            }
        }
    }
    
    None
}

/// 병약 분석 (사주의 가장 큰 문제점 제어)
fn get_byeongyak_analysis(pillars: &FourPillars, strength: &crate::analysis::strength::StrengthAnalysis) -> Option<RecommendedYongshin> {
    // 일간을 극하거나 설기하는 기운이 너무 강할 때 (병)
    // 병을 제어하는 기운 (약)
    if strength.strength_type == StrengthType::Weak {
        if strength.deuk_se.guanxing_count >= 3 {
             // 관다신약: 관성(Metal/etc)이 병 -> 인성(약)
             return Some(RecommendedYongshin {
                 yongshin_type: YongshinType::Byeongyak,
                 element: pillars.day_master_element().generated_by(),
                 reason: "관성(官星)이 너무 강해 병이 되었으므로 인성으로 살을 화해야 함".to_string(),
             });
        }
        if strength.deuk_se.shishang_count >= 3 {
            // 식다신약: 식상(Wood/etc)이 병 -> 인성(약, 극)
            return Some(RecommendedYongshin {
                yongshin_type: YongshinType::Byeongyak,
                element: pillars.day_master_element().generated_by(),
                reason: "식상(食傷)이 너무 강해 기운이 빠지므로 인성으로 제어해야 함".to_string(),
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
            writeln!(f, "● {} : {} ({})", rec.yongshin_type.hangul(), rec.element.hangul(), rec.element.hanja())?;
            writeln!(f, "   - 사유: {}", rec.reason)?;
        }
        
        writeln!(f)?;
        writeln!(f, "▶ 제1용신: {} ({})", self.primary.hangul(), self.primary.hanja())?;
        writeln!(f, "▶ 희신(喜神): {} ({})", self.assistant.hangul(), self.assistant.hanja())?;
        
        Ok(())
    }
}

impl FourPillars {
    /// 용신 분석
    pub fn yongshin(&self) -> YongshinAnalysis {
        YongshinAnalysis::from_pillars(self)
    }
}

/// 억부용신 결정 이유 상세화
fn get_eokbu_reason(strength: &StrengthAnalysis, eokbu_element: Element) -> String {
    match strength.strength_type {
        StrengthType::Weak => {
            format!("일간이 신약하여 이를 돕는 인성/비겁인 {}가 필요함", eokbu_element.hangul())
        },
        StrengthType::Strong => {
            let yinxing = strength.deuk_se.yinxing_count as f32;
            let bijie = strength.deuk_se.bijie_count as f32;
            
            if yinxing > bijie * 1.5 {
                format!("인성(印星) 과다로 신강하므로, 인성을 극하는 재성({})을 쓰는 용재파인(用財破印)이 필요함", eokbu_element.hangul())
            } else if bijie > yinxing * 1.5 {
                format!("비겁(比劫) 과다로 신강하므로, 비겁을 제어하는 관성({})을 쓰는 관살제겁(官殺制劫)이 필요함", eokbu_element.hangul())
            } else {
                format!("일간이 신강하여 그 기운을 설기(泄氣)시키는 식상({})이 필요함 (설기생재)", eokbu_element.hangul())
            }
        },
        StrengthType::Balanced => {
            format!("일간이 중화되어 균형을 유지하는 {}가 필요함", eokbu_element.hangul())
        }
    }
}
