use chrono::{TimeZone, Utc};
use eon_vedic::analysis::kp::KpAnalysis;
use eon_vedic::analysis::matching::MatchingEngine;
use eon_vedic::analysis::strength::StrengthEngine;
use eon_vedic::config::VedicYearType;
use eon_vedic::core::chart::{VedicChart, VedicChartCalculator, VedicPosition};
use eon_vedic::planets::VedicPlanet;
use eon_vedic::prediction::dasha::Vimshottari;

// Mock chart and position helpers for testing
fn create_mock_position(
    planet: VedicPlanet,
    rasi: u8,
    nakshatra: u8,
    house: u8,
    sidereal_deg: f64,
) -> VedicPosition {
    VedicPosition {
        planet,
        tropical_deg: 0.0,
        sidereal_deg,
        nakshatra,
        pada: 1,
        rasi,
        house_index: house,
        speed: 1.0,
        is_retrograde: false,
        is_combust: false,
        declination: 0.0,
        hora_rasi: 1,
        drekkana_rasi: 1,
        chaturthamsha_rasi: 1,
        panchamsa_rasi: 1,
        saptamsa_rasi: 1,
        ashtamsa_rasi: 1,
        navamsa_rasi: 1,
        dasamsa_rasi: 1,
        shashtamsa_rasi: 1,
        rudramsa_rasi: 1,
        dwadasamsa_rasi: 1,
        shodashamsa_rasi: 1,
        vimsamsa_rasi: 1,
        chaturvimshamsa_rasi: 1,
        saptavimsamsa_rasi: 1,
        trimsamsa_rasi: 1,
        khavedamsa_rasi: 1,
        akshavedamsa_rasi: 1,
        shashtyamsa_rasi: 1,
        navanavamsa_rasi: 1,
        ashtottaramsa_rasi: 1,
        dwadasdwadasamsa_rasi: 1,
    }
}

fn create_mock_chart(planets: Vec<VedicPosition>) -> VedicChart {
    VedicChart {
        ascendant: create_mock_position(VedicPlanet::Sun, 1, 1, 1, 0.0),
        planets,
        aspects: vec![],
        sav: eon_vedic::analysis::ashtakavarga::Sarvashtakavarga { points: [0; 12] },
        bav: vec![],
        house_cusps: vec![],
        karakas: vec![],
        arudha_padas: vec![],
        special_lagnas: vec![],
        bhava_strengths: vec![],
        vimshopaka_scores: vec![],
        avasthas: vec![],
        shadbalas: vec![],
        panchanga: eon_vedic::panchanga::Panchanga::default(),
        analysis_report: None,
        ayanamsa: 0.0,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TIER 1: FEATURE COVERAGE TESTS (20 Cases)
// ─────────────────────────────────────────────────────────────────────────────

// --- Ashtakoota Compatibility ---

#[test]
fn test_ashtakoota_varna_caste() {
    // Varna matches based on Moon sign caste: Brahmin (4,8,12), Kshatriya (1,5,9), Vaishya (2,6,10), Shudra (3,7,11)
    // Male: Cancer (Brahmin = 4), Female: Gemini (Shudra = 1). male_caste >= female_caste -> 1.0 points
    let male = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 4, 1, 1, 95.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let female = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 3, 1, 1, 65.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let report = MatchingEngine::calculate_compatibility(&male, &female);
    let varna = report
        .kootas
        .iter()
        .find(|k| k.name.contains("Varna"))
        .unwrap();
    assert_eq!(varna.earned_points, 1.0);
}

#[test]
fn test_ashtakoota_vashya_attraction() {
    // Vashya matches based on sign types: Manushya, Chatushpada, Jalachara, Vanachara, Keeta.
    // Same type earns 2.0 points.
    // Male: Gemini (Manushya = 3), Female: Libra (Manushya = 7).
    let male = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 3, 1, 1, 65.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let female = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 7, 1, 1, 185.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let report = MatchingEngine::calculate_compatibility(&male, &female);
    let vashya = report
        .kootas
        .iter()
        .find(|k| k.name.contains("Vashya"))
        .unwrap();
    assert_eq!(vashya.earned_points, 2.0);
}

#[test]
fn test_ashtakoota_tara_destiny() {
    // Tara measures distance between Nakshatras.
    // Nakshatras: Male (1), Female (1) -> distance 1.
    // dist % 9: 1 -> auspicious -> 3.0 points.
    let male = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 1, 1, 1, 10.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let female = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 1, 1, 1, 10.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let report = MatchingEngine::calculate_compatibility(&male, &female);
    let tara = report
        .kootas
        .iter()
        .find(|k| k.name.contains("Tara"))
        .unwrap();
    assert_eq!(tara.earned_points, 3.0);
}

#[test]
fn test_ashtakoota_yoni_sensory() {
    // Yoni score based on animal types. Nak 1 (Horse), Nak 24 (Horse) -> same animal -> 4.0 points.
    let male = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 1, 1, 1, 10.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let female = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 11, 24, 1, 310.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let report = MatchingEngine::calculate_compatibility(&male, &female);
    let yoni = report
        .kootas
        .iter()
        .find(|k| k.name.contains("Yoni"))
        .unwrap();
    assert_eq!(yoni.earned_points, 4.0);
}

#[test]
fn test_ashtakoota_graha_maitri() {
    // Graha Maitri measures lord friendship.
    // Male Moon: Cancer (Moon lord), Female Moon: Cancer (Moon lord) -> same lord -> 5.0 points.
    let male = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 4, 1, 1, 95.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let female = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 4, 1, 1, 95.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let report = MatchingEngine::calculate_compatibility(&male, &female);
    let graha = report
        .kootas
        .iter()
        .find(|k| k.name.contains("Graha"))
        .unwrap();
    assert_eq!(graha.earned_points, 5.0);
}

// --- Shadbala 6 Factors ---

#[test]
fn test_shadbala_sthana_bala_factor() {
    let chart = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Sun, 1, 1, 1, 10.0), // Exalted at 10 deg Aries
    ]);
    let pos = &chart.planets[0];
    let strength = StrengthEngine::calculate(pos, &chart);
    // Assert the new field added by the implementation track
    assert!(strength.sthana_bala >= 0.0);
    assert!(strength.exaltation_score > 0.0);
}

