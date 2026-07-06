use chrono::{DateTime, Utc};
use eon_saju::core::ganzi::GanZi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QimenPan {
    pub time: DateTime<Utc>,
    pub hour_pillar: GanZi,
    pub is_yin_ju: bool, // 둔국 (Yin/Yang)
    pub ju_number: u8,   // 국수 (1~9)
}
