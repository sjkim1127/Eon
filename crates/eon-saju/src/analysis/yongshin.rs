//! 용신(用神, Useful God) 정밀 분석
//! 
//! 억부(抑扶), 조후(調候), 통관(通關), 병약(病藥)의 관점에서
//! 사주의 균형을 맞추는 최적의 오행을 찾습니다.

use serde::{Deserialize, Serialize};
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use crate::analysis::strength::StrengthType;
use crate::core::branch::EarthlyBranch;

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

        // 1. 조후용신(調候) 판단 (급한 것 우선)
        let thermal_index = calculate_thermal_index(pillars);
        if let Some(johu) = get_johu_analysis(pillars, thermal_index) {
            recommendations.push(johu);
        }

        // 2. 억부용신(抑扶) 판단
        let eokbu_element = match strength.strength_type {
            StrengthType::Weak => {
                // 신약하면 인성이나 비겁이 필요한데, 인성이 더 안정적인 경우가 많음
                // 하지만 주변 오행 상황에 따라 비겁이 필요할 수도 있음
                if strength.deuk_se.yinxing_count == 0 {
                    day_master_el.generated_by() // 인성
                } else {
                    day_master_el // 비겁
                }
            },
            StrengthType::Strong => {
                // 신강하면 식재관 중 선택
                // 관성(나를 극함) -> 재성(내가 극함) -> 식상(내가 생함) 순으로 고려
                let controller = day_master_el.controlled_by();
                let _controlled = day_master_el.controls();
                let _generating = day_master_el.generates();
                
                // 원국에 있는 오행을 우선하는 경향이 있음 (단순화)
                controller
            },
            StrengthType::Balanced => day_master_el,
        };
        
        recommendations.push(RecommendedYongshin {
            yongshin_type: YongshinType::Eokbu,
            element: eokbu_element,
            reason: format!("일간이 {}하여 이를 {}하는 {}가 필요함", 
                strength.strength_type.hangul(),
                if strength.strength_type == StrengthType::Weak { "돕는" } else { "누르는" },
                eokbu_element.hangul()
            ),
        });

        // 3. 통관용신(通關) 판단 (대립 해소)
        if let Some(tonggwan) = get_tonggwan_analysis(pillars) {
            recommendations.push(tonggwan);
        }

        // 4. 병약용신(病藥) 판단 (최악의 기운 제어)
        if let Some(byeongyak) = get_byeongyak_analysis(pillars, &strength) {
            recommendations.push(byeongyak);
        }

        // 제1용신 결정 로직
        // 조후가 극단적이면 조후 우선, 아니면 억부 우선
        let is_extreme_thermal = thermal_index.abs() >= 30;
        let primary = if is_extreme_thermal && !recommendations.is_empty() {
            // 조후용신이 0번에 있도록 정렬하거나 명시적으로 찾음
            recommendations.iter()
                .find(|r| r.yongshin_type == YongshinType::Johu)
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
        match s.element() {
            Element::Fire | Element::Wood => score += 10,
            Element::Water | Element::Metal => score -= 10,
            Element::Earth => {}
        }
    }
    
    for b in branches {
        match b.element() {
            Element::Fire | Element::Wood => score += 10,
            Element::Water | Element::Metal => score -= 10,
            Element::Earth => {}
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
fn get_tonggwan_analysis(pillars: &FourPillars) -> Option<RecommendedYongshin> {
    // 오행별 개수 계산
    let mut counts = [0u8; 5];
    for s in [pillars.year.stem, pillars.month.stem, pillars.day.stem, pillars.hour.stem] {
        counts[s.element().index() as usize] += 1;
    }
    for b in [pillars.year.branch, pillars.month.branch, pillars.day.branch, pillars.hour.branch] {
        counts[b.element().index() as usize] += 1;
    }
    
    // 금목상쟁 (Metal vs Wood)
    if counts[Element::Metal.index() as usize] >= 2 && counts[Element::Wood.index() as usize] >= 2 {
        return Some(RecommendedYongshin {
            yongshin_type: YongshinType::Tonggwan,
            element: Element::Water,
            reason: "금(金)과 목(木)이 대립하고 있어 이를 유통시키는 수(水)가 필요함".to_string(),
        });
    }
    
    // 수화상쟁 (Water vs Fire)
    if counts[Element::Water.index() as usize] >= 2 && counts[Element::Fire.index() as usize] >= 2 {
        return Some(RecommendedYongshin {
            yongshin_type: YongshinType::Tonggwan,
            element: Element::Wood,
            reason: "수(水)와 화(火)가 대립하고 있어 이를 유통시키는 목(木)이 필요함".to_string(),
        });
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
