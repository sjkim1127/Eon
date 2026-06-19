
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
        birth.clone(),
        Some(false),
        Some(now)
    );

    let transit_input = TransitAnalysisInput::new(
        saju_input.clone(),
        Some(now)
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
}

#[test]
fn test_vedic_contract_snapshot() {
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

    let now = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let vedic_input = VedicAnalysisInput::new(
        birth.clone(),
        Some(false),
        Some(now)
    );

    let res = facade::analyze_vedic(vedic_input).expect("Vedic failed");

    // 1. Jaimini Completion Verification
    assert!(!res.report.all_karakas.is_empty(), "all_karakas must be populated");
    assert!(!res.report.chara_dasha_timeline.is_empty(), "chara_dasha_timeline must be populated");
    
    // 2. Gochara Explainability Verification
    for transit in &res.gochara.transits {
        assert!(!transit.summary.is_empty(), "Gochara summary must not be empty");
        assert!(!transit.description.is_empty(), "Gochara description must not be empty");
        assert!(!transit.reasons.is_empty(), "Gochara reasons must not be empty");
    }

    // 3. Serialization Check (Verify camelCase)
    let json = serde_json::to_string(&res).expect("Serialization failed");
    assert!(json.contains("\"charaDashaTimeline\""), "Should follow camelCase");
    assert!(json.contains("\"allKarakas\""), "Should follow camelCase");
    assert!(json.contains("\"isBeneficTransit\""), "Should follow camelCase");
}
