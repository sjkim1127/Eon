//! 🧪 Milestone 1 (R1) Part 2 Empirical Adversarial Stress Tests
//!
//! Empirical adversarial test suite verifying:
//! 1. Samjae calculation across 12 birth year branches for 입삼재, 눌삼재, 날삼재.
//! 2. Gongmang dissolution when voided branch is clashed or combined (and bug detection).
//! 3. Noble Spirit Marker annulment (`(귀인공망)`) on voided branches vs restoration on clash/combination (`(공망해충/해합 구원)`).
//! 4. 12-Unseong Yin-stem option (`yin_stem_reverse: false` vs `true`).

use eon_saju::analysis::shinsal::{calculate_samjae, SamjaeStage};
use eon_saju::analysis::spirit_markers::SpiritMarker;
use eon_saju::analysis::supplementary_pillars::InterpretationLevel;
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::config::AnalysisConfig;
use eon_saju::core::ganzi::GanZi;
use eon_saju::core::pillars::FourPillars;
use eon_saju::core::stem::HeavenlyStem;
use eon_saju::core::twelve_stages::{calculate_twelve_stage_with_config, TwelveStage};

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
    let dummy_input = eon_saju::SajuInput::new_solar(1990, 1, 1, 12, 0);
    FourPillars {
        year: GanZi::new(y_s, y_b),
        month: GanZi::new(m_s, m_b),
        day: GanZi::new(d_s, d_b),
        hour: GanZi::new(h_s, h_b),
        birth_time: chrono::Utc::now(),
        gender: eon_core::Gender::Male,
        raw_input: dummy_input,
        supplementary_pillars: Default::default(),
    }
}

// ----------------------------------------------------
// 1. Samjae calculation across 12 birth year branches
// ----------------------------------------------------
#[test]
fn test_samjae_12_birth_year_branches() {
    let all_branches = EarthlyBranch::ALL;

    for &year_b in &all_branches {
        for &transit_b in &all_branches {
            let samjae = calculate_samjae(year_b, transit_b);

            let expected = match year_b {
                EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => match transit_b {
                    EarthlyBranch::Yin => Some(SamjaeStage::Entrance),
                    EarthlyBranch::Mao => Some(SamjaeStage::Dwelling),
                    EarthlyBranch::Chen => Some(SamjaeStage::Exit),
                    _ => None,
                },
                EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => match transit_b {
                    EarthlyBranch::Shen => Some(SamjaeStage::Entrance),
                    EarthlyBranch::You => Some(SamjaeStage::Dwelling),
                    EarthlyBranch::Xu => Some(SamjaeStage::Exit),
                    _ => None,
                },
                EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => match transit_b {
                    EarthlyBranch::Hai => Some(SamjaeStage::Entrance),
                    EarthlyBranch::Zi => Some(SamjaeStage::Dwelling),
                    EarthlyBranch::Chou => Some(SamjaeStage::Exit),
                    _ => None,
                },
                EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => match transit_b {
                    EarthlyBranch::Si => Some(SamjaeStage::Entrance),
                    EarthlyBranch::Wu => Some(SamjaeStage::Dwelling),
                    EarthlyBranch::Wei => Some(SamjaeStage::Exit),
                    _ => None,
                },
            };

            assert_eq!(
                samjae, expected,
                "Samjae mismatch for birth year {:?} and transit year {:?}",
                year_b, transit_b
            );
        }
    }
}

// ----------------------------------------------------
// 2. Gongmang dissolution when voided branch is clashed or combined
// ----------------------------------------------------
#[test]
fn test_gongmang_dissolution_clash_and_combination() {
    // Day pillar: 甲子 (Void branches: 戌, 亥)
    // Month pillar: 戊戌 (Voided branch 戌)
    // Year pillar: 庚辰 (Branch 辰 clashes with 戌 -> 辰戌沖)
    // Hour pillar: 丙寅
    let pillars_clash = make_pillars(
        HeavenlyStem::Geng,
        EarthlyBranch::Chen,
        HeavenlyStem::Wu,
        EarthlyBranch::Xu,
        HeavenlyStem::Jia,
        EarthlyBranch::Zi,
        HeavenlyStem::Bing,
        EarthlyBranch::Yin,
    );

    let void_analysis_clash = pillars_clash.void_analysis();
    let xu_void = void_analysis_clash
        .mapped_voids
        .iter()
        .find(|v| v.branch == EarthlyBranch::Xu);

    assert!(
        xu_void.is_some(),
        "Month branch 戌 must be identified as void"
    );
    let xu_v = xu_void.unwrap();
    assert!(
        xu_v.is_dissolved,
        "Voided branch 戌 must be dissolved due to 辰戌沖"
    );
    assert!(
        xu_v.dissolution_reason.as_ref().unwrap().contains("충(沖)"),
        "Dissolution reason must state clash"
    );

    // Day pillar: 甲子 (Void branches: 戌, 亥)
    // Month pillar: 戊戌 (Voided branch 戌)
    // Year pillar: 己卯 (Branch 卯 combines with 戌 -> 卯戌六合)
    // Hour pillar: 丙寅
    let pillars_comb = make_pillars(
        HeavenlyStem::Ji,
        EarthlyBranch::Mao,
        HeavenlyStem::Wu,
        EarthlyBranch::Xu,
        HeavenlyStem::Jia,
        EarthlyBranch::Zi,
        HeavenlyStem::Bing,
        EarthlyBranch::Yin,
    );

    let void_analysis_comb = pillars_comb.void_analysis();
    let xu_void_comb = void_analysis_comb
        .mapped_voids
        .iter()
        .find(|v| v.branch == EarthlyBranch::Xu);

    assert!(xu_void_comb.is_some());
    let xu_vc = xu_void_comb.unwrap();
    assert!(
        xu_vc.is_dissolved,
        "Voided branch 戌 must be dissolved due to 卯戌六合"
    );
    assert!(
        xu_vc
            .dissolution_reason
            .as_ref()
            .unwrap()
            .contains("육합(六合)"),
        "Dissolution reason must state six combination"
    );
}

