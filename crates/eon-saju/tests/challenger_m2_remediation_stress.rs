//! 🧪 Challenger Stress Test Suite for Milestone 2 (R2) Remediation
//!
//! Empirical adversarial test harness verifying all 5 defects:
//! 1. Wolwun Saju year calculation for early January dates before XiaoHan.
//! 2. Elemental power integrity across non-Earth and transformed branches under apply_correction: true.
//! 3. GaeGo unsealed stem single-counting in SajuVM (mem_dump vs gaego).
//! 4. IpMyo trapped element matching Day Master element for all 10 Heavenly Stems (Yang & Yin).
//! 5. Gyeokguk fulfillment exclusion of BiJian/JieCai, and luck-pillar requirement for GaeGo events.
//! 6. Property-based fuzzer across random charts for numerical stability and invariants.

use chrono::{Duration, NaiveDate, TimeZone, Utc};
use eon_astro::AstroEngine;
use eon_core::Gender;
use eon_saju::{
    analysis::{
        dynamic_luck::{DynamicLuckAnalysis, GyeokStatus},
        periodic_luck::MonthlyLuck,
        power::{AnalysisOptions, IntegratedAnalysis},
    },
    core::{
        branch::EarthlyBranch,
        config::AnalysisConfig,
        element::Element,
        ganzi::GanZi,
        pillars::{FourPillars, SajuInput},
        stem::HeavenlyStem,
        twelve_stages::{calculate_twelve_stage, TwelveStage},
    },
    engine::vm::SajuVM,
};

