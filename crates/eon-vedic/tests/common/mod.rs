use eon_vedic::calc::varga::VargaType;
use eon_vedic::core::chart::{VedicChart, VedicChartCalculator};
use chrono::{Utc, TimeZone};

pub fn get_varga_type(id: &str) -> Option<VargaType> {
    match id.to_lowercase().as_str() {
        "d1" | "rasi" => Some(VargaType::D1),
        "d2" | "hora" => Some(VargaType::D2),
        "d3" | "drekkana" => Some(VargaType::D3),
        "d4" | "chaturthamsha" => Some(VargaType::D4),
        "d5" | "panchamsa" => Some(VargaType::D5),
        "d6" | "shashtamsa" => Some(VargaType::D6),
        "d7" | "saptamsa" => Some(VargaType::D7),
        "d8" | "ashtamsa" => Some(VargaType::D8),
        "d9" | "navamsa" => Some(VargaType::D9),
        "d10" | "dasamsa" => Some(VargaType::D10),
        "d11" | "rudramsa" => Some(VargaType::D11),
        "d12" | "dwadasamsa" => Some(VargaType::D12),
        "d16" | "shodashamsa" => Some(VargaType::D16),
        "d20" | "vimsamsa" => Some(VargaType::D20),
        "d24" | "chaturvimshamsa" => Some(VargaType::D24),
        "d27" | "saptavimsamsa" => Some(VargaType::D27),
        "d30" | "trimsamsa" => Some(VargaType::D30),
        "d40" | "khavedamsa" => Some(VargaType::D40),
        "d45" | "akshavedamsa" => Some(VargaType::D45),
        "d60" | "shashtyamsa" => Some(VargaType::D60),
        "d81" | "navanavamsa" => Some(VargaType::D81),
        "d108" | "ashtottaramsa" => Some(VargaType::D108),
        "d144" | "dwadasdwadasamsa" => Some(VargaType::D144),
        _ => None,
    }
}

pub fn create_test_chart(year: i32, month: u32, day: u32, hour: u32, lat: f64, lon: f64) -> VedicChart {
    let calc = VedicChartCalculator::default();
    let dt = Utc.with_ymd_and_hms(year, month, day, hour, 0, 0).unwrap();
    calc.calculate(dt, lat, lon)
}

pub fn assert_approx_eq(actual: f64, expected: f64, epsilon: f64, msg: &str) {
    if (actual - expected).abs() > epsilon {
        panic!("{}: expected {}, got {}", msg, expected, actual);
    }
}
