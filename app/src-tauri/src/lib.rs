use eon_vedic::analysis::report::VedicAnalysisReport;
use eon_vedic::core::chart::{VedicChart, VedicChartCalculator};
use eon_vedic::analysis::compatibility::CompatibilityEngine;

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
use eon_saju::engine::interprocess::{CompatibilityAuditor, CompatibilityAudit};
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
    timezone: String,
) -> Result<serde_json::Value, String> {
    let birth_info = BirthInfo::solar(year, month, day, hour, minute)
        .with_timezone(&timezone);
    let dt = birth_info.to_utc()
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

    // corrected_datetime()이 이미 진태양시 보정 완료 → 이중 보정 방지
    let input = SajuInput::new_solar(cy, cm, cd, ch, cmin).with_gender(gender);

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
    // corrected_datetime()이 이미 진태양시 보정 완료 → 이중 보정 방지
    let input = SajuInput::new_solar(cy, cm, cd, ch, cmin).with_gender(gender);
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

    // LifeFrame의 tags를 문자열 배열로 변환 (TraceTag enum은 JSON 객체로 직렬화됨)
    #[derive(serde::Serialize)]
    struct LifeFrameDto {
        age: u32,
        ganzi: eon_saju::core::ganzi::GanZi,
        major_ganzi: eon_saju::core::ganzi::GanZi,
        score: f32,
        tags: Vec<String>,
        esil_trace: String,
        register_state: eon_saju::engine::vm::QiRegisters,
    }

    #[derive(serde::Serialize)]
    struct TransitResult {
        yearly_luck: YearlyLuck,
        monthly_luck: MonthlyLuck,
        current_age: u32,
        current_frame: Option<LifeFrameDto>,
        nearby_diagnostics: Vec<LoadBalanceDiagnostic>,
    }

    let current_frame_dto = current_frame.map(|f| LifeFrameDto {
        age: f.age,
        ganzi: f.ganzi,
        major_ganzi: f.major_ganzi,
        score: f.score,
        tags: f.tags_as_strings(),
        esil_trace: f.esil_trace,
        register_state: f.register_state,
    });

    let result = TransitResult {
        yearly_luck: yearly,
        monthly_luck: monthly,
        current_age,
        current_frame: current_frame_dto,
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
    // corrected_datetime()이 이미 진태양시 보정 완료 → 이중 보정 방지
    let input = SajuInput::new_solar(cy, cm, cd, ch, cmin).with_gender(gender);
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

/// 사주 궁합 분석 (두 사람의 원국 비교)
#[tauri::command]
fn get_saju_compatibility(
    // 사람 1
    year1: i32, month1: u32, day1: u32, hour1: u32, minute1: u32,
    is_male1: bool, lon1: f64, lat1: f64,
    // 사람 2
    year2: i32, month2: u32, day2: u32, hour2: u32, minute2: u32,
    is_male2: bool, lon2: f64, lat2: f64,
    timezone: String,
) -> Result<serde_json::Value, String> {
    let make_pillars = |y: i32, mo: u32, d: u32, h: u32, mi: u32, male: bool, lon: f64, lat: f64| -> Result<FourPillars, String> {
        let gender = if male { Gender::Male } else { Gender::Female };
        let location = Location::new("출생지", lat, lon, 135.0);
        let birth_info = BirthInfo::solar(y, mo, d, h, mi)
            .with_timezone(&timezone)
            .with_location(location)
            .with_true_solar_time(true)
            .with_gender(gender);
        let (cy, cm, cd, ch, cmin) = birth_info.corrected_datetime();
        // corrected_datetime()이 이미 진태양시 보정 완료 → 이중 보정 방지
        let input = SajuInput::new_solar(cy, cm, cd, ch, cmin).with_gender(gender);
        FourPillars::calculate(&input).map_err(|e| format!("사주 계산 실패: {}", e))
    };

    let pillars1 = make_pillars(year1, month1, day1, hour1, minute1, is_male1, lon1, lat1)?;
    let pillars2 = make_pillars(year2, month2, day2, hour2, minute2, is_male2, lon2, lat2)?;

    let vm1 = SajuVM::new(pillars1);
    let vm2 = SajuVM::new(pillars2);
    let audit: CompatibilityAudit = CompatibilityAuditor::audit(&vm1, &vm2);

    serde_json::to_value(&audit).map_err(|e| format!("직렬화 실패: {}", e))
}

/// 베딕 궁합 분석 (Ashta Kuta 36점 만점)
#[tauri::command]
async fn get_vedic_compatibility(
    year1: i32, month1: u32, day1: u32, hour1: u32, minute1: u32, lat1: f64, lon1: f64,
    year2: i32, month2: u32, day2: u32, hour2: u32, minute2: u32, lat2: f64, lon2: f64,
    timezone: String,
) -> Result<serde_json::Value, String> {
    let calculator = VedicChartCalculator::new();
    let dt1 = BirthInfo::solar(year1, month1, day1, hour1, minute1)
        .with_timezone(&timezone)
        .to_utc()
        .ok_or_else(|| "Invalid date/time (person 1)".to_string())?;
    let dt2 = BirthInfo::solar(year2, month2, day2, hour2, minute2)
        .with_timezone(&timezone)
        .to_utc()
        .ok_or_else(|| "Invalid date/time (person 2)".to_string())?;
    let chart1 = calculator.calculate(dt1, lat1, lon1);
    let chart2 = calculator.calculate(dt2, lat2, lon2);
    let result = CompatibilityEngine::analyze(&chart1, &chart2);
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
            get_saju_compatibility,
            get_vedic_compatibility,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