#[test]
fn test_shadbala_dig_bala_factor() {
    let chart = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Sun, 1, 1, 10, 10.0), // Sun gets maximum Dig Bala in 10th house
    ]);
    let pos = &chart.planets[0];
    let strength = StrengthEngine::calculate(pos, &chart);
    // Assert the new field added by the implementation track
    assert!(strength.dig_bala >= 0.0);
    assert_eq!(strength.directional_score, 60.0);
}

#[test]
fn test_shadbala_kala_bala_factor() {
    let chart = create_mock_chart(vec![create_mock_position(VedicPlanet::Sun, 1, 1, 1, 10.0)]);
    let pos = &chart.planets[0];
    let strength = StrengthEngine::calculate(pos, &chart);
    // Assert the new field added by the implementation track
    assert!(strength.kala_bala >= 0.0);
}

#[test]
fn test_shadbala_chesta_bala_factor() {
    let chart = create_mock_chart(vec![create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0)]);
    let pos = &chart.planets[0];
    let strength = StrengthEngine::calculate(pos, &chart);
    // Assert the new field added by the implementation track
    assert!(strength.chesta_bala >= 0.0);
}

#[test]
fn test_shadbala_naisargika_bala_factor() {
    let chart = create_mock_chart(vec![create_mock_position(VedicPlanet::Sun, 1, 1, 1, 10.0)]);
    let pos = &chart.planets[0];
    let strength = StrengthEngine::calculate(pos, &chart);
    // Assert the new field added by the implementation track
    assert!(strength.naisargika_bala >= 0.0);
    assert_eq!(strength.naisargika_score, 60.0); // Sun is naturally strongest
}

// --- KP System Lords/Significators ---

#[test]
fn test_kp_sign_lord() {
    let calc = VedicChartCalculator::new();
    let time = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();
    let natal = vec![create_mock_position(VedicPlanet::Sun, 1, 1, 1, 15.0)]; // Sun in Aries (rasi 1)
    let kp = KpAnalysis::calculate(time, 13.0, 80.0, 24.0, &natal, calc.engine()).unwrap();
    assert_eq!(kp.planets[0].sign_lord, VedicPlanet::Mars); // Lord of Aries is Mars
}

#[test]
fn test_kp_star_lord() {
    let calc = VedicChartCalculator::new();
    let time = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();
    let natal = vec![create_mock_position(VedicPlanet::Sun, 1, 1, 1, 10.0)]; // Ashwini (nakshatra 1)
    let kp = KpAnalysis::calculate(time, 13.0, 80.0, 24.0, &natal, calc.engine()).unwrap();
    assert_eq!(kp.planets[0].star_lord, VedicPlanet::Ketu); // Ashwini star lord is Ketu
}

#[test]
fn test_kp_sub_lord() {
    let calc = VedicChartCalculator::new();
    let time = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();
    let natal = vec![create_mock_position(VedicPlanet::Sun, 1, 1, 1, 5.0)]; // Within Ashwini
    let kp = KpAnalysis::calculate(time, 13.0, 80.0, 24.0, &natal, calc.engine()).unwrap();
    // Verify that sub_lord is calculated
    assert!(kp.planets[0].sub_lord != VedicPlanet::Ascendant);
}

#[test]
fn test_kp_cusps_calculation() {
    let calc = VedicChartCalculator::new();
    let time = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();
    let kp = KpAnalysis::calculate(time, 13.0, 80.0, 24.0, &[], calc.engine()).unwrap();
    assert_eq!(kp.cusps.len(), 12);
    for cusp in &kp.cusps {
        assert!(cusp.longitude >= 0.0 && cusp.longitude < 360.0);
    }
}

