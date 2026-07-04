use crate::analysis::Analyzable;
use crate::core::config::AnalysisConfig;
use crate::core::pillars::FourPillars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemediesAnalysis {
    pub lucky_colors: Vec<String>,
    pub lucky_numbers: Vec<u32>,
    pub lucky_directions: Vec<String>,
    pub lifestyle_advice: String,
    pub warning: String,
}

impl Analyzable for RemediesAnalysis {
    type Output = RemediesAnalysis;

    fn analyze(_pillars: &FourPillars, _config: &AnalysisConfig) -> Self::Output {
        // TODO: 용신/희신 기반으로 상세하게 계산하도록 확장
        RemediesAnalysis {
            lucky_colors: vec!["Black".to_string(), "White".to_string()],
            lucky_numbers: vec![1, 6, 4, 9],
            lucky_directions: vec!["North".to_string(), "West".to_string()],
            lifestyle_advice: "물을 자주 마시고 흑색 계열의 옷이나 악세서리를 활용하면 부족한 기운이 보충되어 행운이 따릅니다.".to_string(),
            warning: "과도한 스트레스를 피하고 충분한 수면을 취하는 것이 중요합니다.".to_string(),
        }
    }
}
