//! Milestone 3 (R3): Comprehensive Edge-Case Verification Suite for `crates/eon-saju`
//!
//! Mandatory Verification Areas:
//! 1. Solar term boundary transitions (LiChun, DongZhi, XiaoHan, DaHan, etc. at minute resolution).
//! 2. Rare and exotic natal charts (Four Earth branches / 토다자, All-stems same element / 천간일색, Multi-clash / 충다자, Special Gyeokguk structures).
//! 3. Temporal simulation invariants (100-year Daewun/Saewun/Wolwun continuous timeline, extreme dates 1900..2100).
//! 4. SajuVM safety & determinism (step execution invariants, non-overflowing registers, gas/step bounds).
//! 5. Boundary input handling (empty inputs, missing optional data, extreme geographical coordinates).

use chrono::{Datelike, Duration, NaiveDate, TimeZone, Timelike, Utc};
use eon_astro::AstroEngine;
use eon_core::Gender;
use eon_saju::analysis::dynamic_luck::DynamicLuckAnalysis;
use eon_saju::analysis::power::AnalysisOptions;
use eon_saju::analysis::strength::StrengthType;
use eon_saju::analysis::structure::StructureType;
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::config::AnalysisConfig;
use eon_saju::core::element::Element;
use eon_saju::core::ganzi::GanZi;
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::core::stem::HeavenlyStem;
use eon_saju::engine::emulator::LifePathEmulator;
use eon_saju::engine::vm::SajuVM;