fn make_pillars(
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
// Defect 1: Wolwun Saju Year & Month GanZi Across Boundary Conditions
// ============================================================================

#[test]
fn test_stress_wolwun_january_dates_multi_year() {
    // For years 2020 through 2035, verify that ALL dates in January (Jan 1 to Jan 31):
    // 1. Have saju_year equal to calendar_year - 1.
    // 2. Have valid month branches (子 before XiaoHan, 丑 after XiaoHan).
    // 3. Have month stems following year stem formula.
    let engine = AstroEngine::new();

    for year in 2020..=2035 {
        let year_start = NaiveDate::from_ymd_opt(year, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();

        // Solar term 22 is XiaoHan (소한)
        let xiaohan = engine.find_solar_term_time(year_start, 22).unwrap();

        for day in 1..=31 {
            let dt = NaiveDate::from_ymd_opt(year, 1, day)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
                .and_utc();

            let wolwun = MonthlyLuck::month_ganzi_at(dt);

            // Saju year for any day in January MUST be year - 1
            let expected_saju_year = year - 1;
            let expected_year_stem = GanZi::from_year(expected_saju_year).stem;
            let first_month_stem_idx = match expected_year_stem.index() % 5 {
                0 => 2,
                1 => 4,
                2 => 6,
                3 => 8,
                4 => 0,
                _ => 0,
            };

            if dt < xiaohan {
                // Before XiaoHan: 11th Saju month (子월)
                assert_eq!(
                    wolwun.branch,
                    EarthlyBranch::Zi,
                    "Jan {} {} before XiaoHan must be Zi branch",
                    day,
                    year
                );
                let expected_stem_idx = (first_month_stem_idx + 10) % 10;
                assert_eq!(
                    wolwun.stem,
                    HeavenlyStem::from_index(expected_stem_idx),
                    "Jan {} {} month stem mismatch before XiaoHan",
                    day,
                    year
                );
            } else {
                // After XiaoHan (and before LiChun): 12th Saju month (丑월)
                assert_eq!(
                    wolwun.branch,
                    EarthlyBranch::Chou,
                    "Jan {} {} after XiaoHan must be Chou branch",
                    day,
                    year
                );
                let expected_stem_idx = (first_month_stem_idx + 11) % 10;
                assert_eq!(
                    wolwun.stem,
                    HeavenlyStem::from_index(expected_stem_idx),
                    "Jan {} {} month stem mismatch after XiaoHan",
                    day,
                    year
                );
            }
        }
    }
}

#[test]
fn test_stress_wolwun_all_24_solar_terms_continuity_2026() {
    let engine = AstroEngine::new();
    let year_start = NaiveDate::from_ymd_opt(2026, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();

    // Check all 24 solar terms in 2026
    let mut term_times = Vec::new();
    for term_idx in 0..24 {
        let t = engine.find_solar_term_time(year_start, term_idx).unwrap();
        term_times.push((term_idx, t));
    }

    for (term_idx, t) in term_times {
        let dt_before = t - Duration::minutes(1);
        let dt_after = t + Duration::minutes(1);

        let g_before = MonthlyLuck::month_ganzi_at(dt_before);
        let g_after = MonthlyLuck::month_ganzi_at(dt_after);

        let is_major_term = term_idx % 2 == 0;
        if is_major_term {
            // Major term: Saju month ordinal transition
            // term 0 (LiChun): before = 12th month (Chou 丑), after = 1st month (Yin 寅)
            // term 2 (JingZhi): before = 1st month (Yin 寅), after = 2nd month (Mao 卯)
            // ...
            // term 22 (XiaoHan): before = 11th month (Zi 子), after = 12th month (Chou 丑)
            let ordinal_before = if term_idx == 0 {
                12
            } else {
                (term_idx / 2) as u32
            };
            let ordinal_after = if term_idx == 0 { 1 } else { ordinal_before + 1 };

            let expected_branch_before =
                EarthlyBranch::from_index(((ordinal_before + 1) % 12) as i32);
            let expected_branch_after =
                EarthlyBranch::from_index(((ordinal_after + 1) % 12) as i32);

            assert_eq!(
                g_before.branch, expected_branch_before,
                "Term {} before branch mismatch",
                term_idx
            );
            assert_eq!(
                g_after.branch, expected_branch_after,
                "Term {} after branch mismatch",
                term_idx
            );
        } else {
            // Minor term (중기): Month GanZi should remain IDENTICAL before and after
            assert_eq!(
                g_before, g_after,
                "Minor solar term {} should not change Wolwun GanZi",
                term_idx
            );
        }
    }
}

// ============================================================================
// Defect 2: Elemental Power Integrity across Non-Earth and Transformed Branches
// ============================================================================

#[test]
fn test_stress_non_earth_branches_untransformed_correction_integrity() {
    let non_earth_branches = [
        EarthlyBranch::Zi,   // Water
        EarthlyBranch::Yin,  // Wood
        EarthlyBranch::Mao,  // Wood
        EarthlyBranch::Si,   // Fire
        EarthlyBranch::Wu,   // Fire
        EarthlyBranch::Shen, // Metal
        EarthlyBranch::You,  // Metal
        EarthlyBranch::Hai,  // Water
    ];

    let config = AnalysisConfig::default();
    let opts = AnalysisOptions {
        apply_transform: true,
        apply_correction: true,
    };

    // Test each non-Earth branch under a neutral month branch (Xu) so no triple/seasonal alliance forms
    for &branch in &non_earth_branches {
        let pillars = make_pillars(
            HeavenlyStem::Jia,
            branch,
            HeavenlyStem::Bing,
            EarthlyBranch::Xu, // Neutral Earth month branch to prevent unintended alliance transformations
            HeavenlyStem::Wu,
            branch,
            HeavenlyStem::Geng,
            branch,
        );

        let analysis = IntegratedAnalysis::calculate_expanded(&pillars, None, None, opts, &config);

        // Verify that non-Earth branch element is preserved and NOT corrupted by climate correction
        let orig_el = branch.element();
        let score_tuple = analysis
            .element_scores
            .iter()
            .find(|(el, _, _)| *el == orig_el);

        assert!(
            score_tuple.is_some() && score_tuple.unwrap().2 > 0.0,
            "Branch {:?} original element {:?} must have non-zero score under neutral month",
            branch,
            orig_el
        );
    }
}

#[test]
fn test_stress_transformed_earth_branches_preserve_transformed_element() {
    // 辰 (Chen Earth) in 申子辰 Water Triple Alliance
    let pillars = make_pillars(
        HeavenlyStem::Jia,
        EarthlyBranch::Shen,
        HeavenlyStem::Bing,
        EarthlyBranch::Zi,
        HeavenlyStem::Wu,
        EarthlyBranch::Chen,
        HeavenlyStem::Geng,
        EarthlyBranch::Yin,
    );

    let config = AnalysisConfig::default();

    // Test under both apply_correction: true and apply_correction: false
    for apply_corr in [false, true] {
        let opts = AnalysisOptions {
            apply_transform: true,
            apply_correction: apply_corr,
        };

        let analysis = IntegratedAnalysis::calculate_expanded(&pillars, None, None, opts, &config);

        let water_score = analysis
            .element_scores
            .iter()
            .find(|(el, _, _)| *el == Element::Water)
            .map(|(_, _, s)| *s)
            .unwrap_or(0.0);

        let earth_score = analysis
            .element_scores
            .iter()
            .find(|(el, _, _)| *el == Element::Earth)
            .map(|(_, _, s)| *s)
            .unwrap_or(0.0);

        assert!(
            water_score > earth_score,
            "Transformed Chen branch must yield higher Water score than Earth under apply_correction: {}",
            apply_corr
        );
    }
}

// ============================================================================
// Defect 3: GaeGo Double-Scoring Prevention in SajuVM
// ============================================================================

#[test]
fn test_stress_gaego_vm_single_counting_invariants() {
    // Create natal pillars with 辰 and 戌 (storage branches)
    let pillars = make_pillars(
        HeavenlyStem::Jia,
        EarthlyBranch::Chen,
        HeavenlyStem::Bing,
        EarthlyBranch::Xu,
        HeavenlyStem::Wu,
        EarthlyBranch::Chou,
        HeavenlyStem::Geng,
        EarthlyBranch::Wei,
    );

    let major_xu = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Xu);
    let vm = SajuVM::new(pillars);
    let frame = vm.step(30, major_xu, major_xu, None, None, None);

    let trace = &frame.esil_trace;
    println!("VM Trace: {}", trace);

    // Extract all mem_dump entries and gaego entries
    let mem_dumps: Vec<&str> = trace
        .split("; ")
        .filter(|s| s.starts_with("mem_dump:"))
        .collect();
    let gaegos: Vec<&str> = trace
        .split("; ")
        .filter(|s| s.starts_with("gaego:"))
        .collect();

    // Verify: No stem that appears in gaego trace appears in mem_dump trace for the same branch
    for g_entry in &gaegos {
        // Format: gaego:술(신),bonus:1.5
        if let Some(open_paren) = g_entry.find('(') {
            if let Some(close_paren) = g_entry.find(')') {
                let branch_name = &g_entry[6..open_paren];
                let stem_name = &g_entry[open_paren + 1..close_paren];

                let duplicate_in_mem_dump = mem_dumps
                    .iter()
                    .any(|m| m.contains(branch_name) && m.contains(&format!("({})", stem_name)));

                assert!(
                    !duplicate_in_mem_dump,
                    "Stem {}({}) was double-counted in both mem_dump and gaego trace!",
                    branch_name, stem_name
                );
            }
        }
    }
}

// ============================================================================
// Defect 4: IpMyo Trapped Element Matching Day Master Element for All Stems
// ============================================================================

#[test]
fn test_stress_ipmyo_trapped_element_all_10_stems() {
    // Test all 10 Heavenly Stems as Day Master entering their respective 墓 (Mu) stage
    let all_stems = HeavenlyStem::ALL;

    for &stem in &all_stems {
        // Find which branch puts this stem into TwelveStage::Mu
        let mut tomb_branch = None;
        for &branch in &EarthlyBranch::ALL {
            if calculate_twelve_stage(stem, branch) == TwelveStage::Mu {
                tomb_branch = Some(branch);
                break;
            }
        }

        let branch = tomb_branch.expect("Every stem must have a 墓 (Mu) branch");

        let pillars = make_pillars(
            HeavenlyStem::Jia,
            EarthlyBranch::Zi,
            HeavenlyStem::Bing,
            EarthlyBranch::Yin,
            stem, // Day Master
            EarthlyBranch::Wu,
            HeavenlyStem::Geng,
            EarthlyBranch::Shen,
        );

        let major_luck = Some(GanZi::new(HeavenlyStem::Ren, branch));
        let analysis = DynamicLuckAnalysis::analyze(&pillars, major_luck, None, None, None, None);

        assert_eq!(
            analysis.ipmyo_events.len(),
            1,
            "Day Master {:?} at branch {:?} (Mu stage) must generate exactly 1 IpMyo event",
            stem,
            branch
        );

        let event = &analysis.ipmyo_events[0];
        assert_eq!(
            event.element,
            stem.element(),
            "IpMyo trapped element for Day Master {:?} must match Day Master element {:?}, found {:?}",
            stem,
            stem.element(),
            event.element
        );
    }
}

// ============================================================================
// Defect 5: Gyeokguk Fulfillment & Natal GaeGo Asymmetry
// ============================================================================

#[test]
fn test_stress_gyeokguk_fulfillment_exclusion_of_bijian_jiecai() {
    // Natal: Day Master 己 (Ji Earth). Month branch 辰 (Chen Earth, contains hidden stems 乙 Wood, 癸 Water, 戊 Earth).
    // Use branches that do NOT form any natal triple or seasonal alliances (e.g. Zi, Chen, Mao, Si).
    let pillars = make_pillars(
        HeavenlyStem::Bing,
        EarthlyBranch::Zi,
        HeavenlyStem::Wu,
        EarthlyBranch::Chen, // Month branch containing 戊 (Jiecai for 己 Day Master)
        HeavenlyStem::Ji,    // Day Master 己
        EarthlyBranch::Mao,
        HeavenlyStem::Geng,
        EarthlyBranch::Si,
    );

    // 1. Major luck stem = 戊 (Wu Earth -> Jiecai / 劫財 for 己 DM).
    // Month branch hidden stems include 戊.
    let major_jiecai = Some(GanZi::new(HeavenlyStem::Wu, EarthlyBranch::Zi));
    let analysis_jiecai =
        DynamicLuckAnalysis::analyze(&pillars, major_jiecai, None, None, None, None);

    assert_eq!(
        analysis_jiecai.structure_state.status,
        GyeokStatus::Stable,
        "Emergence of Jiecai (劫財) must NOT set Gyeokguk status to Fulfilled"
    );

    // 2. Major luck stem = 己 (Ji Earth -> Bijian / 比肩 for 己 DM).
    let major_bijian = Some(GanZi::new(HeavenlyStem::Ji, EarthlyBranch::Zi));
    let analysis_bijian =
        DynamicLuckAnalysis::analyze(&pillars, major_bijian, None, None, None, None);

    assert_eq!(
        analysis_bijian.structure_state.status,
        GyeokStatus::Stable,
        "Emergence of Bijian (比肩) must NOT set Gyeokguk status to Fulfilled"
    );

    // 3. Major luck stem = 乙 (Yi Wood -> PianGuan / 七殺 for 己 DM).
    // Month branch 辰 contains 乙.
    let major_pianguan = Some(GanZi::new(HeavenlyStem::Yi, EarthlyBranch::Zi));
    let analysis_pianguan =
        DynamicLuckAnalysis::analyze(&pillars, major_pianguan, None, None, None, None);

    assert_eq!(
        analysis_pianguan.structure_state.status,
        GyeokStatus::Fulfilled,
        "Emergence of PianGuan (七殺) matching month hidden stem MUST set Gyeokguk status to Fulfilled"
    );
}

#[test]
fn test_stress_natal_only_no_gaego_vs_luck_pillar_gaego() {
    // Natal pillars with storage branch 辰 and 戌 (creates natal 辰-戌 clash)
    let pillars = make_pillars(
        HeavenlyStem::Jia,
        EarthlyBranch::Chen,
        HeavenlyStem::Bing,
        EarthlyBranch::Yin,
        HeavenlyStem::Wu,
        EarthlyBranch::Xu,
        HeavenlyStem::Geng,
        EarthlyBranch::Shen,
    );

    // 1. Static natal analysis (major: None, yearly: None)
    let static_analysis = DynamicLuckAnalysis::analyze(&pillars, None, None, None, None, None);

    assert!(
        static_analysis.gaego_events.is_empty(),
        "Static natal-only analysis with 辰-戌 clash MUST NOT produce GaeGo events without luck pillar"
    );

    // 2. Dynamic analysis with luck pillar 辰 (creates 戌(natal) - 辰(luck) clash)
    let major_luck = Some(GanZi::new(HeavenlyStem::Ren, EarthlyBranch::Chen));
    let dynamic_analysis =
        DynamicLuckAnalysis::analyze(&pillars, major_luck, None, None, None, None);

    assert!(
        !dynamic_analysis.gaego_events.is_empty(),
        "Dynamic luck analysis with luck pillar clash MUST emit GaeGo events"
    );
}

// ============================================================================
// Fuzzer / Property Invariants
// ============================================================================

#[test]
fn test_fuzz_random_charts_no_panics_or_nans() {
    // Simple deterministic PRNG fuzzer
    let mut state: u64 = 0x123456789ABCDEF0;
    let mut next_rand = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (state >> 32) as u32
    };

    let config = AnalysisConfig::default();
    let opts = AnalysisOptions {
        apply_transform: true,
        apply_correction: true,
    };

    for _ in 0..100 {
        let ys = HeavenlyStem::from_index((next_rand() % 10) as i32);
        let yb = EarthlyBranch::from_index((next_rand() % 12) as i32);
        let ms = HeavenlyStem::from_index((next_rand() % 10) as i32);
        let mb = EarthlyBranch::from_index((next_rand() % 12) as i32);
        let ds = HeavenlyStem::from_index((next_rand() % 10) as i32);
        let db = EarthlyBranch::from_index((next_rand() % 12) as i32);
        let hs = HeavenlyStem::from_index((next_rand() % 10) as i32);
        let hb = EarthlyBranch::from_index((next_rand() % 12) as i32);

        let pillars = make_pillars(ys, yb, ms, mb, ds, db, hs, hb);

        let major_s = HeavenlyStem::from_index((next_rand() % 10) as i32);
        let major_b = EarthlyBranch::from_index((next_rand() % 12) as i32);
        let major_gz = GanZi::new(major_s, major_b);
        let major = Some(major_gz);

        let yearly_s = HeavenlyStem::from_index((next_rand() % 10) as i32);
        let yearly_b = EarthlyBranch::from_index((next_rand() % 12) as i32);
        let yearly_gz = GanZi::new(yearly_s, yearly_b);
        let yearly = Some(yearly_gz);

        // 1. Dynamic Luck Analysis
        let dyn_analysis = DynamicLuckAnalysis::analyze(&pillars, major, yearly, None, None, None);
        assert!(!dyn_analysis.structure_state.description.is_empty());

        // 2. Integrated Power Analysis
        let power_analysis =
            IntegratedAnalysis::calculate_expanded(&pillars, major, yearly, opts, &config);
        for (el, pct, val) in &power_analysis.element_scores {
            assert!(
                !pct.is_nan() && *pct >= 0.0 && *pct <= 100.0,
                "Element {:?} pct invalid: {}",
                el,
                pct
            );
            assert!(
                !val.is_nan() && *val >= 0.0,
                "Element {:?} val invalid: {}",
                el,
                val
            );
        }

        // 3. SajuVM Step Execution
        let vm = SajuVM::new(pillars);
        let frame = vm.step(30, major_gz, yearly_gz, None, None, None);
        assert!(!frame.esil_trace.is_empty());
    }
}

// ============================================================================
// Adversarial Stress Test: Alliance Branch Position Isolation (Defect Detection)
// ============================================================================

#[test]
fn test_stress_alliance_suppression_position_isolation() {
    // Year: 申, Month: 子, Day: 辰 (Forms 申子辰 Triple Alliance)
    // Hour: 辰 (Second 辰 in chart, NOT part of 申子辰 alliance)
    // Major Luck: 戌 (Clashes with 辰)
    let pillars = make_pillars(
        HeavenlyStem::Bing,
        EarthlyBranch::Shen,
        HeavenlyStem::Ren,
        EarthlyBranch::Zi,
        HeavenlyStem::Wu,
        EarthlyBranch::Chen,
        HeavenlyStem::Bing,
        EarthlyBranch::Chen,
    );

    let major_xu = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Xu);
    let dyn_analysis =
        DynamicLuckAnalysis::analyze(&pillars, Some(major_xu), None, None, None, None);

    println!(
        "Triple Combinations: {:?}",
        dyn_analysis.combined_relations.triple_combinations
    );
    println!(
        "Branch Clashes: {:?}",
        dyn_analysis.combined_relations.branch_clashes
    );

    // Documented Finding: alliance_branches uses HashSet<EarthlyBranch> without position context.
    // As a result, the Triple Alliance on Day 辰 erroneously suppresses the valid clash on Hour 辰.
    let hour_chen_clash_exists =
        dyn_analysis
            .combined_relations
            .branch_clashes
            .iter()
            .any(|(_, p1, p2)| {
                (p1 == "시지" && p2 == "대운지지") || (p1 == "대운지지" && p2 == "시지")
            });

    assert!(
        hour_chen_clash_exists,
        "Major Luck 戌 vs Hour 辰 clash must exist despite Day 辰 participating in Triple Alliance!"
    );
}

