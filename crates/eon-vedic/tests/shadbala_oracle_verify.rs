mod common;

use eon_vedic::planets::VedicPlanet;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct ShadbalaOracleCase {
    case_id: String,
    input: InputData,
    test_planet: String,
    expected: ExpectedShadbala,
}

#[derive(Debug, Deserialize)]
struct InputData {
    year: i32, month: u32, day: u32, hour: u32,
    lat: f64, lon: f64,
}

#[derive(Debug, Deserialize)]
struct ExpectedShadbala {
    uchcha_bala: Option<f64>,
    dig_bala: Option<f64>,
    naisargika_bala: Option<f64>,
    kala_score: Option<f64>,
    chesta_bala: Option<f64>,
    paksha_bala: Option<f64>,
    ayana_bala: Option<f64>,
    saptavargaja_bala: Option<f64>,
    drik_bala: Option<f64>,
    yuddha_bala_abs_min: Option<f64>,
    is_night_birth: Option<bool>,
    total_score: Option<f64>,
}

fn get_planet_enum(name: &str) -> VedicPlanet {
    match name.to_lowercase().as_str() {
        "sun" => VedicPlanet::Sun,
        "moon" => VedicPlanet::Moon,
        "mars" => VedicPlanet::Mars,
        "mercury" => VedicPlanet::Mercury,
        "jupiter" => VedicPlanet::Jupiter,
        "venus" => VedicPlanet::Venus,
        "saturn" => VedicPlanet::Saturn,
        _ => panic!("Unknown planet: {}", name),
    }
}

#[test]
fn verify_shadbala_oracle_fixtures() {
    let data = fs::read_to_string("tests/fixtures/shadbala_oracle.json")
        .expect("Unable to read shadbala_oracle.json");
    let cases: Vec<ShadbalaOracleCase> = serde_json::from_str(&data)
        .expect("JSON was not well-formatted");

    for case in cases {
        println!("Verifying Shadbala case: {}", case.case_id);
        
        let chart = common::create_test_chart(
            case.input.year, case.input.month, case.input.day, case.input.hour,
            case.input.lat, case.input.lon
        );
        
        let target_planet = get_planet_enum(&case.test_planet);
        let planet_pos = chart.planets.iter().find(|p| p.planet == target_planet)
            .expect(&format!("Planet {} not found in chart", case.test_planet));
            
        let strength = eon_vedic::analysis::strength::StrengthEngine::calculate(planet_pos, &chart);

        // 1. Position/Dignity
        if let Some(expected_val) = case.expected.uchcha_bala {
            common::assert_approx_eq(strength.exaltation_score, expected_val, 1.0, &case.case_id);
        }
        if let Some(expected_val) = case.expected.dig_bala {
            common::assert_approx_eq(strength.directional_score, expected_val, 0.1, &case.case_id);
        }
        if let Some(expected_val) = case.expected.naisargika_bala {
            common::assert_approx_eq(strength.naisargika_score, expected_val, 0.1, &case.case_id);
        }

        // 2. Motion/Phase
        if let Some(expected_val) = case.expected.chesta_bala {
            common::assert_approx_eq(strength.chesta_score, expected_val, 0.1, &case.case_id);
        }
        if let Some(expected_val) = case.expected.paksha_bala {
            common::assert_approx_eq(strength.paksha_score, expected_val, 1.0, &case.case_id);
        }

        // 3. Time/Declination
        if let Some(expected_val) = case.expected.kala_score {
            common::assert_approx_eq(strength.kala_score, expected_val, 1.0, &case.case_id);
        }
        if let Some(expected_val) = case.expected.ayana_bala {
            common::assert_approx_eq(strength.ayana_score, expected_val, 1.0, &case.case_id);
        }

        // 4. Varga/Relations
        if let Some(expected_val) = case.expected.saptavargaja_bala {
            common::assert_approx_eq(strength.saptavargaja_score, expected_val, 5.0, &case.case_id);
        }

        // 5. External Factors (War/Aspect)
        if let Some(expected_val) = case.expected.drik_bala {
            common::assert_approx_eq(strength.drik_score, expected_val, 5.0, &case.case_id);
        }
        if let Some(min_abs_yuddha) = case.expected.yuddha_bala_abs_min {
            assert!(strength.yuddha_bala.abs() >= min_abs_yuddha, 
                    "Case {} failed: Yuddha Bala {} abs should be >= {}", 
                    case.case_id, strength.yuddha_bala, min_abs_yuddha);
        }

        // 6. Context
        if let Some(expected_night) = case.expected.is_night_birth {
            assert_eq!(chart.panchanga.is_night_birth, expected_night, "Case {} is_night_birth mismatch", case.case_id);
        }

        // Soft log for total
        if let Some(expected_total) = case.expected.total_score {
            println!("Case {}: Overall Shadbala Actual: {:.2}, Expected: {:.2}", 
                     case.case_id, strength.total_score, expected_total);
        }
    }
}
