use crate::dto::{
    AnalysisInput, OmniDestinyTierInput, SajuAnalysisInput, TransitAnalysisInput,
    VedicAnalysisInput, WesternAnalysisInput, ZwdsAnalysisInput,
};
use crate::facade;

fn create_sample_base_input() -> AnalysisInput {
    AnalysisInput {
        year: 1990,
        month: 5,
        day: 15,
        hour: 10,
        minute: 30,
        is_lunar: false,
        is_leap_month: false,
        lat: 37.5665,
        lon: 126.9780,
        timezone: "Asia/Seoul".to_string(),
    }
}

#[test]
fn test_omni_destiny_tier_synthesis() {
    let base = create_sample_base_input();

    let saju_input = SajuAnalysisInput::new(base.clone(), true, false, Some(false));
    let vedic_input = VedicAnalysisInput::new(base.clone(), Some(false), None);
    let western_input = WesternAnalysisInput::new(base.clone(), "Placidus".to_string());
    let zwds_input = ZwdsAnalysisInput::new(base.clone(), true, None);

    let saju_res = facade::analyze_saju(saju_input).unwrap();
    let vedic_res = facade::analyze_vedic(vedic_input).unwrap();
    let western_res = facade::analyze_western(western_input).ok();
    let zwds_res = facade::analyze_zwds(zwds_input).ok();

    let omni_input = OmniDestinyTierInput {
        saju: saju_res,
        vedic: vedic_res,
        western: western_res,
        zwds: zwds_res,
        transit: None,
    };

    let tier_res = facade::analyze_destiny_tier_omni(omni_input).unwrap();

    assert_eq!(tier_res.version, "v5.1_quantum_synergy_model");
    assert_eq!(tier_res.tier_model_version, "5.1.0");
    assert!(tier_res.destiny_tier_score >= 0.0 && tier_res.destiny_tier_score <= 100.0);
    assert!(!tier_res.detailed_components.is_empty());
    assert!(tier_res
        .detailed_components
        .iter()
        .any(|c| c.key == "western_astrology"));
    assert!(tier_res
        .detailed_components
        .iter()
        .any(|c| c.key == "zwds_harmony"));

    // Destiny Tier 5.1 Verification
    assert!(!tier_res.quantum_synergies.is_empty());
    assert_eq!(tier_res.domain_radar.len(), 8);
    assert!(!tier_res.tier_trajectory.is_empty());
}

#[test]
fn test_omni_destiny_tier_female_case() {
    let base = create_sample_base_input();

    let saju_input = SajuAnalysisInput::new(base.clone(), false, false, Some(false));
    let vedic_input = VedicAnalysisInput::new(base.clone(), Some(false), None);
    let western_input = WesternAnalysisInput::new(base.clone(), "Placidus".to_string());
    let zwds_input = ZwdsAnalysisInput::new(base.clone(), false, None);

    let saju_res = facade::analyze_saju(saju_input).unwrap();
    let vedic_res = facade::analyze_vedic(vedic_input).unwrap();
    let western_res = facade::analyze_western(western_input).ok();
    let zwds_res = facade::analyze_zwds(zwds_input).ok();

    let omni_input = OmniDestinyTierInput {
        saju: saju_res,
        vedic: vedic_res,
        western: western_res,
        zwds: zwds_res,
        transit: None,
    };

    let tier_res = facade::analyze_destiny_tier_omni(omni_input).unwrap();
    assert!(tier_res.destiny_tier_score >= 0.0 && tier_res.destiny_tier_score <= 100.0);
    assert_eq!(tier_res.domain_radar.len(), 8);
}

#[test]
fn test_omni_destiny_tier_with_transit() {
    let base = create_sample_base_input();

    let saju_input = SajuAnalysisInput::new(base.clone(), true, false, Some(false));
    let vedic_input = VedicAnalysisInput::new(base.clone(), Some(false), None);
    let transit_input = TransitAnalysisInput::new(saju_input.clone(), None);

    let saju_res = facade::analyze_saju(saju_input).unwrap();
    let vedic_res = facade::analyze_vedic(vedic_input).unwrap();
    let transit_res = facade::analyze_transit(transit_input).unwrap();

    let omni_input = OmniDestinyTierInput {
        saju: saju_res,
        vedic: vedic_res,
        western: None,
        zwds: None,
        transit: Some(transit_res),
    };

    let tier_res = facade::analyze_destiny_tier_omni(omni_input).unwrap();
    assert!(tier_res.destiny_score >= 0.0);
    assert_eq!(tier_res.domain_radar.len(), 8);
}

#[test]
fn test_omni_destiny_tier_weight_sum_is_one() {
    let base = create_sample_base_input();

    let saju_input = SajuAnalysisInput::new(base.clone(), true, false, Some(false));
    let vedic_input = VedicAnalysisInput::new(base.clone(), Some(false), None);

    let saju_res = facade::analyze_saju(saju_input).unwrap();
    let vedic_res = facade::analyze_vedic(vedic_input).unwrap();

    let omni_input = OmniDestinyTierInput {
        saju: saju_res,
        vedic: vedic_res,
        western: None,
        zwds: None,
        transit: None,
    };

    let tier_res = facade::analyze_destiny_tier_omni(omni_input).unwrap();

    let weight_sum: f32 = tier_res.detailed_components.iter().map(|c| c.weight).sum();
    assert!(
        (weight_sum - 1.0).abs() < 1e-4,
        "Weights must sum to 1.0, got {:.4}",
        weight_sum
    );
}

#[test]
fn test_legacy_analyze_delegation_parity() {
    let base = create_sample_base_input();

    let saju_input = SajuAnalysisInput::new(base.clone(), true, false, Some(false));
    let vedic_input = VedicAnalysisInput::new(base.clone(), Some(false), None);

    let saju_res = facade::analyze_saju(saju_input).unwrap();
    let vedic_res = facade::analyze_vedic(vedic_input).unwrap();

    let legacy_res =
        facade::analyze_destiny_tier(saju_res.clone(), vedic_res.clone(), None).unwrap();

    let omni_input = OmniDestinyTierInput {
        saju: saju_res,
        vedic: vedic_res,
        western: None,
        zwds: None,
        transit: None,
    };
    let omni_res = facade::analyze_destiny_tier_omni(omni_input).unwrap();

    assert_eq!(legacy_res.destiny_score, omni_res.destiny_score);
    assert_eq!(legacy_res.destiny_tier.grade, omni_res.destiny_tier.grade);
}
