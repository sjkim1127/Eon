use chrono::{TimeZone, Utc};
use eon_vedic::chart::VedicChartCalculator;
use eon_vedic::planets::VedicPlanet;

#[test]
fn test_vedic_chart_calculation() {
    // 2000-01-01 12:00:00 UTC
    let time = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    
    let calculator = VedicChartCalculator::new();
    let chart = calculator.calculate(time, 0.0, 0.0).unwrap();
    
    println!("Vedic Chart for {}", time);
    for pos in &chart.planets {
        println!("{:?}: Sidereal {:.4} (Rasi: {}, Nak: {}-{})", 
            pos.planet, pos.sidereal_deg, pos.rasi, pos.nakshatra, pos.pada);
    }

    // Verify Sun position (approximate)
    // Tropical Sun ~ 280 deg (Capricorn)
    // Ayanamsa ~ 24 deg
    // Sidereal Sun ~ 256 deg (Sagittarius)
    
    let sun = chart.planets.iter().find(|p| p.planet == VedicPlanet::Sun).unwrap();
    
    // Check if Sun is in Sagittarius (Sign 9) or late Scorpio
    // 280 - 24 = 256 -> Sign 9 (Sagittarius is 240-270)
    // Rasi 9
    assert_eq!(sun.rasi, 9, "Sun should be in Sagittarius in Sidereal zodiac");
}
