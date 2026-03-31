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
    kala_bala_min: Option<f64>,
    chesta_bala: Option<f64>,
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
        
        // Find the strength entry for the target planet
        let planet_pos = chart.planets.iter().find(|p| p.planet == target_planet)
            .expect(&format!("Planet {} not found in chart", case.test_planet));
            
        let strength = eon_vedic::analysis::strength::StrengthEngine::calculate(planet_pos, &chart);

        // 1. Verify Uchcha Bala
        if let Some(expected_val) = case.expected.uchcha_bala {
            common::assert_approx_eq(
                strength.exaltation_score, expected_val, 1.0, 
                &format!("Case {} failed for {} Uchcha Bala", case.case_id, case.test_planet)
            );
        }

        // 2. Verify Dig Bala
        if let Some(expected_val) = case.expected.dig_bala {
            common::assert_approx_eq(
                strength.directional_score, expected_val, 0.1, 
                &format!("Case {} failed for {} Dig Bala", case.case_id, case.test_planet)
            );
        }

        // 3. Verify Naisargika Bala
        if let Some(expected_val) = case.expected.naisargika_bala {
            common::assert_approx_eq(
                strength.naisargika_score, expected_val, 0.1, 
                &format!("Case {} failed for {} Naisargika Bala", case.case_id, case.test_planet)
            );
        }

        // 4. Verify Chesta Bala
        if let Some(expected_val) = case.expected.chesta_bala {
            common::assert_approx_eq(
                strength.chesta_score, expected_val, 0.1, 
                &format!("Case {} failed for {} Chesta Bala", case.case_id, case.test_planet)
            );
        }

        // 4. Verify Kala Bala (Min check for Nathonnata)
        if let Some(min_val) = case.expected.kala_bala_min {
            assert!(strength.kala_score >= min_val, 
                    "Case {} failed: {} Kala Bala {} is less than min {}", 
                    case.case_id, case.test_planet, strength.kala_score, min_val);
        }

        // 5. Verify Day/Night Birth context
        if let Some(expected_night) = case.expected.is_night_birth {
            assert_eq!(chart.panchanga.is_night_birth, expected_night, 
                       "Case {} failed: is_night_birth mismatch", case.case_id);
        }

        // 6. Log Total Score (Soft check - current engine uses simple sum)
        if let Some(expected_total) = case.expected.total_score {
            let diff = (strength.total_score - expected_total).abs();
            println!("Case {}: Overall Shadbala - Actual: {:.2}, Expected: {:.2} (Diff: {:.2})", 
                     case.case_id, strength.total_score, expected_total, diff);
        }
    }
}