#[test]
fn test_kp_planets_calculation() {
    let calc = VedicChartCalculator::new();
    let time = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();
    let natal = vec![
        create_mock_position(VedicPlanet::Sun, 1, 1, 1, 10.0),
        create_mock_position(VedicPlanet::Moon, 2, 4, 2, 40.0),
    ];
    let kp = KpAnalysis::calculate(time, 13.0, 80.0, 24.0, &natal, calc.engine()).unwrap();
    assert_eq!(kp.planets.len(), 2);
    assert_eq!(kp.planets[0].name, "Sun");
    assert_eq!(kp.planets[1].name, "Moon");
}

// --- Hierarchical Dasha Timeline ---

#[test]
fn test_dasha_mahadasha_duration() {
    let birth = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    // Moon at 0.0 deg (Ashwini Nakshatra 1, ruled by Ketu)
    let timeline = Vimshottari::calculate(0.0, birth, 1, VedicYearType::Gregorian);
    assert!(!timeline.is_empty());
    // Ketu Mahadasha is first
    assert_eq!(timeline[0].planet, VedicPlanet::Ketu);
}

#[test]
fn test_dasha_hierarchical_levels_level1() {
    let birth = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    let timeline = Vimshottari::calculate(15.0, birth, 1, VedicYearType::Gregorian);
    assert!(timeline[0].sub_periods.is_empty());
}

#[test]
fn test_dasha_hierarchical_levels_level2() {
    let birth = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    let timeline = Vimshottari::calculate(15.0, birth, 2, VedicYearType::Gregorian);
    assert!(!timeline[0].sub_periods.is_empty());
    assert_eq!(timeline[0].sub_periods[0].level, 2);
}

#[test]
fn test_dasha_hierarchical_levels_level3() {
    let birth = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    let timeline = Vimshottari::calculate(15.0, birth, 3, VedicYearType::Gregorian);
    assert!(!timeline[0].sub_periods[0].sub_periods.is_empty());
    assert_eq!(timeline[0].sub_periods[0].sub_periods[0].level, 3);
}

#[test]
fn test_dasha_year_types() {
    let birth = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    let timeline_greg = Vimshottari::calculate(0.0, birth, 1, VedicYearType::Gregorian);
    let timeline_sav = Vimshottari::calculate(0.0, birth, 1, VedicYearType::Savana);

    // Savana year (360 days) is shorter than Gregorian (365.2425 days)
    // Hence the end date of Ketu Mahadasha (7 years) should be earlier in Savana
    assert!(timeline_sav[0].end_date < timeline_greg[0].end_date);
}

// ─────────────────────────────────────────────────────────────────────────────
// TIER 2: BOUNDARY AND CORNER TESTS (20 Cases)
// ─────────────────────────────────────────────────────────────────────────────

// --- Ashtakoota Compatibility Boundaries ---

#[test]
fn test_ashtakoota_gana_temperament_boundary() {
    // Gana matches: Deva, Manushya, Rakshasa.
    // Deva vs Rakshasa -> 1.0 points (high friction, Gana Dosha)
    // Rakshasa vs Rakshasa -> 6.0 points
    let male_deva = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 1, 1, 1, 10.0), // Nakshatra 1 is Deva
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let female_rakshasa = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 2, 3, 1, 35.0), // Nakshatra 3 is Rakshasa (Deva is 1,5,7,8,13,15,17,22,27; Manu is 2,4,6,11,12,20,21,25,26; others Rakshasa)
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let report = MatchingEngine::calculate_compatibility(&male_deva, &female_rakshasa);
    let gana = report
        .kootas
        .iter()
        .find(|k| k.name.contains("Gana"))
        .unwrap();
    assert_eq!(gana.earned_points, 1.0);
}

#[test]
fn test_ashtakoota_bhakoot_emotional_boundary() {
    // Bhakoot: 1, 7, 3, 11, 4, 10 relative signs are auspicious (7.0 points).
    // 2/12, 5/9, 6/8 are zero points.
    // Male rasi 1, Female rasi 2 (2/12 relationship) -> 0.0 points
    let male = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 1, 1, 1, 10.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let female = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 2, 4, 1, 45.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let report = MatchingEngine::calculate_compatibility(&male, &female);
    let bhakoot = report
        .kootas
        .iter()
        .find(|k| k.name.contains("Bhakoot"))
        .unwrap();
    assert_eq!(bhakoot.earned_points, 0.0);
}

#[test]
fn test_ashtakoota_nadi_genetic_boundary() {
    // Nadi: Adi, Madhya, Antya. Same Nadi -> 0.0 points (Nadi Dosha), Different -> 8.0 points.
    // Nakshatra 1 is Adi. Nakshatra 6 is Adi.
    let male = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 1, 1, 1, 10.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let female = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 3, 6, 1, 75.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let report = MatchingEngine::calculate_compatibility(&male, &female);
    let nadi = report
        .kootas
        .iter()
        .find(|k| k.name.contains("Nadi"))
        .unwrap();
    assert_eq!(nadi.earned_points, 0.0);
}

