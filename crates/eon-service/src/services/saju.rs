use crate::birth::prepare_birth_context;
use crate::dto::{AnalysisMeta, SajuAnalysisInput, SajuAnalysisOutput};
use crate::error::ServiceError;
use eon_core::Gender;
use eon_saju::analysis::analytics::Analyzer;
use eon_saju::analysis::major_luck::MajorLuckAnalysis;
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::engine::complexity::DestinyComplexity;
use eon_saju::engine::emulator::LifePathEmulator;
use eon_saju::engine::entropy::DestinyEntropy;
use eon_saju::engine::fuzzer::DestinyFuzzer;
use eon_saju::engine::linter::DestinyLinter;
use eon_saju::engine::load_balancer::KarmaLoadBalancer;
use eon_saju::engine::topology::QiTopology;
use eon_saju::engine::vm::SajuVM;
use eon_saju::report::SajuReport;

pub fn analyze(input: SajuAnalysisInput) -> Result<SajuAnalysisOutput, ServiceError> {
    let gender = if input.is_male {
        Gender::Male
    } else {
        Gender::Female
    };

    let birth_ctx = prepare_birth_context(&input.base, Some(gender), true)?;
    let birth_utc = birth_ctx
        .birth_info
        .to_utc()
        .map_err(|e| ServiceError::InvalidInput(e.to_string()))?;

    let saju_input = SajuInput::new_solar(
        birth_ctx.corrected_year,
        birth_ctx.corrected_month,
        birth_ctx.corrected_day,
        birth_ctx.corrected_hour,
        birth_ctx.corrected_minute,
    )
    .with_birth_utc(birth_utc)
    .with_gender(gender)
    .with_night_rat_hour(input.use_night_rat_hour);

    let pillars = FourPillars::calculate(&saju_input)
        .map_err(|e| ServiceError::Saju(format!("Four Pillars calculation failed: {}", e)))?;

    // Static engine calculations
    let lints = DestinyLinter::lint(&pillars);
    let entropy = DestinyEntropy::analyze(&pillars);
    let qi_topology = QiTopology::analyze(&pillars);
    let relationships = pillars.relationships();
    let void_analysis = pillars.void_analysis();

    let mut report = SajuReport::new(pillars.clone());
    let mut load_diagnostics = Vec::new();
    let mut crash_count = 0;
    let mut vulnerability_report = None;
    let mut complexity = None;

    if let Ok(major_luck) = MajorLuckAnalysis::calculate_astro(
        &pillars,
        gender,
        birth_ctx.corrected_year,
        birth_ctx.corrected_month,
        birth_ctx.corrected_day,
        birth_ctx.corrected_hour,
        birth_ctx.corrected_minute,
    ) {
        let emulator = LifePathEmulator::new(pillars.clone(), gender, birth_ctx.corrected_year);
        if let Ok(life_report) = emulator.emulate() {
            let golden_time = Analyzer::find_golden_time(&life_report.timeline, 10);

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

    Ok(SajuAnalysisOutput {
        meta: AnalysisMeta {
            precision: input.precision,
            input_time: birth_ctx.input_time_string,
            corrected_time: birth_ctx.corrected_time_string,
            is_dst: birth_ctx.is_dst,
            dst_offset_hours: birth_ctx.dst_offset_hours,
            analysis_timezone: input.base.timezone,
        },
        report,
        lints,
        entropy,
        qi_topology,
        load_diagnostics,
        crash_count,
        vulnerability_report,
        relationships,
        void_analysis,
        complexity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::AnalysisInput;

    #[test]
    fn test_solar_term_boundary_after_lichun_2024_02_04() {
        // 2024-02-04 17:45 KST in Seoul. Actual Lichun was at 17:27 KST.
        // Longitude correction for Seoul is ~ -32 min (17:13).
        // Year pillar must be Jia-Chen (甲辰) and Month pillar must be Bing-Yin (丙寅).
        let input = SajuAnalysisInput::new(
            AnalysisInput::new(2024, 2, 4, 17, 45, "Asia/Seoul", 37.5665, 126.9780)
                .with_city("Seoul".to_string()),
            true,
            false,
            None,
        );

        let output = analyze(input).expect("Saju analysis should succeed");
        let pillars = output.report.pillars;

        assert_eq!(pillars.year.to_string(), "甲辰");
        assert_eq!(pillars.month.to_string(), "丙寅");
        assert_eq!(pillars.day.to_string(), "戊戌");
        assert_eq!(pillars.hour.to_string(), "辛酉");
    }
}
