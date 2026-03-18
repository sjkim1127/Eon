pub mod tier;

use eon_service::dto::{
    AnalysisInput, BirthTimePrecision, CurrentContext, SajuAnalysisInput,
    TransitAnalysisInput,
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
) -> Result<JsValue, JsValue> {
    let input = AnalysisInput {
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
    };

    let result = facade::analyze_vedic(input, None).map_err(|e| JsError::new(&e.to_string()))?;

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
    _current_year: i32,
    _current_month: u32,
    _current_day: u32,
    unknown_time: Option<bool>,
    now_utc_str: Option<String>, // WASM에서는 ISO string으로 받는 게 편할 수 있음
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

#[wasm_bindgen]
pub fn get_ai_audit(
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

    let result = facade::analyze_ai_audit(input).map_err(|e| JsError::new(&e.to_string()))?;

    use serde::Serialize as _;
    let js_val = result
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| JsError::new(&format!("직렬화 오류: {}", e)))?;
    Ok(js_val)
}

#[wasm_bindgen]
pub fn get_saju_compatibility(
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
) -> Result<JsValue, JsValue> {
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
            precision: BirthTimePrecision::Exact,
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
            precision: BirthTimePrecision::Exact,
        },
    };

    let result = facade::analyze_compatibility(input).map_err(|e| JsError::new(&e.to_string()))?;

    use serde::Serialize as _;
    let js_val = result.saju
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| JsError::new(&format!("직렬화 오류: {}", e)))?;
    Ok(js_val)
}

#[wasm_bindgen]
pub fn get_vedic_compatibility(
    year1: i32,
    month1: u32,
    day1: u32,
    hour1: u32,
    minute1: u32,
    is_lunar1: bool,
    is_leap_month1: bool,
    lat1: f64,
    lon1: f64,
    timezone1: String,
    year2: i32,
    month2: u32,
    day2: u32,
    hour2: u32,
    minute2: u32,
    is_lunar2: bool,
    is_leap_month2: bool,
    lat2: f64,
    lon2: f64,
    timezone2: String,
) -> Result<JsValue, JsValue> {
    let input = eon_service::dto::VedicCompatibilityInput {
        person1: AnalysisInput {
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
        person2: AnalysisInput {
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
    };

    let result = facade::analyze_vedic_compatibility(input).map_err(|e| JsError::new(&e.to_string()))?;

    use serde::Serialize as _;
    let js_val = result
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| JsError::new(&format!("직렬화 오류: {}", e)))?;
    Ok(js_val)
}
