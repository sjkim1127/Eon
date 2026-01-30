use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Datelike};
use crate::chart::VedicChart;
use crate::planets::VedicPlanet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panchanga {
    pub vara: String,
    pub tithi: u8,        // 1~30
    pub tithi_name: String,
    pub nakshatra: u8,    // 1~27
    pub yoga: u8,         // 1~27 (Nitya Yoga)
    pub karana: u8,       // 1~60 (Sequential) or 1~11 (Types)
    pub karana_name: String,
}

pub struct PanchangaEngine;

impl PanchangaEngine {
    pub fn calculate(chart: &VedicChart, time: DateTime<Utc>) -> Panchanga {
        let sun = chart.planets.iter().find(|p| p.planet == VedicPlanet::Sun).map(|p| p.sidereal_deg).unwrap_or(0.0);
        let moon = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon).map(|p| p.sidereal_deg).unwrap_or(0.0);

        // 1. Vara (Weekday)
        let vara = time.weekday().to_string();

        // 2. Tithi (Moon - Sun) / 12
        let tithi_deg = (moon - sun + 360.0) % 360.0;
        let tithi = (tithi_deg / 12.0).floor() as u8 + 1;
        let tithi_name = Self::get_tithi_name(tithi);

        // 3. Nakshatra (Moon / 13.333)
        let nakshatra = (moon / (360.0 / 27.0)).floor() as u8 + 1;

        // 4. Yoga (Sun + Moon) / 13.333
        let yoga_deg = (sun + moon) % 360.0;
        let yoga = (yoga_deg / (360.0 / 27.0)).floor() as u8 + 1;

        // 5. Karana (Tithi_deg / 6)
        // Karana is half of a Tithi.
        let karana_idx = (tithi_deg / 6.0).floor() as u16 + 1;
        let karana_name = Self::get_karana_name(karana_idx);

        Panchanga {
            vara,
            tithi,
            tithi_name,
            nakshatra,
            yoga,
            karana: (karana_idx % 11) as u8 + 1, // Simplified type
            karana_name,
        }
    }

    fn get_tithi_name(tithi: u8) -> String {
        let names = [
            "Prathama", "Dwitiya", "Tritiya", "Chaturthi", "Panchami",
            "Shashti", "Saptami", "Ashtami", "Navami", "Dashami",
            "Ekadashi", "Dwadashi", "Trayodashi", "Chaturdashi", "Purnima",
            "Prathama (K)", "Dwitiya (K)", "Tritiya (K)", "Chaturthi (K)", "Panchami (K)",
            "Shashti (K)", "Saptami (K)", "Ashtami (K)", "Navami (K)", "Dashami (K)",
            "Ekadashi (K)", "Dwadashi (K)", "Trayodashi (K)", "Chaturdashi (K)", "Amavasya"
        ];
        names.get(tithi as usize - 1).unwrap_or(&"Unknown").to_string()
    }

    fn get_karana_name(idx: u16) -> String {
        // First Karana (Kimstughna) is special.
        // Then 7 movable Karanas repeat: Bava, Balava, Kaulava, Taitila, Gara, Vanija, Vishti.
        // Last few are fixed: Shakuni, Chatushpada, Naga, Kimstughna (again loop end).
        if idx == 1 { return "Kimstughna".to_string(); }
        if idx >= 58 {
             match idx {
                 58 => return "Shakuni".to_string(),
                 59 => return "Chatushpada".to_string(),
                 60 => return "Naga".to_string(),
                 _ => {}
             }
        }
        let movables = ["Bava", "Balava", "Kaulava", "Taitila", "Gara", "Vanija", "Vishti"];
        let m_idx = ((idx - 2) % 7) as usize;
        movables[m_idx].to_string()
    }
}
