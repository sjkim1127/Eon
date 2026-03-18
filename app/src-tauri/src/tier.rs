use eon_service::dto::{SajuAnalysisOutput, TransitAnalysisOutput, VedicAnalysisOutput};
use eon_service::facade;

#[tauri::command]
pub fn get_destiny_tier_analysis(
    saju_val: serde_json::Value,
    vedic_val: serde_json::Value,
    transit_val: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let saju: SajuAnalysisOutput = serde_json::from_value(saju_val).map_err(|e| format!("Saju Parse Err: {}", e))?;
    let vedic: VedicAnalysisOutput = serde_json::from_value(vedic_val).map_err(|e| format!("Vedic Parse Err: {}", e))?;
    let transit: Option<TransitAnalysisOutput> = serde_json::from_value(transit_val).ok();

    let result = facade::analyze_destiny_tier(saju, vedic, transit).map_err(|e| e.to_string())?;

    serde_json::to_value(&result)
        .map_err(|e| format!("직렬화 오류: {}", e))
}
