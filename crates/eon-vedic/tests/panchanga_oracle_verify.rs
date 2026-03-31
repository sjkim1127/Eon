mod common;

use eon_vedic::calc::panchanga::PanchangaEngine;
use eon_vedic::planets::VedicPlanet;
use chrono::{DateTime, Utc, TimeZone};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct PanchangaOracleCase {
    case_id: String,
    input: InputData,
    expected: ExpectedData,
}

#[derive(Debug, Deserialize)]
struct InputData {
    year: i32, month: u32, day: u32, hour: u32,
    lat: f64, lon: f64,
    sun_deg: f64, moon_deg: f64,
}

#[derive(Debug, Deserialize)]
struct ExpectedData {
    sunrise_utc: String,
    sunset_utc: String,
    vara: Option<String>,
    tithi: Option<u8>,
    day_lord: Option<String>,
}

fn get_planet_name(p: VedicPlanet) -> String {
    format!("{:?}", p)
}

#[test]
fn verify_panchanga_oracle_fixtures() {
    let data = fs::read_to_string("tests/fixtures/panchanga_oracle.json")
        .expect("Unable to read panchanga_oracle.json");
    let cases: Vec<PanchangaOracleCase> = serde_json::from_str(&data)
        .expect("JSON was not well-formatted");

    for case in cases {
        println!("Verifying Panchanga case: {}", case.case_id);
        
        let time = Utc.with_ymd_and_hms(
            case.input.year, case.input.month, case.input.day, case.input.hour, 0, 0
        ).unwrap();

        let actual = PanchangaEngine::calculate(
            case.input.sun_deg,
            case.input.moon_deg,
            time,
            case.input.lat,
            case.input.lon
        );

        // 1. Verify Sunrise/Sunset (Allow 2 minute error for simplified NOAA)
        let expected_sunrise = DateTime::parse_from_rfc3339(&case.expected.sunrise_utc).unwrap().with_timezone(&Utc);
        let expected_sunset = DateTime::parse_from_rfc3339(&case.expected.sunset_utc).unwrap().with_timezone(&Utc);

        let rise_diff = (actual.sunrise - expected_sunrise).num_seconds().abs();
        let set_diff = (actual.sunset - expected_sunset).num_seconds().abs();

        assert!(rise_diff < 120, "Case {} sunrise error: {}s", case.case_id, rise_diff);
        assert!(set_diff < 120, "Case {} sunset error: {}s", case.case_id, set_diff);

        // 2. Verify Vara
        if let Some(expected_vara) = &case.expected.vara {
            assert_eq!(actual.vara, *expected_vara, "Case {} vara mismatch", case.case_id);
        }

        // 3. Verify Tithi
        if let Some(expected_tithi) = case.expected.tithi {
            assert_eq!(actual.tithi, expected_tithi, "Case {} tithi mismatch", case.case_id);
        }

        // 4. Verify Day Lord
        if let Some(expected_lord) = &case.expected.day_lord {
            assert_eq!(get_planet_name(actual.day_lord), *expected_lord, "Case {} day lord mismatch", case.case_id);
        }
    }
}
