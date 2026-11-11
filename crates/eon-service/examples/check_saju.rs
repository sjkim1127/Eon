use eon_service::dto::{AnalysisInput, SajuAnalysisInput};
use eon_service::facade;

fn main() {
    let base = AnalysisInput::new(1990, 5, 15, 10, 0, "Asia/Seoul", 37.5665, 126.9780);

    let input = SajuAnalysisInput::new(
        base,
        true,        // is_male
        false,       // use_night_rat_hour
        Some(false), // unknown_time
    );

    let result = facade::analyze_saju(input).expect("Failed to analyze saju");

    println!("=== Saju Chart (May 15, 1990, 10:00 AM Seoul) ===");
    println!("Year: {:?}", result.report.pillars.year);
    println!("Month: {:?}", result.report.pillars.month);
    println!("Day: {:?}", result.report.pillars.day);
    println!("Hour: {:?}", result.report.pillars.hour);
    if let Some(ml) = result.report.major_luck {
        println!("Major Luck Direction: {}", ml.direction);
        println!("Start Age: {}", ml.start_age);
    }
}
