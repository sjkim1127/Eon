//! Challenger Verification Suite for Milestone 2 (R2)
//! Empirical stress testing of GaeGo, IpMyo, and Dynamic Gyeokguk in SajuVM.

use eon_saju::analysis::dynamic_luck::{DynamicLuckAnalysis, GyeokStatus};
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::ganzi::GanZi;
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::core::stem::HeavenlyStem;
use eon_saju::engine::vm::SajuVM;

#[test]
fn test_empirical_gaego_unsealing_and_double_scoring() {
    // 1990-10-10 08:00
    let input = SajuInput::new_solar(1990, 10, 10, 8, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!("Natal Pillars: {}", pillars);

    let major_xu = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Xu);
    let yearly_chen = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Chen);

    let dyn_analysis = DynamicLuckAnalysis::analyze(
        &pillars,
        Some(major_xu),
        Some(yearly_chen),
        None,
        None,
        None,
    );

    println!("=== GaeGo Events ===");
    for ev in &dyn_analysis.gaego_events {
        println!(
            "Branch: {}, Position: {}, Trigger: {}, Stems: {:?}",
            ev.branch.hangul(),
            ev.position,
            ev.trigger,
            ev.unsealed_stems
        );
    }

    let vm = SajuVM::new(pillars);
    let frame = vm.step(30, major_xu, yearly_chen, None, None, None);

    println!("=== ESIL Trace ===");
    println!("{}", frame.esil_trace);

    let mem_dump_count = frame.esil_trace.matches("mem_dump:").count();
    let gaego_count = frame.esil_trace.matches("gaego:").count();

    println!(
        "mem_dump count: {}, gaego count: {}",
        mem_dump_count, gaego_count
    );
}

#[test]
fn test_empirical_ipmyo_yin_stem_mismatch() {
    let mut im_pillars = None;
    let mut eul_pillars = None;

    for day in 1..=60 {
        let input = SajuInput::new_solar(1990, 1, day, 12, 0);
        if let Ok(p) = FourPillars::calculate(&input) {
            if p.day_master() == HeavenlyStem::Ren && im_pillars.is_none() {
                im_pillars = Some(p.clone());
            }
            if p.day_master() == HeavenlyStem::Yi && eul_pillars.is_none() {
                eul_pillars = Some(p.clone());
            }
        }
    }

    let pillars_im = im_pillars.expect("Should find Im Day Master");
    let pillars_eul = eul_pillars.expect("Should find Eul Day Master");

    println!("Im Day Master: {}", pillars_im.day_master().hangul());
    println!("Eul Day Master: {}", pillars_eul.day_master().hangul());

    // Test 1: Im (壬 Yang Water) + Chen (辰) Major Luck
    let major_chen = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Chen);
    let dyn_im =
        DynamicLuckAnalysis::analyze(&pillars_im, Some(major_chen), None, None, None, None);

    println!("=== Im (Yang Water) IpMyo Events ===");
    for ev in &dyn_im.ipmyo_events {
        println!(
            "DM: Im(Water), Tomb Branch: {}, Trapped Element: {:?}, Trigger: {}",
            ev.tomb_branch.hangul(),
            ev.element,
            ev.trigger
        );
    }

    // Test 2: Eul (乙 Yin Wood) + Xu (戌) Major Luck
    let major_xu = GanZi::new(HeavenlyStem::Bing, EarthlyBranch::Xu);
    let dyn_eul =
        DynamicLuckAnalysis::analyze(&pillars_eul, Some(major_xu), None, None, None, None);

    println!("=== Eul (Yin Wood) IpMyo Events ===");
    for ev in &dyn_eul.ipmyo_events {
        println!(
            "DM: Eul(Wood), Tomb Branch: {}, Trapped Element: {:?}, Trigger: {}",
            ev.tomb_branch.hangul(),
            ev.element,
            ev.trigger
        );
        assert_eq!(
            ev.element,
            eon_saju::core::element::Element::Wood,
            "Trapped element for Yin Wood DM at Xu must be Wood"
        );
    }
}

#[test]
fn test_empirical_gyeokguk_jiecai_fulfillment_flaw() {
    let mut target_pillars = None;

    for day in 1..=60 {
        let input = SajuInput::new_solar(1984, 4, day, 12, 0);
        if let Ok(p) = FourPillars::calculate(&input) {
            if p.month.branch == EarthlyBranch::Chen {
                target_pillars = Some(p);
                break;
            }
        }
    }

    let pillars = target_pillars.expect("Should find Chen month branch");
    println!("Target Pillars: {}", pillars);
    println!("Day Master: {}", pillars.day_master().hangul());
    println!(
        "Month Branch: {}, Hidden Stems: {:?}",
        pillars.month.branch.hangul(),
        pillars.month.branch.hidden_stems()
    );

    // Major Luck stem: Wu (戊) -> Rob Wealth (劫財) for Ji Day Master, hidden stem of Chen
    let major_wu = GanZi::new(HeavenlyStem::Wu, EarthlyBranch::Zi);
    let dyn_analysis_wu =
        DynamicLuckAnalysis::analyze(&pillars, Some(major_wu), None, None, None, None);

    println!("=== Gyeokguk State with Luck Stem = 戊 (Rob Wealth / 劫財) ===");
    println!("Status: {:?}", dyn_analysis_wu.structure_state.status);
    println!(
        "Description: {}",
        dyn_analysis_wu.structure_state.description
    );

    assert_ne!(
        dyn_analysis_wu.structure_state.status,
        GyeokStatus::Fulfilled,
        "Rob Wealth (겁재) stem MUST NOT trigger Gyeokguk fulfillment"
    );

    // Major Luck stem: Yi (乙) -> Seven Killings (偏官) for Ji Day Master, hidden stem of Chen
    let major_eul = GanZi::new(HeavenlyStem::Yi, EarthlyBranch::Zi);
    let dyn_analysis_eul =
        DynamicLuckAnalysis::analyze(&pillars, Some(major_eul), None, None, None, None);

    println!("=== Gyeokguk State with Luck Stem = 乙 (Seven Killings / 偏官) ===");
    println!("Status: {:?}", dyn_analysis_eul.structure_state.status);
    println!(
        "Description: {}",
        dyn_analysis_eul.structure_state.description
    );

    assert_eq!(
        dyn_analysis_eul.structure_state.status,
        GyeokStatus::Fulfilled,
        "Seven Killings (편관) stem SHOULD trigger Gyeokguk fulfillment"
    );
}
