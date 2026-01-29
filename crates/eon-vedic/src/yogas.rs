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
             if sun.rasi == mercury.rasi {
                 results.push(YogaResult {
                    name: "Budhaditya Yoga".to_string(),
                    yoga_type: YogaType::Budhaditya,
                    description: "Sun and Mercury conjunction. Intelligence and communication skills.".to_string(),
                    planets_involved: vec![VedicPlanet::Sun, VedicPlanet::Mercury],
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
