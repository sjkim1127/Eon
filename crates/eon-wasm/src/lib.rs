pub mod tier;

use eon_service::dto::{SajuAnalysisInput, TransitAnalysisInput, VedicAnalysisInput, SajuAnalysisRequest, VedicAnalysisRequest, TransitAnalysisRequest};
use eon_service::facade;
use eon_service::error::ServiceError;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_vedic_analysis(request: JsValue) -> Result<JsValue, JsValue> {
    let req: VedicAnalysisRequest = serde_wasm_bindgen::from_value(request)
        .map_err(|e| JsError::new(&format!("Invalid VedicAnalysisRequest: {e}")))?;

    let input: VedicAnalysisInput = req
        .try_into()
        .map_err(|e: ServiceError| JsError::new(&e.to_string()))?;

    let result = facade::analyze_vedic(input).map_err(|e| JsError::new(&e.to_string()))?;

    use serde::Serialize as _;
    let js_val = result
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| JsError::new(&format!("직렬화 오류: {}", e)))?;
    Ok(js_val)
}

#[wasm_bindgen]
pub fn get_saju_analysis(request: JsValue) -> Result<JsValue, JsValue> {
    let req: SajuAnalysisRequest = serde_wasm_bindgen::from_value(request)
        .map_err(|e| JsError::new(&format!("Invalid SajuAnalysisRequest: {e}")))?;

    let input: SajuAnalysisInput = req
        .try_into()
        .map_err(|e: ServiceError| JsError::new(&e.to_string()))?;

    let result = facade::analyze_saju(input).map_err(|e| JsError::new(&e.to_string()))?;

    use serde::Serialize as _;
    let js_val = result
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| JsError::new(&format!("직렬화 오류: {}", e)))?;
    Ok(js_val)
}

#[wasm_bindgen]
pub fn get_transit_analysis(request: JsValue) -> Result<JsValue, JsValue> {
    let req: TransitAnalysisRequest = serde_wasm_bindgen::from_value(request)
        .map_err(|e| JsError::new(&format!("Invalid TransitAnalysisRequest: {e}")))?;

    let input: TransitAnalysisInput = req
        .try_into()
        .map_err(|e: ServiceError| JsError::new(&e.to_string()))?;

    let result = facade::analyze_transit(input).map_err(|e| JsError::new(&e.to_string()))?;

    use serde::Serialize as _;
    let js_val = result
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| JsError::new(&format!("직렬화 오류: {}", e)))?;
    Ok(js_val)
}

