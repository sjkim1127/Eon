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
        // BPHS: Strength depends on the Rasi (Sign) type at the house.
        // House 1: Human, House 4: Watery, House 7: Insect/Keeta, House 10: Quadruped.
        let sign_at_house = ((chart.ascendant.rasi as u8 + house - 2) % 12) + 1;
        let dig_score = match house {
            1 => if Self::is_human_sign(sign_at_house) { 60.0 } else { 30.0 }, // (Simplified weighting)
            4 => if Self::is_watery_sign(sign_at_house) { 60.0 } else { 20.0 },
            7 => if sign_at_house == 8 { 60.0 } else { 15.0 }, // Scorpio (Insect)
            10 => if Self::is_quadruped_sign(sign_at_house) { 60.0 } else { 30.0 },
            _ => 10.0,
        };

        // 3. Bhava Drishti (Aspect Sum with Virupa Score)
        // Full aspect (60 Virupas = ~100% weight) when exact.
        let mut drishti_score = 0.0;
        let house_center_deg = (chart.ascendant.sidereal_deg + (house as f64 - 1.0) * 30.0) % 360.0;

        for pos in &chart.planets {
            let aspect_strength = Self::calculate_aspect_strength(pos, house_center_deg);
            if aspect_strength > 0.0 {
                let weight = match pos.planet {
                    VedicPlanet::Jupiter => 0.5, // Jupiter is most benefic
                    VedicPlanet::Venus => 0.4,
                    VedicPlanet::Mercury | VedicPlanet::Moon => 0.2,
                    VedicPlanet::Sun | VedicPlanet::Mars | VedicPlanet::Saturn => -0.3,
                    VedicPlanet::Rahu | VedicPlanet::Ketu => -0.2,
                    _ => 0.0,
                };
                // Score = logic (aspect_strength / 60.0) * weight * 100.0
                drishti_score += (aspect_strength / 60.0) * weight * 100.0;
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


    fn calculate_aspect_strength(pos: &VedicPosition, house_center: f64) -> f64 {
        // Simple Virupa-like scaling
        // Distance between planet and target house center
        let p_deg = pos.sidereal_deg;
        let diff = (house_center - p_deg + 360.0) % 360.0;
        
        let mut max_strength = 0.0;
        
        // 7th House Aspect (All)
        max_strength = f64::max(max_strength, Self::virupa_at(diff, 180.0));
        
        // Special Aspects
        match pos.planet {
            VedicPlanet::Mars => {
                max_strength = f64::max(max_strength, Self::virupa_at(diff, 90.0));  // 4th
                max_strength = f64::max(max_strength, Self::virupa_at(diff, 210.0)); // 8th
            }
            VedicPlanet::Jupiter | VedicPlanet::Rahu | VedicPlanet::Ketu => {
                max_strength = f64::max(max_strength, Self::virupa_at(diff, 120.0)); // 5th
                max_strength = f64::max(max_strength, Self::virupa_at(diff, 240.0)); // 9th
            }
            VedicPlanet::Saturn => {
                max_strength = f64::max(max_strength, Self::virupa_at(diff, 60.0));  // 3rd
                max_strength = f64::max(max_strength, Self::virupa_at(diff, 270.0)); // 10th
            }
            _ => {}
        }
        
        max_strength
    }

    fn virupa_at(diff: f64, target_angle: f64) -> f64 {
        let orb = 15.0; // 15 degrees orb
        let distance = (diff - target_angle).abs();
        let distance = if distance > 180.0 { 360.0 - distance } else { distance };
        
        if distance < orb {
            // Linear scale from 60 (at 0 distance) to 0 (at orb distance)
            60.0 * (1.0 - distance / orb)
        } else {
            0.0
        }
    }

    fn is_human_sign(rasi: u8) -> bool {
        matches!(rasi, 3 | 6 | 7 | 11) // Gemini, Virgo, Libra, Aquarius
    }

    fn is_watery_sign(rasi: u8) -> bool {
        matches!(rasi, 4 | 12) // Cancer, Pisces (Also 2nd half Capricorn, but simplified)
    }

    fn is_quadruped_sign(rasi: u8) -> bool {
        matches!(rasi, 1 | 2 | 5) // Aries, Taurus, Leo (Also part of Sag/Cap)
    }
}
