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
    let birth_time = Utc.with_ymd_and_hms(2000, 1, 1, 3, 0, 0).unwrap();
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

        let strength = StrengthEngine::calculate(pos, &chart);

        let avasthas = eon_vedic::analysis::avasthas::AvasthaEngine::calculate(pos);

        println!("{:<12} | H: {:>2} | Sid: {:>6.2}° | Nature: {:<12} | Str: {:>5.1} (D:{:>4.1},K:{:>2.0},A:{:>4.1},P:{:>4.1}) I/K:{:0>2.0}/{:0>2.0} ({}){}{}", 
            format!("{:?}", pos.planet), 
            pos.house_index,
            pos.sidereal_deg, 
            nature_str,
            strength.total_score,
            strength.drik_score,
            strength.kala_score,
            strength.ayana_score,
            strength.paksha_score,
            strength.ishta_phala,
            strength.kashta_phala,
            strength.status,
            if pos.is_retrograde { " (Rx)" } else { "" },
            if pos.is_combust { " (C)" } else { "" }
        );
        println!("             | Avasthas: {:?}, {:?}", avasthas.baladi, avasthas.jagradadi);

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

    println!("\n[6] Trikona Shodhana (Sun Sample)");
    if let Some(sun_bav) = chart.planets.iter().find(|p| p.planet == VedicPlanet::Sun)
        .map(|_| eon_vedic::analysis::ashtakavarga::AshtakavargaEngine::calculate_bav(VedicPlanet::Sun, &chart)) 
    {
        println!("  Raw:     {:?}", sun_bav.points);
        println!("  Reduced: {:?}", sun_bav.shodhana_points);
        println!("  Pinda:   {}", sun_bav.sodya_pinda);
    }

    println!("\n[5] Planetary Aspects (Drishti)");
    for rel in &chart.aspects {
        println!("  {:<12} aspects Houses: {:?}", format!("{:?}", rel.aspecting_planet), rel.aspected_houses);
    }

    println!("\n[9] Jaimini Chara Karakas (8-Karaka System)");
    for k in &chart.karakas {
        println!("  {:<12} -> {:?} ({:.2}°)", format!("{:?}", k.planet), k.role, k.degree_in_rasi);
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

    println!("\n[3] Vimshottari Dasha Timeline");
    let dashas = Vimshottari::calculate(moon_long, birth_time, 1);
    
    for d in dashas.iter().take(5) {
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

    println!("\n[10] Yogini Dasha Timeline (Sub-periods)");
    let yd = eon_vedic::dasha::Yogini::calculate(moon_long, birth_time, 2);
    for d in yd.iter().take(5) {
        println!("▶ {:<8} ({:?}) : {} ~ {}", d.name.as_ref().unwrap(), d.planet, d.start_date.format("%Y-%m-%d"), d.end_date.format("%Y-%m-%d"));
        for sub in d.sub_periods.iter().take(3) {
            println!("   - {:<8} ({:?}) : {} ~ {}", sub.name.as_ref().unwrap(), sub.planet, sub.start_date.format("%Y-%m-%d"), sub.end_date.format("%Y-%m-%d"));
        }
    }

    println!("\n[11] Panchanga (Time Elements)");
    let panchanga = eon_vedic::panchanga::PanchangaEngine::calculate(&chart, birth_time);
    println!("  Vara:      {}", panchanga.vara);
    println!("  Tithi:     {} ({})", panchanga.tithi, panchanga.tithi_name);
    println!("  Nakshatra: {}", panchanga.nakshatra);
    println!("  Yoga:      {}", panchanga.yoga);
    println!("  Karana:    {} ({})", panchanga.karana, panchanga.karana_name);

    println!("\n[12] Compatibility (Ashta Kuta Sample)");
    // Compare same chart with itself to see max score (or near max)
    let comp = eon_vedic::analysis::compatibility::CompatibilityEngine::analyze(&chart, &chart);
    println!("  Score:   {:.1}/36.0", comp.total_score);
    println!("  Summary: {}", comp.message);
    println!("  Breakdown: Nadi:{}, Bhakoot:{}, Gana:{}, Maitri:{}, Yoni:{}, Tara:{}, Vashya:{}, Varna:{}", 
        comp.nadi, comp.bhakoot, comp.gana, comp.maitri, comp.yoni, comp.tara, comp.vashya, comp.varna);

    // 5. Final Polish Verification
    println!("\n[7] Vimshopaka Bala (Varga Strength)");
    println!("{:<10} | Shadvarga (20pt) | Shodashavarga (Avg)", "Planet");
    println!("{}", "-".repeat(50));
    for pos in &chart.planets {
        let v_score = eon_vedic::analysis::vimshopaka::VimshopakaEngine::calculate(pos, &chart);
        println!("{:<10} | {:>14.2} | {:>18.2}", 
            format!("{:?}", pos.planet), 
            v_score.shadvarga_score, 
            v_score.shodashavarga_score
        );
    }

    println!("\n[8] Gochara (Transits) - relative to Natal Moon");
    if let Some(natal_moon) = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon) {
        println!("Natal Moon Rasi: {}", natal_moon.rasi);
        let summary = eon_vedic::analysis::gochara::GocharaEngine::analyze(natal_moon.rasi, &chart);
        println!("Sade Sati Status: {:?}", summary.sade_sati);

        for t in summary.transits {
            if matches!(t.planet, VedicPlanet::Sun | VedicPlanet::Mars | VedicPlanet::Jupiter | VedicPlanet::Saturn | VedicPlanet::Rahu) {
                 println!("  {:<10} in House {:>2} from Moon -> {}{} | Murti: {:?}", 
                    format!("{:?}", t.planet), 
                    t.house_from_moon,
                    if t.is_benefic_transit { "Benefic 🟢" } else { "Malefic 🔴" },
                    if t.is_blocked { " (Blocked by Vedha ⚡)" } else { "" },
                    t.murti
                );
            }
        }
    }

    println!("\n[11] Bhava Bala (House Strength)");
    println!("House |       Lord |        Dig |    Drishti |      Total");
    println!("------------------------------------------------------------");
    for b in &chart.bhava_strengths {
        println!("{:<6} | {:>10.1} | {:>10.1} | {:>10.1} | {:>10.1}", 
            b.house, b.lord_score, b.dig_score, b.drishti_score, b.total_score);
    }

    if let Some(report) = &chart.analysis_report {
        println!("\n[12] FINAL ANALYSIS SUMMARY");
        println!("{}", report.to_text_summary());
    }
}
