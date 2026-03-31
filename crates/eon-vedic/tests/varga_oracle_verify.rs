use eon_vedic::calc::varga::VargaType;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct VargaOracleCase {
    case_id: String,
    input_longitude: f64,
    expected_rasi: HashMap<String, u8>,
    expected_effective_long: Option<HashMap<String, f64>>,
}

fn get_varga_type(id: &str) -> Option<VargaType> {
    match id.to_lowercase().as_str() {
        "d1" | "rasi" => Some(VargaType::D1),
        "d2" | "hora" => Some(VargaType::D2),
        "d3" | "drekkana" => Some(VargaType::D3),
        "d4" | "chaturthamsha" => Some(VargaType::D4),
        "d9" | "navamsa" => Some(VargaType::D9),
        "d12" | "dwadasamsa" => Some(VargaType::D12),
        "d30" | "trimsamsa" => Some(VargaType::D30),
        "d60" | "shashtyamsa" => Some(VargaType::D60),
        "d81" | "navanavamsa" => Some(VargaType::D81),
        "d108" | "ashtottaramsa" => Some(VargaType::D108),
        "d144" | "dwadasdwadasamsa" => Some(VargaType::D144),
        _ => None,
    }
}

#[test]
fn verify_varga_oracle_fixtures() {
    let data = fs::read_to_string("tests/fixtures/varga_oracle.json")
        .expect("Unable to read varga_oracle.json");
    let cases: Vec<VargaOracleCase> = serde_json::from_str(&data)
        .expect("JSON was not well-formatted");

    for case in cases {
        println!("Verifying case: {}", case.case_id);
        
        // 1. Verify Signs
        for (v_id, expected_sign) in &case.expected_rasi {
            if let Some(v_type) = get_varga_type(v_id) {
                let actual = v_type.calculate_rasi(case.input_longitude);
                assert_eq!(
                    actual, *expected_sign,
                    "Case {} failed for {}: expected {}, got {}",
                    case.case_id, v_id, expected_sign, actual
                );
            }
        }

        // 2. Verify Effective Longitudes (for Nakshatra projection)
        if let Some(eff_map) = &case.expected_effective_long {
            for (v_id, expected_long) in eff_map {
                if let Some(v_type) = get_varga_type(v_id) {
                    let rasi = v_type.calculate_rasi(case.input_longitude);
                    let actual_long = v_type.effective_longitude_for_nakshatra(case.input_longitude, rasi);
                    
                    assert!(
                        (actual_long - expected_long).abs() < 1e-4,
                        "Case {} failed for {} effective longitude: expected {}, got {}",
                        case.case_id, v_id, expected_long, actual_long
                    );
                }
            }
        }
    }
}