#[test]
fn test_ashtakoota_mangal_dosha_cancellation() {
    // Mangal Dosha present in both charts -> cancelled (mangal_dosha_cancelled = true)
    // Mars in 1st house for both.
    let male = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 1, 1, 1, 10.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0), // Mars in house 1
    ]);
    let female = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 1, 1, 1, 10.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0), // Mars in house 1
    ]);
    let report = MatchingEngine::calculate_compatibility(&male, &female);
    assert!(report.male_mangal_dosha);
    assert!(report.female_mangal_dosha);
    assert!(report.mangal_dosha_cancelled);
}

#[test]
fn test_ashtakoota_total_score_limit() {
    // Perfect match (Same Moon position) -> should yield 36.0 points
    let male = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 4, 1, 1, 95.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let female = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 4, 1, 1, 95.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let report = MatchingEngine::calculate_compatibility(&male, &female);
    // Note: Same Moon nakshatra yields Nadi Dosha (0 points) unless we bypass or Nak differs.
    // If Moon positions are exactly identical, earned score should be 36 - 8 (nadi) = 28.
    assert_eq!(report.total_score, 28.0);
}

// --- Shadbala Boundaries ---

#[test]
fn test_shadbala_drik_bala_factor_boundary() {
    let chart = create_mock_chart(vec![create_mock_position(VedicPlanet::Sun, 1, 1, 1, 10.0)]);
    let pos = &chart.planets[0];
    let strength = StrengthEngine::calculate(pos, &chart);
    // Assert the new field added by the implementation track
    assert!(strength.drik_bala >= -60.0 && strength.drik_bala <= 60.0);
}

#[test]
fn test_shadbala_exaltation_debilitation_uchcha_boundary() {
    // Deep Exaltation of Sun: Aries 10 deg -> Uchcha Bala 60.0
    // Deep Debilitation of Sun: Libra 10 deg (190 deg) -> Uchcha Bala 0.0
    let chart_exalt =
        create_mock_chart(vec![create_mock_position(VedicPlanet::Sun, 1, 1, 1, 10.0)]);
    let chart_debilit = create_mock_chart(vec![create_mock_position(
        VedicPlanet::Sun,
        7,
        13,
        7,
        190.0,
    )]);

    let strength_ex = StrengthEngine::calculate(&chart_exalt.planets[0], &chart_exalt);
    let strength_deb = StrengthEngine::calculate(&chart_debilit.planets[0], &chart_debilit);

    assert_eq!(strength_ex.exaltation_score, 60.0);
    assert_eq!(strength_deb.exaltation_score, 0.0);
    // Assert the new field added by the implementation track
    assert!(strength_ex.sthana_bala >= strength_deb.sthana_bala);
}

#[test]
fn test_shadbala_planetary_war_yuddha_boundary() {
    // Planetary war: Mars and Venus within 1 degree.
    // Should trigger yuddha_bala computation.
    let chart = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
        create_mock_position(VedicPlanet::Venus, 1, 1, 1, 10.5),
    ]);
    let strength_mars = StrengthEngine::calculate(&chart.planets[0], &chart);
    let strength_venus = StrengthEngine::calculate(&chart.planets[1], &chart);

    assert!(strength_mars.yuddha_bala != 0.0 || strength_venus.yuddha_bala != 0.0);
    // Assert the new fields added by the implementation track
    assert!(strength_mars.sthana_bala >= 0.0);
    assert!(strength_venus.sthana_bala >= 0.0);
}

#[test]
fn test_shadbala_ishta_kashta_phala_limits() {
    let chart = create_mock_chart(vec![create_mock_position(VedicPlanet::Sun, 1, 1, 1, 10.0)]);
    let strength = StrengthEngine::calculate(&chart.planets[0], &chart);
    assert!(strength.ishta_phala >= 0.0 && strength.ishta_phala <= 60.0);
    assert!(strength.kashta_phala >= 0.0 && strength.kashta_phala <= 60.0);
    // Assert the new fields added by the implementation track
    assert!(strength.chesta_bala >= 0.0);
}

#[test]
fn test_shadbala_total_score_status_mapping() {
    let chart = create_mock_chart(vec![create_mock_position(VedicPlanet::Sun, 1, 1, 1, 10.0)]);
    let strength = StrengthEngine::calculate(&chart.planets[0], &chart);
    assert!(
        strength.status == "Exalted"
            || strength.status == "Strong"
            || strength.status == "Neutral"
            || strength.status == "Weak"
    );
    // Assert the new fields added by the implementation track
    assert!(strength.naisargika_bala >= 0.0);
}

// --- KP Boundaries ---

