use crate::dto::*;
use crate::facade;
use chrono::{Utc, TimeZone};

#[test]
fn test_tier_v3_contract_snapshot() {
    // 1. Setup a stable input
    let birth = AnalysisInput {
        year: 1985,
        month: 11,
        day: 27,
        hour: 14,
        minute: 30,
        is_lunar: false,
        is_leap_month: false,
        lat: 37.5665,
        lon: 126.9780,
        timezone: "Asia/Seoul".to_string(),
    };

    let saju_input = SajuAnalysisInput {
        base: birth.clone(),
        is_male: true,
        use_night_rat_hour: false,
        precision: BirthTimePrecision::Exact,
    };

    let now = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    
    let vedic_input = VedicAnalysisInput::new(
        birth.year, birth.month, birth.day, birth.hour, birth.minute,
        birth.is_lunar, birth.is_leap_month,
        birth.lat, birth.lon, birth.timezone.clone(),
        Some(false), Some(now)
    );

    let transit_input = TransitAnalysisInput::new(
        birth.year, birth.month, birth.day, birth.hour, birth.minute,
        birth.is_lunar, birth.is_leap_month,
        saju_input.is_male, saju_input.use_night_rat_hour,
        birth.lon, birth.lat, birth.timezone.clone(),
        Some(false), Some(now)
    );

    // 2. Execute analysis
    let saju_res = facade::analyze_saju(saju_input).expect("Saju failed");
    let vedic_res = facade::analyze_vedic(vedic_input).expect("Vedic failed");
    let transit_res = facade::analyze_transit(transit_input).ok();

    let tier_res = facade::analyze_destiny_tier(saju_res, vedic_res, transit_res).expect("Tier analysis failed");

    // 3. Verify V3 Contract Fields (Manual Check Points)
    assert_eq!(tier_res.tier_model_version, "3.0.0");
    assert_eq!(tier_res.version, "v3_spread_model");
    
    // Detailed components must exist and have 12 items
    assert_eq!(tier_res.detailed_components.len(), 12);
    
    // Check specific keys to ensure contract alignment with frontend
    let keys: Vec<String> = tier_res.detailed_components.iter().map(|c| c.key.clone()).collect();
    assert!(keys.contains(&"saju_strength".to_string()));
    assert!(keys.contains(&"vedic_houses".to_string()));
    assert!(keys.contains(&"luck_cycle".to_string()));
    
    // Verify scores are within range
    assert!(tier_res.destiny_score >= 0.0 && tier_res.destiny_score <= 100.0);
    assert!(tier_res.destiny_tier_score >= 0.0 && tier_res.destiny_tier_score <= 100.0);
    
    // Verify camelCase serialization (via serde test if needed, but here we check struct fields)
    // The dto.rs has #[serde(rename_all = "camelCase")] for TierResult's components.
}
