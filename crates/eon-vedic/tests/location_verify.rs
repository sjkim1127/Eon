mod common;

use chrono::{TimeZone, Utc, Timelike};
use eon_vedic::core::chart::VedicChartCalculator;

#[test]
fn verify_tokyo_location_calculation() {
    // 2024-04-15 12:00 JST = 2024-04-15 03:00 UTC
    // Tokyo: 35.6764 N, 139.6500 E
    let year = 2024;
    let month = 4;
    let day = 15;
    let hour_utc = 3;
    let lat = 35.6764;
    let lon = 139.6500;

    let calc = VedicChartCalculator::default();
    let dt = Utc.with_ymd_and_hms(year, month, day, hour_utc, 0, 0).unwrap();
    let chart = calc.calculate(dt, lat, lon);

    println!("Tokyo Calculation Trace:");
    println!("  UTC Time: {}", dt);
    println!("  Location: {} N, {} E", lat, lon);
    println!("  Ascendant Sidereal: {:.4}", chart.ascendant.sidereal_deg);
    println!("  Sun Sidereal: {:.4}", chart.planets.iter().find(|p| p.planet == eon_vedic::planets::VedicPlanet::Sun).unwrap().sidereal_deg);
    println!("  Sunrise UTC: {}", chart.panchanga.sunrise);
    println!("  Sunset UTC: {}", chart.panchanga.sunset);

    // Basic consistency checks
    assert!(chart.ascendant.sidereal_deg >= 0.0 && chart.ascendant.sidereal_deg < 360.0);
    
    // For Tokyo on April 15, sunrise should be around 05:10 JST (20:10 UTC previous day)
    let sunrise_jst = chart.panchanga.sunrise.with_timezone(&chrono_tz::Asia::Tokyo);
    println!("  Sunrise JST: {}", sunrise_jst);
    
    // Check if sunrise is within reasonable range (05:00 - 05:30 JST)
    assert!(sunrise_jst.hour() == 5);
}