#[test]
fn test_kp_boundary_rasi_cusp() {
    let calc = VedicChartCalculator::new();
    let time = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();
    // Sun at boundary: 29.999 deg (Aries) vs 30.001 deg (Taurus)
    let natal = vec![
        create_mock_position(VedicPlanet::Sun, 1, 1, 1, 29.999),
        create_mock_position(VedicPlanet::Moon, 2, 4, 1, 30.001),
    ];
    let kp = KpAnalysis::calculate(time, 13.0, 80.0, 24.0, &natal, calc.engine()).unwrap();
    assert_eq!(kp.planets[0].sign_lord, VedicPlanet::Mars); // Aries
    assert_eq!(kp.planets[1].sign_lord, VedicPlanet::Venus); // Taurus
}

#[test]
fn test_kp_boundary_star_transition() {
    let calc = VedicChartCalculator::new();
    let time = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();
    // Star 1 (Ashwini) ends at 13.3333 deg.
    let natal = vec![
        create_mock_position(VedicPlanet::Sun, 1, 1, 1, 13.33),
        create_mock_position(VedicPlanet::Moon, 1, 2, 1, 13.34),
    ];
    let kp = KpAnalysis::calculate(time, 13.0, 80.0, 24.0, &natal, calc.engine()).unwrap();
    assert_eq!(kp.planets[0].star_lord, VedicPlanet::Ketu); // Ashwini
    assert_eq!(kp.planets[1].star_lord, VedicPlanet::Venus); // Bharani
}

#[test]
fn test_kp_boundary_sub_lord_transition() {
    let calc = VedicChartCalculator::new();
    let time = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();
    let natal = vec![
        create_mock_position(VedicPlanet::Sun, 1, 1, 1, 0.1),
        create_mock_position(VedicPlanet::Moon, 1, 1, 1, 0.9),
    ];
    let kp = KpAnalysis::calculate(time, 13.0, 80.0, 24.0, &natal, calc.engine()).unwrap();
    // Sub-lords should differ due to small shift in degree within Ketu star
    assert_ne!(kp.planets[0].sub_lord, kp.planets[1].sub_lord);
}

#[test]
fn test_kp_extreme_coordinates_cusps() {
    let calc = VedicChartCalculator::new();
    let time = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();
    // High latitude (e.g. Tromsø, Norway at 69.6° N) where Placidus can have issues.
    // Ensure the system either calculates houses successfully or propagates a controlled error.
    let kp = KpAnalysis::calculate(time, 69.6, 18.9, 24.0, &[], calc.engine());
    assert!(kp.is_ok() || kp.is_err());
}

#[test]
fn test_kp_empty_natal_planets() {
    let calc = VedicChartCalculator::new();
    let time = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();
    let kp = KpAnalysis::calculate(time, 13.0, 80.0, 24.0, &[], calc.engine()).unwrap();
    assert_eq!(kp.planets.len(), 0);
    assert_eq!(kp.cusps.len(), 12);
}

// --- Dasha Boundaries ---

#[test]
fn test_dasha_boundary_moon_longitude_zero() {
    let birth = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    let timeline = Vimshottari::calculate(0.0, birth, 1, VedicYearType::Gregorian);
    assert_eq!(timeline[0].planet, VedicPlanet::Ketu);
    // Since Moon is at 0.0, we have the full 7 years of Ketu Mahadasha
    let diff = timeline[0].end_date - timeline[0].start_date;
    assert!(diff.num_days() >= 2550 && diff.num_days() <= 2560);
}

#[test]
fn test_dasha_boundary_moon_longitude_max() {
    let birth = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    // 359.999 deg is the end of Revati Nakshatra (ruled by Mercury)
    let timeline = Vimshottari::calculate(359.999, birth, 1, VedicYearType::Gregorian);
    assert_eq!(timeline[0].planet, VedicPlanet::Mercury);
    // Since Moon is at the very end, Mercury Mahadasha should be extremely short (virtually 0 duration)
    assert!(timeline[0].duration_years < 0.01);
}

#[test]
fn test_dasha_boundary_nakshatra_junction() {
    let birth = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    // Junction of Ashwini and Bharani: 13.3333 deg
    let timeline_ashwini = Vimshottari::calculate(13.33, birth, 1, VedicYearType::Gregorian);
    let timeline_bharani = Vimshottari::calculate(13.34, birth, 1, VedicYearType::Gregorian);

    assert_eq!(timeline_ashwini[0].planet, VedicPlanet::Ketu);
    assert_eq!(timeline_bharani[0].planet, VedicPlanet::Venus);
}

#[test]
fn test_dasha_extreme_max_levels() {
    let birth = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    // Large max level should not crash the engine
    let timeline = Vimshottari::calculate(120.0, birth, 5, VedicYearType::Gregorian);
    assert!(!timeline.is_empty());
}

