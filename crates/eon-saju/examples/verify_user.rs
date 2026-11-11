//! User Saju Data Precision Calibration Analysis Test

use eon_core::{BirthInfo, Gender, Location};
use eon_saju::{AnalysisOptions, FourPillars, SajuInput};

fn main() {
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║       Kim Sung-ju Saju Precision Analysis (4 Calibration Modes)       ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    // 1. Birth Information
    let birth = BirthInfo::solar(2004, 11, 27, 22, 0)
        .with_location(Location::ansan())
        .with_korea_timezone()
        .with_true_solar_time(true)
        .with_gender(Gender::Male);

    // 2. Four Pillars Calculation
    let (y, m, d, h) = birth.for_saju();
    let input = SajuInput::new_solar(y, m, d, h, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!("【Four Pillars】: {}\n", pillars.hangul());

    // Destiny It Easy (DIE) Quick Scan
    use eon_saju::DestinyItEasy;
    println!("{}", DestinyItEasy::scan(&pillars));

    // Structure (Gyeokguk) Analysis
    println!("{}", pillars.structure());
    println!();

    // 2. Saju Analysis & Summary
    use eon_saju::DestinyEntropy;
    println!("{}", DestinyEntropy::analyze(&pillars));
    println!();

    let info = pillars.analyze();
    println!("【Saju Static Analysis】");
    println!("{}", info);

    // Destiny Linter (saju-clippy) Execution
    println!("【Destiny Linter: Diagnostics】");
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

    // Qi Topology Analysis Execution
    println!("{}", QiTopology::analyze(&pillars));
    println!();

    // Yongshin Analysis
    println!("{}", pillars.yongshin());
    println!();

    // Void Analysis
    println!("{}", pillars.void_analysis());
    println!();

    // Command (Saryeong) Analysis
    if let Ok(sar) = pillars.saryeong() {
        println!("{}", sar);
    }
    println!();

    // 12 Life Stages Analysis
    println!("{}", pillars.twelve_stages());
    println!();

    // Spirit Markers Analysis
    println!("{}", pillars.spirit_markers());
    println!();

    // Major Luck Precision Analysis (Swiss Ephemeris)
    println!("【Major Luck Precision Analysis (Swiss Ephemeris)】");
    // 2004-11-27 22:00 KST = 2004-11-27 13:00 UTC
    let luck = pillars
        .major_luck(Gender::Male, 2004, 11, 27, 13, 0)
        .unwrap();
    println!("{}", luck);
    println!();

    // Dynamic Interactions & Hidden Combinations (2026 Bing-Wu Baseline)
    use eon_saju::{DynamicLuckAnalysis, EarthlyBranch, GanZi, HeavenlyStem};
    let major_2nd = luck.cycles[1].ganzi; // Ding-Chou Major Luck
    let yearly_2026 = GanZi::new(HeavenlyStem::Bing, EarthlyBranch::Wu); // 2026 Bing-Wu Year

    println!(
        "{}",
        DynamicLuckAnalysis::analyze(
            &pillars,
            Some(major_2nd),
            Some(yearly_2026),
            None,
            None,
            None
        )
    );
    println!();

    // Saju-VM Life Path Emulation Simulation
    println!("【Saju-VM Life Path Emulation (100-Year Simulation)】");
    use eon_saju::LifePathEmulator;
    let emulator = LifePathEmulator::new(pillars.clone(), Gender::Male, 2004);
    let report = emulator.emulate().unwrap();

    println!(
        "Life Peak: Age {} (Energy Score: {:.1})",
        report.peak_age, report.frames[report.peak_age as usize].score
    );
    println!(
        "Life Valley: Age {} (Energy Score: {:.1})",
        report.valley_age, report.frames[report.valley_age as usize].score
    );

    // Karma LoadBalancer Diagnosis
    println!("\n【Karma LoadBalancer: Life Traffic Diagnosis】");
    use eon_saju::{DestinyComplexity, KarmaLoadBalancer};
    let diagnostics = KarmaLoadBalancer::diagnose(&report.frames);
    for diag in diagnostics.iter().take(5) {
        // Output top 5 major variations only
        let icon = match diag.status {
            eon_saju::TrafficStatus::Overloaded => "[OVERLOAD]",
            eon_saju::TrafficStatus::SystemDown => "[DOWN]",
            _ => "[INFO]",
        };
        println!("{} [Age {}] {}", icon, diag.age, diag.reason);
        println!("   └─ Strategy: {}", diag.strategy);
    }

    // Destiny Complexity Analysis Execution
    println!("\n{}", DestinyComplexity::analyze(&report.frames));

    // Destiny TTD (Time Travel Debugging)
    println!("\n【Destiny TTD: Reverse Debugging & Root Cause Analysis】");
    use eon_saju::DestinyDebugger;

    // 1. Backtrace: Valley Root Cause Analysis
    if let Some(rc) = DestinyDebugger::backtrace(&report, report.valley_age, "") {
        println!("Backtrace (Valley Root Cause):");
        println!("   Target: Age {} (Critical State)", rc.target_age);
        println!(
            "   Origin: Age {} (Entry Point Identified)",
            rc.root_cause_age
        );
        println!("   Reason: {}", rc.reason);
    }

    // 2. Breakpoints: Locate points where success signatures occur
    let breakpoints = DestinyDebugger::find_breakpoints(&report, |f| {
        f.signatures
            .iter()
            .any(|s| s.severity == eon_saju::SignatureSeverity::Success)
    });
    println!("\nHidden Breakpoints (Success Events):");
    println!("   Points: {:?}", breakpoints);

    // 3. Life Diff: Delta analysis based on environment variance
    let mut input_alt = input.clone();
    input_alt.hour = (input_alt.hour + 1) % 24;
    let pillars_alt = FourPillars::calculate(&input_alt).unwrap();
    let emulator_alt = LifePathEmulator::new(pillars_alt, Gender::Male, input_alt.year);
    let report_alt = emulator_alt.emulate().unwrap();

    let diffs = DestinyDebugger::diff(&report, &report_alt);
    println!("\nLife Path Diff (Timezone/Environment Correction):");
    if diffs.is_empty() {
        println!("   No significant delta found.");
    } else {
        for d in diffs.iter().take(3) {
            println!(
                "   [Age {}] Score Delta: {:+.1} | Impact: {:?}",
                d.age, d.score_delta, d.added_tags
            );
        }
        println!("   ... total {} diff segments detected.", diffs.len());
    }

    println!("\n[Life Energy Graph (10-Year Summary)]");
    for frame in report.frames {
        let bar_len = (frame.score / 5.0) as usize;
        let bar = "■".repeat(bar_len);
        let mut tags = frame.tags_as_strings().join(",");
        if !frame.signatures.is_empty() {
            let sig_names: Vec<String> = frame
                .signatures
                .iter()
                .map(|s| format!("[{}]", s.name))
                .collect();
            tags = format!("{} {}", tags, sig_names.join(" "));
        }
        println!(
            "Age {:>3} ({}): {:<20} ({:.1}){}",
            frame.age,
            frame.ganzi.hanja(),
            bar,
            frame.score,
            tags
        );
    }
    println!();

    // Destiny Fuzzer (Vulnerability Audit)
    println!("【Destiny Fuzzer: Vulnerability Audit】");
    use eon_saju::{DestinyFuzzer, SajuVM};
    let fuzzer = DestinyFuzzer::new(SajuVM::new(pillars.clone()));

    println!("1. Auditing yearly luck based on current major luck (Wu-Yin)...");
    let major_2027 = pillars
        .major_luck(Gender::Male, 2004, 11, 27, 13, 0)
        .unwrap()
        .cycles[2]
        .ganzi;
    let audit_report = fuzzer.audit(major_2027);
    println!("Vulnerabilities discovered: {}", audit_report.total_crashes);
    for v in audit_report.critical_vectors {
        println!("{}", v);
    }

    println!("\n2. Fuzzing 10,000 random major/yearly luck combinations...");
    let fuzz_report = fuzzer.fuzz_random(10000);
    println!(
        "Potential crashes discovered: {} (Top 5 risk vectors extracted)",
        fuzz_report.total_crashes
    );
    for v in fuzz_report.critical_vectors {
        println!("{}", v);
    }
    println!();

    // 3. Execute 4 calibration mode analyses
    let cases = [
        (false, false, "1. Base (No Correction)"),
        (true, false, "2. Transformation Only"),
        (
            true,
            true,
            "3. Transformation + Position/Climate Correction",
        ),
        (false, true, "4. Position/Climate Correction Only"),
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
