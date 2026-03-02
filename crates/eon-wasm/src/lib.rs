use chrono::{TimeZone, Utc};
use eon_vedic::analysis::report::VedicAnalysisReport;
use eon_vedic::chart::VedicChartCalculator;
use wasm_bindgen::prelude::*;

// === Saju (四柱) imports ===
use eon_saju::analysis::analytics::Analyzer;
use eon_saju::analysis::major_luck::MajorLuckAnalysis;
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::engine::vm::SajuVM;
use eon_saju::report::SajuReport;

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

    Ok(serde_wasm_bindgen::to_value(&report)?)
}

/// 사주(四柱) 분석 — WASM에서 호출 가능
///
/// 생년월일시 + 성별 + 좌표를 받아 `SajuReport`를 반환합니다.
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
) -> Result<JsValue, JsError> {
    // 1. SajuInput 생성
    let gender = if is_male {
        eon_core::Gender::Male
    } else {
        eon_core::Gender::Female
    };
    let input =
        SajuInput::new_solar_at(year, month, day, hour, minute, lon, lat).with_gender(gender);

    // 2. 사주팔자 계산
    let pillars = FourPillars::calculate(&input)
        .map_err(|e| JsError::new(&format!("사주 계산 실패: {}", e)))?;

    // 3. 기본 리포트 생성 (역량, 용신, 신살, 격국)
    let mut report = SajuReport::new(pillars.clone());

    // 4. 대운 계산
    if let Ok(major_luck) =
        MajorLuckAnalysis::calculate_astro(&pillars, gender, year, month, day, hour, minute)
    {
        // 5. VM 시뮬레이션 (0~100세)
        let vm = SajuVM::new(pillars.clone());
        let frames = vm.simulate_life(0, 100);

        // 6. 골든타임 분석 (10년 윈도우)
        let golden_time = Analyzer::find_golden_time(&frames, 10);

        report = report
            .with_major_luck(major_luck)
            .with_vm_simulation(frames);

        if let Some(gt) = golden_time {
            report = report.with_golden_time(gt);
        }
    }

    Ok(serde_wasm_bindgen::to_value(&report)?)
}