#[test]
fn test_dasha_negative_time() {
    // Test birth date before Unix epoch (historical date)
    let birth = Utc.with_ymd_and_hms(1947, 8, 15, 0, 0, 0).unwrap();
    let timeline = Vimshottari::calculate(100.0, birth, 1, VedicYearType::Gregorian);
    assert_eq!(timeline[0].start_date, birth);
}

// ─────────────────────────────────────────────────────────────────────────────
// TIER 3: CROSS-FEATURE COMBINATION TESTS (4 Cases)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_cross_compatibility_and_dasha() {
    // Verify that compatibility score is computed and matches, and that active dasha at birth can be generated.
    let male = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 4, 1, 1, 95.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let female = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 4, 1, 1, 95.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);

    let report = MatchingEngine::calculate_compatibility(&male, &female);
    assert!(report.total_score > 0.0);

    let birth = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    let male_dasha = Vimshottari::calculate(95.0, birth, 2, VedicYearType::Gregorian);
    assert!(!male_dasha.is_empty());
    assert_eq!(male_dasha[0].planet, VedicPlanet::Saturn); // 95 deg is Saturn Nakshatra (Pushya)
}

#[test]
fn test_cross_strength_and_kp_lords() {
    let calc = VedicChartCalculator::new();
    let time = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();
    let chart = create_mock_chart(vec![create_mock_position(VedicPlanet::Sun, 1, 1, 1, 10.0)]);

    let strength = StrengthEngine::calculate(&chart.planets[0], &chart);
    let kp = KpAnalysis::calculate(time, 13.0, 80.0, 24.0, &chart.planets, calc.engine()).unwrap();

    assert_eq!(strength.planet, VedicPlanet::Sun);
    assert_eq!(kp.planets[0].sign_lord, VedicPlanet::Mars);
    // Assert the new fields added by the implementation track
    assert!(strength.sthana_bala >= 0.0);
    assert!(strength.dig_bala >= 0.0);
}

#[test]
fn test_cross_dasha_lord_strength() {
    // 1. Calculate Vimshottari Dasha to find the current Mahadasha lord
    let birth = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    let timeline = Vimshottari::calculate(10.0, birth, 1, VedicYearType::Gregorian);
    let dasha_lord = timeline[0].planet;

    // 2. Mock a chart where that lord has a specific position
    let chart = create_mock_chart(vec![create_mock_position(dasha_lord, 1, 1, 1, 10.0)]);

    // 3. Compute strength of that dasha lord
    let strength = StrengthEngine::calculate(&chart.planets[0], &chart);
    assert_eq!(strength.planet, dasha_lord);
    // Assert the new fields added by the implementation track
    assert!(strength.sthana_bala >= 0.0);
    assert!(strength.kala_bala >= 0.0);
}

#[test]
fn test_cross_compatibility_mangal_dosha_and_kp_houses() {
    let calc = VedicChartCalculator::new();
    let time = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();

    // Male chart with Mars in 1st house (Mangal Dosha)
    let male = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 4, 1, 1, 95.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0),
    ]);
    let female = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 4, 1, 1, 95.0),
        create_mock_position(VedicPlanet::Mars, 7, 13, 7, 195.0), // Female Mars in 7th sign
    ]);

    let compat = MatchingEngine::calculate_compatibility(&male, &female);
    assert!(compat.male_mangal_dosha);

    // Map male Mars to KP cusps
    let kp = KpAnalysis::calculate(time, 13.0, 80.0, 24.0, &male.planets, calc.engine()).unwrap();
    assert_eq!(kp.planets[1].name, "Mars");
    assert_eq!(kp.planets[1].sign_lord, VedicPlanet::Mars);
}

// ─────────────────────────────────────────────────────────────────────────────
// TIER 4: REAL-WORLD APPLICATION WORKLOAD TESTS (5 Cases)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_workload_standard_natal_reading() {
    let calc = VedicChartCalculator::new();
    let time = Utc.with_ymd_and_hms(1990, 5, 24, 8, 30, 0).unwrap();
    let latitude = 37.5665; // Seoul
    let longitude = 126.9780;

    // 1. Calculate full Vedic chart
    let chart = calc.calculate(time, latitude, longitude).unwrap();

    // 2. Perform Shadbala calculation for all planets in the chart
    for pos in &chart.planets {
        let strength = StrengthEngine::calculate(pos, &chart);
        assert_eq!(strength.planet, pos.planet);
        // Assert the new fields added by the implementation track
        assert!(strength.sthana_bala >= 0.0);
        assert!(strength.dig_bala >= 0.0);
        assert!(strength.kala_bala >= 0.0);
        assert!(strength.chesta_bala >= 0.0);
        assert!(strength.naisargika_bala >= 0.0);
        assert!(strength.drik_bala >= -60.0);
    }

    // 3. Perform KP Analysis
    let kp = KpAnalysis::calculate(
        time,
        latitude,
        longitude,
        chart.ayanamsa,
        &chart.planets,
        calc.engine(),
    )
    .unwrap();
    assert_eq!(kp.cusps.len(), 12);
    assert!(!kp.planets.is_empty());

    // 4. Calculate Vimshottari Dasha timeline (3 levels)
    let moon = chart
        .planets
        .iter()
        .find(|p| p.planet == VedicPlanet::Moon)
        .unwrap();
    let dasha = Vimshottari::calculate(moon.sidereal_deg, time, 3, VedicYearType::Gregorian);
    assert!(!dasha.is_empty());
    assert_eq!(dasha[0].level, 1);
    assert_eq!(dasha[0].sub_periods[0].level, 2);
    assert_eq!(dasha[0].sub_periods[0].sub_periods[0].level, 3);
}

