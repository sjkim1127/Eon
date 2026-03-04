use eon_vedic::analysis::compatibility::CompatibilityEngine;
use eon_vedic::analysis::report::VedicAnalysisReport;
use eon_vedic::core::chart::{VedicChart, VedicChartCalculator};
use serde::Serialize;
use wasm_bindgen::prelude::*;

/// WASM 패닉 메시지를 브라우저 콘솔에 표시
#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// === Saju (四柱) imports ===
use eon_saju::analysis::analytics::Analyzer;
use eon_saju::analysis::major_luck::MajorLuckAnalysis;
use eon_saju::analysis::periodic_luck::{MonthlyLuck, YearlyLuck};
use eon_saju::analysis::relationships::RelationshipAnalysis;
use eon_saju::analysis::void::VoidAnalysis;
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::engine::complexity::{ComplexityAnalysis, DestinyComplexity};
use eon_saju::engine::emulator::LifePathEmulator;
use eon_saju::engine::entropy::{DestinyEntropy, EntropyAnalysis};
use eon_saju::engine::fuzzer::DestinyFuzzer;
use eon_saju::engine::interprocess::{CompatibilityAudit, CompatibilityAuditor};
use eon_saju::engine::linter::{DestinyLinter, SajuLint};
use eon_saju::engine::load_balancer::{KarmaLoadBalancer, LoadBalanceDiagnostic};
use eon_saju::engine::topology::{QiTopology, TopologyAnalysis};
use eon_saju::engine::vm::SajuVM;
use eon_saju::report::SajuReport;

// === Core imports (BirthInfo, Location, DST) ===
use eon_core::{BirthInfo, Gender, Location};

// Chrono imports for timezone-based standard meridian calculation
use chrono::TimeZone;
use chrono_tz::Tz;

/// Compute the standard meridian (degrees) from an IANA timezone string.
/// e.g. "Asia/Seoul" (UTC+9) → 135.0, "America/New_York" (UTC-5) → -75.0
fn standard_meridian_from_tz(timezone: &str) -> f64 {
    if let Ok(tz) = timezone.parse::<Tz>() {
        let ref_dt = tz.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).single();
        if let Some(dt) = ref_dt {
            use chrono_tz::OffsetComponents;
            let base_offset_secs = dt.offset().base_utc_offset().num_seconds() as f64;
            return (base_offset_secs / 3600.0) * 15.0;
        }
    }
    135.0 // Fallback: Korean Standard Meridian
}

