use eon_service::dto::DestinyTierRequest;
use eon_service::facade;
use eon_service::error::ServiceError;

#[tauri::command]
pub fn get_destiny_tier_analysis(request: DestinyTierRequest) -> Result<serde_json::Value, ServiceError> {
    let result = facade::analyze_destiny_tier(request.saju, request.vedic, request.transit)?;

    serde_json::to_value(&result)
        .map_err(|e| ServiceError::Serialization(e.to_string()))
}
