use crate::core::elements::{Deity, Door, Palace, Star};
use chrono::{DateTime, Utc};
use eon_saju::core::ganzi::GanZi;
use eon_saju::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PalaceState {
    pub palace: Palace,
    pub earth_stem: Option<HeavenlyStem>, // 지반 (Earth Plate) 간
    pub heaven_stem: Option<HeavenlyStem>, // 천반 (Heaven Plate) 간
    pub door: Option<Door>,               // 8문
    pub star: Option<Star>,               // 9성
    pub deity: Option<Deity>,             // 8신
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QimenPan {
    pub time: DateTime<Utc>,
    pub hour_pillar: GanZi,
    pub day_pillar: GanZi,
    pub month_pillar: GanZi,
    pub year_pillar: GanZi,
    pub is_yin_ju: bool,                // 둔국 (Yin/Yang)
    pub ju_number: u8,                  // 국수 (1~9)
    pub palaces: Vec<PalaceState>,      // 1~9궁 (항상 9개)
    pub value_chief_star: Option<Star>, // 직부 (Value Chief - Star)
    pub value_envoy_door: Option<Door>, // 직사 (Value Envoy - Door)
}
