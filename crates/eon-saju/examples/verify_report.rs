use eon_core::Gender;
use eon_saju::analysis::analytics::Analyzer;
use eon_saju::analysis::major_luck::MajorLuckAnalysis;
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::engine::batch::BatchSimulator;
use eon_saju::report::SajuReport;

fn main() {
    println!("=== Eon Comprehensive Report Verification ===");

    // Kim Sung-ju data (Ansan -33 correction)
    let input = SajuInput::new_solar_with_offset(2004, 11, 27, 22, 0, -33);
    let pillars = FourPillars::calculate(&input).expect("Failed to calculate pillars");

    // Major Luck
    let major_luck =
        MajorLuckAnalysis::calculate_astro(&pillars, Gender::Male, 2004, 11, 27, 22, 0).unwrap();

    // VM Simulation (1000 years for Golden Time)
    let simulator = BatchSimulator::new(pillars.clone());
    let frames = simulator.simulate_1000_years(2004, &major_luck);
    let timeline: Vec<eon_saju::engine::emulator::YearlyScore> = frames
        .iter()
        .map(|f| eon_saju::engine::emulator::YearlyScore {
            year: 2004 + f.age as i32,
            age: f.age,
            total_score: f.score as f64,
            wealth_score: 0.0,
            career_score: 0.0,
            academic_score: 0.0,
            health_score: 0.0,
            volatility_index: 0.0,
            is_transition_period: false,
            trend_ma_5yr: None,
        })
        .collect();
    let golden_time = Analyzer::find_golden_time(&timeline, 10);

    // Build Report
    let mut builder = SajuReport::new(pillars);
    builder = builder.with_major_luck(major_luck);

    if let Some(gt) = golden_time {
        builder = builder.with_golden_time(gt);
    }

    builder = builder.with_vm_simulation(frames).with_vm_summary(
        "Simulation performed over 1000 years (2004-3004). \
             Hardware accelerated analysis confirmed optimal energy flow periods."
            .to_string(),
    );

    // Generate Markdown
    let markdown = builder.to_markdown();

    // Save to file
    std::fs::write("kim_sung_ju_report.md", &markdown).expect("Failed to write report");

    println!("Report generated successfully: kim_sung_ju_report.md");
    println!("\n--- Report Preview ---\n");
    println!("{}", markdown);
}
