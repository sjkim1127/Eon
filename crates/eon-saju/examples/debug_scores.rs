use eon_core::Gender;
use eon_saju::core::pillars::FourPillars;
use eon_saju::core::pillars::SajuInput;
use eon_saju::engine::emulator::LifePathEmulator;

fn main() {
    let input = SajuInput::new_solar(1990, 1, 1, 12, 0).with_gender(Gender::Male);
    let pillars = FourPillars::calculate(&input).unwrap();
    let emulator = LifePathEmulator::new(pillars.clone(), Gender::Male, 1990);
    let report = emulator.emulate().unwrap();

    println!("Timeline length: {}", report.timeline.len());
    for yr in report.timeline.iter().take(10) {
        println!(
            "Age {:3}: total={:6.2} wealth={:6.2} career={:6.2} academic={:6.2} health={:6.2} vol={:5.2} ma={:?}",
            yr.age, yr.total_score, yr.wealth_score, yr.career_score, yr.academic_score, yr.health_score, yr.volatility_index, yr.trend_ma_5yr
        );
    }
    // Also check ages 25, 50, 75
    for age in [25u32, 50, 75] {
        if let Some(yr) = report.timeline.iter().find(|y| y.age == age) {
            println!(
                "Age {:3}: total={:6.2} wealth={:6.2} career={:6.2} academic={:6.2} health={:6.2}",
                yr.age,
                yr.total_score,
                yr.wealth_score,
                yr.career_score,
                yr.academic_score,
                yr.health_score
            );
        }
    }
}
