//! 🧪 Milestone 1 (R1) Yongsin & Gyeokguk Stress Tests
//!
//! Empirical adversarial test suite verifying:
//! 1. Weak DM with heavy Caisheng (재다신약) -> Yongsin is BiGeop.
//! 2. Equal 50/50 Metal-Wood clash -> Tonggwan gets primary Yongsin (Water).
//! 3. Extreme Winter/Summer charts -> Johu stem preference (丙火 / 癸水).
//! 4. Jin-Jong vs Ga-Jong -> Root score checks differentiate true vs fake Jong (and bug verification).
//! 5. Samhap Jeonwang -> 곡직격, 염상격, 가색격, 종혁격, 윤하격.

use eon_saju::analysis::structure::StructureType;
use eon_saju::analysis::yongshin::YongshinType;
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::element::Element;
use eon_saju::core::ganzi::GanZi;
use eon_saju::core::pillars::FourPillars;
use eon_saju::core::stem::HeavenlyStem;

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
// 1. Weak DM with heavy Caisheng (재다신약): Yongsin = BiGeop
// ----------------------------------------------------
#[test]
fn test_m1_weak_dm_heavy_caisheng() {
    // DM: 甲木
    // Year: 戊辰 (Earth/Earth)
    // Month: 己未 (Earth/Earth)
    // Day: 甲辰 (Wood DM / Earth)
    // Hour: 甲戌 (Wood / Earth)
    let pillars = make_pillars(
        HeavenlyStem::Wu,
        EarthlyBranch::Chen,
        HeavenlyStem::Ji,
        EarthlyBranch::Wei,
        HeavenlyStem::Jia,
        EarthlyBranch::Chen,
        HeavenlyStem::Jia,
        EarthlyBranch::Xu,
    );

    let strength = pillars.strength();
    let yongshin = pillars.yongshin();
    let structure = pillars.structure();

    println!("Strength: {:?}", strength.strength_type);
    println!("Structure: {:?}", structure.structure);
    println!("Primary Yongshin: {:?}", yongshin.primary);

    assert_eq!(
        strength.strength_type,
        eon_saju::analysis::strength::StrengthType::Weak
    );

    let eokbu_rec = yongshin
        .recommendations
        .iter()
        .find(|r| r.yongshin_type == YongshinType::Eokbu);
    assert!(eokbu_rec.is_some(), "Eokbu Yongshin should be present");
    assert_eq!(
        eokbu_rec.unwrap().element,
        Element::Wood,
        "Yongsin for 재다신약 must be BiGeop (Wood)"
    );
}

// ----------------------------------------------------
// 2. Equal 50/50 Metal-Wood clash: Tonggwan gets primary Yongsin
// ----------------------------------------------------
#[test]
fn test_m1_equal_metal_wood_clash_tonggwan() {
    // Year: 庚申 (Metal / Metal)
    // Month: 庚申 (Metal / Metal)
    // Day: 甲寅 (Wood DM / Wood)
    // Hour: 甲寅 (Wood / Wood)
    let pillars = make_pillars(
        HeavenlyStem::Geng,
        EarthlyBranch::Shen,
        HeavenlyStem::Geng,
        EarthlyBranch::Shen,
        HeavenlyStem::Jia,
        EarthlyBranch::Yin,
        HeavenlyStem::Jia,
        EarthlyBranch::Yin,
    );

    let yongshin = pillars.yongshin();

    let tonggwan_rec = yongshin
        .recommendations
        .iter()
        .find(|r| r.yongshin_type == YongshinType::Tonggwan);

    assert!(
        tonggwan_rec.is_some(),
        "Tonggwan Yongshin should be triggered for 50/50 clash"
    );
    assert_eq!(
        tonggwan_rec.unwrap().element,
        Element::Water,
        "Tonggwan element for Metal-Wood clash must be Water"
    );
    assert_eq!(
        yongshin.primary,
        Element::Water,
        "Primary Yongshin for equal 50/50 clash must be Tonggwan (Water)"
    );
}

