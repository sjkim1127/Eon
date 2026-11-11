//! Advanced Manseryuk Calendar Test Cases (5 Edge Cases)
//!
//! Verifies the robustness of the Eon engine.

use eon_core::{BirthInfo, Gender, Location};
use eon_saju::{EarthlyBranch, FourPillars, HeavenlyStem, SajuInput};

// ============================================
// Case 1: Lichun boundary line
// ============================================

/// 2024 Lichun boundary test
/// Lichun time: 2024-02-04 16:27 UTC (approx)
/// KST = UTC + 9 = 17:27 immediately before/after
///
/// Note: Calculates solar term exact times via eon-astro engine.
#[test]
fn test_case_1_lichun_boundary_before() {
    // 2024-02-03 - Day before Lichun (Gui-Mao year)
    let input = SajuInput::new_solar(2024, 2, 3, 12, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!(
        "Day before Lichun (2/3): {} {}",
        pillars.year.stem.hanja(),
        pillars.year.branch.hanja()
    );

    // Before Lichun so Gui-Mao year
    assert_eq!(pillars.year.stem, HeavenlyStem::Gui, "Year stem is not Gui");
    assert_eq!(
        pillars.year.branch,
        EarthlyBranch::Mao,
        "Year branch is not Mao"
    );
}

#[test]
fn test_case_1_lichun_boundary_after() {
    // 2024-02-05 - Day after Lichun (Jia-Chen year)
    let input = SajuInput::new_solar(2024, 2, 5, 12, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!(
        "Day after Lichun (2/5): {} {}",
        pillars.year.stem.hanja(),
        pillars.year.branch.hanja()
    );

    // After Lichun so Jia-Chen year
    assert_eq!(pillars.year.stem, HeavenlyStem::Jia, "Year stem is not Jia");
    assert_eq!(
        pillars.year.branch,
        EarthlyBranch::Chen,
        "Year branch is not Chen"
    );
}

// ============================================
// Case 2: Night Rat vs Early Rat Hour
// ============================================

/// Zi hour boundary test - 21-23 is Hai hour
#[test]
fn test_case_2_midnight_boundary_hai() {
    // 21:00 - 22:59 is Hai hour
    let input = SajuInput::new_solar(2023, 1, 1, 21, 30);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!("21:30 Hour pillar: {}", pillars.hour.branch.hanja());

    // 21:00-22:59 is Hai hour
    assert_eq!(
        pillars.hour.branch,
        EarthlyBranch::Hai,
        "21:30 must be Hai hour"
    );
}

#[test]
fn test_case_2_midnight_boundary_zi_early() {
    // Default: use_night_rat_hour = false -> Day pillar advances at 23:00 (Early Rat)
    let input = SajuInput::new_solar(2023, 1, 1, 23, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!(
        "23:00 Hour pillar: {} Day pillar: {}",
        pillars.hour.branch.hanja(),
        pillars.day.branch.hanja()
    );

    // 23:00 is Zi hour
    assert_eq!(
        pillars.hour.branch,
        EarthlyBranch::Zi,
        "23:00 must be Zi hour"
    );
}

#[test]
fn test_case_2_midnight_boundary_zi_late() {
    // 00:01 - definitely Zi hour
    let input = SajuInput::new_solar(2023, 1, 2, 0, 1);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!(
        "00:01 Hour pillar: {} Day pillar: {}",
        pillars.hour.branch.hanja(),
        pillars.day.branch.hanja()
    );

    // 00:00-00:59 is Zi hour
    assert_eq!(
        pillars.hour.branch,
        EarthlyBranch::Zi,
        "00:01 must be Zi hour"
    );
}

/// Night Rat Hour vs Early Rat Hour comparison test
///
/// - Night Rat: 23:00~23:59 maintains current day pillar
/// - Early Rat: 23:00~23:59 advances to next day pillar (Default)
#[test]
fn test_case_2_night_rat_hour_comparison() {
    // 2023-01-01 23:30

    // 1) Early Rat mode (Default) - Day pillar advances to next day
    let jojasiinput = SajuInput::new_solar(2023, 1, 1, 23, 30);
    let jojasi_pillars = FourPillars::calculate(&jojasiinput).unwrap();

    // 2) Night Rat mode - Day pillar maintained
    let yajasi_input = SajuInput::new_solar(2023, 1, 1, 23, 30).with_night_rat_hour(true);
    let yajasi_pillars = FourPillars::calculate(&yajasi_input).unwrap();

    println!("=== 23:30 Night Rat vs Early Rat Comparison ===");
    println!(
        "Early Rat (Default): Day pillar {} Hour pillar {}",
        jojasi_pillars.day.hangul(),
        jojasi_pillars.hour.hangul()
    );
    println!(
        "Night Rat: Day pillar {} Hour pillar {}",
        yajasi_pillars.day.hangul(),
        yajasi_pillars.hour.hangul()
    );

    // Both modes have Zi hour
    assert_eq!(
        jojasi_pillars.hour.branch,
        EarthlyBranch::Zi,
        "Early Rat: Hour pillar Zi"
    );
    assert_eq!(
        yajasi_pillars.hour.branch,
        EarthlyBranch::Zi,
        "Night Rat: Hour pillar Zi"
    );

    // Day pillar differs (Early Rat is one day later)
    assert_ne!(
        jojasi_pillars.day, yajasi_pillars.day,
        "Day pillar must differ between Early/Night Rat at 23:00"
    );

    // Early Rat day pillar is next GanZi
    let yajasi_day_idx = yajasi_pillars.day.index();
    let jojasi_day_idx = jojasi_pillars.day.index();
    assert_eq!(
        (yajasi_day_idx + 1) % 60,
        jojasi_day_idx,
        "Early Rat day pillar must be next GanZi of Night Rat"
    );
}

// ============================================
// Case 3: 1988 Olympic Summer Time (DST)
// ============================================

/// 1988-05-15 13:30 Seoul (DST active)
/// Clock: 13:30 -> DST removal (-1h): 12:30 -> Longitude correction (-30m): approx 12:00 -> Wu hour
#[test]
fn test_case_3_1988_summer_time() {
    let birth = BirthInfo::solar(1988, 5, 15, 13, 30)
        .with_location(Location::seoul())
        .with_timezone("Asia/Seoul")
        .with_true_solar_time(true);

    // Check DST
    println!("1988 DST active: {}", birth.is_dst());
    println!("DST offset: {:?} hours", birth.dst_offset_hours());

    let (y, m, d, h) = birth.for_saju();
    println!(
        "After correction: {} year {} month {} day {} hour",
        y, m, d, h
    );

    let input = SajuInput::new_solar(y, m, d, h, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!("1988 DST Hour pillar: {}", pillars.hour.branch.hanja());

    // 13:30 (Wei) should become 12:00 (Wu) after correction
    // DST -1h + longitude -30m = approx 12:00
    assert_eq!(
        pillars.hour.branch,
        EarthlyBranch::Wu,
        "Must be Wu hour after 1988 DST + longitude correction"
    );
}

/// 1988 Non-DST period (January)
#[test]
fn test_case_3_1988_winter_no_dst() {
    let birth = BirthInfo::solar(1988, 1, 15, 13, 30)
        .with_location(Location::seoul())
        .with_timezone("Asia/Seoul")
        .with_true_solar_time(true);

    // Check DST (none in winter)
    println!("1988 January DST: {}", birth.is_dst());
    assert!(!birth.is_dst(), "1988 January should not have DST");
}

// ============================================
// Case 4: 127E vs 135E (Local Solar Time)
// ============================================

/// Same time, different location -> different Hour pillar
/// 2024-06-01 11:29 - Boundary entering Wu hour
#[test]
fn test_case_4_longitude_ulleungdo() {
    // Ulleungdo (130.8E) - Closer to standard meridian
    let ulleungdo = Location::new("Ulleungdo", 37.48, 130.8, 135.0);

    let birth = BirthInfo::solar(2024, 6, 1, 11, 29)
        .with_location(ulleungdo)
        .with_true_solar_time(true);

    let offset = birth.longitude_offset_minutes();
    println!("Ulleungdo longitude correction: {:+} min", offset);

    let (y, m, d, h) = birth.for_saju();
    let input = SajuInput::new_solar(y, m, d, h, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!(
        "Ulleungdo 11:29 corrected time: {} -> Hour pillar: {}",
        h,
        pillars.hour.branch.hanja()
    );

    // Ulleungdo (130.8E): correction = (130.8 - 135) * 4 = -16.8 min
    // 11:29 - 17m = 11:12 -> still Si hour
}

#[test]
fn test_case_4_longitude_mokpo() {
    // Mokpo (126.3E) - Western location
    let mokpo = Location::new("Mokpo", 34.81, 126.3, 135.0);

    let birth = BirthInfo::solar(2024, 6, 1, 11, 29)
        .with_location(mokpo)
        .with_true_solar_time(true);

    let offset = birth.longitude_offset_minutes();
    println!("Mokpo longitude correction: {:+} min", offset);

    let (y, m, d, h) = birth.for_saju();
    let input = SajuInput::new_solar(y, m, d, h, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!(
        "Mokpo 11:29 corrected time: {} -> Hour pillar: {}",
        h,
        pillars.hour.branch.hanja()
    );

    // Mokpo (126.3E): correction = (126.3 - 135) * 4 = -34.8 min
    // 11:29 - 35m = 10:54 -> still Si hour
}

#[test]
fn test_case_4_longitude_comparison() {
    // Testing divergence hour: 11:45

    // Ulleungdo: 11:45 - 17m = 11:28 -> Si hour
    let birth_u = BirthInfo::solar(2024, 6, 1, 11, 45)
        .with_location(Location::new("Ulleungdo", 37.48, 130.8, 135.0))
        .with_true_solar_time(true);
    let (_, _, _, h_u) = birth_u.for_saju();

    // Mokpo: 11:45 - 35m = 11:10 -> Si hour
    let birth_m = BirthInfo::solar(2024, 6, 1, 11, 45)
        .with_location(Location::new("Mokpo", 34.81, 126.3, 135.0))
        .with_true_solar_time(true);
    let (_, _, _, h_m) = birth_m.for_saju();

    println!("Ulleungdo corrected: {} hour", h_u);
    println!("Mokpo corrected: {} hour", h_m);

    // 12:20 test - divergence possible in this range
    let birth_u2 = BirthInfo::solar(2024, 6, 1, 12, 20)
        .with_location(Location::new("Ulleungdo", 37.48, 130.8, 135.0))
        .with_true_solar_time(true);
    let birth_m2 = BirthInfo::solar(2024, 6, 1, 12, 20)
        .with_location(Location::new("Mokpo", 34.81, 126.3, 135.0))
        .with_true_solar_time(true);

    let (_, _, _, h_u2) = birth_u2.for_saju();
    let (_, _, _, h_m2) = birth_m2.for_saju();

    println!("12:20 Ulleungdo corrected: {} hour", h_u2); // 12:20 - 17 = 12:03 -> Wu hour
    println!("12:20 Mokpo corrected: {} hour", h_m2); // 12:20 - 35 = 11:45 -> Si hour boundary
}

// ============================================
// Case 5: Leap Month
// ============================================

/// 2020-05-23 = Lunar Leap Month 4 Day 1
/// Saju is solar-term based, calculated via solar calendar
#[test]
fn test_case_5_leap_month() {
    // Lunar Leap Month 4 Day 1 = Solar 2020-05-23
    // Saju is based on solar terms, calculated after conversion to solar date

    // Method 1: Direct solar input
    let input = SajuInput::new_solar(2020, 5, 23, 10, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!("2020-05-23 (Lunar Leap Month 4 Day 1)");
    println!("Year pillar: {}", pillars.year);
    println!("Month pillar: {}", pillars.month);
    println!("Day pillar: {}", pillars.day);
    println!("Hour pillar: {}", pillars.hour);

    // Between XiaoMan (5/20) and MangZhong (6/5) -> 4th month (Si month)
    // 2020-05-23 is after XiaoMan (5/20) so Si month
    assert_eq!(
        pillars.month.branch,
        EarthlyBranch::Si,
        "Must be Si month after XiaoMan"
    );

    // 2020 = Geng-Zi year
    assert_eq!(
        pillars.year.stem,
        HeavenlyStem::Geng,
        "2020 Year stem is Geng"
    );
    assert_eq!(
        pillars.year.branch,
        EarthlyBranch::Zi,
        "2020 Year branch is Zi"
    );
}

/// Lunar input test (using BirthInfo::lunar)
#[test]
fn test_case_5_lunar_input() {
    // Verify input info is set properly using BirthInfo::lunar.

    let birth = BirthInfo::lunar(2020, 4, 1, 10, 0, true); // Leap month 4

    println!("Lunar input: {}", birth);
    println!("Calendar type: {:?}", birth.calendar);

    // Currently verifying interface
    // Verifies interface only
    assert!(birth.calendar.is_lunar());
}

// ============================================
// Comprehensive verification: User chart (Kim Sung-ju)
// ============================================

#[test]
fn test_user_saju_complete() {
    // Kim Sung-ju: 2004-11-27 22:00 Ansan
    let birth = BirthInfo::solar(2004, 11, 27, 22, 0)
        .with_location(Location::ansan())
        .with_timezone("Asia/Seoul")
        .with_true_solar_time(true)
        .with_gender(Gender::Male);

    println!("=== Full Verification of Kim Sung-ju Chart ===");
    println!("{}", birth);

    // Check local solar time correction
    let offset = birth.longitude_offset_minutes();
    println!("Ansan longitude correction: {:+} min", offset);
    assert!(offset < 0, "Ansan is west of 135E so negative offset");

    let (y, m, d, h) = birth.for_saju();
    println!(
        "After correction: {} year {} month {} day {} hour",
        y, m, d, h
    );

    let input = SajuInput::new_solar(y, m, d, h, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!("{}", pillars);

    // Verification: Jia-Shen year, Yi-Hai month, Geng-Xu day, Ding-Hai hour
    assert_eq!(pillars.year.stem, HeavenlyStem::Jia, "Year stem Jia");
    assert_eq!(pillars.year.branch, EarthlyBranch::Shen, "Year branch Shen");
    assert_eq!(pillars.month.stem, HeavenlyStem::Yi, "Month stem Yi");
    assert_eq!(pillars.month.branch, EarthlyBranch::Hai, "Month branch Hai");
    assert_eq!(pillars.day.stem, HeavenlyStem::Geng, "Day stem Geng");
    assert_eq!(pillars.day.branch, EarthlyBranch::Xu, "Day branch Xu");
    assert_eq!(pillars.hour.stem, HeavenlyStem::Ding, "Hour stem Ding");
    assert_eq!(pillars.hour.branch, EarthlyBranch::Hai, "Hour branch Hai");
}
#[test]
fn test_case_6_advanced_analysis_refinements() {
    // 1. Clash weighting refinement test
    let cardinal_input = SajuInput::new_solar(2020, 6, 21, 0, 0);
    let _cardinal_strength = FourPillars::calculate(&cardinal_input).unwrap().strength();

    let storage_input = SajuInput::new_solar(2021, 7, 15, 0, 0);
    let storage_strength = FourPillars::calculate(&storage_input).unwrap().strength();

    // Verify label when storage clash (Chou-Wei) occurs
    assert!(storage_strength
        .deuk_ji
        .root_positions
        .iter()
        .any(|p| p.contains("StorageClash")));

    // 2. Strong DM weighting and Eokbu refinement test
    let strong_input = SajuInput::new_solar(1978, 6, 3, 12, 0);
    let s_pillars = FourPillars::calculate(&strong_input).unwrap();
    let s_strength = s_pillars.strength();
    let s_yongshin = s_pillars.yongshin();

    // Verify DeukJi weight sum (Grade A=1.0, B=0.5 etc)
    assert!(s_strength.deuk_ji.stage_weight_sum >= 2.0);

    // Verify Eokbu Yongshin detailed reasons
    let eokbu = s_yongshin
        .recommendations
        .iter()
        .find(|r| r.yongshin_type == eon_saju::analysis::yongshin::YongshinType::Eokbu)
        .expect("Eokbu Yongshin must exist");

    assert!(
        eokbu.summary.contains("UseWealthBreakResource")
            || eokbu.summary.contains("OfficerControlsRob")
            || eokbu.summary.contains("ExhaustionProducesWealth")
            || eokbu
                .reasons
                .iter()
                .any(|r| r.contains("UseWealthBreakResource")
                    || r.contains("OfficerControlsRob")
                    || r.contains("ExhaustionProducesWealth")),
        "Eokbu reason must contain terminology: summary={:?}, reasons={:?}",
        eokbu.summary,
        eokbu.reasons
    );
}

#[test]
fn test_case_7_structure_yongshin_integration() {
    use eon_saju::analysis::structure::StructureType;
    use eon_saju::core::pillars::{FourPillars, SajuInput};

    // 1. Follower structure test case: DM extremely weak, Wealth strong
    // 1995-05-27 12:00 logic test case
    let input = SajuInput::new_solar(1995, 5, 27, 12, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    let structure = pillars.structure();
    let yongshin = pillars.yongshin();

    // If determined as follower structure
    if matches!(
        structure.structure,
        StructureType::JongAh | StructureType::JongJae | StructureType::JongSal
    ) {
        let eokbu = yongshin
            .recommendations
            .iter()
            .find(|r| r.yongshin_type == eon_saju::analysis::yongshin::YongshinType::Eokbu)
            .expect("Follower structures should still recommend Eokbu classification");

        assert!(
            eokbu.summary.contains("Follow") || eokbu.reasons.iter().any(|r| r.contains("Follow")),
            "Reason must contain Follow: summary={:?}, reasons={:?}",
            eokbu.summary,
            eokbu.reasons
        );
        assert!(
            eokbu.summary.contains(structure.structure.hangul())
                || eokbu
                    .reasons
                    .iter()
                    .any(|r| r.contains(structure.structure.hangul())),
            "Reason must contain structure name: summary={:?}, reasons={:?}",
            eokbu.summary,
            eokbu.reasons
        );
    }
}

#[test]
fn test_case_8_transformation_impact() {
    use eon_saju::analysis::strength::StrengthAnalysis;
    use eon_saju::core::pillars::{FourPillars, SajuInput};

    // Jia-Ji combination example
    // 1984-04-15 12:00 example
    // Transformation simulation
    let input = SajuInput::new_solar(1984, 4, 15, 12, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    // 1. Without transformation (base strength)
    let strength_raw = StrengthAnalysis::from_pillars_with_options(
        &pillars,
        false,
        &eon_saju::AnalysisConfig::default(),
    );

    // 2. With transformation (adjusted strength)
    let strength_transformed = StrengthAnalysis::from_pillars_with_options(
        &pillars,
        true,
        &eon_saju::AnalysisConfig::default(),
    );

    println!(
        "Raw Strength: {:.1}%, Transformed Strength: {:.1}%",
        strength_raw.strength_score, strength_transformed.strength_score
    );

    // Outcomes should differ when transformation occurs
    let trans = pillars.transformations();
    let has_stem_transform = trans.year_stem.reason.is_some()
        || trans.month_stem.reason.is_some()
        || trans.day_stem.reason.is_some()
        || trans.hour_stem.reason.is_some();

    if has_stem_transform {
        // Check transformation propagation
        // Verifying propagation logic
        assert!(
            strength_transformed.deuk_se.support_ratio != strength_raw.deuk_se.support_ratio
                || strength_transformed.strength_score != strength_raw.strength_score,
            "Transformation should adjust strength score"
        );
    }
}

#[test]
fn test_case_9_vm_dynamic_combination() {
    use eon_saju::core::ganzi::GanZi;
    use eon_saju::core::pillars::{FourPillars, SajuInput};
    use eon_saju::engine::vm::SajuVM;

    // Natal chart with Shen and Chen
    // 1980-08-11 chart
    let input = SajuInput::new_solar(1980, 8, 11, 12, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    let vm = SajuVM::new(pillars);

    // 1. Year without triple alliance completion
    // 1983 test
    let frame_no_triple = vm.step(
        4,
        GanZi::from_index(0),
        GanZi::from_index(59),
        None,
        None,
        None,
    );

    // 2. Year completing Shen-Zi-Chen alliance (1984)
    // Jia-Zi is index 0
    let frame_triple = vm.step(
        5,
        GanZi::from_index(0),
        GanZi::from_index(0),
        None,
        None,
        None,
    );

    println!(
        "No Triple Score: {:.1}, Triple Score: {:.1}",
        frame_no_triple.score, frame_triple.score
    );

    // Triple combination completion requires tag
    let has_tag = frame_triple
        .tags
        .iter()
        .any(|t| t.contains_pattern("TripleComboCompleted"));
    assert!(
        has_tag,
        "Triple alliance year must include tag. Tags: {:?}",
        frame_triple.tags
    );
}

#[test]
fn test_case_10_vm_parallel_simulation() {
    use eon_saju::core::pillars::{FourPillars, SajuInput};
    use eon_saju::engine::vm::SajuVM;

    let input = SajuInput::new_solar(1984, 4, 15, 12, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    let vm = SajuVM::new(pillars);

    // 100-year simulation via Rayon
    let start = std::time::Instant::now();
    let frames = vm.simulate_life(1, 100);
    let duration = start.elapsed();

    assert_eq!(frames.len(), 100);
    println!("100 years simulation took: {:?}", duration);
}

#[test]
fn test_case_11_vm_simulation_precision() {
    use eon_saju::core::pillars::{FourPillars, SajuInput};
    use eon_saju::engine::vm::SajuVM;

    // Male born 2004-11-27 (Major luck start age 3)
    let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    let vm = SajuVM::new(pillars);

    let frames = vm.simulate_life(1, 20);

    // Major luck before and after start age 3 must differ
    let luck_1 = frames.iter().find(|f| f.age == 1).unwrap().major_ganzi;
    let luck_3 = frames.iter().find(|f| f.age == 3).unwrap().major_ganzi;
    let luck_13 = frames.iter().find(|f| f.age == 13).unwrap().major_ganzi;

    println!(
        "Age 1 Major Luck: {}, Age 3 Major Luck: {}, Age 13 Major Luck: {}",
        luck_1, luck_3, luck_13
    );

    // Before major luck start age
    // Ages 3 and 13 must have different major luck
    assert_ne!(luck_3, luck_13, "Major luck must advance every 10 years");
    assert_ne!(luck_1, luck_3, "Major luck must differ before/after age 3");
}

#[test]
fn test_case_12_vm_void_and_talgong() {
    use eon_saju::core::pillars::{FourPillars, SajuInput};
    use eon_saju::engine::vm::SajuVM;

    // Jia-Zi day pillar void is Xu-Hai
    // 1984-11-20 12:00 -> Jia-Zi day pillar has Xu-Hai void
    let input = SajuInput::new_solar(1984, 11, 20, 12, 0); // Natal chart void condition
    let pillars = FourPillars::calculate(&input).unwrap();
    let vm = SajuVM::new(pillars);

    // 1. Entering void year (Xu/Hai) test
    // 1994 (Jia-Xu) void year test
    use eon_saju::core::branch::EarthlyBranch;
    use eon_saju::core::ganzi::GanZi;
    use eon_saju::core::stem::HeavenlyStem;

    let gap_sul = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Xu); // Jia-Xu (Xu void)
                                                                    // Yin combines with Xu -> causes escaped void
                                                                    // Pure void requires uncombined branches
                                                                    // Complex chart interaction

    // Injecting void GanZi into step method

    // A. Normal year (Wu-Chen)
    let frame_normal = vm.step(
        20,
        GanZi::from_index(0),
        GanZi::from_index(4),
        None,
        None,
        None,
    ); // Wu-Chen

    // B. Void year (Jia-Xu) - Xu is void for Jia-Zi
    let frame_void = vm.step(21, GanZi::from_index(0), gap_sul, None, None, None);

    println!(
        "Normal Score: {:.1}, Void Score: {:.1}",
        frame_normal.score, frame_void.score
    );

    // Void reduces score unless escaped
    // Natal: Zi, Hai, Zi, Wu
    // Branch interaction sweep
    // Semi-fire alliance escapes void
    // Heavenly Net Earthly Trap

    // Verifying void tag attached
    // Verifying void tag attached

    let has_void_tag = frame_void
        .tags
        .iter()
        .any(|t| t.contains_pattern("VoidLuck"));
    let has_escape_tag = frame_void
        .tags
        .iter()
        .any(|t| t.contains_pattern("EscapedVoid"));

    if has_escape_tag {
        println!(
            "Void escaped due to combination/clash. Tags: {:?}",
            frame_void.tags
        );
    } else {
        assert!(has_void_tag, "Void year must have VoidLuck tag");
        assert!(
            frame_void.score < frame_normal.score - 5.0,
            "Score must decrease significantly during void"
        );
    }
}

#[test]
fn test_case_13_vm_shinsal_and_patterns() {
    use eon_saju::core::branch::EarthlyBranch;
    use eon_saju::core::ganzi::GanZi;
    use eon_saju::core::pillars::{FourPillars, SajuInput};
    use eon_saju::core::stem::HeavenlyStem;
    use eon_saju::engine::vm::SajuVM;

    // Chart: Jia-Shen year, Yi-Hai month, Geng-Yin day, Ding-Hai hour
    let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    let vm = SajuVM::new(pillars);

    // 1. Spirit marker test: Yeokmasal
    // Shen year is Yeokmasal for Yin day branch
    let gyeong_shen = GanZi::new(HeavenlyStem::Geng, EarthlyBranch::Shen); // Geng-Shen

    // 2. 12 Life Stages test: Jianlu stage
    // Shen is Jianlu stage for Geng Day Master

    let frame_shen = vm.step(20, GanZi::from_index(0), gyeong_shen, None, None, None);

    println!("Shen Year Tags: {:?}", frame_shen.tags);

    // Verify Yeokmasal
    let has_yeokma = frame_shen
        .tags
        .iter()
        .any(|t| t.contains_pattern("Shinsal:Yeokmasal"));
    assert!(has_yeokma, "Shen year must be Yeokmasal for Yin day branch");

    // Verify 12 Life Stage peak
    let has_geonrok = frame_shen
        .tags
        .iter()
        .any(|t| t.contains_pattern("LifeStage:Jianlu"));
    assert!(
        has_geonrok,
        "Shen year must be Jianlu stage for Geng Day Master"
    );

    // 3. Ten Gods pattern: Hurting Officer controls Officer
    // Direct Officer (Ding Fire) in natal chart.
    // Gui Water (Hurting Officer) forms HurtingOfficerControlsOfficer with Ding Fire
    let gui_you = GanZi::new(HeavenlyStem::Gui, EarthlyBranch::You); // Gui-You
    let frame_gui = vm.step(21, GanZi::from_index(0), gui_you, None, None, None);

    println!("Gui Year Tags: {:?}", frame_gui.tags);

    let has_shangguan_gyeongwan = frame_gui
        .tags
        .iter()
        .any(|t| t.contains_pattern("Pattern:HurtingOfficerControlsOfficer"));
    assert!(
        has_shangguan_gyeongwan,
        "Gui Water meeting Ding Fire must trigger HurtingOfficerControlsOfficer pattern"
    );

    // Check score reduction (HurtingOfficerControlsOfficer is inauspicious)
    let frame_normal = vm.step(
        22,
        GanZi::from_index(0),
        GanZi::from_index(0),
        None,
        None,
        None,
    );
    println!(
        "Gui Score: {:.1}, Normal Score: {:.1}",
        frame_gui.score, frame_normal.score
    );
    assert!(
        frame_gui.score < frame_normal.score,
        "Score must be lower during HurtingOfficerControlsOfficer"
    );
}

#[test]
fn test_milestone2_r2_wolwun_astronomical_alignment() {
    use chrono::{TimeZone, Utc};
    use eon_saju::analysis::periodic_luck::MonthlyLuck;
    use eon_saju::core::branch::EarthlyBranch;

    let dt_before = Utc.with_ymd_and_hms(2026, 2, 2, 10, 0, 0).unwrap();
    let dt_after = Utc.with_ymd_and_hms(2026, 2, 10, 10, 0, 0).unwrap();

    let ganzi_before = MonthlyLuck::month_ganzi_at(dt_before);
    let ganzi_after = MonthlyLuck::month_ganzi_at(dt_after);

    // Feb 2 2026 is still Chou (丑) month of 2025
    assert_eq!(ganzi_before.branch, EarthlyBranch::Chou);
    // Feb 10 2026 is Yin (寅) month of 2026
    assert_eq!(ganzi_after.branch, EarthlyBranch::Yin);
}

#[test]
fn test_milestone2_r2_dynamic_precedence_hierarchy() {
    use eon_saju::analysis::dynamic_luck::DynamicLuckAnalysis;
    use eon_saju::core::branch::EarthlyBranch;
    use eon_saju::core::ganzi::GanZi;
    use eon_saju::core::pillars::{FourPillars, SajuInput};
    use eon_saju::core::stem::HeavenlyStem;

    let input = SajuInput::new_solar(1992, 12, 15, 14, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    let major_chen = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Chen);
    let yearly_yin = GanZi::new(HeavenlyStem::Bing, EarthlyBranch::Yin);

    let dyn_analysis = DynamicLuckAnalysis::analyze(
        &pillars,
        Some(major_chen),
        Some(yearly_yin),
        None,
        None,
        None,
    );

    assert!(!dyn_analysis
        .combined_relations
        .triple_combinations
        .is_empty());

    let has_yin_shen_clash = dyn_analysis
        .combined_relations
        .branch_clashes
        .iter()
        .any(|(c, _, _)| *c == eon_saju::analysis::relationships::BranchClash::YinShen);
    assert!(
        !has_yin_shen_clash,
        "Yin-Shen clash involving Shen must be suppressed by Triple Alliance"
    );
}

#[test]
fn test_milestone2_r2_augmented_transformation_analysis() {
    use eon_saju::analysis::transformations::TransformationAnalysis;
    use eon_saju::core::branch::EarthlyBranch;
    use eon_saju::core::ganzi::GanZi;
    use eon_saju::core::pillars::{FourPillars, SajuInput};
    use eon_saju::core::stem::HeavenlyStem;

    let input = SajuInput::new_solar(1984, 5, 20, 10, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    let major = Some(GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi));
    let yearly = Some(GanZi::new(HeavenlyStem::Bing, EarthlyBranch::Chen));

    let trans_exp = TransformationAnalysis::from_expanded(&pillars, major, yearly);
    assert!(trans_exp.major_stem.is_some());
    assert!(trans_exp.saewun_branch.is_some());

    let power_exp = pillars.integrated_analysis_expanded(
        major,
        yearly,
        Default::default(),
        &Default::default(),
    );
    assert!(!power_exp.element_scores.is_empty());
}

#[test]
fn test_milestone2_r2_jijanggan_gaego_and_ipmyo() {
    use eon_saju::analysis::dynamic_luck::DynamicLuckAnalysis;
    use eon_saju::core::branch::EarthlyBranch;
    use eon_saju::core::ganzi::GanZi;
    use eon_saju::core::pillars::{FourPillars, SajuInput};
    use eon_saju::core::stem::HeavenlyStem;
    use eon_saju::engine::vm::SajuVM;

    let input = SajuInput::new_solar(1990, 10, 10, 8, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    let major_xu = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Xu);
    let yearly_chen = GanZi::new(HeavenlyStem::Bing, EarthlyBranch::Chen);

    let dyn_analysis = DynamicLuckAnalysis::analyze(
        &pillars,
        Some(major_xu),
        Some(yearly_chen),
        None,
        None,
        None,
    );
    assert!(
        !dyn_analysis.gaego_events.is_empty(),
        "Chen-Xu clash on storage branch must trigger GaeGo event"
    );

    let vm = SajuVM::new(pillars);
    let frame = vm.step(30, major_xu, yearly_chen, None, None, None);

    let has_gaego_tag = frame.tags.iter().any(|t| t.to_string().contains("GaeGo:"));
    assert!(
        has_gaego_tag,
        "VM execution must record GaeGo tag in life frame"
    );
}

#[test]
fn test_milestone2_r2_dynamic_gyeokguk_state_transitions() {
    use eon_saju::analysis::dynamic_luck::{DynamicLuckAnalysis, GyeokStatus};
    use eon_saju::core::branch::EarthlyBranch;
    use eon_saju::core::ganzi::GanZi;
    use eon_saju::core::pillars::{FourPillars, SajuInput};
    use eon_saju::core::stem::HeavenlyStem;
    use eon_saju::engine::vm::SajuVM;

    let input = SajuInput::new_solar(1995, 3, 15, 12, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    let major_gz = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Hai);
    let yearly_gz = GanZi::new(HeavenlyStem::Bing, EarthlyBranch::Wei);

    let dyn_analysis =
        DynamicLuckAnalysis::analyze(&pillars, Some(major_gz), Some(yearly_gz), None, None, None);
    assert_ne!(dyn_analysis.structure_state.status, GyeokStatus::Stable);

    let vm = SajuVM::new(pillars);
    let frame = vm.step(25, major_gz, yearly_gz, None, None, None);
    let has_gyeok_tag = frame
        .tags
        .iter()
        .any(|t| t.to_string().contains("Structure:"));
    assert!(
        has_gyeok_tag,
        "VM frame should include dynamic Gyeokguk tag"
    );
}

// ============================================================================
// Milestone 3 (R3): Comprehensive Edge-Case & Fuzzer Verification Suite
// ============================================================================

fn make_pillars(
    y_s: eon_saju::HeavenlyStem,
    y_b: eon_saju::EarthlyBranch,
    m_s: eon_saju::HeavenlyStem,
    m_b: eon_saju::EarthlyBranch,
    d_s: eon_saju::HeavenlyStem,
    d_b: eon_saju::EarthlyBranch,
    h_s: eon_saju::HeavenlyStem,
    h_b: eon_saju::EarthlyBranch,
) -> eon_saju::FourPillars {
    use chrono::{TimeZone, Utc};
    use eon_core::Gender;
    use eon_saju::{GanZi, SajuInput};
    let dummy_input = SajuInput::new_solar(1990, 1, 1, 12, 0);
    eon_saju::FourPillars {
        year: GanZi::new(y_s, y_b),
        month: GanZi::new(m_s, m_b),
        day: GanZi::new(d_s, d_b),
        hour: GanZi::new(h_s, h_b),
        birth_time: Utc.with_ymd_and_hms(1990, 1, 1, 12, 0, 0).unwrap(),
        gender: Gender::Male,
        raw_input: dummy_input,
        supplementary_pillars: Default::default(),
    }
}

/// a. JeonWang 5-Pattern (GokJik, YeomSang, GaSaek, JongHyeok, YoonHa) charts
#[test]
fn test_m3_jeonwang_5_gyeok_patterns() {
    use eon_saju::analysis::structure::StructureType;
    use eon_saju::core::branch::EarthlyBranch;
    use eon_saju::core::element::Element;
    use eon_saju::core::stem::HeavenlyStem;

    // 1. GokJik (Wood JeonWang): Jia/Yi DM, Mao Month Branch, Heavy Wood
    let gokjik_pillars = make_pillars(
        HeavenlyStem::Gui,
        EarthlyBranch::Hai,
        HeavenlyStem::Yi,
        EarthlyBranch::Mao,
        HeavenlyStem::Jia,
        EarthlyBranch::Yin,
        HeavenlyStem::Gui,
        EarthlyBranch::Hai,
    );
    let struct_gokjik = gokjik_pillars.structure();
    assert_eq!(
        struct_gokjik.structure,
        StructureType::GokJik,
        "Must identify GokJik"
    );
    let yong_gokjik = gokjik_pillars.yongshin();
    assert!(
        yong_gokjik
            .recommendations
            .iter()
            .any(|r| r.element == Element::Wood),
        "GokJik recommended element must be Wood"
    );

    // 2. YeomSang (Fire JeonWang): Bing/Ding DM, Wu Month Branch, Heavy Fire
    let yeomsang_pillars = make_pillars(
        HeavenlyStem::Yi,
        EarthlyBranch::Si,
        HeavenlyStem::Ding,
        EarthlyBranch::Wu,
        HeavenlyStem::Bing,
        EarthlyBranch::Wu,
        HeavenlyStem::Jia,
        EarthlyBranch::Wu,
    );
    let struct_yeomsang = yeomsang_pillars.structure();
    assert_eq!(
        struct_yeomsang.structure,
        StructureType::YeomSang,
        "Must identify YeomSang"
    );
    let yong_yeomsang = yeomsang_pillars.yongshin();
    assert!(
        yong_yeomsang
            .recommendations
            .iter()
            .any(|r| r.element == Element::Fire),
        "YeomSang recommended element must be Fire"
    );

    // 3. GaSaek (Earth JeonWang): Wu/Ji DM, Wei Month Branch, Heavy Earth
    let gasaek_pillars = make_pillars(
        HeavenlyStem::Wu,
        EarthlyBranch::Chen,
        HeavenlyStem::Ji,
        EarthlyBranch::Wei,
        HeavenlyStem::Wu,
        EarthlyBranch::Xu,
        HeavenlyStem::Ji,
        EarthlyBranch::Chou,
    );
    let struct_gasaek = gasaek_pillars.structure();
    assert_eq!(
        struct_gasaek.structure,
        StructureType::GaSaek,
        "Must identify GaSaek"
    );
    let yong_gasaek = gasaek_pillars.yongshin();
    assert!(
        yong_gasaek
            .recommendations
            .iter()
            .any(|r| r.element == Element::Earth),
        "GaSaek recommended element must be Earth"
    );

    // 4. JongHyeok (Metal JeonWang): Geng/Xin DM, You Month Branch, Heavy Metal
    let jonghyeok_pillars = make_pillars(
        HeavenlyStem::Ji,
        EarthlyBranch::Si,
        HeavenlyStem::Xin,
        EarthlyBranch::You,
        HeavenlyStem::Geng,
        EarthlyBranch::Shen,
        HeavenlyStem::Ji,
        EarthlyBranch::Si,
    );
    let struct_jonghyeok = jonghyeok_pillars.structure();
    assert_eq!(
        struct_jonghyeok.structure,
        StructureType::JongHyeok,
        "Must identify JongHyeok"
    );
    let yong_jonghyeok = jonghyeok_pillars.yongshin();
    assert!(
        yong_jonghyeok
            .recommendations
            .iter()
            .any(|r| r.element == Element::Metal),
        "JongHyeok recommended element must be Metal"
    );

    // 5. YoonHa (Water JeonWang): Ren/Gui DM, Zi Month Branch, Heavy Water
    let yoonha_pillars = make_pillars(
        HeavenlyStem::Ren,
        EarthlyBranch::Shen,
        HeavenlyStem::Ren,
        EarthlyBranch::Zi,
        HeavenlyStem::Gui,
        EarthlyBranch::Hai,
        HeavenlyStem::Gui,
        EarthlyBranch::Chou,
    );
    let struct_yoonha = yoonha_pillars.structure();
    assert_eq!(
        struct_yoonha.structure,
        StructureType::YoonHa,
        "Must identify YoonHa"
    );
    let yong_yoonha = yoonha_pillars.yongshin();
    assert!(
        yong_yoonha
            .recommendations
            .iter()
            .any(|r| r.element == Element::Water),
        "YoonHa recommended element must be Water"
    );
}

/// b. GwanSalHonJab 官殺混雜 patterns and clear/control logic
#[test]
fn test_m3_gwansalhonjab_and_clear_logic() {
    use eon_saju::analysis::dynamic_luck::DynamicLuckAnalysis;
    use eon_saju::analysis::structure::StructureType;
    use eon_saju::core::branch::EarthlyBranch;
    use eon_saju::core::ganzi::GanZi;
    use eon_saju::core::stem::HeavenlyStem;

    // 甲木 Day Master with both 庚金 (Seven Killings 偏官) and 辛金 (Direct Officer 正官) exposed in stems
    // Year: 庚申, Month: 辛巳, Day: 甲寅, Hour: 丙寅
    let pillars = make_pillars(
        HeavenlyStem::Geng,
        EarthlyBranch::Shen,
        HeavenlyStem::Xin,
        EarthlyBranch::Si,
        HeavenlyStem::Jia,
        EarthlyBranch::Yin,
        HeavenlyStem::Bing,
        EarthlyBranch::Yin,
    );

    let struct_res = pillars.structure();
    assert_eq!(
        struct_res.structure,
        StructureType::GwanSalHonJab,
        "Exposed Geng (PianGuan) and Xin (ZhengGuan) must form GwanSalHonJab"
    );

    // Test Dynamic Luck interaction: When luck brings 乙木 (Rob Wealth 劫財),
    // 乙庚合 (Stem Combination) combines with 庚 (Seven Killings), clearing the killing (去殺留官)!
    let major_yi = GanZi::new(HeavenlyStem::Yi, EarthlyBranch::Mao);
    let dyn_analysis =
        DynamicLuckAnalysis::analyze(&pillars, Some(major_yi), None, None, None, None);

    let has_stem_combo = !dyn_analysis.combined_relations.stem_combinations.is_empty();
    assert!(
        has_stem_combo,
        "Stem combination with luck stem must resolve or combine exposed killing"
    );
}

/// c. Void & Tianyi Noble annulment under clash or Void
#[test]
fn test_m3_void_and_tianyi_noble_annulment() {
    use eon_saju::analysis::shinsal::Gilsin;
    use eon_saju::analysis::void::VoidAnalysis;
    use eon_saju::core::branch::EarthlyBranch;
    use eon_saju::core::ganzi::GanZi;
    use eon_saju::core::stem::HeavenlyStem;
    use eon_saju::engine::vm::SajuVM;

    // Day: 甲寅 (Void: 子, 丑)
    // Day Master 甲 has Tianyi Nobles on 丑 and 未.
    // Year: 癸丑 (Branch 丑 is a Tianyi Noble for 甲, BUT 丑 is in Void for 甲寅 day!)
    let pillars = make_pillars(
        HeavenlyStem::Gui,
        EarthlyBranch::Chou,
        HeavenlyStem::Bing,
        EarthlyBranch::Chen,
        HeavenlyStem::Jia,
        EarthlyBranch::Yin,
        HeavenlyStem::Ding,
        EarthlyBranch::Mao,
    );

    // 1. Verify Tianyi Noble calculation returns Chou & Wei for Jia
    let nobles = Gilsin::cheoneul_branches(HeavenlyStem::Jia);
    assert!(nobles.contains(&EarthlyBranch::Chou));

    // 2. Verify Void analysis detects Chou in void_branches
    let void_res = VoidAnalysis::from_pillars(&pillars);
    assert!(
        void_res.void_branches.contains(&EarthlyBranch::Chou),
        "丑 must be in Void for 甲寅 day"
    );

    // 3. Test VM step when luck branch is 丑 (Void Tianyi Noble) vs 未 (Clashing Tianyi Noble)
    let vm = SajuVM::new(pillars);
    let major_chou = GanZi::new(HeavenlyStem::Yi, EarthlyBranch::Chou);
    let yearly_yin = GanZi::new(HeavenlyStem::Bing, EarthlyBranch::Yin);
    let frame_chou = vm.step(20, major_chou, yearly_yin, None, None, None);

    let has_void_tag = frame_chou
        .tags
        .iter()
        .any(|t| t.to_string().contains("Void"));
    assert!(
        has_void_tag,
        "Entering Void noble branch must record Void tag"
    );

    // Luck branch Chou in Void, clashed by Yearly Luck Wei (Clash unseals Void -> EscapedVoid)
    let yearly_wei = GanZi::new(HeavenlyStem::Ji, EarthlyBranch::Wei);
    let frame_wei = vm.step(30, major_chou, yearly_wei, None, None, None);
    let has_escape_tag = frame_wei
        .tags
        .iter()
        .any(|t| t.to_string().contains("EscapedVoid"));
    assert!(
        has_escape_tag,
        "Clash on Void branch must trigger Escaped Void"
    );
}

/// d. Daewun Triple/Seasonal Alliance dynamic transformation and Gyeokguk shifting
#[test]
fn test_m3_daewun_alliance_and_gyeokguk_shifting() {
    use eon_saju::analysis::dynamic_luck::{DynamicLuckAnalysis, GyeokStatus};
    use eon_saju::core::branch::EarthlyBranch;
    use eon_saju::core::ganzi::GanZi;
    use eon_saju::core::stem::HeavenlyStem;
    use eon_saju::engine::vm::SajuVM;

    // Natal chart with 申 and 辰 (missing 子 for 申子辰 Triple Alliance)
    let pillars = make_pillars(
        HeavenlyStem::Geng,
        EarthlyBranch::Shen,
        HeavenlyStem::Geng,
        EarthlyBranch::Chen,
        HeavenlyStem::Jia,
        EarthlyBranch::Yin,
        HeavenlyStem::Bing,
        EarthlyBranch::Yin,
    );

    // Base structure status without luck
    let base_dyn = DynamicLuckAnalysis::analyze(&pillars, None, None, None, None, None);
    assert_eq!(base_dyn.structure_state.status, GyeokStatus::Stable);

    // Daewun brings 子 (completes 申子辰 Water Triple Alliance!)
    let major_zi = GanZi::new(HeavenlyStem::Ren, EarthlyBranch::Zi);
    let dyn_alliance =
        DynamicLuckAnalysis::analyze(&pillars, Some(major_zi), None, None, None, None);

    assert_eq!(
        dyn_alliance.structure_state.status,
        GyeokStatus::Transformed,
        "Completing Triple Alliance via Daewun must set GyeokStatus::Transformed"
    );

    let vm = SajuVM::new(pillars);
    let frame = vm.step(35, major_zi, GanZi::from_index(0), None, None, None);
    let has_triple_tag = frame
        .tags
        .iter()
        .any(|t| t.to_string().contains("TripleComboCompleted"));
    assert!(
        has_triple_tag,
        "VM frame must include Triple Combination tag when completed"
    );
}

/// e. Jijanggan GaeGo & IpMyo events across all 10 Heavenly Stems and Day Master polarities
#[test]
fn test_m3_jijanggan_gaego_and_ipmyo_all_10_stems() {
    use eon_saju::analysis::dynamic_luck::DynamicLuckAnalysis;
    use eon_saju::core::branch::EarthlyBranch;
    use eon_saju::core::element::Element;
    use eon_saju::core::ganzi::GanZi;
    use eon_saju::core::stem::HeavenlyStem;
    use eon_saju::engine::vm::SajuVM;

    let all_stems = [
        HeavenlyStem::Jia,
        HeavenlyStem::Yi,
        HeavenlyStem::Bing,
        HeavenlyStem::Ding,
        HeavenlyStem::Wu,
        HeavenlyStem::Ji,
        HeavenlyStem::Geng,
        HeavenlyStem::Xin,
        HeavenlyStem::Ren,
        HeavenlyStem::Gui,
    ];

    let tombs = [
        (EarthlyBranch::Chen, EarthlyBranch::Xu, Element::Water),
        (EarthlyBranch::Xu, EarthlyBranch::Chen, Element::Fire),
        (EarthlyBranch::Chou, EarthlyBranch::Wei, Element::Metal),
        (EarthlyBranch::Wei, EarthlyBranch::Chou, Element::Wood),
    ];

    for &stem in &all_stems {
        for &(tomb, clashing_branch, trapped_el) in &tombs {
            let pillars = make_pillars(
                HeavenlyStem::Jia,
                tomb,
                HeavenlyStem::Bing,
                EarthlyBranch::Si,
                stem,
                EarthlyBranch::Si,
                HeavenlyStem::Ding,
                EarthlyBranch::Si,
            );

            let clashing_luck = GanZi::new(HeavenlyStem::Geng, clashing_branch);
            let dyn_res =
                DynamicLuckAnalysis::analyze(&pillars, Some(clashing_luck), None, None, None, None);

            // 1. Verify GaeGo events on storage clash
            assert!(
                !dyn_res.gaego_events.is_empty(),
                "Clash on storage branch {:?} by {:?} for stem {:?} must produce GaeGo event",
                tomb,
                clashing_branch,
                stem
            );

            let event = &dyn_res.gaego_events[0];
            assert_eq!(event.branch, tomb);
            assert!(
                !event.unsealed_stems.is_empty(),
                "Unsealed stems must not be empty"
            );

            // 2. Verify SajuVM handles GaeGo and IpMyo without panic
            let vm = SajuVM::new(pillars);
            let frame = vm.step(25, clashing_luck, GanZi::from_index(0), None, None, None);
            assert!(
                frame.score >= 0.0 && frame.score <= 100.0,
                "Score must be bounded"
            );
            assert!(
                !trapped_el.hangul().is_empty(),
                "Trapped element hangul valid"
            );
        }
    }
}

/// f. Property-based fuzzing / automated random chart generation (1,000+ charts)
#[test]
fn test_m3_fuzz_1000_random_charts_robustness() {
    use eon_saju::analysis::dynamic_luck::DynamicLuckAnalysis;
    use eon_saju::core::ganzi::GanZi;
    use eon_saju::core::pillars::{FourPillars, SajuInput};
    use eon_saju::engine::vm::SajuVM;

    // Simple deterministic LCG random number generator (seed-based, zero external dependencies)
    let mut rng_state: u64 = 0xdeadbeef12345678;
    let mut next_u32 = || {
        rng_state = rng_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (rng_state >> 32) as u32
    };

    let total_charts = 1000;
    for idx in 0..total_charts {
        let year = 1900 + ((next_u32() % 150) as i32); // 1900..2050
        let month = 1 + (next_u32() % 12); // 1..12
        let day = 1 + (next_u32() % 28); // 1..28
        let hour = next_u32() % 24; // 0..23
        let minute = next_u32() % 60; // 0..59
        let use_dst = (next_u32() % 2) == 1;

        let input =
            SajuInput::new_solar(year, month, day, hour, minute).with_night_rat_hour(use_dst);

        let pillars = match FourPillars::calculate(&input) {
            Ok(p) => p,
            Err(_) => continue, // Skip invalid dates if any
        };

        // Random luck parameters
        let major_gz = GanZi::from_index((next_u32() % 60) as i32);
        let yearly_gz = GanZi::from_index((next_u32() % 60) as i32);
        let monthly_gz = GanZi::from_index((next_u32() % 60) as i32);

        // 1. Static analyses
        let _strength = pillars.strength();
        let _structure = pillars.structure();
        let _yongshin = pillars.yongshin();
        let _integrated = pillars.integrated_analysis(Default::default(), &Default::default());

        // 2. Dynamic Luck Analysis
        let _dyn_res = DynamicLuckAnalysis::analyze(
            &pillars,
            Some(major_gz),
            Some(yearly_gz),
            Some(monthly_gz),
            None,
            None,
        );

        // 3. VM Simulation & Energy Register Invariant Verification
        let vm = SajuVM::new(pillars.clone());
        let frame = vm.step(
            20 + (idx % 60),
            major_gz,
            yearly_gz,
            Some(monthly_gz),
            None,
            None,
        );

        // Invariant 1: Score must be finite and bounded [0.0, 100.0]
        assert!(
            !frame.score.is_nan() && !frame.score.is_infinite(),
            "Chart #{}: Score must be finite",
            idx
        );
        assert!(
            frame.score >= 0.0 && frame.score <= 100.0,
            "Chart #{}: Score must be bounded in [0, 100], got {}",
            idx,
            frame.score
        );

        // Invariant 2: Normalized QiRegisters must sum to 100.0% (within 0.5% floating point tolerance)
        let reg = &frame.register_state;
        let sum = reg.r0_wood + reg.r1_fire + reg.r2_earth + reg.r3_metal + reg.r4_water;
        assert!(
            (sum - 100.0).abs() < 0.5,
            "Chart #{}: Normalized QiRegisters sum must be ~100%, got {}",
            idx,
            sum
        );

        // Invariant 3: simulate_life must execute 10 frames without error
        let frames = vm.simulate_life(1, 10);
        assert_eq!(frames.len(), 10, "simulate_life must produce 10 frames");
    }
}
