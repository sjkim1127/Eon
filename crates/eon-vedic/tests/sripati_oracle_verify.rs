use chrono::{TimeZone, Utc};
use eon_vedic::config::{HouseSystem, VedicConfig};
use eon_vedic::core::chart::VedicChartCalculator;

#[test]
fn sripati_houses_match_swiss_ephemeris() {
    let mut config = VedicConfig::default();
    config.house_system = HouseSystem::Sripati;
    let calculator = VedicChartCalculator::with_config(config);
    let time = Utc.with_ymd_and_hms(2024, 4, 15, 12, 0, 0).unwrap();
    let chart = calculator.calculate(time, 37.5665, 126.9780).unwrap();

    let expected_ascendant = 205.2551911344;
    assert!((chart.ascendant.sidereal_deg - expected_ascendant).abs() < 0.01);

    let expected_cusps = [
        191.8437433499,
        221.8437433499,
        255.0208477807,
        288.1979522116,
        318.1979522116,
        345.0208477807,
        11.8437433499,
        41.8437433499,
        75.0208477807,
        108.1979522116,
        138.1979522116,
        165.0208477807,
    ];
    assert_eq!(chart.house_cusps.len(), expected_cusps.len());
    for (actual, expected) in chart.house_cusps.iter().zip(expected_cusps) {
        assert!(
            (actual - expected).abs() < 0.01,
            "expected {expected}, got {actual}"
        );
    }
}
