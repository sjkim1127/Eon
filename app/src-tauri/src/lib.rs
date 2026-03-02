use chrono::{TimeZone, Utc};
use eon_vedic::analysis::report::VedicAnalysisReport;
use eon_vedic::core::chart::{VedicChart, VedicChartCalculator};

// Saju imports
use eon_saju::analysis::analytics::Analyzer;
use eon_saju::analysis::major_luck::MajorLuckAnalysis;
use eon_saju::analysis::periodic_luck::{YearlyLuck, MonthlyLuck};
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::engine::vm::SajuVM;
use eon_saju::engine::linter::{DestinyLinter, SajuLint};
use eon_saju::engine::topology::{QiTopology, TopologyAnalysis};
use eon_saju::engine::entropy::{DestinyEntropy, EntropyAnalysis};
use eon_saju::engine::load_balancer::{KarmaLoadBalancer, LoadBalanceDiagnostic};
use eon_saju::engine::fuzzer::DestinyFuzzer;
use eon_saju::report::SajuReport;

// Core imports (BirthInfo, Location, DST)
use eon_core::{BirthInfo, Gender, Location};

// AI audit
use eon_ai::DestinyAIAuditor;
use eon_saju::engine::emulator::LifePathEmulator;

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

    // 고급 정적 엔진 계산 (원국 기반)
    let lints: Vec<SajuLint> = DestinyLinter::lint(&pillars);
    let entropy: EntropyAnalysis = DestinyEntropy::analyze(&pillars);
    let qi_topology: TopologyAnalysis = QiTopology::analyze(&pillars);

    let mut report = SajuReport::new(pillars.clone());
    let mut load_diagnostics: Vec<LoadBalanceDiagnostic> = Vec::new();
    let mut crash_count: u32 = 0;

    if let Ok(major_luck) =
        MajorLuckAnalysis::calculate_astro(&pillars, gender, cy, cm, cd, ch, cmin)
    {
        let vm = SajuVM::new(pillars.clone());
        let frames = vm.simulate_life(0, 100);
        let golden_time = Analyzer::find_golden_time(&frames, 10);

        // 동적 엔진 계산 (시뮬레이션 기반)
        load_diagnostics = KarmaLoadBalancer::diagnose(&frames);

        let vm_fuzz = SajuVM::new(pillars.clone());
        let fuzzer = DestinyFuzzer::new(vm_fuzz);
        let fuzzer_report = fuzzer.audit(pillars.month);
        crash_count = fuzzer_report.total_crashes as u32;

        report = report
            .with_major_luck(major_luck)
            .with_vm_simulation(frames);

        if let Some(gt) = golden_time {
            report = report.with_golden_time(gt);
        }
    }

    #[derive(serde::Serialize)]
    struct SajuAnalysisResult {
        report: SajuReport,
        is_dst: bool,
        dst_offset_hours: Option<i32>,
        corrected_time: String,
        lints: Vec<SajuLint>,
        entropy: EntropyAnalysis,
        qi_topology: TopologyAnalysis,
        load_diagnostics: Vec<LoadBalanceDiagnostic>,
        crash_count: u32,
    }

    let result = SajuAnalysisResult {
        report,
        is_dst,
        dst_offset_hours: dst_offset,
        corrected_time: format!("{:04}-{:02}-{:02} {:02}:{:02}", cy, cm, cd, ch, cmin),
        lints,
        entropy,
        qi_topology,
        load_diagnostics,
        crash_count,
    };

    serde_json::to_value(&result).map_err(|e| format!("직렬화 실패: {}", e))
}

/// 현재 운세(세운/월운) 분석
#[tauri::command]
fn get_transit_analysis(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    is_male: bool,
    lon: f64,
    lat: f64,
    timezone: String,
    current_year: i32,
    current_month: u32,
) -> Result<serde_json::Value, String> {
    let gender = if is_male { Gender::Male } else { Gender::Female };

    let location = Location::new("출생지", lat, lon, 135.0);
    let birth_info = BirthInfo::solar(year, month, day, hour, minute)
        .with_timezone(&timezone)
        .with_location(location)
        .with_true_solar_time(true)
        .with_gender(gender);

    let (cy, cm, cd, ch, cmin) = birth_info.corrected_datetime();
    let input = SajuInput::new_solar_at(cy, cm, cd, ch, cmin, lon, lat).with_gender(gender);
    let pillars = FourPillars::calculate(&input).map_err(|e| format!("사주 계산 실패: {}", e))?;

    // 세운/월운 계산
    let yearly = YearlyLuck::calculate(current_year, &pillars);
    let monthly = MonthlyLuck::calculate(current_year, current_month, &pillars);

    // 현재 나이 계산 및 해당 LifeFrame 추출
    let current_age = (current_year - year).max(0) as u32;
    let vm = SajuVM::new(pillars.clone());
    let frames = vm.simulate_life(0, 105);
    let current_frame = frames
        .iter()
        .find(|f| f.age == current_age)
        .cloned();

    // 현재 시점 전후 5년 부하 진단
    let all_diagnostics = KarmaLoadBalancer::diagnose(&frames);
    let nearby_diagnostics: Vec<LoadBalanceDiagnostic> = all_diagnostics
        .into_iter()
        .filter(|d| d.age >= current_age.saturating_sub(3) && d.age <= current_age + 5)
        .collect();

    #[derive(serde::Serialize)]
    struct TransitResult {
        yearly_luck: YearlyLuck,
        monthly_luck: MonthlyLuck,
        current_age: u32,
        current_frame: Option<eon_saju::engine::vm::LifeFrame>,
        nearby_diagnostics: Vec<LoadBalanceDiagnostic>,
    }

    let result = TransitResult {
        yearly_luck: yearly,
        monthly_luck: monthly,
        current_age,
        current_frame,
        nearby_diagnostics,
    };

    serde_json::to_value(&result).map_err(|e| format!("직렬화 실패: {}", e))
}

/// AI 기반 운명 감사 (eon-ai 연동)
#[tauri::command]
fn get_ai_audit(
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
    let gender = if is_male { Gender::Male } else { Gender::Female };

    let location = Location::new("출생지", lat, lon, 135.0);
    let birth_info = BirthInfo::solar(year, month, day, hour, minute)
        .with_timezone(&timezone)
        .with_location(location)
        .with_true_solar_time(true)
        .with_gender(gender);

    let (cy, cm, cd, ch, cmin) = birth_info.corrected_datetime();
    let input = SajuInput::new_solar_at(cy, cm, cd, ch, cmin, lon, lat).with_gender(gender);
    let pillars = FourPillars::calculate(&input).map_err(|e| format!("사주 계산 실패: {}", e))?;

    let emulator = LifePathEmulator::new(pillars.clone(), gender, cy);
    let life_report = emulator.emulate().map_err(|e| format!("에뮬레이션 실패: {}", e))?;

    let context = DestinyAIAuditor::generate_context(&pillars, &life_report);

    #[derive(serde::Serialize)]
    struct AiAuditResult {
        context_dump: String,
        peak_age: u32,
        valley_age: u32,
    }

    let result = AiAuditResult {
        context_dump: context,
        peak_age: life_report.peak_age,
        valley_age: life_report.valley_age,
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
            get_saju_analysis,
            get_transit_analysis,
            get_ai_audit,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