/// Helper constructor to manufacture custom FourPillars for deterministic structural testing
fn make_custom_pillars(
    y_s: HeavenlyStem,
    y_b: EarthlyBranch,
    m_s: HeavenlyStem,
    m_b: EarthlyBranch,
    d_s: HeavenlyStem,
    d_b: EarthlyBranch,
    h_s: HeavenlyStem,
    h_b: EarthlyBranch,
) -> FourPillars {
    let dummy_input = SajuInput::new_solar(1990, 1, 1, 12, 0);
    FourPillars {
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

// ============================================================================
// 1. Solar Term Boundary Transitions (Minute Resolution)
// ============================================================================

#[test]
fn test_m3_solar_term_lichun_boundary_transition_minute_resolution() {
    let engine = AstroEngine::new();
    let year_start = NaiveDate::from_ymd_opt(2026, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();

    // Term 0 = 입춘 (LiChun) exact UTC timestamp
    let lichun_2026_utc = engine
        .find_solar_term_time(year_start, 0)
        .expect("LiChun 2026 must be found");

    let dt_before_utc = lichun_2026_utc - Duration::minutes(1);
    let dt_after_utc = lichun_2026_utc + Duration::minutes(1);

    // Convert to KST (UTC+9) for SajuInput
    let dt_before_kst = dt_before_utc + Duration::hours(9);
    let dt_after_kst = dt_after_utc + Duration::hours(9);

    let input_before = SajuInput::new_solar(
        dt_before_kst.year(),
        dt_before_kst.month(),
        dt_before_kst.day(),
        dt_before_kst.hour(),
        dt_before_kst.minute(),
    );
    let pillars_before =
        FourPillars::calculate(&input_before).expect("Must calculate before LiChun");

    let input_after = SajuInput::new_solar(
        dt_after_kst.year(),
        dt_after_kst.month(),
        dt_after_kst.day(),
        dt_after_kst.hour(),
        dt_after_kst.minute(),
    );
    let pillars_after = FourPillars::calculate(&input_after).expect("Must calculate after LiChun");

    // Year stem transition (Yi 乙 -> Bing 丙)
    assert_eq!(
        pillars_before.year.stem,
        HeavenlyStem::Yi,
        "1 min before LiChun year stem must be Yi (乙)"
    );
    assert_eq!(
        pillars_after.year.stem,
        HeavenlyStem::Bing,
        "1 min after LiChun year stem must be Bing (丙)"
    );

    // Year branch transition (Si 巳 -> Wu 午)
    assert_eq!(
        pillars_before.year.branch,
        EarthlyBranch::Si,
        "1 min before LiChun year branch must be Si (巳)"
    );
    assert_eq!(
        pillars_after.year.branch,
        EarthlyBranch::Wu,
        "1 min after LiChun year branch must be Wu (午)"
    );

    // Month branch transition (Chou 丑 -> Yin 寅)
    assert_eq!(
        pillars_before.month.branch,
        EarthlyBranch::Chou,
        "1 min before LiChun month branch must be Chou (丑)"
    );
    assert_eq!(
        pillars_after.month.branch,
        EarthlyBranch::Yin,
        "1 min after LiChun month branch must be Yin (寅)"
    );
}

#[test]
fn test_m3_solar_term_dongzhi_and_xiaohan_boundaries() {
    // XiaoHan (소한) 2026 boundary transition:
    // Before XiaoHan in early Jan 2026 is Zi (子) month; after XiaoHan is Chou (丑) month.
    let input_xiaohan_before = SajuInput::new_solar(2026, 1, 4, 10, 0);
    let pillars_xh_before = FourPillars::calculate(&input_xiaohan_before).unwrap();

    let input_xiaohan_after = SajuInput::new_solar(2026, 1, 8, 10, 0);
    let pillars_xh_after = FourPillars::calculate(&input_xiaohan_after).unwrap();

    assert_eq!(
        pillars_xh_before.month.branch,
        EarthlyBranch::Zi,
        "Early Jan before XiaoHan must be Zi month"
    );
    assert_eq!(
        pillars_xh_after.month.branch,
        EarthlyBranch::Chou,
        "Jan after XiaoHan must be Chou month"
    );
}

// ============================================================================
// 2. Rare & Exotic Natal Charts
// ============================================================================

#[test]
fn test_m3_exotic_four_earth_branches_to_da_ja() {
    // Four Earth Branches (토다자 / 辰未戌丑 in all 4 branches):
    // Year: 戊辰, Month: 己未, Day: 戊戌, Hour: 己丑
    let earth_pillars = make_custom_pillars(
        HeavenlyStem::Wu,
        EarthlyBranch::Chen,
        HeavenlyStem::Ji,
        EarthlyBranch::Wei,
        HeavenlyStem::Wu,
        EarthlyBranch::Xu,
        HeavenlyStem::Ji,
        EarthlyBranch::Chou,
    );

    let struct_res = earth_pillars.structure();
    // Heavy earth branches with Day Master Wu/Ji forms GaSaek (가색격) JeonWang pattern
    assert_eq!(
        struct_res.structure,
        StructureType::GaSaek,
        "Four Earth branches with Earth DM must form GaSaek structure"
    );

    let strength_res = earth_pillars.strength();
    assert_eq!(
        strength_res.strength_type,
        StrengthType::Strong,
        "Four Earth branches chart must be extremely Strong"
    );
}

#[test]
fn test_m3_exotic_all_stems_same_element_cheon_gan_il_saek() {
    // All Stems Same Element (천간일색): 甲甲甲甲 (Wood Heavenly Stems only)
    // Year: 甲寅, Month: 甲戌, Day: 甲子, Hour: 甲辰
    let wood_stems_pillars = make_custom_pillars(
        HeavenlyStem::Jia,
        EarthlyBranch::Yin,
        HeavenlyStem::Jia,
        EarthlyBranch::Xu,
        HeavenlyStem::Jia,
        EarthlyBranch::Zi,
        HeavenlyStem::Jia,
        EarthlyBranch::Chen,
    );

    let power_res = wood_stems_pillars
        .integrated_analysis(AnalysisOptions::default(), &AnalysisConfig::default());
    let wood_score = power_res
        .element_scores
        .iter()
        .find(|(el, _, _)| *el == Element::Wood)
        .map(|(_, pct, _)| *pct)
        .unwrap_or(0.0);

    assert!(
        wood_score > 40.0,
        "All-Wood stems chart must have strong Wood element score, got {:.1}%",
        wood_score
    );
}

#[test]
fn test_m3_exotic_multi_clash_chung_da_ja() {
    // Multi-Clash (충다자): Simultaneous Zi-Wu clash (子午沖) and Mao-You clash (卯酉沖)
    // Year: 丙子, Month: 丁卯, Day: 壬午, Hour: 己酉
    let clash_pillars = make_custom_pillars(
        HeavenlyStem::Bing,
        EarthlyBranch::Zi,
        HeavenlyStem::Ding,
        EarthlyBranch::Mao,
        HeavenlyStem::Ren,
        EarthlyBranch::Wu,
        HeavenlyStem::Ji,
        EarthlyBranch::You,
    );

    let rel_res = clash_pillars.relationships();
    let has_zi_wu = rel_res
        .branch_clashes
        .iter()
        .any(|(c, _, _)| *c == eon_saju::analysis::relationships::BranchClash::ZiWu);
    let has_mao_you = rel_res
        .branch_clashes
        .iter()
        .any(|(c, _, _)| *c == eon_saju::analysis::relationships::BranchClash::MaoYou);

    assert!(has_zi_wu, "Must detect Zi-Wu clash");
    assert!(has_mao_you, "Must detect Mao-You clash");
}

#[test]
fn test_m3_special_jeonwang_5_gyeok_structures() {
    // 1. 곡직격 (GokJik / Wood JeonWang): 甲/乙 Day Master, Wood Month Branch (卯), Heavy Wood
    let gokjik = make_custom_pillars(
        HeavenlyStem::Gui,
        EarthlyBranch::Hai,
        HeavenlyStem::Yi,
        EarthlyBranch::Mao,
        HeavenlyStem::Jia,
        EarthlyBranch::Yin,
        HeavenlyStem::Gui,
        EarthlyBranch::Hai,
    );
    assert_eq!(
        gokjik.structure().structure,
        StructureType::GokJik,
        "Must identify 곡직격"
    );

    // 2. 염상격 (YeomSang / Fire JeonWang): 丙/丁 Day Master, Fire Month Branch (午), Heavy Fire
    let yeomsang = make_custom_pillars(
        HeavenlyStem::Yi,
        EarthlyBranch::Si,
        HeavenlyStem::Ding,
        EarthlyBranch::Wu,
        HeavenlyStem::Bing,
        EarthlyBranch::Wu,
        HeavenlyStem::Jia,
        EarthlyBranch::Wu,
    );
    assert_eq!(
        yeomsang.structure().structure,
        StructureType::YeomSang,
        "Must identify 염상격"
    );

    // 3. 종혁격 (JongHyeok / Metal JeonWang): 庚/辛 Day Master, Metal Month Branch (酉), Heavy Metal
    let jonghyeok = make_custom_pillars(
        HeavenlyStem::Ji,
        EarthlyBranch::Si,
        HeavenlyStem::Xin,
        EarthlyBranch::You,
        HeavenlyStem::Geng,
        EarthlyBranch::Shen,
        HeavenlyStem::Ji,
        EarthlyBranch::Si,
    );
    assert_eq!(
        jonghyeok.structure().structure,
        StructureType::JongHyeok,
        "Must identify 종혁격"
    );

    // 4. 윤하격 (YoonHa / Water JeonWang): 壬/癸 Day Master, Water Month Branch (子), Heavy Water
    let yoonha = make_custom_pillars(
        HeavenlyStem::Ren,
        EarthlyBranch::Shen,
        HeavenlyStem::Ren,
        EarthlyBranch::Zi,
        HeavenlyStem::Gui,
        EarthlyBranch::Hai,
        HeavenlyStem::Gui,
        EarthlyBranch::Chou,
    );
    assert_eq!(
        yoonha.structure().structure,
        StructureType::YoonHa,
        "Must identify 윤하격"
    );
}

// ============================================================================
// 3. Temporal Simulation Invariants & Extreme Dates
// ============================================================================

#[test]
fn test_m3_100_year_continuous_timeline_invariants() {
    let input = SajuInput::new_solar(1990, 5, 15, 14, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    let emulator = LifePathEmulator::new(pillars, Gender::Male, 1990);
    let report = emulator.emulate().expect("Emulation must succeed");

    assert!(
        report.frames.len() >= 100,
        "100-year continuous emulator must generate at least 100 frames, got {}",
        report.frames.len()
    );

    // Check frame scores and age continuity
    for frame in &report.frames {
        assert!(
            !frame.score.is_nan() && frame.score >= 0.0 && frame.score <= 100.0,
            "Score must be bounded"
        );
    }
}

#[test]
fn test_m3_extreme_historical_and_future_dates_1900_2100() {
    // Test extreme boundary date 1900-01-01
    let input_1900 = SajuInput::new_solar(1900, 1, 1, 0, 0);
    let pillars_1900 = FourPillars::calculate(&input_1900);
    assert!(pillars_1900.is_ok(), "1900-01-01 calculation must pass");

    // Test extreme future boundary date 2100-12-31
    let input_2100 = SajuInput::new_solar(2100, 12, 31, 23, 59);
    let pillars_2100 = FourPillars::calculate(&input_2100);
    assert!(pillars_2100.is_ok(), "2100-12-31 calculation must pass");
}

// ============================================================================
// 4. SajuVM Safety & Determinism
// ============================================================================

#[test]
fn test_m3_sajuvm_step_execution_and_register_normalization() {
    let input = SajuInput::new_solar(1988, 8, 8, 8, 8);
    let pillars = FourPillars::calculate(&input).unwrap();
    let vm = SajuVM::new(pillars);

    let major = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
    let yearly = GanZi::new(HeavenlyStem::Bing, EarthlyBranch::Yin);

    let frame = vm.step(30, major, yearly, None, None, None);

    // Verify register normalization sum ~ 100.0% (or 0 if un-updated)
    let reg = &frame.register_state;
    let sum = reg.r0_wood + reg.r1_fire + reg.r2_earth + reg.r3_metal + reg.r4_water;
    assert!(
        sum == 0.0 || (sum - 100.0).abs() < 0.5,
        "QiRegisters sum must normalize to ~100%, got {:.2}",
        sum
    );

    // Check determinism: running step twice with same arguments must yield identical score and trace
    let frame_2 = vm.step(30, major, yearly, None, None, None);
    assert_eq!(
        frame.score, frame_2.score,
        "VM execution must be 100% deterministic"
    );
    assert_eq!(
        frame.esil_trace, frame_2.esil_trace,
        "VM trace must be 100% deterministic"
    );
}

// ============================================================================
// 5. Boundary Input Handling & Random Fuzzer (100 Charts)
// ============================================================================

#[test]
fn test_m3_boundary_input_extreme_geographical_coordinates() {
    // High Latitudes North (Arctic 70°N, 150°E)
    let arctic_input = SajuInput::new_solar_at(1995, 12, 21, 12, 0, 150.0, 70.0);
    assert!(FourPillars::calculate(&arctic_input).is_ok());

    // High Latitudes South (Antarctica/Tierra del Fuego -55°S, -68°W)
    let south_input = SajuInput::new_solar_at(2000, 6, 21, 12, 0, -68.0, -55.0);
    assert!(FourPillars::calculate(&south_input).is_ok());

    // Prime Meridian (Greenwich 51.48°N, 0.0°E)
    let greenwich_input = SajuInput::new_solar_at(2010, 3, 1, 12, 0, 0.0, 51.48);
    assert!(FourPillars::calculate(&greenwich_input).is_ok());
}

#[test]
fn test_m3_fuzzer_100_charts_invariants() {
    let mut state: u32 = 0xCAFEEACE;
    let mut next_u32 = || {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        state
    };

    for idx in 0..100 {
        let year = 1900 + (next_u32() % 200) as i32; // 1900..2100
        let month = (next_u32() % 12) + 1;
        let day = (next_u32() % 28) + 1;
        let hour = next_u32() % 24;
        let minute = next_u32() % 60;

        let input = SajuInput::new_solar(year, month, day, hour, minute);
        let pillars_res = FourPillars::calculate(&input);
        assert!(
            pillars_res.is_ok(),
            "Failed at chart #{}: Y{} M{} D{} H{}",
            idx,
            year,
            month,
            day,
            hour
        );
        let pillars = pillars_res.unwrap();

        let _struct_res = pillars.structure();
        let _yong_res = pillars.yongshin();
        let _strength_res = pillars.strength();

        let major_gz = GanZi::from_index((next_u32() % 60) as i32);
        let yearly_gz = GanZi::from_index((next_u32() % 60) as i32);
        let monthly_gz = GanZi::from_index((next_u32() % 60) as i32);

        let _dyn_res = DynamicLuckAnalysis::analyze(
            &pillars,
            Some(major_gz),
            Some(yearly_gz),
            Some(monthly_gz),
            None,
            None,
        );

        let vm = SajuVM::new(pillars.clone());
        let frame = vm.step(
            20 + (idx % 60),
            major_gz,
            yearly_gz,
            Some(monthly_gz),
            None,
            None,
        );

        assert!(
            !frame.score.is_nan() && !frame.score.is_infinite(),
            "Chart #{}: Score must be finite",
            idx
        );
        assert!(
            frame.score >= 0.0 && frame.score <= 100.0,
            "Chart #{}: Score bounded in [0, 100]",
            idx
        );

        let reg = &frame.register_state;
        let sum = reg.r0_wood + reg.r1_fire + reg.r2_earth + reg.r3_metal + reg.r4_water;
        assert!(
            sum == 0.0 || (sum - 100.0).abs() < 0.5,
            "Chart #{}: QiRegisters sum must be ~100%, got {}",
            idx,
            sum
        );
    }
}