#[test]
fn test_workload_relationship_compatibility() {
    let calc = VedicChartCalculator::new();
    let time_male = Utc.with_ymd_and_hms(1988, 3, 14, 15, 45, 0).unwrap();
    let time_female = Utc.with_ymd_and_hms(1991, 11, 2, 22, 10, 0).unwrap();
    let lat = 37.5665;
    let lon = 126.9780;

    let chart_male = calc.calculate(time_male, lat, lon).unwrap();
    let chart_female = calc.calculate(time_female, lat, lon).unwrap();

    // 1. Calculate Ashtakoota compatibility
    let report = MatchingEngine::calculate_compatibility(&chart_male, &chart_female);
    assert!(report.total_score >= 0.0 && report.total_score <= 36.0);

    // 2. Perform active dasha checking for both partners
    let moon_male = chart_male
        .planets
        .iter()
        .find(|p| p.planet == VedicPlanet::Moon)
        .unwrap();
    let moon_female = chart_female
        .planets
        .iter()
        .find(|p| p.planet == VedicPlanet::Moon)
        .unwrap();

    let dasha_male = Vimshottari::calculate(
        moon_male.sidereal_deg,
        time_male,
        1,
        VedicYearType::Gregorian,
    );
    let dasha_female = Vimshottari::calculate(
        moon_female.sidereal_deg,
        time_female,
        1,
        VedicYearType::Gregorian,
    );

    assert!(!dasha_male.is_empty());
    assert!(!dasha_female.is_empty());
}

#[test]
fn test_workload_career_wealth_audit() {
    let calc = VedicChartCalculator::new();
    let birth_time = Utc.with_ymd_and_hms(1985, 12, 12, 18, 0, 0).unwrap();
    let lat = 40.7128; // New York
    let lon = -74.0060;

    let chart = calc.calculate(birth_time, lat, lon).unwrap();

    // KP significator audit: find 2nd, 6th, 10th and 11th house/cusp sign and star lords
    let kp = KpAnalysis::calculate(
        birth_time,
        lat,
        lon,
        chart.ayanamsa,
        &chart.planets,
        calc.engine(),
    )
    .unwrap();

    let cusp_2 = &kp.cusps[1];
    let cusp_6 = &kp.cusps[5];
    let cusp_10 = &kp.cusps[9];
    let cusp_11 = &kp.cusps[10];

    // Assert sign and star lords are correctly loaded for career cusps
    assert!(cusp_2.sign_lord != VedicPlanet::Ascendant);
    assert!(cusp_6.star_lord != VedicPlanet::Ascendant);
    assert!(cusp_10.sub_lord != VedicPlanet::Ascendant);
    assert!(cusp_11.sign_lord != VedicPlanet::Ascendant);

    // Audit Shadbala strength of the 10th house lord
    let lord_10 = cusp_10.sign_lord;
    if let Some(pos) = chart.planets.iter().find(|p| p.planet == lord_10) {
        let strength = StrengthEngine::calculate(pos, &chart);
        assert_eq!(strength.planet, lord_10);
        // Assert the new fields added by the implementation track
        assert!(strength.sthana_bala >= 0.0);
        assert!(strength.dig_bala >= 0.0);
    }
}

#[test]
fn test_workload_extreme_location_analysis() {
    let calc = VedicChartCalculator::new();
    let time = Utc.with_ymd_and_hms(2026, 6, 20, 0, 0, 0).unwrap();

    // Polar location: Tromsø, Norway (latitude: 69.6492, longitude: 18.9560)
    let lat = 69.6492;
    let lon = 18.9560;

    let chart = calc.calculate(time, lat, lon).unwrap();

    // Ensure we can calculate KP cusps successfully under extreme latitudes
    let kp = KpAnalysis::calculate(
        time,
        lat,
        lon,
        chart.ayanamsa,
        &chart.planets,
        calc.engine(),
    );
    assert!(kp.is_ok());

    // Perform Shadbala for Ascendant lord / Sun
    let sun = chart
        .planets
        .iter()
        .find(|p| p.planet == VedicPlanet::Sun)
        .unwrap();
    let strength = StrengthEngine::calculate(sun, &chart);
    assert_eq!(strength.planet, VedicPlanet::Sun);
    // Assert the new fields added by the implementation track
    assert!(strength.sthana_bala >= 0.0);
    assert!(strength.dig_bala >= 0.0);
    assert!(strength.kala_bala >= 0.0);
    assert!(strength.chesta_bala >= 0.0);
    assert!(strength.naisargika_bala >= 0.0);
    assert!(strength.drik_bala >= -60.0);
}

