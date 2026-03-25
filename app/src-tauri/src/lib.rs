pub mod tier;

use eon_service::dto::{
    AnalysisInput, BirthTimePrecision, CurrentContext, SajuAnalysisInput,
    TransitAnalysisInput, VedicAnalysisInput,
};
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
    let input = VedicAnalysisInput {
        base: AnalysisInput {
            year,
            month,
            day,
            hour,
            minute,
            is_lunar,
            is_leap_month,
            lat,
            lon,
            timezone: timezone.clone(),
        },
        precision: if unknown_time.unwrap_or(false) {
            BirthTimePrecision::UnknownTimeNoonProxy
        } else {
            BirthTimePrecision::Exact
        },
        current: CurrentContext {
            now_utc: now_utc.unwrap_or_else(chrono::Utc::now),
            analysis_timezone: timezone,
        },
    };

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
    let precision = if unknown_time.unwrap_or(false) {
        BirthTimePrecision::UnknownTimeNoonProxy
    } else {
        BirthTimePrecision::Exact
    };

    let input = SajuAnalysisInput {
        base: AnalysisInput {
            year,
            month,
            day,
            hour,
            minute,
            is_lunar,
            is_leap_month,
            lat,
            lon,
            timezone,
        },
        is_male,
        use_night_rat_hour,
        precision,
    };

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
    let precision = if unknown_time.unwrap_or(false) {
        BirthTimePrecision::UnknownTimeNoonProxy
    } else {
        BirthTimePrecision::Exact
    };

    let final_now = now_utc.unwrap_or_else(chrono::Utc::now);

    let input = TransitAnalysisInput {
        base: SajuAnalysisInput {
            base: AnalysisInput {
                year,
                month,
                day,
                hour,
                minute,
                is_lunar,
                is_leap_month,
                lat,
                lon,
                timezone: timezone.clone(),
            },
            is_male,
            use_night_rat_hour,
            precision,
        },
        current: CurrentContext {
            now_utc: final_now,
            analysis_timezone: timezone,
        },
    };

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