// ----------------------------------------------------
// 3. Extreme Winter/Summer charts: Johu stem preference (丙火 / 癸水)
// ----------------------------------------------------
#[test]
fn test_m1_extreme_winter_johu_stem_bing() {
    let pillars = make_pillars(
        HeavenlyStem::Ren,
        EarthlyBranch::Zi,
        HeavenlyStem::Ren,
        EarthlyBranch::Zi,
        HeavenlyStem::Jia,
        EarthlyBranch::Zi,
        HeavenlyStem::Bing,
        EarthlyBranch::Zi,
    );

    let yongshin = pillars.yongshin();
    let johu_rec = yongshin
        .recommendations
        .iter()
        .find(|r| r.yongshin_type == YongshinType::Johu);

    assert!(johu_rec.is_some());
    let johu = johu_rec.unwrap();
    assert_eq!(johu.element, Element::Fire);
    assert!(
        johu.preferred_stems
            .as_ref()
            .unwrap()
            .contains(&HeavenlyStem::Bing),
        "Extreme winter Johu preferred stem must include 丙火 (Bing)"
    );
}

#[test]
fn test_m1_extreme_summer_johu_stem_gui() {
    let pillars = make_pillars(
        HeavenlyStem::Bing,
        EarthlyBranch::Wu,
        HeavenlyStem::Bing,
        EarthlyBranch::Wu,
        HeavenlyStem::Jia,
        EarthlyBranch::Wu,
        HeavenlyStem::Wu,
        EarthlyBranch::Wu,
    );

    let yongshin = pillars.yongshin();
    let johu_rec = yongshin
        .recommendations
        .iter()
        .find(|r| r.yongshin_type == YongshinType::Johu);

    assert!(johu_rec.is_some());
    let johu = johu_rec.unwrap();
    assert_eq!(johu.element, Element::Water);
    assert!(
        johu.preferred_stems
            .as_ref()
            .unwrap()
            .contains(&HeavenlyStem::Gui),
        "Extreme summer Johu preferred stem must include 癸水 (Gui)"
    );
}

// ----------------------------------------------------
// 4. Jin-Jong vs Ga-Jong: Differentiation & Bug Detection
// ----------------------------------------------------
#[test]
fn test_m1_jin_jong_vs_ga_jong_root_check() {
    // DM: 丙火 (Fire)
    // ShiShang (Earth): 戊戌, 己未, 戊辰
    // Pure ShiShang Follower chart (JongAh):
    // Year: 戊辰 (Earth) - 辰 has 乙, 癸 (Inseong/BiGeop roots for 丙火? No! Wood is Inseong for Fire, 乙 is Wood!)
    // Let's check DM 丙火 (Fire):
    // Inseong for Fire is Wood (甲, 乙). BiGeop is Fire (丙, 丁).
    // Branch 戌 has 辛, 丁(Fire), 戊. (Contains 丁 Fire root!)
    // Branch 巳 has 丙(Fire), 庚, 戊. (Contains 丙 Fire root!)
    // Branch 申 has 戊, 壬, 庚. (NO Fire, NO Wood root!)
    // Branch 酉 has 辛. (NO Fire, NO Wood root!)

    // True Jin-Jong (No Fire/Wood root in 申, 酉, 丑, 亥):
    // Note: Due to DeukSe count bug (shishang=2, cai=2, guan=2), this chart evaluates to JongAh/GaJongAh.
    let pillars_jin = make_pillars(
        HeavenlyStem::Wu,
        EarthlyBranch::Shen,
        HeavenlyStem::Ji,
        EarthlyBranch::You,
        HeavenlyStem::Bing,
        EarthlyBranch::Shen,
        HeavenlyStem::Wu,
        EarthlyBranch::You,
    );

    let struct_jin = pillars_jin.structure();
    println!("Jin Jong Structure: {:?}", struct_jin.structure);
    assert_eq!(
        struct_jin.structure,
        StructureType::JongJae,
        "No Wood/Fire root in branches & Wealth energy dominant -> Jin-JongJae (眞從財格)"
    );

    // Fake Ga-Jong (Contains hidden Wood/Fire root, e.g. 寅 in hour branch):
    let pillars_ga = make_pillars(
        HeavenlyStem::Wu,
        EarthlyBranch::Shen,
        HeavenlyStem::Ji,
        EarthlyBranch::You,
        HeavenlyStem::Bing,
        EarthlyBranch::Shen,
        HeavenlyStem::Wu,
        EarthlyBranch::Yin, // 寅 contains 甲, 丙, 戊 -> Fire & Wood root!
    );

    let struct_ga = pillars_ga.structure();
    println!("Ga Jong Structure: {:?}", struct_ga.structure);
    assert_eq!(
        struct_ga.structure,
        StructureType::GaJongJae,
        "Contains Wood/Fire root in Yin branch -> Ga-JongJae (假從財格)"
    );
}

