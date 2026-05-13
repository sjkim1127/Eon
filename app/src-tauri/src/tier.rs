use eon_service::dto::DestinyTierRequest;
use eon_service::facade;

#[tauri::command]
pub fn get_destiny_tier_analysis(request: DestinyTierRequest) -> Result<serde_json::Value, String> {
    let result = facade::analyze_destiny_tier(request.saju, request.vedic, request.transit).map_err(|e| e.to_string())?;

    serde_json::to_value(&result)
        .map_err(|e| format!("직렬화 오류: {}", e))
}
