use crate::analysis::Analyzable;
use crate::core::config::AnalysisConfig;
use crate::core::pillars::FourPillars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ThemedAnalysis {
    pub career: CareerAnalysis,
    pub wealth: WealthAnalysis,
    pub romance: RomanceAnalysis,
    pub health: HealthAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CareerAnalysis {
    pub score: u32, // 0-100
    pub keywords: Vec<String>,
    pub summary: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WealthAnalysis {
    pub score: u32,
    pub keywords: Vec<String>,
    pub summary: String,
    pub flow: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RomanceAnalysis {
    pub score: u32,
    pub keywords: Vec<String>,
    pub summary: String,
    pub advice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HealthAnalysis {
    pub score: u32,
    pub keywords: Vec<String>,
    pub summary: String,
    pub vulnerable_organs: Vec<String>,
}

impl Analyzable for ThemedAnalysis {
    type Output = ThemedAnalysis;

    fn analyze(pillars: &FourPillars, config: &AnalysisConfig) -> Self::Output {
        // TODO: 제대로 된 점수 계산 로직은 차후 구체화, 현재는 구조와 기본값 중심 구현
        let strength = pillars.strength_with_config(config);
        
        ThemedAnalysis {
            career: CareerAnalysis {
                score: 75,
                keywords: vec!["전문성".to_string(), "독립적".to_string()],
                summary: "자신의 재능을 살려 독립적으로 일할 때 가장 큰 성과를 거둡니다.".to_string(),
                recommendation: "조직에 얽매이기보다는 프리랜서나 전문직, 또는 주도적으로 프로젝트를 이끌 수 있는 환경이 적합합니다.".to_string(),
            },
            wealth: WealthAnalysis {
                score: if strength.deuk_se.caisheng_count > 0 { 80 } else { 60 },
                keywords: vec!["안정적 수익".to_string(), "투자 주의".to_string()],
                summary: "재물이 안정적으로 들어오지만 관리가 중요합니다.".to_string(),
                flow: "큰 돈을 한 번에 벌기보다는 차곡차곡 모으는 것이 유리합니다.".to_string(),
            },
            romance: RomanceAnalysis {
                score: 70,
                keywords: vec!["신중함".to_string(), "안정감".to_string()],
                summary: "신중하게 관계를 발전시키는 타입입니다.".to_string(),
                advice: "상대방의 마음을 편안하게 해주는 매력이 있습니다.".to_string(),
            },
            health: HealthAnalysis {
                score: 85,
                keywords: vec!["활력".to_string(), "스트레스 주의".to_string()],
                summary: "대체로 건강하지만 신경성 질환에 유의해야 합니다.".to_string(),
                vulnerable_organs: vec!["위장".to_string(), "신경계".to_string()],
            },
        }
    }
}
