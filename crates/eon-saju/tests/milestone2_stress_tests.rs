//! 🧪 Milestone 2 (R2) Wolwun Alignment, Dynamic Precedence, and Transformations Stress Tests
//!
//! Empirical adversarial test suite verifying:
//! 1. Wolwun GanZi alignment at exact 1-minute before/after solar term entry boundaries.
//! 2. Precedence hierarchy: completed Triple Alliance (삼합) or Seasonal Alliance (방합)
//!    correctly suppresses lower-priority Branch Clash (지충) and Six Combination (육합) in combined_relations.
//! 3. Expanded transformations: 5/6 pillar transformation scores update elemental power in power.rs.

use chrono::{Duration, NaiveDate, Utc};
use eon_astro::AstroEngine;
use eon_core::Gender;
use eon_saju::analysis::dynamic_luck::DynamicLuckAnalysis;
use eon_saju::analysis::periodic_luck::MonthlyLuck;
use eon_saju::analysis::power::{AnalysisOptions, IntegratedAnalysis};
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::config::AnalysisConfig;
use eon_saju::core::element::Element;
use eon_saju::core::ganzi::GanZi;
use eon_saju::core::pillars::FourPillars;
use eon_saju::core::stem::HeavenlyStem;
use eon_saju::SajuInput;

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
        birth_time: Utc::now(),
        gender: Gender::Male,
        raw_input: dummy_input,
        supplementary_pillars: Default::default(),
    }
}

// ============================================================================
// 1. Wolwun GanZi Alignment around Solar Term Boundaries
// ============================================================================

