pub mod v1;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OracleFixture {
    pub id: String,
    pub description: String,
    pub input: crate::dto::AnalysisInput,
    pub is_male: bool,
    pub expected_saju: ExpectedSaju,
    pub expected_vedic: ExpectedVedic,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpectedSaju {
    pub year_stem: String,
    pub year_branch: String,
    pub month_stem: String,
    pub month_branch: String,
    pub day_stem: String,
    pub day_branch: String,
    pub hour_stem: String,
    pub hour_branch: String,
    pub day_master: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpectedVedic {
    pub ascendant_rasi: u8,
    pub moon_rasi: u8,
    pub moon_nakshatra: u8,
}
