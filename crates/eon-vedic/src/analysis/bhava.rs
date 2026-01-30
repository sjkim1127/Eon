use serde::{Deserialize, Serialize};
use crate::planets::VedicPlanet;
use crate::chart::{VedicChart, VedicPosition};
use crate::analysis::strength::StrengthEngine;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BhavaStrength {
    pub house: u8, // 1~12
    pub lord_score: f64,
    pub dig_score: f64,
    pub drishti_score: f64,
    pub total_score: f64,
}

pub struct BhavaEngine;

impl BhavaEngine {
    pub fn calculate_all(chart: &VedicChart) -> Vec<BhavaStrength> {
        let mut results = Vec::with_capacity(12);
        
        for h in 1..=12 {
            results.push(Self::calculate_house(h, chart));
        }
        
        results
    }

    pub fn calculate_house(house: u8, chart: &VedicChart) -> BhavaStrength {
        // 1. Lord Strength (Bhavadhipati Bala)
        let lagna_rasi = chart.ascendant.rasi;
        let rasi_idx = ((lagna_rasi as i32 + house as i32 - 1 - 1) % 12 + 1) as u8;
        let lord = VedicPlanet::get_ruler_of(rasi_idx);
        
        let lord_pos = chart.planets.iter().find(|p| p.planet == lord);
        let lord_score = if let Some(pos) = lord_pos {
            StrengthEngine::calculate(pos, chart).total_score
        } else {
            0.0 // Rahu/Ketu don't own signs in standard PARASHARA
        };

        // 2. Bhava Dig Bala
        // Standard: 
        // H1 (East): Mer, Jup increase
        // H4 (North): Mon, Ven increase
        // H7 (West): Sat increase
        // H10 (South): Sun, Mar increase
        // Simplified: Fixed bonus if Lagna belongs to certain groups? 
        // No, Bhava Dig Bala is usually a score added.
        // Let's use a simplified constant based on house index.
        let dig_score = match house {
            1 | 10 => 30.0, // Major directions
            4 | 7 => 20.0,
            _ => 10.0,
        };

        // 3. Bhava Drishti (Aspect Sum)
        // Benefics (Jup, Ven, Mer, Mon) add power. Malefics substract.
        let mut drishti_score = 0.0;
        for pos in &chart.planets {
            if Self::is_planet_aspecting_house(pos, house, chart) {
                let weight = match pos.planet {
                    VedicPlanet::Jupiter => 30.0,
                    VedicPlanet::Venus => 20.0,
                    VedicPlanet::Mercury | VedicPlanet::Moon => 10.0,
                    VedicPlanet::Sun | VedicPlanet::Mars | VedicPlanet::Saturn => -15.0,
                    VedicPlanet::Rahu | VedicPlanet::Ketu => -10.0,
                    _ => 0.0,
                };
                drishti_score += weight;
            }
        }

        BhavaStrength {
            house,
            lord_score,
            dig_score,
            drishti_score,
            total_score: lord_score + dig_score + drishti_score,
        }
    }

    fn is_planet_aspecting_house(pos: &VedicPosition, house: u8, chart: &VedicChart) -> bool {
        // We can reuse aspects.rs logic here.
        // For simplicity, check if the house is in AspectRelation for this planet.
        if let Some(rel) = chart.aspects.iter().find(|a| a.aspecting_planet == pos.planet) {
            return rel.aspected_houses.contains(&house);
        }
        false
    }
}