#[test]
fn test_m1_bug_deuk_se_count_prevents_jong_jae() {
    // EMPIRICAL BUG DEMONSTRATION:
    // Chart with DM 甲木, pure Earth (Caisheng): 戊戌, 己未, 甲辰, 己丑.
    // Earth is > 85% (Caisheng). 식상 (Fire) is 0%.
    // Expected Structure: JongJae (진종재격) or GaJongJae.
    // Actual Structure: JongAh (진종아격) because shishang_count == caisheng_count == 2 in DeukSe!
    let pillars_cai = make_pillars(
        HeavenlyStem::Wu,
        EarthlyBranch::Xu,
        HeavenlyStem::Ji,
        EarthlyBranch::Wei,
        HeavenlyStem::Jia,
        EarthlyBranch::Chen,
        HeavenlyStem::Ji,
        EarthlyBranch::Chou,
    );

    let struct_cai = pillars_cai.structure();
    println!(
        "Heavy Wealth Chart Structure Actual: {:?}",
        struct_cai.structure
    );

    assert!(
        matches!(
            struct_cai.structure,
            StructureType::GaJongJae | StructureType::JongJae
        ),
        "Heavy Wealth chart must be classified as GaJongJae or JongJae, got {:?}",
        struct_cai.structure
    );
}

// ----------------------------------------------------
// 5. Samhap Jeonwang: 곡직격, 염상격, 가색격, 종혁격, 윤하격
// ----------------------------------------------------
#[test]
fn test_m1_samhap_jeonwang_all_five() {
    // 1. 곡직격 (GokJik - Wood)
    let gokjik = make_pillars(
        HeavenlyStem::Gui,
        EarthlyBranch::Hai,
        HeavenlyStem::Yi,
        EarthlyBranch::Mao,
        HeavenlyStem::Jia,
        EarthlyBranch::Yin,
        HeavenlyStem::Gui,
        EarthlyBranch::Wei,
    );
    assert_eq!(gokjik.structure().structure, StructureType::GokJik);

    // 2. 염상격 (YeomSang - Fire)
    let yeomsang = make_pillars(
        HeavenlyStem::Jia,
        EarthlyBranch::Yin,
        HeavenlyStem::Bing,
        EarthlyBranch::Wu,
        HeavenlyStem::Bing,
        EarthlyBranch::Xu,
        HeavenlyStem::Ding,
        EarthlyBranch::Si,
    );
    assert_eq!(yeomsang.structure().structure, StructureType::YeomSang);

    // 3. 가색격 (GaSaek - Earth)
    let gasaek = make_pillars(
        HeavenlyStem::Wu,
        EarthlyBranch::Chen,
        HeavenlyStem::Ji,
        EarthlyBranch::Wei,
        HeavenlyStem::Wu,
        EarthlyBranch::Xu,
        HeavenlyStem::Ji,
        EarthlyBranch::Chou,
    );
    assert_eq!(gasaek.structure().structure, StructureType::GaSaek);

    // 4. 종혁격 (JongHyeok - Metal)
    let jonghyeok = make_pillars(
        HeavenlyStem::Ji,
        EarthlyBranch::Si,
        HeavenlyStem::Xin,
        EarthlyBranch::You,
        HeavenlyStem::Geng,
        EarthlyBranch::Chou,
        HeavenlyStem::Geng,
        EarthlyBranch::Shen,
    );
    assert_eq!(jonghyeok.structure().structure, StructureType::JongHyeok);

    // 5. 윤하격 (YoonHa - Water)
    let yoonha = make_pillars(
        HeavenlyStem::Geng,
        EarthlyBranch::Shen,
        HeavenlyStem::Ren,
        EarthlyBranch::Zi,
        HeavenlyStem::Ren,
        EarthlyBranch::Chen,
        HeavenlyStem::Gui,
        EarthlyBranch::Hai,
    );
    assert_eq!(yoonha.structure().structure, StructureType::YoonHa);
}
