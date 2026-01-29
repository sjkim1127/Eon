use eon_vedic::chart::VedicChartCalculator;
use eon_vedic::dasha::Vimshottari;
use eon_vedic::planets::VedicPlanet;
use chrono::{TimeZone, Utc};

fn main() {
    println!("=== Eon Vedic Engine Verification ===");

    // 1. Setup Birth Info: Seoul, Korea at 2000-01-01 12:00 KST (03:00 UTC)
    let birth_time = Utc.ymd(2000, 1, 1).and_hms(3, 0, 0);
    let lat = 37.5665;
    let lon = 126.9780;

    println!("Birth Time: {}", birth_time);
    println!("Location: {}, {}", lat, lon);

    // 2. Calculate Chart (Lagna + Planets)
    let calculator = VedicChartCalculator::new();
    let positions = calculator.calculate(birth_time, lat, lon);

    println!("\n[1] Planetary Positions Check");
    let mut moon_long = 0.0;
    
    for pos in &positions {
        let planet_name = match pos.planet {
            VedicPlanet::Ascendant => "Lagna(ASC)",
            _ => {
               // derive Debug is available
               &format!("{:?}", pos.planet)
            }
        };
        
        println!("{:<12} | Sidereal: {:>6.2}° | Rasi: {:>2} | Nak: {:>2} | Pada: {}", 
            planet_name, pos.sidereal_deg, pos.rasi, pos.nakshatra, pos.pada);

        if pos.planet == VedicPlanet::Moon {
            moon_long = pos.sidereal_deg;
        }
    }

    // 3. Verify Ayanamsa (Indirectly checked via positions, but let's see if we can expose it via calculator if needed, 
    // or just trust the positions are sidereal).
    // The calculator uses get_lahiri_ayanamsa internally.
    
    // 4. Verify Vimshottari Dasha
    println!("\n[2] Vimshottari Dasha Timeline (Moon Long: {:.2}°)", moon_long);
    let dashas = Vimshottari::calculate(moon_long, birth_time, 2); // Depth 2 for Antardasha
    
    for d in dashas {
        println!("▶ {:?} Mahadasha: {:.1} years", d.planet, d.duration_years);
        println!("  Start: {}", d.start_date.format("%Y-%m-%d"));
        println!("  End:   {}", d.end_date.format("%Y-%m-%d"));
        
        // Print first few Antardashas
        for sub in d.sub_periods.iter().take(3) {
             println!("    - {:?} Antardasha: {} ~ {}", sub.planet, sub.start_date.format("%Y-%m-%d"), sub.end_date.format("%Y-%m-%d"));
        }
        if d.sub_periods.len() > 3 {
            println!("    - ...");
        }
    }
}
