pub mod tier;

use eon_service::dto::{
    AnalysisInput, BirthTimePrecision, CurrentContext, SajuAnalysisInput,
    TransitAnalysisInput, VedicAnalysisInput,
};
use eon_service::facade;
use serde_json::Value;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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
        current: Some(CurrentContext {
            now_utc: now_utc.unwrap_or_else(chrono::Utc::now),
            analysis_timezone: timezone,
        }),
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

#[tauri::command]
fn get_ai_audit(
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

    let result = facade::analyze_ai_audit(input).map_err(|e| e.to_string())?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_compatibility_analysis(
    // 사람 1
    year1: i32,
    month1: u32,
    day1: u32,
    hour1: u32,
    minute1: u32,
    is_lunar1: bool,
    is_leap_month1: bool,
    is_male1: bool,
    lon1: f64,
    lat1: f64,
    use_night_rat_hour1: bool,
    timezone1: String,
    unknown_time1: Option<bool>,
    // 사람 2
    year2: i32,
    month2: u32,
    day2: u32,
    hour2: u32,
    minute2: u32,
    is_lunar2: bool,
    is_leap_month2: bool,
    is_male2: bool,
    lon2: f64,
    lat2: f64,
    use_night_rat_hour2: bool,
    timezone2: String,
    unknown_time2: Option<bool>,
) -> Result<Value, String> {
    let input = eon_service::dto::CompatibilityInput {
        person1: SajuAnalysisInput {
            base: AnalysisInput {
                year: year1,
                month: month1,
                day: day1,
                hour: hour1,
                minute: minute1,
                is_lunar: is_lunar1,
                is_leap_month: is_leap_month1,
                lat: lat1,
                lon: lon1,
                timezone: timezone1,
            },
            is_male: is_male1,
            use_night_rat_hour: use_night_rat_hour1,
            precision: if unknown_time1.unwrap_or(false) { BirthTimePrecision::UnknownTimeNoonProxy } else { BirthTimePrecision::Exact },
        },
        person2: SajuAnalysisInput {
            base: AnalysisInput {
                year: year2,
                month: month2,
                day: day2,
                hour: hour2,
                minute: minute2,
                is_lunar: is_lunar2,
                is_leap_month: is_leap_month2,
                lat: lat2,
                lon: lon2,
                timezone: timezone2,
            },
            is_male: is_male2,
            use_night_rat_hour: use_night_rat_hour2,
            precision: if unknown_time2.unwrap_or(false) { BirthTimePrecision::UnknownTimeNoonProxy } else { BirthTimePrecision::Exact },
        },
    };

    let result = facade::analyze_compatibility(input).map_err(|e| e.to_string())?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_vedic_analysis,
            get_saju_analysis,
            get_transit_analysis,
            get_ai_audit,
            get_compatibility_analysis,
            tier::get_destiny_tier_analysis,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
