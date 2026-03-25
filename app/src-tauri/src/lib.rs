pub mod tier;

use eon_service::dto::{SajuAnalysisInput, TransitAnalysisInput, VedicAnalysisInput};
use eon_service::facade;
use serde_json::Value;


#[tauri::command]
async fn get_vedic_analysis(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    is_lunar: bool,
    is_leap_month: bool,
    lat: f64,
    lon: f64,
    timezone: String,
    unknown_time: Option<bool>,
    now_utc: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<Value, String> {
    let input = VedicAnalysisInput::new(
        year, month, day, hour, minute,
        is_lunar, is_leap_month,
        lat, lon, timezone,
        unknown_time, now_utc,
    );

    let result = facade::analyze_vedic(input).map_err(|e| e.to_string())?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_saju_analysis(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    is_lunar: bool,
    is_leap_month: bool,
    is_male: bool,
    use_night_rat_hour: bool,
    lon: f64,
    lat: f64,
    timezone: String,
    unknown_time: Option<bool>,
) -> Result<Value, String> {
    let input = SajuAnalysisInput::new(
        year, month, day, hour, minute,
        is_lunar, is_leap_month, is_male, use_night_rat_hour,
        lon, lat, timezone, unknown_time,
    );

    let result = facade::analyze_saju(input).map_err(|e| e.to_string())?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_transit_analysis(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    is_lunar: bool,
    is_leap_month: bool,
    is_male: bool,
    use_night_rat_hour: bool,
    lon: f64,
    lat: f64,
    timezone: String,
    unknown_time: Option<bool>,
    now_utc: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<Value, String> {
    let input = TransitAnalysisInput::new(
        year, month, day, hour, minute,
        is_lunar, is_leap_month, is_male, use_night_rat_hour,
        lon, lat, timezone, unknown_time, now_utc,
    );

    let result = facade::analyze_transit(input).map_err(|e| e.to_string())?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_vedic_analysis,
            get_saju_analysis,
            get_transit_analysis,
            tier::get_destiny_tier_analysis,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
