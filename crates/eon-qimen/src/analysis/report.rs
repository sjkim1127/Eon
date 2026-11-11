use crate::core::QimenPan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QimenAnalysisReport {
    pub pan: QimenPan,
    pub summary: String,
}

impl QimenAnalysisReport {
    pub fn generate(pan: QimenPan) -> Self {
        let mode = if pan.is_yin_ju { "Yin Dun" } else { "Yang Dun" };
        let ju = pan.ju_number;
        let chief_star = pan
            .value_chief_star
            .map(|star| format!("Chief Star {:?}", star))
            .unwrap_or_else(|| "Chief Star Unknown".to_string());
        let envoy_door = pan
            .value_envoy_door
            .map(|door| format!("Envoy Door {:?}", door))
            .unwrap_or_else(|| "Envoy Door Unknown".to_string());
        Self {
            pan,
            summary: format!(
                "QiMen DunJia {} Ju {} — {}, {}",
                mode, ju, chief_star, envoy_door
            ),
        }
    }
}
