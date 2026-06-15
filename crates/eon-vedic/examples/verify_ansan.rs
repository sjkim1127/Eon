use eon_vedic::chart::VedicChartCalculator;
use chrono::{TimeZone, Utc};

fn main() {
    println!("=== Ansan Chart Verification ===");

    // 2004-11-27 22:00 KST = 13:00 UTC
    let birth_time = Utc.with_ymd_and_hms(2004, 11, 27, 13, 0, 0).unwrap();
    let lat = 37.3167; // 37°19'N
    let lon = 126.8167; // 126°49'E

    let calculator = VedicChartCalculator::new();
    let chart = calculator.calculate(birth_time, lat, lon).unwrap();

    println!("Birth Time: {}", birth_time);
    println!("Ascendant (Lagna): {:.2}° (Rasi: {})", chart.ascendant.sidereal_deg, chart.ascendant.rasi);
    println!("Nakshatra: {}, Pada: {}", chart.ascendant.nakshatra, chart.ascendant.pada);

    println!("\nPlanetary Positions:");
    for pos in &chart.planets {
        println!("{:<12} | Sidereal: {:>6.2}° | Rasi: {:>2} | House: {:>2} | Nak: {:>2} | Pada: {}", 
            format!("{:?}", pos.planet), 
            pos.sidereal_deg, 
            pos.rasi,
            pos.house_index,
            pos.nakshatra,
            pos.pada
        );
    }
}
