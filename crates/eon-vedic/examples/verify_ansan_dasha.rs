use eon_vedic::chart::VedicChartCalculator;
use eon_vedic::dasha::Vimshottari;
use chrono::{TimeZone, Utc};

fn main() {
    println!("=== Ansan Dasha Verification ===");

    // 2004-11-27 22:00 KST = 13:00 UTC
    let birth_time = Utc.with_ymd_and_hms(2004, 11, 27, 13, 0, 0).unwrap();
    let lat = 37.3167; 
    let lon = 126.8167; 

    let calculator = VedicChartCalculator::new();
    let chart = calculator.calculate(birth_time, lat, lon);

    let moon = chart.planets.iter().find(|p| p.planet == eon_vedic::planets::VedicPlanet::Moon).unwrap();
    println!("Moon Sidereal: {:.4}°", moon.sidereal_deg);

    // Calculate Mahadashas
    let mahadashas = Vimshottari::calculate(moon.sidereal_deg, birth_time, 2);
    
    println!("\n[Mahadasha Timeline]");
    for m in &mahadashas {
        println!("{:?}: {} ~ {} (Antardashas: {})", m.planet, m.start_date.format("%Y-%m-%d"), m.end_date.format("%Y-%m-%d"), m.sub_periods.len());
        
        // Show Antardashas for current or first few
        if m.planet == eon_vedic::planets::VedicPlanet::Moon || m.planet == eon_vedic::planets::VedicPlanet::Mars {
             println!("  [Antardashas]");
             for a in &m.sub_periods {
                 println!("    - {:?}: {}", a.planet, a.start_date.format("%Y-%m-%d"));
             }
        }
    }
}
