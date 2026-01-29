use serde::{Deserialize, Serialize};
use crate::chart::VedicChart;
use crate::planets::VedicPlanet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum YogaType {
    RajaYoga, // Generic Raja Yoga
    GajaKesari, // Jupiter + Moon
    Sunapha, // Planet in 2nd from Moon
    Anapha, // Planet in 12th from Moon
    DhanaYoga, // Wealth Yoga
    Budhaditya, // Sun + Mercury
    DharmaKarmaAdhipati, // 9th + 10th Lord
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YogaResult {
    pub name: String,
    pub yoga_type: YogaType,
    pub description: String,
    pub planets_involved: Vec<VedicPlanet>,
}

pub struct YogaEngine;

impl YogaEngine {
    pub fn check_yogas(chart: &VedicChart) -> Vec<YogaResult> {
        let mut results = Vec::new();

        // Helper: Get planet position
        let get_planet = |p: VedicPlanet| -> Option<&crate::chart::VedicPosition> {
            chart.planets.iter().find(|pos| pos.planet == p)
        };

        // --- 1. Basic Yogas (Gajakesari, etc.) ---
        if let (Some(moon), Some(jupiter)) = (get_planet(VedicPlanet::Moon), get_planet(VedicPlanet::Jupiter)) {
            // Gajakesari: Jupiter in Kendra (1, 4, 7, 10) from Moon
            let diff = (jupiter.rasi as i32 - moon.rasi as i32);
            let dist = if diff >= 0 { diff + 1 } else { diff + 13 };
            
            if [1, 4, 7, 10].contains(&dist) {
                results.push(YogaResult {
                    name: "Gaja Kesari Yoga".to_string(),
                    yoga_type: YogaType::GajaKesari,
                    description: "Jupiter in Kendra from Moon. Wisdom, virtue, reputation.".to_string(),
                    planets_involved: vec![VedicPlanet::Moon, VedicPlanet::Jupiter],
                });
            }
        }
        
        if let (Some(sun), Some(mercury)) = (get_planet(VedicPlanet::Sun), get_planet(VedicPlanet::Mercury)) {
             // In same sign
             if sun.rasi == mercury.rasi {
                 results.push(YogaResult {
                    name: "Budhaditya Yoga".to_string(),
                    yoga_type: YogaType::Budhaditya,
                    description: "Sun and Mercury conjunction. Intelligence and communication skills.".to_string(),
                    planets_involved: vec![VedicPlanet::Sun, VedicPlanet::Mercury],
                });
             }
        }

        // --- 2. Advanced Lordship Yogas ---
        // Lagna Rasi
        let lagna_rasi = chart.ascendant.rasi;

        // Determine Lords of Houses
        // Helper to find planet owning a specific house relative to Lagna
        let get_lord_of_house = |house_idx: u8| -> VedicPlanet {
            // House 1 = Lagna Rasi
            // House N = (Lagna Rasi + N - 1 - 1) % 12 + 1 ... or simpler:
            // Rasi = (Lagna + House - 1 - 1) % 12 + 1
            let rasi_idx = ((lagna_rasi as i32 + house_idx as i32 - 1 - 1) % 12 + 1) as u8;
            VedicPlanet::get_ruler_of(rasi_idx)
        };

        let lord_9 = get_lord_of_house(9);
        let lord_10 = get_lord_of_house(10);
        
        // Check Dharma-Karma Adhipati (Conjunction of 9th and 10th Lords)
        if let (Some(p9), Some(p10)) = (get_planet(lord_9), get_planet(lord_10)) {
            if p9.rasi == p10.rasi {
                 results.push(YogaResult {
                    name: "Dharma-Karma Adhipati Yoga".to_string(),
                    yoga_type: YogaType::DharmaKarmaAdhipati,
                    description: "Conjunction of 9th and 10th Lords. Great success in career and life purpose.".to_string(),
                    planets_involved: vec![lord_9, lord_10],
                });
            }
        }

        // Dhana Yoga (Wealth): Lords of 2, 5, 9, 11 in association
        // Simpler check: Lord of 2 in 11, or Lord of 11 in 2.
        let lord_2 = get_lord_of_house(2);
        let lord_11 = get_lord_of_house(11);
        
        if let Some(p2) = get_planet(lord_2) {
            if p2.house_index == 11 {
                 results.push(YogaResult {
                    name: "Dhana Yoga (2 in 11)".to_string(),
                    yoga_type: YogaType::DhanaYoga,
                    description: "2nd Lord in 11th House. Great wealth potential.".to_string(),
                    planets_involved: vec![lord_2],
                });
            }
        }
        if let Some(p11) = get_planet(lord_11) {
            if p11.house_index == 2 {
                 results.push(YogaResult {
                    name: "Dhana Yoga (11 in 2)".to_string(),
                    yoga_type: YogaType::DhanaYoga,
                    description: "11th Lord in 2nd House. Financial gains.".to_string(),
                    planets_involved: vec![lord_11],
                });
            }
        }


        // Sunapha / Anapha (Planets in 2nd / 12th from Moon, excluding Sun, Rahu, Ketu)
        if let Some(moon) = get_planet(VedicPlanet::Moon) {
             let moon_rasi = moon.rasi;
             
             // 2nd from Moon
             let next_rasi = if moon_rasi == 12 { 1 } else { moon_rasi + 1 };
             let has_sunapha = chart.planets.iter().any(|p| 
                 p.rasi == next_rasi && 
                 p.planet != VedicPlanet::Sun && 
                 p.planet != VedicPlanet::Rahu && 
                 p.planet != VedicPlanet::Ketu
             );
             
             if has_sunapha {
                 results.push(YogaResult {
                    name: "Sunapha Yoga".to_string(),
                    yoga_type: YogaType::Sunapha,
                    description: "Planets in 2nd from Moon. Wealth and intelligence.".to_string(),
                    planets_involved: vec![VedicPlanet::Moon], // Simplified list
                });
             }
             
             // 12th from Moon
             let prev_rasi = if moon_rasi == 1 { 12 } else { moon_rasi - 1 };
             let has_anapha = chart.planets.iter().any(|p| 
                 p.rasi == prev_rasi && 
                 p.planet != VedicPlanet::Sun && 
                 p.planet != VedicPlanet::Rahu && 
                 p.planet != VedicPlanet::Ketu
             );
             
             if has_anapha {
                 results.push(YogaResult {
                    name: "Anapha Yoga".to_string(),
                    yoga_type: YogaType::Anapha,
                    description: "Planets in 12th from Moon. Good health and character.".to_string(),
                    planets_involved: vec![VedicPlanet::Moon],
                });
             }
        }

        results
    }
}
