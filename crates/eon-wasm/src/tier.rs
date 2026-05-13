use eon_service::dto::DestinyTierRequest;
use eon_service::facade;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_destiny_tier_analysis(request: JsValue) -> Result<JsValue, JsValue> {
    let req: DestinyTierRequest = serde_wasm_bindgen::from_value(request)
        .map_err(|e| JsError::new(&format!("Invalid DestinyTierRequest: {e}")))?;

    let result = facade::analyze_destiny_tier(req.saju, req.vedic, req.transit)
        .map_err(|e| JsError::new(&e.to_string()))?;

    use serde::Serialize as _;
    let js_val = result
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| JsError::new(&format!("직렬화 오류: {}", e)))?;
    Ok(js_val)
}
