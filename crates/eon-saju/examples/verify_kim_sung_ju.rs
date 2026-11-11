use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::ganzi::GanZi;
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::core::stem::HeavenlyStem;
use eon_saju::engine::vm::SajuVM;

fn main() {
    println!("=== Eon Precision Audit: Kim Sung-ju (Ansan Case) ===");

    // Kim Sung-ju chart: 2004/11/27 22:00 Ansan (-33m correction)
    // Before correction: 22:00 -> After correction: 21:27 (Hour pillar boundary check)
    let input = SajuInput::new_solar_with_offset(2004, 11, 27, 22, 0, -33);

    let pillars = FourPillars::calculate(&input).expect("Calculation failed");
    println!("\n[1. Four Pillars (with Time Correction)]");
    println!("{}", pillars);
    println!("Romanized: {}", pillars.hangul());

    // Four Criteria (Strength)
    let strength = pillars.strength();
    println!("\n[2. Four Criteria (Strength/Weakness Index)]");
    println!("{}", strength);

    // Spirit Markers (12 Shinsal)
    let markers = pillars.spirit_markers();
    println!("\n[3. Spirit Markers (12 Shinsal)]");
    for m in &markers.markers {
        println!("  - {}", m);
    }

    // VM emulation and ESIL trace
    let vm = SajuVM::new(pillars.clone());

    // 2026 Major/Yearly Luck
    // Age 23 Wu-Yin major luck, 2026 Bing-Wu yearly luck
    let major_luck = GanZi::new(HeavenlyStem::Wu, EarthlyBranch::Yin); // Age 23 major luck
    let year_luck = GanZi::new(HeavenlyStem::Bing, EarthlyBranch::Wu); // 2026 yearly luck

    let frame = vm.step(23, major_luck, year_luck, None, None, None);

    println!("\n[4. VM Emulation (ESIL Trace & System Health)]");
    println!("Score: {:.1}", frame.score);
    println!("Tags: {:?}", frame.tags);
    println!("ESIL Trace: {}", frame.esil_trace);

    println!("\n=== Audit Complete ===");
}
