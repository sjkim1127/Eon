use serde::{Deserialize, Serialize};
use crate::planets::VedicPlanet;
use crate::chart::VedicPosition;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetStrength {
    pub planet: VedicPlanet,
    pub exaltation_score: f64, // 0.0 ~ 60.0 (Uchcha Bala)
    pub directional_score: f64,// 0.0 ~ 60.0 (Dig Bala)
    pub chesta_score: f64,     // 0.0 ~ 60.0 (Chesta Bala - Motion)
    pub total_score: f64,      // Aggregate for MVP
    pub status: String,        // "Exalted", "Debilitated", "Strong", "Weak", "Neutral"
}

pub struct StrengthEngine;

impl StrengthEngine {
    /// Calculate basic strength metrics (Shadbala Lite)
    pub fn calculate(pos: &VedicPosition) -> PlanetStrength {
        let ex_score = Self::calculate_uchcha_bala(pos.planet, pos.sidereal_deg);
        let dig_score = Self::calculate_dig_bala(pos.planet, pos.house_index);
        let chesta_score = Self::calculate_chesta_bala(pos);
        
        let total = ex_score + dig_score + chesta_score;
        
        // Simple status determination
        let status = if ex_score >= 50.0 {
            "Exalted".to_string()
        } else if ex_score <= 10.0 {
            "Debilitated".to_string()
        } else if total > 120.0 {
            "Strong".to_string()
        } else if total < 60.0 {
            "Weak".to_string()
        } else {
            "Neutral".to_string()
        };

        PlanetStrength {
            planet: pos.planet,
            exaltation_score: ex_score,
            directional_score: dig_score,
            chesta_score,
            total_score: total,
            status,
        }
    }

    /// Chesta Bala (Motion Strength)
    /// Simplified: Planets gain strength when retrograde or moving slowly.
    fn calculate_chesta_bala(pos: &VedicPosition) -> f64 {
        if pos.planet == VedicPlanet::Sun || pos.planet == VedicPlanet::Moon {
            // Luminaries gain strength from other factors (Ayana/Paksha), but here we return a neutral 30.
            return 30.0;
        }

        if pos.is_retrograde {
            // Retrograde planets are considered strong in Chesta Bala.
            return 60.0;
        }

        // Stationary or very slow planets are also strong.
        // Average speeds: Mars 0.5, Merc 1.4, Jup 0.1, Ven 1.2, Sat 0.03
        let avg_speed = match pos.planet {
            VedicPlanet::Mars => 0.5,
            VedicPlanet::Mercury => 1.4,
            VedicPlanet::Jupiter => 0.08,
            VedicPlanet::Venus => 1.2,
            VedicPlanet::Saturn => 0.03,
            _ => 1.0,
        };

        let ratio = (pos.speed.abs() / avg_speed).min(2.0);
        // Strength is inversely proportional to speed relative to average.
        // Max 60 units.
        (60.0 * (1.1 - (ratio / 2.0))).max(0.0).min(60.0)
    }

    /// Uchcha Bala (Exaltation Strength)
    /// Max 60 units at Deep Exaltation point, 0 units at Deep Debilitation point.
    fn calculate_uchcha_bala(planet: VedicPlanet, longitude: f64) -> f64 {
        let deep_exalt_deg = match planet {
            VedicPlanet::Sun => 10.0,      // Aries 10
            VedicPlanet::Moon => 33.0,     // Taurus 3
            VedicPlanet::Mars => 298.0,    // Capricorn 28
            VedicPlanet::Mercury => 165.0, // Virgo 15
            VedicPlanet::Jupiter => 95.0,   // Cancer 5
            VedicPlanet::Venus => 357.0,   // Pisces 27
            VedicPlanet::Saturn => 200.0,  // Libra 20
            _ => return 30.0, // Nodes/ASC default
        };

        let deep_debilit_deg = (deep_exalt_deg + 180.0) % 360.0;
        
        // Arc distance from Deep Debilitation point
        let mut arc = (longitude - deep_debilit_deg).abs();
        if arc > 180.0 { 
            arc = 360.0 - arc; 
        }
        
        // Score = Distance / 3 (since 180 degrees = 60 units)
        arc / 3.0
    }

    /// Dig Bala (Directional Strength)
    /// Max 60 units at powerful house, 0 units at opposite (weakest) house.
    fn calculate_dig_bala(planet: VedicPlanet, house: u8) -> f64 {
        let power_house = match planet {
            VedicPlanet::Mercury | VedicPlanet::Jupiter => 1,
            VedicPlanet::Sun | VedicPlanet::Mars => 10,
            VedicPlanet::Saturn => 7,
            VedicPlanet::Moon | VedicPlanet::Venus => 4,
            _ => return 30.0,
        };
        
        let weak_house = match power_house {
            1 => 7,
            10 => 4,
            7 => 1,
            4 => 10,
            _ => 1,
        };
        
        // Shortest distance in houses (12 houses total)
        let diff = (house as i32 - weak_house as i32).abs();
        let dist_houses = if diff > 6 { 12 - diff } else { diff };
        
        // Score = (Houses Dist / 6) * 60 = Houses Dist * 10
        dist_houses as f64 * 10.0
    }
}