/// Adversarial stress test: check false positive dissolution when Seasonal Combination exists in chart but does NOT contain the void branch.
#[test]
fn test_gongmang_dissolution_unrelated_seasonal_comb_bug() {
    // Day pillar: 丙辰 (xun_start=2 -> 갑인순, void branches: 子, 丑)
    // Hour pillar: 丁丑 (Voided branch 丑)
    // Year pillar: 戊寅
    // Month pillar: 乙卯
    // Note: Year(寅), Month(卯), Day(辰) forms 寅卯辰 Seasonal Combination (방합/木局).
    // Hour branch is 丑 (Earth). 丑 is NOT part of 寅卯辰!
    let pillars = make_pillars(
        HeavenlyStem::Wu,
        EarthlyBranch::Yin,
        HeavenlyStem::Yi,
        EarthlyBranch::Mao,
        HeavenlyStem::Bing,
        EarthlyBranch::Chen,
        HeavenlyStem::Ding,
        EarthlyBranch::Chou,
    );

    let void_analysis = pillars.void_analysis();
    let chou_void = void_analysis
        .mapped_voids
        .iter()
        .find(|v| v.branch == EarthlyBranch::Chou);

    assert!(
        chou_void.is_some(),
        "Hour branch 丑 must be identified as void"
    );
    let cv = chou_void.unwrap();

    println!("Chou void is_dissolved: {}", cv.is_dissolved);
    if let Some(ref r) = cv.dissolution_reason {
        println!("Chou void dissolution reason: {}", r);
    }

    assert!(
        !cv.is_dissolved,
        "Hour branch 丑 is not part of 寅卯辰 Seasonal Combination, must NOT be marked dissolved"
    );
    assert_eq!(cv.dissolution_reason, None);
}

// ----------------------------------------------------
// 3. Noble Spirit Marker annulment vs restoration (and String Mismatch Bug)
// ----------------------------------------------------
#[test]
fn test_noble_spirit_annulment_and_string_mismatch_bug() {
    // Day stem: 庚 (Tianyi noble branches: 丑, 未)
    // Day pillar: 庚寅 (Xun group: 甲申旬 -> Void branches: 午, 未)
    // Year pillar: 癸未 (Branch 未 is Tianyi Noble AND Void!)
    // Month pillar: 丙子 (no clash or combination with 未)
    // Hour pillar: 戊寅
    let pillars_void_noble = make_pillars(
        HeavenlyStem::Gui,
        EarthlyBranch::Wei,
        HeavenlyStem::Bing,
        EarthlyBranch::Zi,
        HeavenlyStem::Geng,
        EarthlyBranch::Yin,
        HeavenlyStem::Wu,
        EarthlyBranch::Yin,
    );

    let spirit_analysis = pillars_void_noble.spirit_markers();
    let tianyi_detail = spirit_analysis.mapped_markers.iter().find(|m| {
        m.marker == SpiritMarker::Tianyi
            && m.position == eon_saju::analysis::spirit_markers::PillarPosition::Year
    });

    assert!(
        tianyi_detail.is_some(),
        "Tianyi noble must be found at Year pillar"
    );
    let td = tianyi_detail.unwrap();
    println!("Void Tianyi summary: {}", td.summary);
    println!("Void Tianyi level: {:?}", td.level);
    assert!(
        td.summary.contains("(귀인공망)"),
        "Unclashed void Tianyi must have '(귀인공망)' in summary"
    );
    assert_eq!(
        td.level,
        InterpretationLevel::Neutral,
        "Unclashed void Tianyi must be annulled to Neutral"
    );

    // Test String Mismatch Bug on Restoration:
    // Year: 癸未 (Void Tianyi), Month: 己丑 (丑未沖), Day: 庚寅, Hour: 戊寅
    // In spirit_markers.rs, m.position.hangul() returns "년주", while rel_analysis.branch_clashes stores "년지".
    // "년주" != "년지", so is_clashed remains false in spirit_markers.rs!
    let pillars_clash_restore = make_pillars(
        HeavenlyStem::Gui,
        EarthlyBranch::Wei,
        HeavenlyStem::Ji,
        EarthlyBranch::Chou,
        HeavenlyStem::Geng,
        EarthlyBranch::Yin,
        HeavenlyStem::Wu,
        EarthlyBranch::Yin,
    );

    let spirit_analysis_clash = pillars_clash_restore.spirit_markers();
    let tianyi_clash = spirit_analysis_clash.mapped_markers.iter().find(|m| {
        m.marker == SpiritMarker::Tianyi
            && m.position == eon_saju::analysis::spirit_markers::PillarPosition::Year
    });

    assert!(tianyi_clash.is_some());
    let tc = tianyi_clash.unwrap();
    println!("Clashed restored Tianyi summary (actual): {}", tc.summary);

    assert!(
        tc.is_clashed,
        "Year pillar 未 is clashed with Month pillar 丑 (丑未沖), is_clashed must be true"
    );
    assert!(
        tc.summary.contains("(공망해충/해합 구원)"),
        "Clashed void Tianyi noble spirit must be restored with '(공망해충/해합 구원)' in summary, got: {}",
        tc.summary
    );
}

