use eon_core::Gender;
use eon_saju::analysis::analytics::Analyzer;
use eon_saju::analysis::major_luck::MajorLuckAnalysis;
use eon_saju::core::pillars::FourPillars;
use eon_saju::core::pillars::SajuInput;
use eon_saju::engine::emulator::LifePathEmulator;
use eon_saju::engine::load_balancer::KarmaLoadBalancer;
use eon_saju::report::SajuReport;

fn main() {
    let gender = Gender::Male;
    let (cy, cm, cd, ch, cmin) = (1990, 5, 10, 12, 0);
    let input = SajuInput::new_solar(cy, cm, cd, ch, cmin).with_gender(gender);
    let pillars = FourPillars::calculate(&input).unwrap();

    let mut report = SajuReport::new(pillars.clone());
    let major_luck =
        MajorLuckAnalysis::calculate_astro(&pillars, gender, cy, cm, cd, ch, cmin).unwrap();

    let emulator = LifePathEmulator::new(pillars.clone(), gender, cy);
    if let Ok(life_report) = emulator.emulate() {
        let golden_time = Analyzer::find_golden_time(&life_report.timeline, 10);
        let _load_diagnostics = KarmaLoadBalancer::diagnose(&life_report.frames);

        report = report
            .with_major_luck(major_luck)
            .with_timeline(life_report.timeline)
            .with_vm_simulation(life_report.frames);

        if let Some(gt) = golden_time {
            report = report.with_golden_time(gt);
        }
    }

    println!("Timeline Length: {}", report.timeline.len());
}