/// DST 메타데이터 + 고급 분석을 포함한 사주 분석 결과 래퍼
#[derive(Serialize)]
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
    vulnerability_report: Option<eon_saju::engine::fuzzer::VulnerabilityReport>,
    relationships: RelationshipAnalysis,
    void_analysis: VoidAnalysis,
    complexity: Option<ComplexityAnalysis>,
    /// serde_wasm_bindgen workaround: timeline은 JSON 문자열로 전달
    timeline_json: String,
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
    is_lunar: bool,
    is_leap_month: bool,
    lat: f64,
    lon: f64,
    timezone: &str,
) -> Result<JsValue, JsError> {
    let mut birth_info = if is_lunar {
        BirthInfo::lunar(year, month, day, hour, minute, is_leap_month)
    } else {
        BirthInfo::solar(year, month, day, hour, minute)
    };
    birth_info = birth_info.with_timezone(timezone);
    let dt = birth_info
        .to_utc()
        .ok_or_else(|| JsError::new("Invalid date/time"))?;

    let calculator = VedicChartCalculator::new();
    let chart = calculator.calculate(dt, lat, lon);

    let report = VedicAnalysisReport::generate(&chart, dt);

    // Gochara: compute current transit chart and analyze
    let gochara = {
        let natal_moon_rasi = chart
            .planets
            .iter()
            .find(|p| p.planet == eon_vedic::planets::VedicPlanet::Moon)
            .map(|m| m.rasi)
            .unwrap_or(1);
        let transit_chart = calculator.calculate(chrono::Utc::now(), lat, lon);
        eon_vedic::analysis::gochara::GocharaEngine::analyze(natal_moon_rasi, &transit_chart)
    };

    #[derive(Serialize)]
    struct VedicAnalysisResult {
        report: VedicAnalysisReport,
        chart: VedicChart,
        gochara: eon_vedic::analysis::gochara::GocharaSummary,
    }

    let result = VedicAnalysisResult {
        report,
        chart,
        gochara,
    };

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
    is_lunar: bool,
    is_leap_month: bool,
    is_male: bool,
    use_night_rat_hour: bool,
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
    let location = Location::new("출생지", lat, lon, standard_meridian_from_tz(timezone));
    let mut birth_info = if is_lunar {
        BirthInfo::lunar(year, month, day, hour, minute, is_leap_month)
    } else {
        BirthInfo::solar(year, month, day, hour, minute)
    };
    birth_info = birth_info
        .with_timezone(timezone)
        .with_location(location)
        .with_true_solar_time(true)
        .with_gender(gender);

    let is_dst = birth_info.is_dst();
    let dst_offset = birth_info.dst_offset_hours();
    let (cy, cm, cd, ch, cmin) = birth_info.corrected_datetime();

    // corrected_datetime()이 이미 진태양시 보정 완료
    // new_solar_at(lon) 사용 시 이중 보정 발생 → new_solar 사용
    let input = SajuInput::new_solar(cy, cm, cd, ch, cmin)
        .with_gender(gender)
        .with_night_rat_hour(use_night_rat_hour);

    let pillars = FourPillars::calculate(&input)
        .map_err(|e| JsError::new(&format!("사주 계산 실패: {}", e)))?;

    // 고급 정적 엔진 계산 (원국 기반)
    let lints: Vec<SajuLint> = DestinyLinter::lint(&pillars);
    let entropy: EntropyAnalysis = DestinyEntropy::analyze(&pillars);
    let qi_topology: TopologyAnalysis = QiTopology::analyze(&pillars);
    let relationships = pillars.relationships();
    let void_analysis = pillars.void_analysis();

    let mut report = SajuReport::new(pillars.clone());
    let mut load_diagnostics: Vec<LoadBalanceDiagnostic> = Vec::new();
    let mut crash_count: u32 = 0;
    let mut vulnerability_report: Option<eon_saju::engine::fuzzer::VulnerabilityReport> = None;
    let mut complexity: Option<ComplexityAnalysis> = None;

    if let Ok(major_luck) =
        MajorLuckAnalysis::calculate_astro(&pillars, gender, cy, cm, cd, ch, cmin)
    {
        let emulator = LifePathEmulator::new(pillars.clone(), gender, cy);
        if let Ok(life_report) = emulator.emulate() {
            let golden_time = Analyzer::find_golden_time(&life_report.timeline, 10);

            // 동적 엔진 계산 (시뮬레이션 기반)
            load_diagnostics = KarmaLoadBalancer::diagnose(&life_report.frames);
            complexity = Some(DestinyComplexity::analyze(&life_report.frames));

            let vm_fuzz = SajuVM::new(pillars.clone());
            let fuzzer = DestinyFuzzer::new(vm_fuzz);
            let fuzzer_report = fuzzer.audit(pillars.month);
            crash_count = fuzzer_report.total_crashes as u32;
            vulnerability_report = Some(fuzzer_report);

            report = report
                .with_major_luck(major_luck)
                .with_timeline(life_report.timeline)
                .with_vm_simulation(life_report.frames);

            if let Some(gt) = golden_time {
                report = report.with_golden_time(gt);
            }
        }
    }

    // timeline을 JSON 문자열로 직렬화 (serde_wasm_bindgen 우회)
    let timeline_json =
        serde_json::to_string(&report.timeline).unwrap_or_else(|_| "[]".to_string());

    // 결과 래퍼
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
        vulnerability_report,
        relationships,
        void_analysis,
        complexity,
        timeline_json,
    };

    Ok(serde_wasm_bindgen::to_value(&result)?)
}

