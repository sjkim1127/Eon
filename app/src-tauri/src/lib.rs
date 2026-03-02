use chrono::{TimeZone, Utc};
use eon_vedic::analysis::report::VedicAnalysisReport;
use eon_vedic::core::chart::{VedicChart, VedicChartCalculator};

// Saju imports
use eon_saju::analysis::analytics::Analyzer;
use eon_saju::analysis::major_luck::MajorLuckAnalysis;
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::engine::vm::SajuVM;
use eon_saju::report::SajuReport;

// Core imports (BirthInfo, Location, DST)
use eon_core::{BirthInfo, Gender, Location};

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
) -> Result<serde_json::Value, String> {
    let dt = Utc
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .ok_or_else(|| "Invalid date/time".to_string())?;

    let calculator = VedicChartCalculator::new();
    let chart = calculator.calculate(dt, lat, lon);

    let report = VedicAnalysisReport::generate(&chart);

    #[derive(serde::Serialize)]
    struct VedicAnalysisResult {
        report: VedicAnalysisReport,
        chart: VedicChart,
    }

    let result = VedicAnalysisResult { report, chart };

    serde_json::to_value(&result).map_err(|e| format!("직렬화 실패: {}", e))
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
    timezone: String,
) -> Result<serde_json::Value, String> {
    let gender = if is_male {
        Gender::Male
    } else {
        Gender::Female
    };

    // BirthInfo로 DST + 진태양시 보정
    let location = Location::new("출생지", lat, lon, 135.0);
    let birth_info = BirthInfo::solar(year, month, day, hour, minute)
        .with_timezone(&timezone)
        .with_location(location)
        .with_true_solar_time(true)
        .with_gender(gender);

    let is_dst = birth_info.is_dst();
    let dst_offset = birth_info.dst_offset_hours();
    let (cy, cm, cd, ch, cmin) = birth_info.corrected_datetime();

    // 보정된 시간으로 SajuInput 생성
    let input = SajuInput::new_solar_at(cy, cm, cd, ch, cmin, lon, lat).with_gender(gender);

    let pillars = FourPillars::calculate(&input).map_err(|e| format!("사주 계산 실패: {}", e))?;

    let mut report = SajuReport::new(pillars.clone());

    if let Ok(major_luck) =
        MajorLuckAnalysis::calculate_astro(&pillars, gender, cy, cm, cd, ch, cmin)
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

    // WASM과 동일한 응답 구조체 정의 (일회용)
    #[derive(serde::Serialize)]
    struct SajuAnalysisResult {
        report: SajuReport,
        is_dst: bool,
        dst_offset_hours: Option<i32>,
        corrected_time: String,
    }

    let result = SajuAnalysisResult {
        report,
        is_dst,
        dst_offset_hours: dst_offset,
        corrected_time: format!("{:04}-{:02}-{:02} {:02}:{:02}", cy, cm, cd, ch, cmin),
    };

    serde_json::to_value(&result).map_err(|e| format!("직렬화 실패: {}", e))
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