// ============================================================================
// Adversarial Stress Test: Gyeokguk State Transitions Coverage
// ============================================================================

#[test]
fn test_stress_gyeokguk_state_machine_coverage() {
    // 1. Stable -> Fulfilled (via Luck Stem matching Month Branch hidden stem)
    let pillars_stable = make_pillars(
        HeavenlyStem::Bing,
        EarthlyBranch::Wu,
        HeavenlyStem::Wu,
        EarthlyBranch::Chen, // hidden stems: [Yi, Gui, Wu]
        HeavenlyStem::Ji,    // DM: Ji Earth
        EarthlyBranch::Si,
        HeavenlyStem::Geng,
        EarthlyBranch::Shen,
    );

    let major_yi = Some(GanZi::new(HeavenlyStem::Yi, EarthlyBranch::Chou)); // Yi = PianGuan (七殺), Chou = no triple alliance
    let dyn_fulfilled =
        DynamicLuckAnalysis::analyze(&pillars_stable, major_yi, None, None, None, None);
    assert_eq!(
        dyn_fulfilled.structure_state.status,
        GyeokStatus::Fulfilled,
        "Valid luck stem Yi matching month hidden stem MUST transition status to Fulfilled"
    );

    // 2. Stable -> Broken (via Month Branch clash)
    let major_xu = Some(GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Xu)); // Xu clashes with Chen month
    let dyn_broken =
        DynamicLuckAnalysis::analyze(&pillars_stable, major_xu, None, None, None, None);
    assert_eq!(
        dyn_broken.structure_state.status,
        GyeokStatus::Broken,
        "Month branch clash MUST transition status to Broken"
    );

    // 3. Stable -> Transformed (via Triple Alliance)
    let pillars_triple = make_pillars(
        HeavenlyStem::Jia,
        EarthlyBranch::Shen,
        HeavenlyStem::Bing,
        EarthlyBranch::Zi,
        HeavenlyStem::Wu,
        EarthlyBranch::Yin,
        HeavenlyStem::Geng,
        EarthlyBranch::Wu,
    );
    let major_chen = Some(GanZi::new(HeavenlyStem::Ren, EarthlyBranch::Chen)); // Shen-Zi-Chen
    let dyn_transformed =
        DynamicLuckAnalysis::analyze(&pillars_triple, major_chen, None, None, None, None);
    assert_eq!(
        dyn_transformed.structure_state.status,
        GyeokStatus::Transformed,
        "Triple Alliance completion MUST transition status to Transformed"
    );
}
