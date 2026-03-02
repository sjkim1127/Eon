use chrono::{TimeZone, Utc};
use eon_vedic::analysis::report::VedicAnalysisReport;
use eon_vedic::core::chart::{VedicChart, VedicChartCalculator};
use serde::Serialize;
use wasm_bindgen::prelude::*;

// === Saju (四柱) imports ===
use eon_saju::analysis::analytics::Analyzer;
use eon_saju::analysis::major_luck::MajorLuckAnalysis;
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::engine::vm::SajuVM;
use eon_saju::report::SajuReport;

// === Core imports (BirthInfo, Location, DST) ===
use eon_core::{BirthInfo, Gender, Location};

/// DST 메타데이터를 포함한 사주 분석 결과 래퍼
#[derive(Serialize)]
struct SajuAnalysisResult {
    report: SajuReport,
    is_dst: bool,
    dst_offset_hours: Option<i32>,
    corrected_time: String,
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from WASM!", name)
}

#[wasm_bindgen]
pub async fn get_vedic_analysis(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    lat: f64,
    lon: f64,
) -> Result<JsValue, JsError> {
    let dt = Utc
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .ok_or_else(|| JsError::new("Invalid date/time"))?;

    let calculator = VedicChartCalculator::new();
    let chart = calculator.calculate(dt, lat, lon);

    let report = VedicAnalysisReport::generate(&chart);

    #[derive(Serialize)]
    struct VedicAnalysisResult {
        report: VedicAnalysisReport,
        chart: VedicChart,
    }

    let result = VedicAnalysisResult { report, chart };

    Ok(serde_wasm_bindgen::to_value(&result)?)
}

/// 사주(四柱) 분석 — WASM에서 호출 가능
///
/// 생년월일시 + 성별 + 좌표 + 타임존을 받아 사주 분석 결과를 반환합니다.
/// BirthInfo를 사용하여 DST(서머타임) + 경도 기반 진태양시 보정을 자동 적용합니다.
#[wasm_bindgen]
pub fn get_saju_analysis(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    is_male: bool,
    lon: f64,
    lat: f64,
    timezone: &str,
) -> Result<JsValue, JsError> {
    let gender = if is_male {
        Gender::Male
    } else {
        Gender::Female
    };

    // BirthInfo로 DST + 진태양시 보정
    let location = Location::new("출생지", lat, lon, 135.0);
    let birth_info = BirthInfo::solar(year, month, day, hour, minute)
        .with_timezone(timezone)
        .with_location(location)
        .with_true_solar_time(true)
        .with_gender(gender);

    let is_dst = birth_info.is_dst();
    let dst_offset = birth_info.dst_offset_hours();
    let (cy, cm, cd, ch, cmin) = birth_info.corrected_datetime();

    // 보정된 시간으로 SajuInput 생성
    let input = SajuInput::new_solar_at(cy, cm, cd, ch, cmin, lon, lat).with_gender(gender);

    let pillars = FourPillars::calculate(&input)
        .map_err(|e| JsError::new(&format!("사주 계산 실패: {}", e)))?;

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

    // DST 메타데이터를 포함한 결과 래퍼
    let result = SajuAnalysisResult {
        report,
        is_dst,
        dst_offset_hours: dst_offset,
        corrected_time: format!("{:04}-{:02}-{:02} {:02}:{:02}", cy, cm, cd, ch, cmin),
    };

    Ok(serde_wasm_bindgen::to_value(&result)?)
}