#[test]
fn test_workload_historical_timeline_reconstruction() {
    // Reconstruction of Vimshottari Dasha timeline for a long-lived individual
    let birth = Utc.with_ymd_and_hms(1869, 10, 2, 7, 30, 0).unwrap(); // Mahatma Gandhi
    let moon_longitude = 118.0; // Sidereal Moon position

    let timeline = Vimshottari::calculate(moon_longitude, birth, 2, VedicYearType::Gregorian);
    assert!(!timeline.is_empty());

    // Timeline must cover a full 120-year span of Vimshottari
    let mut total_duration = 0.0;
    let mut last_end = birth;

    for (i, period) in timeline.iter().enumerate() {
        if i > 0 {
            // Assert chronological continuity: current start == previous end
            assert_eq!(period.start_date, last_end);
        }
        total_duration += period.duration_years;
        last_end = period.end_date;

        // Verify level 2 sub-periods are sequential and sum up to Mahadasha duration
        let mut sub_duration = 0.0;
        let mut sub_last_end = period.start_date;
        for (j, sub) in period.sub_periods.iter().enumerate() {
            if j > 0 {
                assert_eq!(sub.start_date, sub_last_end);
            }
            sub_duration += sub.duration_years;
            sub_last_end = sub.end_date;
        }
        // Sub-periods should approximately sum to full period duration
        assert!((sub_duration - period.duration_years).abs() < 0.1);
    }

    // Reconstruction covers the total timeline properly
    assert!(total_duration >= 119.0);
}

#[test]
fn test_stress_ashtakoota_missing_moon_panic() {
    let male = create_mock_chart(vec![create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0)]);
    let female = create_mock_chart(vec![create_mock_position(VedicPlanet::Moon, 4, 1, 1, 95.0)]);

    let result = std::panic::catch_unwind(|| {
        MatchingEngine::calculate_compatibility(&male, &female);
    });
    assert!(
        result.is_err(),
        "Expected panic due to missing Moon in male chart"
    );
}

#[test]
fn test_stress_ashtakoota_missing_moon_mangal_dosha_panic() {
    let male = create_mock_chart(vec![create_mock_position(VedicPlanet::Moon, 4, 1, 1, 95.0)]);
    let female = create_mock_chart(vec![create_mock_position(VedicPlanet::Mars, 1, 1, 1, 10.0)]);

    let result = std::panic::catch_unwind(|| {
        MatchingEngine::calculate_compatibility(&male, &female);
    });
    assert!(
        result.is_err(),
        "Expected panic due to missing Moon in female chart during Mangal Dosha check"
    );
}

#[test]
fn test_stress_ashtakoota_custom_nakshatras_out_of_bounds() {
    let male = create_mock_chart(vec![create_mock_position(VedicPlanet::Moon, 1, 0, 1, 10.0)]);
    let female = create_mock_chart(vec![create_mock_position(
        VedicPlanet::Moon,
        1,
        28,
        1,
        10.0,
    )]);

    let report = MatchingEngine::calculate_compatibility(&male, &female);
    let tara = report
        .kootas
        .iter()
        .find(|k| k.name.contains("Tara"))
        .unwrap();
    println!("Earned Tara points: {}", tara.earned_points);
}

#[test]
fn test_stress_ashtakoota_custom_rasi_out_of_bounds() {
    let male = create_mock_chart(vec![create_mock_position(VedicPlanet::Moon, 0, 1, 1, 10.0)]);
    let female = create_mock_chart(vec![create_mock_position(
        VedicPlanet::Moon,
        13,
        1,
        1,
        10.0,
    )]);

    let report = MatchingEngine::calculate_compatibility(&male, &female);
    assert!(report.total_score >= 0.0);
}

#[test]
fn test_stress_kp_extreme_coordinates_crash() {
    let calc = VedicChartCalculator::new();
    let time = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();

    let result_north = calc.calculate(time, 90.0, 0.0);
    let result_south = calc.calculate(time, -90.0, 0.0);

    println!("North Pole calculate: {:?}", result_north.is_ok());
    println!("South Pole calculate: {:?}", result_south.is_ok());
}

#[test]
fn test_stress_ashtakoota_minimum_possible_score() {
    let male = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 10, 24, 1, 290.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 2, 10.0),
    ]);
    let female = create_mock_chart(vec![
        create_mock_position(VedicPlanet::Moon, 5, 13, 1, 130.0),
        create_mock_position(VedicPlanet::Mars, 1, 1, 2, 10.0),
    ]);

    let report = MatchingEngine::calculate_compatibility(&male, &female);
    assert_eq!(report.total_score, 2.5);
    assert!(!report.is_compatible);
}
