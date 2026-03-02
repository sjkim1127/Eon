use chrono::{TimeZone, Utc};
use eon_vedic::analysis::report::VedicAnalysisReport;
use eon_vedic::core::chart::VedicChartCalculator;

// Saju imports
use eon_saju::analysis::analytics::Analyzer;
use eon_saju::analysis::major_luck::MajorLuckAnalysis;
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::engine::vm::SajuVM;
use eon_saju::report::SajuReport;

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
    lat: f64,
    lon: f64,
) -> Result<VedicAnalysisReport, String> {
    let dt = Utc
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .ok_or_else(|| "Invalid date/time".to_string())?;

    let calculator = VedicChartCalculator::new();
    let chart = calculator.calculate(dt, lat, lon);

    let report = VedicAnalysisReport::generate(&chart);
    Ok(report)
}

#[tauri::command]
fn get_saju_analysis(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    is_male: bool,
    lon: f64,
    lat: f64,
) -> Result<serde_json::Value, String> {
    let gender = if is_male {
        eon_core::Gender::Male
    } else {
        eon_core::Gender::Female
    };
    let input =
        SajuInput::new_solar_at(year, month, day, hour, minute, lon, lat).with_gender(gender);

    let pillars = FourPillars::calculate(&input).map_err(|e| format!("사주 계산 실패: {}", e))?;

    let mut report = SajuReport::new(pillars.clone());

    if let Ok(major_luck) =
        MajorLuckAnalysis::calculate_astro(&pillars, gender, year, month, day, hour, minute)
    {
        let vm = SajuVM::new(pillars.clone());
        let frames = vm.simulate_life(0, 100);
        let golden_time = Analyzer::find_golden_time(&frames, 10);

        report = report
            .with_major_luck(major_luck)
            .with_vm_simulation(frames);

        if let Some(gt) = golden_time {
            report = report.with_golden_time(gt);
        }
    }

    serde_json::to_value(&report).map_err(|e| format!("직렬화 실패: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_vedic_analysis,
            get_saju_analysis
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
