mod common;

use chrono::{TimeZone, Utc};
use eon_vedic::core::chart::VedicChartCalculator;
use eon_vedic::core::config::{NodeCalculation, VedicConfig};
use eon_vedic::planets::VedicPlanet;

#[test]
fn verify_true_vs_mean_nodes() {
    let year = 2024;
    let month = 4;
    let day = 15;
    let hour = 12;
    let lat = 37.5665;
    let lon = 126.9780;
    let dt = Utc.with_ymd_and_hms(year, month, day, hour, 0, 0).unwrap();

    // 1. Calculate with True Node
    let mut config_true = VedicConfig::default();
    config_true.node_calc = NodeCalculation::TrueNode;
    let calc_true = VedicChartCalculator::with_config(config_true);
    let chart_true = calc_true.calculate(dt, lat, lon).unwrap();
    let rahu_true = chart_true
        .planets
        .iter()
        .find(|p| p.planet == VedicPlanet::Rahu)
        .unwrap();

    // 2. Calculate with Mean Node
    let mut config_mean = VedicConfig::default();
    config_mean.node_calc = NodeCalculation::MeanNode;
    let calc_mean = VedicChartCalculator::with_config(config_mean);
    let chart_mean = calc_mean.calculate(dt, lat, lon).unwrap();
    let rahu_mean = chart_mean
        .planets
        .iter()
        .find(|p| p.planet == VedicPlanet::Rahu)
        .unwrap();

    println!(
        "Rahu True: tropical={}, sidereal={}, dec={}",
        rahu_true.tropical_deg, rahu_true.sidereal_deg, rahu_true.declination
    );
    println!(
        "Rahu Mean: tropical={}, sidereal={}, dec={}",
        rahu_mean.tropical_deg, rahu_mean.sidereal_deg, rahu_mean.declination
    );

    // True and Mean nodes should be different (usually within a couple of degrees)
    assert!((rahu_true.tropical_deg - rahu_mean.tropical_deg).abs() > 0.0001);

    // Check if declination also differs (the bug we just fixed)
    assert!((rahu_true.declination - rahu_mean.declination).abs() > 0.0001);
}
