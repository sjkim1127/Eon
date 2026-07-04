use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::ganzi::GanZi;
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::core::stem::HeavenlyStem;
use eon_saju::engine::vm::SajuVM;

fn main() {
    println!("=== Eon Precision Audit: Kim Sung-ju (Ansan Case) ===");

    // 김성주님 데이터: 2004/11/27 22:00 안산 (-33분 보정)
    // 보정 전: 22:00 -> 보정 후: 21:27 (시주 boundary 체크)
    let input = SajuInput::new_solar_with_offset(2004, 11, 27, 22, 0, -33);

    let pillars = FourPillars::calculate(&input).expect("Calculation failed");
    println!("\n[1. 사주 원국 (Pillars with Time Correction)]");
    println!("{}", pillars);
    println!("한글: {}", pillars.hangul());

    // 4득 분석 (Strength)
    let strength = pillars.strength();
    println!("\n[2. 4득 분석 (Strength/Weakness Index)]");
    println!("{}", strength);

    // 신살 분석 (Spirit Markers - 12 Sindal)
    let markers = pillars.spirit_markers();
    println!("\n[3. 신살 분석 (12 Spirit Stars)]");
    for m in &markers.markers {
        println!("  - {}", m);
    }

    // VM 에뮬레이션 및 ESIL 트레이스
    let vm = SajuVM::new(pillars.clone());

    // 2026년 대운/세운 (사용자 데이터 기반)
    // 23세 무인대운(예시) 또는 2026 병오년
    let major_luck = GanZi::new(HeavenlyStem::Wu, EarthlyBranch::Yin); // 23 대운
    let year_luck = GanZi::new(HeavenlyStem::Bing, EarthlyBranch::Wu); // 2026 세운

    let frame = vm.step(23, major_luck, year_luck, None, None, None);

    println!("\n[4. VM 에뮬레이션 (ESIL Trace & System Health)]");
    println!("Score: {:.1}", frame.score);
    println!("Tags: {:?}", frame.tags);
    println!("ESIL Trace: {}", frame.esil_trace);

    println!("\n=== Audit Complete ===");
}
