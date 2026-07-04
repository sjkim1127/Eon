mod common;

use chrono::{DateTime, TimeZone, Utc};
use eon_vedic::calc::panchanga::PanchangaEngine;
use eon_vedic::planets::VedicPlanet;
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
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    lat: f64,
    lon: f64,
    sun_deg: f64,
    moon_deg: f64,
}

#[derive(Debug, Deserialize)]
struct ExpectedData {
    sunrise_utc: String,
    sunset_utc: Option<String>,
    vara: Option<String>,
    tithi: Option<u8>,
    nakshatra: Option<u8>,
    yoga: Option<u8>,
    karana: Option<u8>,
    day_lord: Option<String>,
    hora_lord: Option<String>,
    is_day_birth: Option<bool>,
}

fn get_planet_name(p: VedicPlanet) -> String {
    format!("{:?}", p)
}

#[test]
fn verify_panchanga_oracle_fixtures() {
    let data = fs::read_to_string("tests/fixtures/panchanga_oracle.json")
        .expect("Unable to read panchanga_oracle.json");
    let cases: Vec<PanchangaOracleCase> =
        serde_json::from_str(&data).expect("JSON was not well-formatted");

    for case in cases {
        println!("Verifying Panchanga case: {}", case.case_id);

        let time = Utc
            .with_ymd_and_hms(
                case.input.year,
                case.input.month,
                case.input.day,
                case.input.hour,
                0,
                0,
            )
            .unwrap();

        let actual = PanchangaEngine::calculate(
            case.input.sun_deg,
            case.input.moon_deg,
            time,
            case.input.lat,
            case.input.lon,
        );

        // 1. Verify Sunrise (Always present in fixtures)
        let expected_sunrise = DateTime::parse_from_rfc3339(&case.expected.sunrise_utc)
            .unwrap()
            .with_timezone(&Utc);
        let rise_diff = (actual.sunrise - expected_sunrise).num_seconds().abs();
        assert!(
            rise_diff < 120,
            "Case {} sunrise error: {}s (actual: {}, expected: {})",
            case.case_id,
            rise_diff,
            actual.sunrise,
            expected_sunrise
        );

        // 2. Verify Sunset (Optional in fixtures)
        if let Some(expected_sunset_str) = &case.expected.sunset_utc {
            let expected_sunset = DateTime::parse_from_rfc3339(expected_sunset_str)
                .unwrap()
                .with_timezone(&Utc);
            let set_diff = (actual.sunset - expected_sunset).num_seconds().abs();
            assert!(
                set_diff < 120,
                "Case {} sunset error: {}s",
                case.case_id,
                set_diff
            );
        }

        // 3. Verify Vara & Day Lord
        if let Some(expected_vara) = &case.expected.vara {
            assert_eq!(
                actual.vara, *expected_vara,
                "Case {} vara mismatch",
                case.case_id
            );
        }
        if let Some(expected_lord) = &case.expected.day_lord {
            assert_eq!(
                get_planet_name(actual.day_lord),
                *expected_lord,
                "Case {} day lord mismatch",
                case.case_id
            );
        }

        // 4. Verify Tithi, Nakshatra, Yoga, Karana
        if let Some(val) = case.expected.tithi {
            assert_eq!(actual.tithi, val, "Case {} tithi mismatch", case.case_id);
        }
        if let Some(val) = case.expected.nakshatra {
            assert_eq!(
                actual.nakshatra, val,
                "Case {} nakshatra mismatch",
                case.case_id
            );
        }
        if let Some(val) = case.expected.yoga {
            assert_eq!(actual.yoga, val, "Case {} yoga mismatch", case.case_id);
        }
        if let Some(val) = case.expected.karana {
            assert_eq!(actual.karana, val, "Case {} karana mismatch", case.case_id);
        }

        // 5. Verify Day/Night Birth
        if let Some(val) = case.expected.is_day_birth {
            assert_eq!(
                actual.is_day_birth, val,
                "Case {} is_day_birth mismatch",
                case.case_id
            );
        }

        // 6. Verify Hora Lord
        if let Some(expected_hora) = &case.expected.hora_lord {
            assert_eq!(
                get_planet_name(actual.hour_lord),
                *expected_hora,
                "Case {} hora lord mismatch",
                case.case_id
            );
        }
    }
}
