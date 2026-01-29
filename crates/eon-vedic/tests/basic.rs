use chrono::{TimeZone, Utc};
use eon_vedic::chart::VedicChartCalculator;
use eon_vedic::planets::VedicPlanet;

#[test]
fn test_vedic_chart_calculation() {
    // 2000-01-01 12:00:00 UTC
    let time = Utc.ymd(2000, 1, 1).and_hms(12, 0, 0);
    
    let calculator = VedicChartCalculator::new();
    let chart = calculator.calculate(time);
    
    println!("Vedic Chart for {}", time);
    for pos in &chart {
        println!("{:?}: Sidereal {:.4} (Rasi: {}, Nak: {}-{})", 
            pos.planet, pos.sidereal_deg, pos.rasi, pos.nakshatra, pos.pada);
    }

    // Verify Sun position (approximate)
    // Tropical Sun ~ 280 deg (Capricorn)
    // Ayanamsa ~ 24 deg
    // Sidereal Sun ~ 256 deg (Sagittarius)
    
    let sun = chart.iter().find(|p| p.planet == VedicPlanet::Sun).unwrap();
    
    // Check if Sun is in Sagittarius (Sign 9) or late Scorpio
    // 280 - 24 = 256 -> Sign 9 (Sagittarius is 240-270)
    // Rasi 9
    assert_eq!(sun.rasi, 9, "Sun should be in Sagittarius in Sidereal zodiac");
}
