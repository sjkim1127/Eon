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
        let mode = if pan.is_yin_ju { "음둔" } else { "양둔" };
        let ju = pan.ju_number;
        let chief_star = pan
            .value_chief_star
            .map(|star| format!("직부 {:?}", star))
            .unwrap_or_else(|| "직부 미상".to_string());
        let envoy_door = pan
            .value_envoy_door
            .map(|door| format!("직사 {:?}", door))
            .unwrap_or_else(|| "직사 미상".to_string());
        Self {
            pan,
            summary: format!(
                "기문둔갑 {} {}국 — {}, {}",
                mode, ju, chief_star, envoy_door
            ),
        }
    }
}