/// 현재 운세(세운/월운/일운) 분석 — WASM에서 호출 가능
#[wasm_bindgen]
pub fn get_transit_analysis(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    is_lunar: bool,
    is_leap_month: bool,
    is_male: bool,
    use_night_rat_hour: bool,
    lon: f64,
    lat: f64,
    timezone: &str,
    current_year: i32,
    current_month: u32,
    current_day: u32,
) -> Result<JsValue, JsError> {
    let gender = if is_male {
        Gender::Male
    } else {
        Gender::Female
    };

    let location = Location::new("출생지", lat, lon, standard_meridian_from_tz(timezone));
    let mut birth_info = if is_lunar {
        BirthInfo::lunar(year, month, day, hour, minute, is_leap_month)
    } else {
        BirthInfo::solar(year, month, day, hour, minute)
    };
    birth_info = birth_info
        .with_timezone(timezone)
        .with_location(location)
        .with_true_solar_time(true)
        .with_gender(gender);

    let (cy, cm, cd, ch, cmin) = birth_info.corrected_datetime();
    // corrected_datetime()이 이미 진태양시 보정 완료 → 이중 보정 방지
    let input = SajuInput::new_solar(cy, cm, cd, ch, cmin)
        .with_gender(gender)
        .with_night_rat_hour(use_night_rat_hour);
    let pillars = FourPillars::calculate(&input)
        .map_err(|e| JsError::new(&format!("사주 계산 실패: {}", e)))?;

    // 세운/월운 계산
    let yearly = YearlyLuck::calculate(current_year, &pillars);
    let monthly = MonthlyLuck::calculate(current_year, current_month, &pillars);

    // 12개월 전체 월운
    let monthly_lucks: Vec<MonthlyLuck> = (1..=12)
        .map(|m| MonthlyLuck::calculate(current_year, m, &pillars))
        .collect();

    // 일운 계산
    let day_ganzi =
        eon_saju::core::ganzi_utils::calculate_day_ganzi(current_year, current_month, current_day);
    let day_master = pillars.day_master();
    let daily_stem_god = eon_saju::core::ten_gods::TenGod::from_stems(day_master, day_ganzi.stem);
    let daily_branch_god =
        eon_saju::core::ten_gods::TenGod::from_stem_and_branch(day_master, day_ganzi.branch);
    let daily_twelve_stage =
        eon_saju::core::twelve_stages::calculate_twelve_stage(day_master, day_ganzi.branch)
            .hangul()
            .to_string();
    let daily_influence = Some(
        eon_saju::analysis::dynamic_luck::DynamicLuckAnalysis::get_influence(
            day_ganzi, "일운", &pillars,
        ),
    );

    // 현재 나이 계산
    let current_age = (current_year - year).max(0) as u32;

    // SajuVM을 통해 전체 인생 프레임 시뮬레이션
    let vm = SajuVM::new(pillars.clone());
    let frames = vm.simulate_life(0, 105);

    // 현재 나이에 해당하는 프레임 추출
    let current_frame = frames.iter().find(|f| f.age == current_age).cloned();

    // 전후 5년(과거 3년 ~ 미래 5년) 범위의 부하 진단
    let all_diagnostics = KarmaLoadBalancer::diagnose(&frames);
    let nearby_diagnostics: Vec<LoadBalanceDiagnostic> = all_diagnostics
        .into_iter()
        .filter(|d| d.age >= current_age.saturating_sub(3) && d.age <= current_age + 5)
        .collect();

    // LifeFrameDto 정의 (Tauri와 동일한 구조)
    #[derive(Serialize)]
    struct LifeFrameDto {
        age: u32,
        ganzi: eon_saju::core::ganzi::GanZi,
        major_ganzi: eon_saju::core::ganzi::GanZi,
        score: f32,
        tags: Vec<String>,
        esil_trace: String,
        register_state: eon_saju::engine::vm::QiRegisters,
    }

    #[derive(Serialize)]
    struct DailyLuckDto {
        year: i32,
        month: u32,
        day: u32,
        ganzi: eon_saju::core::ganzi::GanZi,
        stem_god: eon_saju::core::ten_gods::TenGod,
        branch_god: eon_saju::core::ten_gods::TenGod,
        influence: Option<eon_saju::analysis::dynamic_luck::LuckInfluence>,
        twelve_stage: Option<String>,
    }

    #[derive(Serialize)]
    struct TransitResult {
        yearly_luck: YearlyLuck,
        monthly_luck: MonthlyLuck,
        monthly_lucks: Vec<MonthlyLuck>,
        daily_luck: DailyLuckDto,
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

    let daily_luck_dto = DailyLuckDto {
        year: current_year,
        month: current_month,
        day: current_day,
        ganzi: day_ganzi,
        stem_god: daily_stem_god,
        branch_god: daily_branch_god,
        influence: daily_influence,
        twelve_stage: Some(daily_twelve_stage),
    };

    let result = TransitResult {
        yearly_luck: yearly,
        monthly_luck: monthly,
        monthly_lucks,
        daily_luck: daily_luck_dto,
        current_age,
        current_frame: current_frame_dto,
        nearby_diagnostics,
    };

    Ok(serde_wasm_bindgen::to_value(&result)?)
}

/// 사주 궁합 분석 - WASM에서 호출 가능
#[wasm_bindgen]
pub fn get_saju_compatibility(
    year1: i32,
    month1: u32,
    day1: u32,
    hour1: u32,
    minute1: u32,
    is_lunar1: bool,
    is_leap_month1: bool,
    is_male1: bool,
    lon1: f64,
    lat1: f64,
    use_night_rat_hour1: bool,
    year2: i32,
    month2: u32,
    day2: u32,
    hour2: u32,
    minute2: u32,
    is_lunar2: bool,
    is_leap_month2: bool,
    is_male2: bool,
    lon2: f64,
    lat2: f64,
    use_night_rat_hour2: bool,
    timezone1: &str,
    timezone2: &str,
) -> Result<JsValue, JsError> {
    let make_pillars = |y: i32,
                        mo: u32,
                        d: u32,
                        h: u32,
                        mi: u32,
                        lunar: bool,
                        leap: bool,
                        male: bool,
                        lon: f64,
                        lat: f64,
                        night_rat: bool,
                        tz: &str|
     -> Result<FourPillars, JsError> {
        let gender = if male { Gender::Male } else { Gender::Female };
        let location = Location::new("출생지", lat, lon, standard_meridian_from_tz(tz));
        let mut birth_info = if lunar {
            BirthInfo::lunar(y, mo, d, h, mi, leap)
        } else {
            BirthInfo::solar(y, mo, d, h, mi)
        };
        birth_info = birth_info
            .with_timezone(tz)
            .with_location(location)
            .with_true_solar_time(true)
            .with_gender(gender);
        let (cy, cm, cd, ch, cmin) = birth_info.corrected_datetime();
        // corrected_datetime()이 이미 진태양시 보정 완료 → 이중 보정 방지
        let input = SajuInput::new_solar(cy, cm, cd, ch, cmin)
            .with_gender(gender)
            .with_night_rat_hour(night_rat);
        FourPillars::calculate(&input).map_err(|e| JsError::new(&format!("사주 계산 실패: {}", e)))
    };
    let pillars1 = make_pillars(
        year1,
        month1,
        day1,
        hour1,
        minute1,
        is_lunar1,
        is_leap_month1,
        is_male1,
        lon1,
        lat1,
        use_night_rat_hour1,
        timezone1,
    )?;
    let pillars2 = make_pillars(
        year2,
        month2,
        day2,
        hour2,
        minute2,
        is_lunar2,
        is_leap_month2,
        is_male2,
        lon2,
        lat2,
        use_night_rat_hour2,
        timezone2,
    )?;
    let vm1 = SajuVM::new(pillars1);
    let vm2 = SajuVM::new(pillars2);
    let audit: CompatibilityAudit = CompatibilityAuditor::audit(&vm1, &vm2);
    Ok(serde_wasm_bindgen::to_value(&audit)?)
}

/// 베딕 궁합 분석 (Ashta Kuta) - WASM에서 호출 가능
#[wasm_bindgen]
pub async fn get_vedic_compatibility(
    year1: i32,
    month1: u32,
    day1: u32,
    hour1: u32,
    minute1: u32,
    is_lunar1: bool,
    is_leap_month1: bool,
    lat1: f64,
    lon1: f64,
    year2: i32,
    month2: u32,
    day2: u32,
    hour2: u32,
    minute2: u32,
    is_lunar2: bool,
    is_leap_month2: bool,
    lat2: f64,
    lon2: f64,
    timezone1: &str,
    timezone2: &str,
) -> Result<JsValue, JsError> {
    let calculator = VedicChartCalculator::new();
    let birth1 = if is_lunar1 {
        BirthInfo::lunar(year1, month1, day1, hour1, minute1, is_leap_month1)
    } else {
        BirthInfo::solar(year1, month1, day1, hour1, minute1)
    };
    let dt1 = birth1
        .with_timezone(timezone1)
        .to_utc()
        .ok_or_else(|| JsError::new("Invalid date/time (person 1)"))?;
    let birth2 = if is_lunar2 {
        BirthInfo::lunar(year2, month2, day2, hour2, minute2, is_leap_month2)
    } else {
        BirthInfo::solar(year2, month2, day2, hour2, minute2)
    };
    let dt2 = birth2
        .with_timezone(timezone2)
        .to_utc()
        .ok_or_else(|| JsError::new("Invalid date/time (person 2)"))?;
    let chart1 = calculator.calculate(dt1, lat1, lon1);
    let chart2 = calculator.calculate(dt2, lat2, lon2);
    let result = CompatibilityEngine::analyze(&chart1, &chart2);
    Ok(serde_wasm_bindgen::to_value(&result)?)
}
