use eon_vedic::chart::VedicChartCalculator;
use eon_vedic::planets::VedicPlanet;
use chrono::{TimeZone, Utc};

fn print_chart(date_str: &str, time: chrono::DateTime<Utc>) {
    let lat = 37.3167; // Ansan
    let lon = 126.8167; 
    let calculator = VedicChartCalculator::new();
    let chart = calculator.calculate(time, lat, lon).unwrap();

    println!("\n--- {} ---", date_str);
    println!("ASC:  {:.2}° | Rasi: {} | Nak: {}", chart.ascendant.sidereal_deg, chart.ascendant.rasi, chart.ascendant.nakshatra);
    
    let sun = chart.planets.iter().find(|p| p.planet == VedicPlanet::Sun).unwrap();
    println!("Sun:  {:.2}° | Rasi: {} | Nak: {}", sun.sidereal_deg, sun.rasi, sun.nakshatra);
    
    let moon = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon).unwrap();
    println!("Moon: {:.2}° | Rasi: {} | Nak: {}", moon.sidereal_deg, moon.rasi, moon.nakshatra);
    
    let merc = chart.planets.iter().find(|p| p.planet == VedicPlanet::Mercury).unwrap();
    println!("Merc: {:.2}° | Rasi: {} | Nak: {}", merc.sidereal_deg, merc.rasi, merc.nakshatra);
}

fn main() {
    // Nov 27, 2004 13:00 UTC (22:00 KST)
    print_chart("2004-11-27 (User Input Date)", Utc.with_ymd_and_hms(2004, 11, 27, 13, 0, 0).unwrap());
    
    // Oct 27, 2004 13:00 UTC (22:00 KST)
    print_chart("2004-10-27 (Suspected Actual Date)", Utc.with_ymd_and_hms(2004, 10, 27, 13, 0, 0).unwrap());
}
