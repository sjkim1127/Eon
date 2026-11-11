use crate::core::elements::{Deity, Door, Palace, Star};
use chrono::{DateTime, Utc};
use eon_saju::core::ganzi::GanZi;
use eon_saju::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PalaceState {
    pub palace: Palace,
    pub earth_stem: Option<HeavenlyStem>,  // Earth Plate Stem
    pub heaven_stem: Option<HeavenlyStem>, // Heaven Plate Stem
    pub door: Option<Door>,                // Eight Doors
    pub star: Option<Star>,                // Nine Stars
    pub deity: Option<Deity>,              // Eight Deities
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QimenPan {
    pub time: DateTime<Utc>,
    pub hour_pillar: GanZi,
    pub day_pillar: GanZi,
    pub month_pillar: GanZi,
    pub year_pillar: GanZi,
    pub is_yin_ju: bool,                // Dun pattern (Yin/Yang)
    pub ju_number: u8,                  // Ju number (1~9)
    pub palaces: Vec<PalaceState>,      // 1~9 Palaces (always 9)
    pub value_chief_star: Option<Star>, // Zhi Fu (Value Chief - Star)
    pub value_envoy_door: Option<Door>, // Zhi Shi (Value Envoy - Door)
}
