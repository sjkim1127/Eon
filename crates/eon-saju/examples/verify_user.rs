//! 사용자 사주 데이터 정밀 보정 분석 테스트

use eon_core::{BirthInfo, Gender, Location};
use eon_saju::{FourPillars, SajuInput, AnalysisOptions};

fn main() {
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║       김성주님 사주 정밀 분석 (4가지 보정 모드)       ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    // 1. 출생 정보
    let birth = BirthInfo::solar(2004, 11, 27, 22, 0)
        .with_location(Location::ansan())
        .with_korea_timezone()
        .with_true_solar_time(true)
        .with_gender(Gender::Male);

    // 2. 사주 팔자 계산
    let (y, m, d, h) = birth.for_saju();
    let input = SajuInput::new_solar(y, m, d, h, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!("【사주 팔자】: {}\n", pillars.hangul());

    // Destiny It Easy (DIE) Quick Scan
    use eon_saju::DestinyItEasy;
    println!("{}", DestinyItEasy::scan(&pillars));

    // 격국 분석
    println!("{}", pillars.structure());
    println!();

    // 2. 사주 분석 및 요약
    use eon_saju::DestinyEntropy;
    println!("{}", DestinyEntropy::analyze(&pillars));
    println!();

    let info = pillars.analyze();
    println!("【사주 정적 분석 (Static Analysis)】");
    println!("{}", info);

    // Destiny Linter (saju-clippy) 실행
    println!("【Destiny Linter: 사주 린트 결과 (Diagnostics)】");
    use eon_saju::{DestinyLinter, QiTopology};
    let lints = DestinyLinter::lint(&pillars);
    if lints.is_empty() {
        println!("No issues found. Perfect structure!");
    } else {
        for lint in lints {
            let label = match lint.severity {
                eon_saju::LintSeverity::Error => "\x1b[31m[ERROR]\x1b[0m",
                eon_saju::LintSeverity::Warning => "\x1b[33m[WARN ]\x1b[0m",
                eon_saju::LintSeverity::Info => "\x1b[34m[INFO ]\x1b[0m",
            };
            println!("{} {}: {}", label, lint.code, lint.message);
            println!("       └─ Advice: {}", lint.advice);
        }
    }
    println!();

    // Qi Topology 분석 실행
    println!("{}", QiTopology::analyze(&pillars));
    println!();

    // 용신 분석
    println!("{}", pillars.yongshin());
    println!();

    // 공망 분석
    println!("{}", pillars.void_analysis());
    println!();

    // 월령분금(사령) 분석
    println!("{}", pillars.saryeong());
    println!();

    // 12운성 분석
    println!("{}", pillars.twelve_stages());
    println!();

    // 신살 분석
    println!("{}", pillars.spirit_markers());
    println!();

    // 대운 정밀 분석 (천문 엔진 자동 계산)
    println!("【대운 정밀 분석 (Swiss Ephemeris 기반)】");
    // 2004-11-27 22:00 KST = 2004-11-27 13:00 UTC
    let luck = pillars.major_luck(Gender::Male, 2004, 11, 27, 13, 0).unwrap();
    println!("{}", luck);
    println!();

    // 동적 합충 및 암합 분석 (2026년 丙午年 기준)
    use eon_saju::{GanZi, HeavenlyStem, EarthlyBranch, DynamicLuckAnalysis};
    let major_2nd = luck.cycles[1].ganzi; // 丁丑 대운
    let yearly_2026 = GanZi::new(HeavenlyStem::Bing, EarthlyBranch::Wu); // 2026년 丙午年

    println!("{}", DynamicLuckAnalysis::analyze(&pillars, Some(major_2nd), Some(yearly_2026), None, None, None));
    println!();

    // Saju-VM 인생 경로 에뮬레이션 시뮬레이션
    println!("【Saju-VM 인생 경로 에뮬레이션 (100년 시뮬레이션)】");
    use eon_saju::LifePathEmulator;
    let emulator = LifePathEmulator::new(pillars.clone(), Gender::Male, 2004);
    let report = emulator.emulate().unwrap();

    println!("인생의 최정점: {}세 (에너지 점수: {:.1})", report.peak_age, report.frames[report.peak_age as usize].score);
    println!("인생의 최저점: {}세 (에너지 점수: {:.1})", report.valley_age, report.frames[report.valley_age as usize].score);

    // Karma LoadBalancer 진단
    println!("\n【Karma LoadBalancer: 인생 트래픽 진단】");
    use eon_saju::{KarmaLoadBalancer, DestinyComplexity};
    let diagnostics = KarmaLoadBalancer::diagnose(&report.frames);
    for diag in diagnostics.iter().take(5) { // 상위 5개 주요 변동 사항만 출력
        let icon = match diag.status {
            eon_saju::TrafficStatus::Overloaded => "🔥",
            eon_saju::TrafficStatus::SystemDown => "🚫",
            _ => "ℹ️",
        };
        println!("{} [{}세] {}", icon, diag.age, diag.reason);
        println!("   └─ Strategy: {}", diag.strategy);
    }

    // Destiny Complexity 분석 실행
    println!("\n{}", DestinyComplexity::analyze(&report.frames));

    // Destiny TTD (Time Travel Debugging)
    println!("\n【Destiny TTD: 리버스 디버깅 & Root Cause Analysis】");
    use eon_saju::DestinyDebugger;
    
    // 1. Backtrace: 최저점(Valley)의 근본 원인 분석
    if let Some(rc) = DestinyDebugger::backtrace(&report, report.valley_age, "") {
        println!("🔍 Backtrace (Valley Root Cause):");
        println!("   Target: {}세 (Critical State)", rc.target_age);
        println!("   Origin: {}세 (Entry Point Identified)", rc.root_cause_age);
        println!("   Reason: {}", rc.reason);
    }

    // 2. Breakpoints: 특정 조건(성공 시그니처)이 발생하는 지점 탐색
    let breakpoints = DestinyDebugger::find_breakpoints(&report, |f| {
        f.signatures.iter().any(|s| s.severity == eon_saju::SignatureSeverity::Success)
    });
    println!("\n📍 Hidden Breakpoints (Success Events):");
    println!("   Points: {:?}", breakpoints);

    // 3. Life Diff: 환경 변동에 따른 델타 분석 (시뮬레이션: 시간을 1시간 조정했을 때)
    let mut input_alt = input.clone();
    input_alt.hour = (input_alt.hour + 1) % 24;
    let pillars_alt = FourPillars::calculate(&input_alt).unwrap();
    let emulator_alt = LifePathEmulator::new(pillars_alt, Gender::Male, input_alt.year);
    let report_alt = emulator_alt.emulate().unwrap();
    
    let diffs = DestinyDebugger::diff(&report, &report_alt);
    println!("\n⚖️ Life Path Diff (Timezone/Environment Correction):");
    if diffs.is_empty() {
        println!("   No significant delta found.");
    } else {
        for d in diffs.iter().take(3) {
            println!("   [{}세] Score Delta: {:+.1} | Impact: {:?}", d.age, d.score_delta, d.added_tags);
        }
        println!("   ... total {} diff segments detected.", diffs.len());
    }

    println!("\n[인생 에너지 그래프 (10년 단위 요약)]");
    for frame in report.frames {
        let bar_len = (frame.score / 5.0) as usize;
        let bar = "■".repeat(bar_len);
        let mut tags = frame.tags.join(",");
        if !frame.signatures.is_empty() {
            let sig_names: Vec<String> = frame.signatures.iter().map(|s| format!("[{}]", s.name)).collect();
            tags = format!("{} {}", tags, sig_names.join(" "));
        }
        println!("{:>3}세 ({}): {:<20} ({:.1}){}", frame.age, frame.ganzi.hanja(), bar, frame.score, tags);
    }
    println!();

    // Destiny Fuzzer (운명 취약점 분석)
    println!("【Destiny Fuzzer: 운명 보안 감사 (Vulnerability Audit)】");
    use eon_saju::{DestinyFuzzer, SajuVM};
    let fuzzer = DestinyFuzzer::new(SajuVM::new(pillars.clone()));
    
    println!("1. 현재 대운(戊寅) 기준 세운 감사(Audit)...");
    let major_2027 = pillars.major_luck(Gender::Male, 2004, 11, 27, 13, 0).unwrap().cycles[2].ganzi;
    let audit_report = fuzzer.audit(major_2027);
    println!("발견된 취약점: {}개", audit_report.total_crashes);
    for v in audit_report.critical_vectors {
        println!("{}", v);
    }

    println!("\n2. 무작위 대운/세운 조합 10,000건 퍼징(Fuzzing)...");
    let fuzz_report = fuzzer.fuzz_random(10000);
    println!("발견된 잠재적 크래시: {}건 (상위 5개 위험 벡터 추출)", fuzz_report.total_crashes);
    for v in fuzz_report.critical_vectors {
        println!("{}", v);
    }
    println!();

    // 3. 4가지 모드 분석 실행
    let cases = [
        (false, false, "1. 기본 (보정 X)"),
        (true, false, "2. 합화만 적용"),
        (true, true, "3. 합화 + 궁성/조후 보정 적용"),
        (false, true, "4. 궁성/조후 보정만 적용"),
    ];

    for (transform, correction, title) in cases {
        let options = AnalysisOptions {
            apply_transform: transform,
            apply_correction: correction,
        };
        
        let result = pillars.integrated_analysis(options, &eon_saju::AnalysisConfig::default());
        
        println!("----------------------------------------------------");
        println!("{}", title);
        println!("{}", result);
    }
}
