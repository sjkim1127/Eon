mod common;

use chrono::{TimeZone, Utc, Timelike};
use eon_vedic::core::chart::VedicChartCalculator;

use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct LocationOracleCase {
    case_id: String,
    input: LocationInput,
    expected: LocationExpected,
}

#[derive(Debug, Deserialize)]
struct LocationInput {
    year: i32, month: u32, day: u32, hour: u32, minute: u32,
    lat: f64, lon: f64,
    timezone: String,
}

#[derive(Debug, Deserialize)]
struct LocationExpected {
    utc_time: String,
    asc_sidereal: f64,
    cusps_sidereal: Option<Vec<f64>>,
    sunrise_jst: Option<String>,
    sunrise_local: Option<String>,
}

#[test]
fn verify_location_oracle_snapshots() {
    let data = fs::read_to_string("tests/fixtures/location_oracle.json")
        .expect("Unable to read location_oracle.json");
    let cases: Vec<LocationOracleCase> = serde_json::from_str(&data)
        .expect("JSON was not well-formatted");

    for case in cases {
        println!("Verifying Location Snapshot: {}", case.case_id);
        
        // 1. Verify UTC Conversion (Matches logic in prepare_birth_context)
        let tz: chrono_tz::Tz = case.input.timezone.parse().expect("Invalid timezone");
        let local_time = tz.with_ymd_and_hms(
            case.input.year, case.input.month, case.input.day, 
            case.input.hour, case.input.minute, 0
        ).unwrap();
        let actual_utc = local_time.with_timezone(&Utc);
        let expected_utc = chrono::DateTime::parse_from_rfc3339(&case.expected.utc_time).unwrap().with_timezone(&Utc);
        
        assert_eq!(actual_utc, expected_utc, "Case {} UTC conversion mismatch", case.case_id);

        // 2. Calculate Chart
        let calc = VedicChartCalculator::default();
        let chart = calc.calculate(actual_utc, case.input.lat, case.input.lon).unwrap();

        // 3. Verify Ascendant (0.1 deg tolerance)
        common::assert_approx_eq(chart.ascendant.sidereal_deg, case.expected.asc_sidereal, 0.1, &format!("Case {} Ascendant", case.case_id));

        // 4. Verify House Cusps
        if let Some(exp_cusps) = &case.expected.cusps_sidereal {
            println!("Actual House Cusps for {}:", case.case_id);
            for (i, actual_cusp) in chart.house_cusps.iter().enumerate() {
                println!("  House {}: {:.4}", i + 1, actual_cusp);
            }
            for (i, expected_cusp) in exp_cusps.iter().enumerate() {
                let actual_cusp = chart.house_cusps[i];
                common::assert_approx_eq(actual_cusp, *expected_cusp, 0.1, &format!("Case {} House {} cusp", case.case_id, i + 1));
            }
        }

        // 5. Verify Sunrise (aligned with timezone)
        if let Some(exp_sr_str) = &case.expected.sunrise_jst {
            let exp_sr = chrono::DateTime::parse_from_rfc3339(exp_sr_str).unwrap().with_timezone(&tz);
            let actual_sr = chart.panchanga.sunrise.with_timezone(&tz);
            let diff = (actual_sr - exp_sr).num_seconds().abs();
            assert!(diff < 120, "Case {} sunrise mismatch: {}s diff", case.case_id, diff);
        }
    }
}
