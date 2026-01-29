use eon_vedic::chart::VedicChartCalculator;
use eon_vedic::dasha::Vimshottari;
use eon_vedic::planets::VedicPlanet;
use eon_vedic::yogas::YogaEngine;
use eon_vedic::analysis::nature::{FunctionalNature, FunctionalStatus};
use chrono::{TimeZone, Utc};

fn main() {
    println!("=== Eon Vedic Engine Verification ===");

    // 1. Setup Birth Info: Seoul, Korea at 2000-01-01 12:00 KST (03:00 UTC)
    let birth_time = Utc.ymd(2000, 1, 1).and_hms(3, 0, 0);
    let lat = 37.5665;
    let lon = 126.9780;

    println!("Birth Time: {}", birth_time);
    println!("Location: {}, {}", lat, lon);

    // 2. Calculate Chart
    let calculator = VedicChartCalculator::new();
    let chart = calculator.calculate(birth_time, lat, lon);

    println!("\n[1] Planetary Positions & Functional Nature");
    let lagna_rasi = chart.ascendant.rasi;
    println!("Ascendant (Lagna): {:.2}° (Rasi: {})", chart.ascendant.sidereal_deg, lagna_rasi);
    
    let mut moon_long = 0.0;
    
    for pos in &chart.planets {
        let nature = FunctionalNature::analyze(lagna_rasi, pos.planet);
        let nature_str = match nature {
            FunctionalStatus::Yogakaraka => "Yogakaraka (Best)",
            FunctionalStatus::FunctionalBenefic => "Benefic",
            FunctionalStatus::Neutral => "Neutral",
            FunctionalStatus::FunctionalMalefic => "Malefic",
            FunctionalStatus::Maraka => "Maraka (Killer)",
        };

        println!("{:<12} | House: {:>2} | Sidereal: {:>6.2}° | Nature: {}", 
            format!("{:?}", pos.planet), 
            pos.house_index,
            pos.sidereal_deg, 
            nature_str
        );

        if pos.planet == VedicPlanet::Moon {
            moon_long = pos.sidereal_deg;
        }
    }

    // 3. Yoga Check
    println!("\n[2] Yoga Analysis");
    let yogas = YogaEngine::check_yogas(&chart);
    if yogas.is_empty() {
        println!("  No major Yogas found.");
    } else {
        for yoga in yogas {
            println!("  ▶ {} ({:?}): {}", yoga.name, yoga.yoga_type, yoga.description);
        }
    }

    // 4. Vimshottari Dasha
    println!("\n[3] Vimshottari Dasha Timeline");
    let dashas = Vimshottari::calculate(moon_long, birth_time, 1); // Depth 1 for brevity
    
    for d in dashas {
         let nature = FunctionalNature::analyze(lagna_rasi, d.planet);
         let nature_icon = match nature {
            FunctionalStatus::Yogakaraka => "🌟",
            FunctionalStatus::FunctionalBenefic => "🟢",
            FunctionalStatus::FunctionalMalefic => "🔴",
            FunctionalStatus::Maraka => "💀",
            _ => "⚪",
         };
         
        println!("▶ {} {:?} Mahadasha: {:.1} years ({} ~ {})", 
            nature_icon, d.planet, d.duration_years, d.start_date.format("%Y-%m-%d"), d.end_date.format("%Y-%m-%d"));
    }
}