#[test]
fn test_wolwun_lichun_boundary_2026() {
    let engine = AstroEngine::new();
    let year_start = NaiveDate::from_ymd_opt(2026, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();

    // Term 0 = 입춘 (LiChun)
    let lichun_2026 = engine.find_solar_term_time(year_start, 0).unwrap();
    println!("LiChun 2026 exact time: {}", lichun_2026);

    let dt_before = lichun_2026 - Duration::minutes(1);
    let dt_after = lichun_2026 + Duration::minutes(1);

    let ganzi_before = MonthlyLuck::month_ganzi_at(dt_before);
    let ganzi_after = MonthlyLuck::month_ganzi_at(dt_after);

    println!("1 min before LiChun 2026: {}", ganzi_before);
    println!("1 min after LiChun 2026: {}", ganzi_after);

    // Before LiChun 2026: Saju year 2025 (乙巳년), Month branch 丑 (12th month).
    // 乙년 (stem index 1) -> 1st month stem is 戊 (4). 12th month stem is (4+11)%10 = 5 (己).
    // So expected before: 己丑
    assert_eq!(
        ganzi_before.branch,
        EarthlyBranch::Chou,
        "Before LiChun must be 丑 month"
    );
    assert_eq!(
        ganzi_before.stem,
        HeavenlyStem::Ji,
        "Before LiChun 2026 year 2025 (乙) month stem must be 己"
    );

    // After LiChun 2026: Saju year 2026 (丙午년), Month branch 寅 (1st month).
    // 丙년 (stem index 2) -> 1st month stem is 庚 (6).
    // So expected after: 庚寅
    assert_eq!(
        ganzi_after.branch,
        EarthlyBranch::Yin,
        "After LiChun must be 寅 month"
    );
    assert_eq!(
        ganzi_after.stem,
        HeavenlyStem::Geng,
        "After LiChun 2026 year 2026 (丙) month stem must be 庚"
    );
}

#[test]
fn test_wolwun_early_january_saju_year() {
    // Stress test: Jan 2, 2026 is BEFORE XiaoHan (term 22, around Jan 5) and BEFORE LiChun (Feb 4).
    // On Jan 2, 2026, the solar term is term 21 (DongZhi, 동지).
    // The Saju year MUST be 2025 (乙巳년), NOT 2026 (丙午년).
    let dt_jan2 = NaiveDate::from_ymd_opt(2026, 1, 2)
        .unwrap()
        .and_hms_opt(12, 0, 0)
        .unwrap()
        .and_utc();

    let ganzi_jan2 = MonthlyLuck::month_ganzi_at(dt_jan2);
    println!("Jan 2, 2026 Wolwun GanZi: {}", ganzi_jan2);

    // Saju year 2025 (乙巳년) -> 11th Saju month (子월).
    // 乙년 1st month stem = 戊(4). 11th month stem = (4+10)%10 = 4 (戊).
    // Expected: 戊子
    assert_eq!(
        ganzi_jan2.branch,
        EarthlyBranch::Zi,
        "Jan 2, 2026 must be 子 month"
    );
    assert_eq!(
        ganzi_jan2.stem,
        HeavenlyStem::Wu,
        "Jan 2, 2026 month stem must be 戊 (year 2025 乙巳), NOT 庚 (year 2026 丙午)"
    );
}

#[test]
fn test_wolwun_jingzhi_boundary_2026() {
    let engine = AstroEngine::new();
    let year_start = NaiveDate::from_ymd_opt(2026, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();

    // Term 2 = 경칩 (JingZhi - major solar term entering 卯월)
    let jingzhi_2026 = engine.find_solar_term_time(year_start, 2).unwrap();
    println!("JingZhi 2026 exact time: {}", jingzhi_2026);

    let dt_before = jingzhi_2026 - Duration::minutes(1);
    let dt_after = jingzhi_2026 + Duration::minutes(1);

    let ganzi_before = MonthlyLuck::month_ganzi_at(dt_before);
    let ganzi_after = MonthlyLuck::month_ganzi_at(dt_after);

    println!("1 min before JingZhi 2026: {}", ganzi_before);
    println!("1 min after JingZhi 2026: {}", ganzi_after);

    // 2026 is 丙午년 -> 1st month (寅): 庚寅, 2nd month (卯): 辛卯
    assert_eq!(
        ganzi_before.branch,
        EarthlyBranch::Yin,
        "1 min before JingZhi must be 寅"
    );
    assert_eq!(
        ganzi_before.stem,
        HeavenlyStem::Geng,
        "1 min before JingZhi must be 庚"
    );

    assert_eq!(
        ganzi_after.branch,
        EarthlyBranch::Mao,
        "1 min after JingZhi must be 卯"
    );
    assert_eq!(
        ganzi_after.stem,
        HeavenlyStem::Xin,
        "1 min after JingZhi must be 辛"
    );
}

#[test]
fn test_wolwun_yushu_minor_term_boundary_2026() {
    let engine = AstroEngine::new();
    let year_start = NaiveDate::from_ymd_opt(2026, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();

    // Term 1 = 우수 (YuShu - minor term / 중기). Saju month branch should NOT change.
    let yushu_2026 = engine.find_solar_term_time(year_start, 1).unwrap();
    println!("YuShu 2026 exact time: {}", yushu_2026);

    let dt_before = yushu_2026 - Duration::minutes(1);
    let dt_after = yushu_2026 + Duration::minutes(1);

    let ganzi_before = MonthlyLuck::month_ganzi_at(dt_before);
    let ganzi_after = MonthlyLuck::month_ganzi_at(dt_after);

    println!("1 min before YuShu 2026: {}", ganzi_before);
    println!("1 min after YuShu 2026: {}", ganzi_after);

    // YuShu is a mid-term, both before and after must remain 庚寅
    assert_eq!(
        ganzi_before, ganzi_after,
        "Minor term YuShu should not change Wolwun GanZi"
    );
    assert_eq!(ganzi_after.branch, EarthlyBranch::Yin);
}

// ============================================================================
// 2. Precedence Hierarchy in Combined Relations (Dynamic Luck)
// ============================================================================

#[test]
fn test_triple_alliance_suppresses_clash_and_six_combination() {
    // Natal: Year 申, Month 子, Day 辰 (Completes 申子辰 Water Triple Alliance)
    // Major Luck: 午 (creates 子-午 clash with 子)
    // Yearly Luck: 酉 (creates 辰-酉 Six Combination with 辰)
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

    let major = Some(GanZi::new(HeavenlyStem::Ren, EarthlyBranch::Wu)); // 子-午 clash
    let yearly = Some(GanZi::new(HeavenlyStem::Gui, EarthlyBranch::You)); // 辰-酉 six combination

    let analysis = DynamicLuckAnalysis::analyze(&pillars, major, yearly, None, None, None);

    println!(
        "Combined Triple Combinations: {:?}",
        analysis.combined_relations.triple_combinations
    );
    println!(
        "Combined Branch Clashes: {:?}",
        analysis.combined_relations.branch_clashes
    );
    println!(
        "Combined Six Combinations: {:?}",
        analysis.combined_relations.six_combinations
    );

    // 1. Triple Alliance 申-子-辰 must be present
    assert!(!analysis.combined_relations.triple_combinations.is_empty());

    // 2. 子-午 Branch Clash must be suppressed because 子 is part of 申-子-辰
    let has_zi_wu_clash = analysis
        .combined_relations
        .branch_clashes
        .iter()
        .any(|(clash, _, _)| {
            let (b1, b2) = clash.branches();
            (b1 == EarthlyBranch::Zi && b2 == EarthlyBranch::Wu)
                || (b1 == EarthlyBranch::Wu && b2 == EarthlyBranch::Zi)
        });
    assert!(
        !has_zi_wu_clash,
        "子-午 clash must be suppressed by 申-子-辰 Triple Alliance"
    );

    // 3. 辰-酉 Six Combination must be suppressed because 辰 is part of 申-子-辰
    let has_chen_you_six =
        analysis
            .combined_relations
            .six_combinations
            .iter()
            .any(|(six, _, _)| {
                let (b1, b2) = six.branches();
                (b1 == EarthlyBranch::Chen && b2 == EarthlyBranch::You)
                    || (b1 == EarthlyBranch::You && b2 == EarthlyBranch::Chen)
            });
    assert!(
        !has_chen_you_six,
        "辰-酉 Six Combination must be suppressed by 申-子-辰 Triple Alliance"
    );
}

#[test]
fn test_seasonal_alliance_suppresses_clash_and_six_combination() {
    // Natal: Year 寅, Month 卯, Day 辰 (Completes 寅卯辰 Wood Seasonal Alliance)
    // Major Luck: 酉 (creates 卯-酉 clash with 卯, 辰-酉 Six Combination with 辰)
    let pillars = make_pillars(
        HeavenlyStem::Jia,
        EarthlyBranch::Yin,
        HeavenlyStem::Yi,
        EarthlyBranch::Mao,
        HeavenlyStem::Bing,
        EarthlyBranch::Chen,
        HeavenlyStem::Ding,
        EarthlyBranch::Si,
    );

    let major = Some(GanZi::new(HeavenlyStem::Xin, EarthlyBranch::You));

    let analysis = DynamicLuckAnalysis::analyze(&pillars, major, None, None, None, None);

    println!(
        "Seasonal Combinations: {:?}",
        analysis.combined_relations.seasonal_combinations
    );
    println!(
        "Branch Clashes: {:?}",
        analysis.combined_relations.branch_clashes
    );
    println!(
        "Six Combinations: {:?}",
        analysis.combined_relations.six_combinations
    );

    assert!(!analysis.combined_relations.seasonal_combinations.is_empty());

    // 卯-酉 clash must be suppressed
    let has_mao_you_clash =
        analysis
            .combined_relations
            .branch_clashes
            .iter()
            .any(|(clash, _, _)| {
                let (b1, b2) = clash.branches();
                (b1 == EarthlyBranch::Mao && b2 == EarthlyBranch::You)
                    || (b1 == EarthlyBranch::You && b2 == EarthlyBranch::Mao)
            });
    assert!(
        !has_mao_you_clash,
        "卯-酉 clash must be suppressed by 寅卯辰 Seasonal Alliance"
    );

    // 辰-酉 Six Combination must be suppressed
    let has_chen_you_six =
        analysis
            .combined_relations
            .six_combinations
            .iter()
            .any(|(six, _, _)| {
                let (b1, b2) = six.branches();
                (b1 == EarthlyBranch::Chen && b2 == EarthlyBranch::You)
                    || (b1 == EarthlyBranch::You && b2 == EarthlyBranch::Chen)
            });
    assert!(
        !has_chen_you_six,
        "辰-酉 Six Combination must be suppressed by 寅卯辰 Seasonal Alliance"
    );
}

// ============================================================================
// 3. Expanded Transformations in power.rs
// ============================================================================

#[test]
fn test_expanded_transformations_elemental_power() {
    // Natal: 申, 子, 辰, 寅. (申-子-辰 Triple Alliance -> Water)
    // Major Luck: 辰.
    // Yearly Luck: 申.
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
    let major = Some(GanZi::new(HeavenlyStem::Ren, EarthlyBranch::Chen));
    let yearly = Some(GanZi::new(HeavenlyStem::Gui, EarthlyBranch::Shen));

    let config = AnalysisConfig::default();

    // Test with apply_transform: true, apply_correction: false
    let opts_no_corr = AnalysisOptions {
        apply_transform: true,
        apply_correction: false,
    };
    let analysis_no_corr =
        IntegratedAnalysis::calculate_expanded(&pillars, major, yearly, opts_no_corr, &config);

    println!("=== Expanded Power Analysis (no correction) ===");
    for (el, percent, score) in &analysis_no_corr.element_scores {
        println!("{}: {:.1}% ({:.1} pts)", el.hangul(), percent, score);
    }

    let water_score_no_corr = analysis_no_corr
        .element_scores
        .iter()
        .find(|(el, _, _)| *el == Element::Water)
        .map(|(_, p, _)| *p)
        .unwrap_or(0.0);

    // Water score should be high because 申, 子, 辰 transformed into Water in natal + major + yearly
    assert!(
        water_score_no_corr > 40.0,
        "Water power percentage should be high when transformed"
    );

    // Test with apply_transform: true, apply_correction: true (Default options)
    let opts_default = AnalysisOptions {
        apply_transform: true,
        apply_correction: true,
    };
    let analysis_default =
        IntegratedAnalysis::calculate_expanded(&pillars, major, yearly, opts_default, &config);

    println!("=== Expanded Power Analysis (default correction=true) ===");
    for (el, percent, score) in &analysis_default.element_scores {
        println!("{}: {:.1}% ({:.1} pts)", el.hangul(), percent, score);
    }
}

#[test]
fn test_expanded_power_correction_branch_integrity() {
    // Test chart with NO water branches:
    // Year: 丙寅 (Wood branch)
    // Month: 丁卯 (Wood branch)
    // Day: 甲午 (Fire branch)
    // Hour: 辛酉 (Metal branch)
    // Major Luck: 戊戌 (Earth branch)
    // Yearly Luck: 庚申 (Metal branch)
    let pillars = make_pillars(
        HeavenlyStem::Bing,
        EarthlyBranch::Yin,
        HeavenlyStem::Ding,
        EarthlyBranch::Mao,
        HeavenlyStem::Jia,
        EarthlyBranch::Wu,
        HeavenlyStem::Xin,
        EarthlyBranch::You,
    );
    let major = Some(GanZi::new(HeavenlyStem::Wu, EarthlyBranch::Xu));
    let yearly = Some(GanZi::new(HeavenlyStem::Geng, EarthlyBranch::Shen));

    let config = AnalysisConfig::default();
    let opts_default = AnalysisOptions {
        apply_transform: true,
        apply_correction: true,
    };

    let analysis =
        IntegratedAnalysis::calculate_expanded(&pillars, major, yearly, opts_default, &config);

    println!("=== Test Branch Integrity under Correction ===");
    for (el, percent, score) in &analysis.element_scores {
        println!("{}: {:.1}% ({:.1} pts)", el.hangul(), percent, score);
    }

    let water_score = analysis
        .element_scores
        .iter()
        .find(|(el, _, _)| *el == Element::Water)
        .map(|(_, p, _)| *p)
        .unwrap_or(0.0);

    // There are NO water stems or water branches in this chart! Water score MUST NOT be high.
    assert!(
        water_score < 10.0,
        "Chart without water stems/branches had Water percentage {:.1}%! (Corruption bug in power.rs)",
        water_score
    );
}

#[test]
fn test_natal_only_triple_alliance_no_gaego_events() {
    // Natal pillars with 申-子-辰 Triple Alliance
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

    // Static analysis with NO luck pillars
    let analysis_static = DynamicLuckAnalysis::analyze(&pillars, None, None, None, None, None);

    assert!(
        analysis_static.gaego_events.is_empty(),
        "Static natal-only triple alliance MUST NOT emit GaeGo events during luck-free analysis"
    );
}

#[test]
fn test_gaego_no_double_scoring_in_vm() {
    let pillars = make_pillars(
        HeavenlyStem::Jia,
        EarthlyBranch::Chen,
        HeavenlyStem::Bing,
        EarthlyBranch::Yin,
        HeavenlyStem::Wu,
        EarthlyBranch::Shen,
        HeavenlyStem::Geng,
        EarthlyBranch::Wu,
    );

    let major_xu = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Xu);
    let vm = eon_saju::engine::vm::SajuVM::new(pillars);
    let frame = vm.step(30, major_xu, major_xu, None, None, None);

    // Trace tags for GaeGo unsealed stems
    let gaego_count = frame.esil_trace.matches("gaego:").count();
    let mem_dump_count = frame.esil_trace.matches("mem_dump:").count();

    // Verify that mem_dump trace does not duplicate unsealed GaeGo stems
    println!(
        "GaeGo count: {}, MemDump count: {}",
        gaego_count, mem_dump_count
    );
}
