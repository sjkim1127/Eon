use crate::dto::{
    AnalysisInput, OmniDestinyTierInput, SajuAnalysisInput, VedicAnalysisInput,
    WesternAnalysisInput, ZwdsAnalysisInput,
};
use crate::facade;

#[test]
fn test_omni_destiny_tier_synthesis() {
    let base = AnalysisInput {
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
    };

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

    assert_eq!(tier_res.version, "v4_omni_model");
    assert_eq!(tier_res.tier_model_version, "4.0.0");
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
}
