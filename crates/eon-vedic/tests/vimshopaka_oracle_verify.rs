mod common;

use eon_vedic::planets::VedicPlanet;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct VimshopakaOracleCase {
    case_id: String,
    input: InputData,
    test_planet: String,
    expected_dignity_points: HashMap<String, f64>,
    expected_shadvarga_score: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct InputData {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    lat: f64,
    lon: f64,
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
        "rahu" => VedicPlanet::Rahu,
        "ketu" => VedicPlanet::Ketu,
        _ => panic!("Unknown planet: {}", name),
    }
}

#[test]
fn verify_vimshopaka_oracle_fixtures() {
    let data = fs::read_to_string("tests/fixtures/vimshopaka_oracle.json")
        .expect("Unable to read vimshopaka_oracle.json");
    let cases: Vec<VimshopakaOracleCase> =
        serde_json::from_str(&data).expect("JSON was not well-formatted");

    for case in cases {
        println!("Verifying Vimshopaka case: {}", case.case_id);

        let chart = common::create_test_chart(
            case.input.year,
            case.input.month,
            case.input.day,
            case.input.hour,
            case.input.lat,
            case.input.lon,
        );

        let target_planet = get_planet_enum(&case.test_planet);
        let planet_pos = chart
            .planets
            .iter()
            .find(|p| p.planet == target_planet)
            .unwrap();
        println!(
            "Case {}: Planet {} sidereal_deg: {}",
            case.case_id, case.test_planet, planet_pos.sidereal_deg
        );

        // Find the Vimshopaka score for the target planet
        let score_entry = chart
            .vimshopaka_scores
            .iter()
            .find(|(p, _)| *p == target_planet)
            .map(|(_, s)| s)
            .unwrap_or_else(|| panic!("Planet {} not found in chart scores", case.test_planet));

        // 1. Verify Dignity Points per Varga (Hierarchy: Sign -> Point)
        for (v_id, expected_point) in &case.expected_dignity_points {
            let v_type = common::get_varga_type(v_id)
                .unwrap_or_else(|| panic!("Unknown varga ID: {}", v_id));

            let actual_rasi = score_entry
                .details
                .iter()
                .find(|(vt, _)| *vt == v_type)
                .map(|_| {
                    chart
                        .planets
                        .iter()
                        .find(|p| p.planet == target_planet)
                        .unwrap()
                        .varga_rasi(v_type)
                })
                .unwrap();

            let actual_point = score_entry
                .details
                .iter()
                .find(|(vt, _)| *vt == v_type)
                .map(|(_, p)| *p)
                .unwrap_or_else(|| panic!("Varga {:?} not found in score details", v_type));

            println!(
                "Case {}: Planet {} in {:?} - Actual Rasi: {}, Point: {}",
                case.case_id, case.test_planet, v_type, actual_rasi, actual_point
            );

            common::assert_approx_eq(
                actual_point,
                *expected_point,
                1e-4,
                &format!(
                    "Case {} failed for {} dignity in {:?}",
                    case.case_id, case.test_planet, v_type
                ),
            );
        }

        // 2. Verify Weighted Shadvarga Score (Hierarchy: Points -> Sum -> Final Score)
        if let Some(expected_shadvarga) = case.expected_shadvarga_score {
            common::assert_approx_eq(
                score_entry.shadvarga_score,
                expected_shadvarga,
                0.1, // Relax epsilon slightly for weighted sum
                &format!("Case {} failed for overall Shadvarga score", case.case_id),
            );
        }
    }
}