// ----------------------------------------------------
// 4. 12-Unseong Yin-stem option (yin_stem_reverse: false vs true)
// ----------------------------------------------------
#[test]
fn test_twelve_stages_yin_stem_reverse_config() {
    let mut config_reverse = AnalysisConfig::default();
    config_reverse.yin_stem_reverse = true;

    let mut config_forward = AnalysisConfig::default();
    config_forward.yin_stem_reverse = false;

    // Test 乙 (Yi Wood): Changsheng is 午
    // With reverse=true:
    // 乙 @ 午 = Changsheng
    // 乙 @ 巳 = Muyu
    // 乙 @ 卯 = Jianlu
    // 乙 @ 寅 = Diwang
    // 乙 @ 亥 = Si
    assert_eq!(
        calculate_twelve_stage_with_config(HeavenlyStem::Yi, EarthlyBranch::Wu, &config_reverse),
        TwelveStage::Changsheng
    );
    assert_eq!(
        calculate_twelve_stage_with_config(HeavenlyStem::Yi, EarthlyBranch::Si, &config_reverse),
        TwelveStage::Muyu
    );
    assert_eq!(
        calculate_twelve_stage_with_config(HeavenlyStem::Yi, EarthlyBranch::Mao, &config_reverse),
        TwelveStage::Jianlu
    );
    assert_eq!(
        calculate_twelve_stage_with_config(HeavenlyStem::Yi, EarthlyBranch::Yin, &config_reverse),
        TwelveStage::Diwang
    );
    assert_eq!(
        calculate_twelve_stage_with_config(HeavenlyStem::Yi, EarthlyBranch::Hai, &config_reverse),
        TwelveStage::Si
    );

    // With reverse=false (forward):
    // 乙 @ 午 = Changsheng
    // 乙 @ 未 = Muyu
    // 乙 @ 申 = Guandai
    // 乙 @ 酉 = Jianlu
    // 乙 @ 戌 = Diwang
    // 乙 @ 亥 = Shuai
    assert_eq!(
        calculate_twelve_stage_with_config(HeavenlyStem::Yi, EarthlyBranch::Wu, &config_forward),
        TwelveStage::Changsheng
    );
    assert_eq!(
        calculate_twelve_stage_with_config(HeavenlyStem::Yi, EarthlyBranch::Wei, &config_forward),
        TwelveStage::Muyu
    );
    assert_eq!(
        calculate_twelve_stage_with_config(HeavenlyStem::Yi, EarthlyBranch::Shen, &config_forward),
        TwelveStage::Guandai
    );
    assert_eq!(
        calculate_twelve_stage_with_config(HeavenlyStem::Yi, EarthlyBranch::You, &config_forward),
        TwelveStage::Jianlu
    );
    assert_eq!(
        calculate_twelve_stage_with_config(HeavenlyStem::Yi, EarthlyBranch::Xu, &config_forward),
        TwelveStage::Diwang
    );
    assert_eq!(
        calculate_twelve_stage_with_config(HeavenlyStem::Yi, EarthlyBranch::Hai, &config_forward),
        TwelveStage::Shuai
    );

    // Yang stem (甲 Geng): should remain unchanged regardless of yin_stem_reverse setting
    assert_eq!(
        calculate_twelve_stage_with_config(HeavenlyStem::Jia, EarthlyBranch::Hai, &config_reverse),
        calculate_twelve_stage_with_config(HeavenlyStem::Jia, EarthlyBranch::Hai, &config_forward)
    );
}
