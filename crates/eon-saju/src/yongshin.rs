//! 용신(用神, Useful God) 정밀 분석
//! 
//! 억부(抑扶), 조후(調候), 통관(通關), 병약(病藥)의 관점에서
//! 사주의 균형을 맞추는 최적의 오행을 찾습니다.

use serde::{Deserialize, Serialize};
use crate::element::Element;
use crate::pillars::FourPillars;
use crate::strength::StrengthType;
use crate::branch::EarthlyBranch;

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
        let month_branch = pillars.month.branch;
        let day_master_el = pillars.day_master_element();

        // 1. 억부용신(抑扶) 판단
        let eokbu_element = match strength.strength_type {
            StrengthType::Weak => {
                // 신약하면 인성(생하는 오행)이나 비겁(같은 오행)이 용신
                // 보통 인성을 우선시함
                day_master_el.generated_by()
            },
            StrengthType::Strong => {
                // 신강하면 식재관(기운을 빼는 오행) 중 선택
                // 여기서는 간단히 관성(극하는 오행)을 대표로 선택
                day_master_el.controlled_by()
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

        // 2. 조후용신(調候) 판단
        if let Some(johu_element) = get_johu_yongshin(month_branch) {
            recommendations.push(RecommendedYongshin {
                yongshin_type: YongshinType::Johu,
                element: johu_element,
                reason: format!("{}월의 추운/더운 기운을 조절하기 위해 {}가 필요함", 
                    month_branch.hanja(), johu_element.hangul()),
            });
        }

        // 제1용신 결정 로직 (조후가 급하면 조후 우선, 아니면 억부 우선)
        // 김성주님처럼 겨울(해월)은 조후가 매우 급함
        let is_winter = matches!(month_branch, EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou);
        let is_summer = matches!(month_branch, EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei);
        
        let primary = if (is_winter || is_summer) && recommendations.len() > 1 {
            recommendations[1].element // 조후용신
        } else {
            recommendations[0].element // 억부용신
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

/// 월지에 따른 조후 용신 판단 헬퍼
fn get_johu_yongshin(month: EarthlyBranch) -> Option<Element> {
    match month {
        // 겨울 (해, 자, 축): 춥기 때문에 화(火)가 필요
        EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou => Some(Element::Fire),
        // 여름 (사, 오, 미): 덥기 때문에 수(水)가 필요
        EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei => Some(Element::Water),
        _ => None,
    }
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
