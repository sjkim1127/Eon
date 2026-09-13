use chrono::{TimeZone, Utc};
use eon_vedic::core::chart::VedicChartCalculator;

/// Swiss Ephemeris 2.10.3.2, Moshier + Lahiri reference positions:
/// Sun=1.8116937568°, Moon=88.3618787076° sidereal at this instant.
#[test]
fn panchanga_rules_match_external_swiss_positions() {
    let time = Utc.with_ymd_and_hms(2024, 4, 15, 12, 0, 0).unwrap();
    let chart = VedicChartCalculator::default()
        .calculate(time, 37.5665, 126.9780)
        .unwrap();

    // Standard rules: tithi=(Moon-Sun)/12°, nakshatra=Moon/13°20',
    // yoga=(Sun+Moon)/13°20', karana=(Moon-Sun)/6°.
    assert_eq!(chart.panchanga.tithi, 8);
    assert_eq!(chart.panchanga.nakshatra, 7);
    assert_eq!(chart.panchanga.yoga, 7);
    assert_eq!(chart.panchanga.karana, 8); // sequential half-tithi 15 -> Vishti
}

#[test]
fn panchanga_rules_hold_for_historical_and_future_oracles() {
    for (time, lat, lon, expected) in [
        (
            Utc.with_ymd_and_hms(1988, 3, 14, 15, 0, 0).unwrap(),
            37.5665,
            126.978,
            (26, 22, 19, 3), // half-tithi 52 maps to the rotating Karana type 3
        ),
        (
            Utc.with_ymd_and_hms(2050, 6, 21, 0, 0, 0).unwrap(),
            -33.8688,
            151.2093,
            (2, 7, 12, 4),
        ),
    ] {
        let chart = VedicChartCalculator::default()
            .calculate(time, lat, lon)
            .unwrap();
        assert_eq!(
            (
                chart.panchanga.tithi,
                chart.panchanga.nakshatra,
                chart.panchanga.yoga,
                chart.panchanga.karana,
            ),
            expected
        );
    }
}
