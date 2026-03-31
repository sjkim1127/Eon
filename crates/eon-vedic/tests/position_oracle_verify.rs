mod common;

use eon_vedic::planets::VedicPlanet;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct PositionOracleCase {
    case_id: String,
    input: InputData,
    expected: ExpectedPositions,
}

#[derive(Debug, Deserialize)]
struct InputData {
    year: i32, month: u32, day: u32, hour: u32,
    lat: f64, lon: f64,
}

#[derive(Debug, Deserialize)]
struct ExpectedPositions {
    ayanamsa: f64,
    planets: HashMap<String, PlanetDeg>,
    ascendant: Option<PlanetDeg>,
}

#[derive(Debug, Deserialize)]
struct PlanetDeg {
    tropical: Option<f64>,
    sidereal: Option<f64>,
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
fn verify_position_oracle_fixtures() {
    let data = fs::read_to_string("tests/fixtures/position_oracle.json")
        .expect("Unable to read position_oracle.json");
    let cases: Vec<PositionOracleCase> = serde_json::from_str(&data)
        .expect("JSON was not well-formatted");

    for case in cases {
        let chart = common::create_test_chart(
            case.input.year, case.input.month, case.input.day, case.input.hour,
            case.input.lat, case.input.lon
        );

        // 1. Verify Planets
        for (name, expected) in &case.expected.planets {
            let p_enum = get_planet_enum(name);
            let actual_pos = chart.planets.iter().find(|p| p.planet == p_enum)
                .expect(&format!("Planet {} not found in chart", name));

            if let Some(exp_trop) = expected.tropical {
                common::assert_approx_eq(
                    actual_pos.tropical_deg % 360.0, exp_trop % 360.0, 0.1,
                    &format!("Case {} failed for {} tropical", case.case_id, name)
                );
            }
            if let Some(exp_sid) = expected.sidereal {
                common::assert_approx_eq(
                    actual_pos.sidereal_deg % 360.0, exp_sid % 360.0, 0.1,
                    &format!("Case {} failed for {} sidereal", case.case_id, name)
                );
            }
        }

        // 2. Verify Ascendant
        if let Some(expected_asc) = &case.expected.ascendant {
            if let Some(exp_sid) = expected_asc.sidereal {
                common::assert_approx_eq(
                    chart.ascendant.sidereal_deg % 360.0, exp_sid % 360.0, 0.1,
                    &format!("Case {} failed for Ascendant sidereal", case.case_id)
                );
            }
        }
    }
}
