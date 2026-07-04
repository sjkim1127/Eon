mod common;
mod fixtures;

use eon_vedic::chart::VedicChartCalculator;
use eon_vedic::config::VedicConfig;
use fixtures::FIXTURES;

#[test]
fn test_vedic_regression_fixtures() {
    for fixture in FIXTURES {
        let mut config = VedicConfig::default();
        config.ayanamsa = fixture.ayanamsa;

        let calculator = VedicChartCalculator::with_config(config);
        let time = fixture.get_time();
        let chart = calculator
            .calculate(time, fixture.latitude, fixture.longitude)
            .unwrap();

        println!("Verifying Fixture: {}", fixture.name);
        fixture.verify(&chart);
    }
}
