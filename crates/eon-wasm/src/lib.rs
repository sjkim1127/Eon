pub mod tier;

use eon_service::dto::{
    AnalysisInput, BirthTimePrecision, CurrentContext, SajuAnalysisInput,
    TransitAnalysisInput, VedicAnalysisInput,
};
use eon_service::facade;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_vedic_analysis(
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
    now_utc_str: Option<String>,
) -> Result<JsValue, JsValue> {
    let now_utc = if let Some(s) = now_utc_str {
        chrono::DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now())
    } else {
        chrono::Utc::now()
    };

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
            now_utc,
            analysis_timezone: timezone,
        },
    };

    let result = facade::analyze_vedic(input).map_err(|e| JsError::new(&e.to_string()))?;

    use serde::Serialize as _;
    let js_val = result
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| JsError::new(&format!("직렬화 오류: {}", e)))?;
    Ok(js_val)
}

#[wasm_bindgen]
pub fn get_saju_analysis(
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
) -> Result<JsValue, JsValue> {
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

    let result = facade::analyze_saju(input).map_err(|e| JsError::new(&e.to_string()))?;

    use serde::Serialize as _;
    let js_val = result
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| JsError::new(&format!("직렬화 오류: {}", e)))?;
    Ok(js_val)
}

#[wasm_bindgen]
pub fn get_transit_analysis(
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
    now_utc_str: Option<String>,
) -> Result<JsValue, JsValue> {
    let precision = if unknown_time.unwrap_or(false) {
        BirthTimePrecision::UnknownTimeNoonProxy
    } else {
        BirthTimePrecision::Exact
    };

    let now_utc = if let Some(s) = now_utc_str {
        chrono::DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now())
    } else {
        chrono::Utc::now()
    };

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
            now_utc,
            analysis_timezone: timezone,
        },
    };

    let result = facade::analyze_transit(input).map_err(|e| JsError::new(&e.to_string()))?;

    use serde::Serialize as _;
    let js_val = result
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| JsError::new(&format!("직렬화 오류: {}", e)))?;
    Ok(js_val)
}

