use chrono::Utc;
use eon_vedic::analysis::ashtakavarga::Sarvashtakavarga;
use eon_vedic::chart::{VedicChart, VedicPosition};
use eon_vedic::panchanga::PanchangaEngine;
use eon_vedic::planets::VedicPlanet;
use eon_vedic::yogas::{YogaEngine, YogaQuality};

fn default_pos(planet: VedicPlanet, rasi: u8, house: u8) -> VedicPosition {
    VedicPosition {
        planet,
        rasi,
        house_index: house,
        tropical_deg: 0.0,
        sidereal_deg: 0.0,
        nakshatra: 1,
        pada: 1,
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

#[test]
fn test_libra_lagna_saturn_yogakaraka() {
    // 1. Create a dummy chart with Libra Ascendant (Rasi 7)
    let ascendant = default_pos(VedicPlanet::Ketu, 7, 1);

    let mut planets = Vec::new();

    // Saturn (Lord of 4 and 5 for Libra) in 5th House (Aquarius) -> Trikona
    // Mercury (Lord of 9 and 12 for Libra) in 4th House (Capricorn) -> Kendra
    // This forms a Raja Yoga (Lord 5 & Lord 9 relationship if aspected or conjoined)
    // Let's put them in conjunction in 5th House (Aquarius)

    // Saturn in Aquarius (Rasi 11)
    let mut sat = default_pos(VedicPlanet::Saturn, 11, 5);
    sat.sidereal_deg = 310.0;
    planets.push(sat);

    // Mercury in Aquarius (Rasi 11)
    let mut merc = default_pos(VedicPlanet::Mercury, 11, 5);
    merc.sidereal_deg = 312.0;
    planets.push(merc);

    // Add other planets to avoid empty checks if any
    let dummy_planets = vec![
        VedicPlanet::Sun,
        VedicPlanet::Moon,
        VedicPlanet::Mars,
        VedicPlanet::Jupiter,
        VedicPlanet::Venus,
        VedicPlanet::Rahu,
        VedicPlanet::Ketu,
    ];
    for p in dummy_planets {
        let mut pos = default_pos(p, 1, 7);
        pos.sidereal_deg = 10.0;
        planets.push(pos);
    }

    let dummy_panchanga = PanchangaEngine::calculate(0.0, 0.0, Utc::now(), 0.0, 0.0);

    let chart = VedicChart {
        ascendant,
        planets,
        aspects: vec![],
        sav: Sarvashtakavarga { points: [0; 12] },
        house_cusps: vec![],
        karakas: vec![],
        bhava_strengths: vec![],
        vimshopaka_scores: vec![],
        panchanga: dummy_panchanga,
        analysis_report: None,
    };

    let results = YogaEngine::check_yogas(&chart);

    // We expect a Raja Yoga between Saturn (L5) and Mercury (L9)
    // Since Saturn is Yogakaraka for Libra Lagna, it should boost Quality.

    let raja_yoga = results.iter().find(|r| r.name.contains("Raja Yoga"));

    // Debug output
    for res in &results {
        println!("{:?}", res);
    }

    assert!(raja_yoga.is_some(), "Should find a Raja Yoga");

    if let Some(yoga) = raja_yoga {
        assert_eq!(
            yoga.quality,
            YogaQuality::High,
            "Saturn is Yogakaraka (+3), but Mercury has 12th House Moolatrikona (-2), resulting in High (Score 1)"
        );
    }
}
