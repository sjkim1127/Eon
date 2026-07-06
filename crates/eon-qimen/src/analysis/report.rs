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
        Self {
            pan,
            summary: "기문둔갑 분석(Qimen Dunjia Analysis) - 스캐폴딩 상태입니다.".to_string(),
        }
    }
}
