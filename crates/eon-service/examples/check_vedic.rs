use eon_service::dto::{AnalysisInput, VedicAnalysisInput};
use eon_service::facade;

fn main() {
    let base = AnalysisInput {
        year: 1990,
        month: 5,
        day: 15,
        hour: 10,
        minute: 0,
        is_lunar: false,
        is_leap_month: false,
        lat: 37.5665,
        lon: 126.9780,
        timezone: "Asia/Seoul".to_string(),
    };

    let input = VedicAnalysisInput::new(base, Some(false), None);

    let result = facade::analyze_vedic(input).expect("Failed to analyze vedic");

    println!("=== Vedic Chart (May 15, 1990, 10:00 AM Seoul) ===");
    println!("Ayanamsa: Lahiri");

    println!("\n--- D1 (Rasi) Chart ---");
    for pos in result.chart.planets.iter() {
        println!(
            "{:?}: {:.2} deg (Rasi: {})",
            pos.planet, pos.sidereal_deg, pos.rasi
        );
    }

    println!(
        "\nAscendant: {:.2} deg (Rasi: {})",
        result.chart.ascendant.sidereal_deg, result.chart.ascendant.rasi
    );

    println!("\n--- D9 (Navamsa) Chart ---");
    for pos in result.chart.planets.iter() {
        println!("{:?}: Navamsa Rasi {}", pos.planet, pos.navamsa_rasi);
    }
    println!(
        "Ascendant: Navamsa Rasi {}",
        result.chart.ascendant.navamsa_rasi
    );
}
