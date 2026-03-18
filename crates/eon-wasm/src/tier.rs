use eon_service::dto::{SajuAnalysisOutput, TransitAnalysisOutput, VedicAnalysisOutput};
use eon_service::facade;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_destiny_tier_analysis(
    saju_val: JsValue,
    vedic_val: JsValue,
    transit_val: JsValue,
) -> Result<JsValue, JsValue> {
    let saju: SajuAnalysisOutput = serde_wasm_bindgen::from_value(saju_val)
        .map_err(|e| JsError::new(&format!("Saju Parse Err: {}", e)))?;
    let vedic: VedicAnalysisOutput = serde_wasm_bindgen::from_value(vedic_val)
        .map_err(|e| JsError::new(&format!("Vedic Parse Err: {}", e)))?;
    let transit: Option<TransitAnalysisOutput> = serde_wasm_bindgen::from_value(transit_val).ok();

    let result = facade::analyze_destiny_tier(saju, vedic, transit)
        .map_err(|e| JsError::new(&e.to_string()))?;

    use serde::Serialize as _;
    let js_val = result
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| JsError::new(&format!("직렬화 오류: {}", e)))?;
    Ok(js_val)
}
