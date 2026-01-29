use eon_vedic::chart::VedicChartCalculator;
use eon_vedic::dasha::Vimshottari;
use eon_vedic::planets::VedicPlanet;
use eon_vedic::yogas::YogaEngine;
use eon_vedic::analysis::nature::{FunctionalNature, FunctionalStatus};
use eon_vedic::analysis::strength::StrengthEngine;
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

    println!("\n[1] Planetary Positions, Nature & Strength");
    let lagna_rasi = chart.ascendant.rasi;
    println!("Ascendant (Lagna): {:.2}° (Rasi: {})", chart.ascendant.sidereal_deg, lagna_rasi);
    
    let mut moon_long = 0.0;
    
    for pos in &chart.planets {
        let nature = FunctionalNature::analyze(lagna_rasi, pos.planet);
        let nature_str = match nature {
            FunctionalStatus::Yogakaraka => "Yogakaraka",
            FunctionalStatus::FunctionalBenefic => "Benefic",
            FunctionalStatus::Neutral => "Neutral",
            FunctionalStatus::FunctionalMalefic => "Malefic",
            FunctionalStatus::Maraka => "Maraka",
        };

        let strength = StrengthEngine::calculate(pos);

        println!("{:<12} | H: {:>2} | Sid: {:>6.2}° | Nature: {:<12} | Str: {:>5.1} ({}){}", 
            format!("{:?}", pos.planet), 
            pos.house_index,
            pos.sidereal_deg, 
            nature_str,
            strength.total_score,
            strength.status,
            if pos.is_retrograde { " (Rx)" } else { "" }
        );

        if pos.planet == VedicPlanet::Moon {
            moon_long = pos.sidereal_deg;
        }
    }

    println!("\n[Deep Varga Scan]");
    for pos in &chart.planets {
         println!("{:<12} | D60: {:>2} | D144: {:>2} | D30: {:>2}", 
            format!("{:?}", pos.planet),
            pos.shashtyamsa_rasi,
            pos.dwadasdwadasamsa_rasi,
            pos.trimsamsa_rasi
         );
    }

    println!("\n[4] Ashtakavarga (SAV) Points");
    print!("Points: ");
    let mut sav_total = 0;
    for (i, p) in chart.sav.points.iter().enumerate() {
        print!("H{}:{} ", i+1, p);
        sav_total += *p as u32;
    }
    println!("\nTotal SAV Points: {}", sav_total);

    println!("\n[5] Planetary Aspects (Drishti)");
    for rel in &chart.aspects {
        println!("  {:<12} aspects Houses: {:?}", format!("{:?}", rel.aspecting_planet), rel.aspected_houses);
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
    let dashas = Vimshottari::calculate(moon_long, birth_time, 1);
    
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
