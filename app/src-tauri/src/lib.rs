use chrono::{TimeZone, Utc};
use eon_vedic::analysis::report::VedicAnalysisReport;
use eon_vedic::core::chart::VedicChartCalculator;

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

    let mut result = serde_json::to_value(&report).map_err(|e| format!("직렬화 실패: {}", e))?;

    // DST 정보 추가
    if let Some(obj) = result.as_object_mut() {
        obj.insert("is_dst".to_string(), serde_json::json!(is_dst));
        if let Some(offset) = dst_offset {
            obj.insert("dst_offset_hours".to_string(), serde_json::json!(offset));
        }
        obj.insert(
            "corrected_time".to_string(),
            serde_json::json!(format!(
                "{:04}-{:02}-{:02} {:02}:{:02}",
                cy, cm, cd, ch, cmin
            )),
        );
    }

    Ok(result)
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
