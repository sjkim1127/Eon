pub mod tier;

use eon_service::dto::{SajuAnalysisInput, TransitAnalysisInput, VedicAnalysisInput, SajuAnalysisRequest, VedicAnalysisRequest, TransitAnalysisRequest};
use eon_service::facade;
use eon_service::error::ServiceError;
use serde_json::Value;

#[tauri::command]
async fn get_vedic_analysis(request: VedicAnalysisRequest) -> Result<Value, String> {
    let input: VedicAnalysisInput = request
        .try_into()
        .map_err(|e: ServiceError| e.to_string())?;

    let result = facade::analyze_vedic(input).map_err(|e| e.to_string())?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_saju_analysis(request: SajuAnalysisRequest) -> Result<Value, String> {
    let input: SajuAnalysisInput = request
        .try_into()
        .map_err(|e: ServiceError| e.to_string())?;

    let result = facade::analyze_saju(input).map_err(|e| e.to_string())?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_transit_analysis(request: TransitAnalysisRequest) -> Result<Value, String> {
    let input: TransitAnalysisInput = request
        .try_into()
        .map_err(|e: ServiceError| e.to_string())?;

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
