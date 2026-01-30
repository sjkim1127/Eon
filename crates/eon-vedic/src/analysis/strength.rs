use serde::{Deserialize, Serialize};
use crate::planets::VedicPlanet;
use crate::chart::VedicPosition;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetStrength {
    pub planet: VedicPlanet,
    pub exaltation_score: f64, // 0.0 ~ 60.0 (Uchcha Bala)
    pub directional_score: f64,// 0.0 ~ 60.0 (Dig Bala)
    pub chesta_score: f64,     // 0.0 ~ 60.0 (Chesta Bala - Motion)
    pub naisargika_score: f64, // 0.0 ~ 60.0 (Natural strength)
    pub kala_score: f64,       // 0.0 ~ 60.0 (Time strength - Day/Night)
    pub drik_score: f64,       // Aspect strength (can be negative)
    pub paksha_score: f64,     // Moon Phase strength
    pub ayana_score: f64,      // Declination strength
    pub ishta_phala: f64,      // Auspiciousness (0-60)
    pub kashta_phala: f64,     // Inauspiciousness (0-60)
    pub total_score: f64,      // Aggregate for MVP
    pub status: String,        // "Exalted", "Debilitated", "Strong", "Weak", "Neutral"
}

pub struct StrengthEngine;

impl StrengthEngine {
    /// Calculate basic strength metrics (Shadbala Lite)
    pub fn calculate(pos: &VedicPosition, chart: &crate::chart::VedicChart) -> PlanetStrength {
        let ex_score = Self::calculate_uchcha_bala(pos.planet, pos.sidereal_deg);
        let dig_score = Self::calculate_dig_bala(pos.planet, pos.house_index);
        let chesta_score = Self::calculate_chesta_bala(pos);
        let naisargika_score = Self::calculate_naisargika_bala(pos.planet);
        
        let sun_house = chart.planets.iter().find(|p| p.planet == VedicPlanet::Sun).map(|p| p.house_index).unwrap_or(1);
        let is_day = sun_house >= 7 && sun_house <= 12;
        let kala_score = Self::calculate_kala_bala(pos.planet, is_day);
        
        let drik_score = Self::calculate_drik_bala(pos, chart);
        let paksha_score = Self::calculate_paksha_bala(pos.planet, chart);
        let ayana_score = Self::calculate_ayana_bala(pos.planet, pos.declination);
        
        // Ishta & Kashta Phala based on Exaltation (Uchcha) and Motion (Chesta)
        let (ishta_phala, kashta_phala) = Self::calculate_ishta_kashta(ex_score, chesta_score);

        let total = ex_score + dig_score + chesta_score + naisargika_score + kala_score + drik_score + paksha_score + ayana_score;
        
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
            naisargika_score,
            kala_score,
            drik_score,
            paksha_score,
            ayana_score,
            ishta_phala,
            kashta_phala,
            total_score: total,
            status,
        }
    }

    /// Ishta & Kashta Phala
    /// Based on Uchcha Bala and Chesta Bala.
    /// Formula:
    /// Ishta = (Uchcha + Chesta) / 2
    /// Kashta = 60 - Ishta
    fn calculate_ishta_kashta(uchcha: f64, chesta: f64) -> (f64, f64) {
        let ishta = (uchcha + chesta) / 2.0;
        let kashta = (60.0 - ishta).max(0.0);
        (ishta, kashta)
    }

    /// Ayana Bala (Equinoctial/Declination Strength)
    /// Based on planet's declination and nature.
    /// Max Dec is approx 24 deg.
    /// Sun, Mars, Jupiter, Venus: Strong in North (+).
    /// Moon, Saturn: Strong in South (-).
    /// Mercury: Always strong (or neutral 30).
    fn calculate_ayana_bala(planet: VedicPlanet, declination: f64) -> f64 {
        // Max declination ~24.0. Normalize to 0~60.
        // Formula: Score = 30 + (Dec / 24) * 30 * DirectionFactor
        
        let direction_factor = match planet {
            VedicPlanet::Sun | VedicPlanet::Mars | VedicPlanet::Jupiter | VedicPlanet::Venus => 1.0,
            VedicPlanet::Moon | VedicPlanet::Saturn => -1.0,
            VedicPlanet::Mercury => 1.0, // Mercury follows Sun usually
            _ => 0.0,
        };
        
        // Dec range -24 to +24
        // If Sun (North pref) has +24 Dec => 30 + 1 * 30 = 60.
        // If Sun has -24 Dec => 30 - 1 * 30 = 0.
        
        let val = (declination / 24.0).max(-1.0).min(1.0); // Clamp -1 to 1
        let score = 30.0 + (val * 30.0 * direction_factor);
        
        score.max(0.0).min(60.0)
    }

    /// Paksha Bala (Moon Phase Strength)
    /// Benefics (Jup, Ven, Mon, Mer) gain in Waxing (Shukla).
    /// Malefics (Sun, Mar, Sat) gain in Waning (Krishna).
    fn calculate_paksha_bala(planet: VedicPlanet, chart: &crate::chart::VedicChart) -> f64 {
        let sun = chart.planets.iter().find(|p| p.planet == VedicPlanet::Sun);
        let moon = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon);
        
        if let (Some(s), Some(m)) = (sun, moon) {
            let mut angle = m.sidereal_deg - s.sidereal_deg;
            if angle < 0.0 { angle += 360.0; }
            
            // Paksha Point (0 to 60)
            // 0 (New Moon) -> 180 (Full Moon) -> 360 (New Moon)
            // Waxing: 0 to 180. Point increases.
            // Waning: 180 to 360. Point decreases.
            
            let moon_strength_base = if angle <= 180.0 {
                // Waxing: 0 -> 60 (at 180 deg)
                angle / 3.0
            } else {
                // Waning: 60 -> 0 
                (360.0 - angle) / 3.0
            };
            
            let is_benefic = match planet {
                VedicPlanet::Jupiter | VedicPlanet::Venus | VedicPlanet::Moon | VedicPlanet::Mercury => true,
                _ => false,
            };
            
            if is_benefic {
                moon_strength_base
            } else {
                60.0 - moon_strength_base
            }
        } else {
            30.0
        }
    }

    /// Drik Bala (Aspect Strength)
    /// Calculates the sum of aspect values (Drishti) from all other planets.
    fn calculate_drik_bala(pos: &VedicPosition, chart: &crate::chart::VedicChart) -> f64 {
        let mut total_drik = 0.0;
        
        for aspector in &chart.planets {
            if aspector.planet == pos.planet { continue; }
            
            let diff = (pos.sidereal_deg - aspector.sidereal_deg + 360.0) % 360.0;
            let val = Self::get_aspect_value(aspector.planet, diff);
            
            // Influence of aspecting planet nature
            let is_malefic = matches!(aspector.planet, VedicPlanet::Sun | VedicPlanet::Mars | VedicPlanet::Saturn | VedicPlanet::Rahu | VedicPlanet::Ketu);
            
            if is_malefic {
                total_drik -= val / 4.0;
            } else {
                total_drik += val / 4.0;
            }
        }
        
        total_drik
    }

    fn get_aspect_value(planet: VedicPlanet, diff: f64) -> f64 {
        let mut val = if diff > 30.0 && diff <= 60.0 {
            diff - 30.0
        } else if diff > 60.0 && diff <= 90.0 {
            15.0 
        } else if diff > 90.0 && diff <= 120.0 {
            (diff - 90.0) + 15.0
        } else if diff > 120.0 && diff <= 150.0 {
            45.0
        } else if diff > 150.0 && diff <= 180.0 {
            (diff - 150.0) / 2.0 + 45.0
        } else if diff > 180.0 && diff <= 300.0 {
            (300.0 - diff) / 2.0
        } else {
            0.0
        };

        // Special Aspects
        match planet {
            VedicPlanet::Mars => {
                 if (diff - 90.0).abs() < 15.0 { val = 60.0; }
                 if (diff - 210.0).abs() < 15.0 { val = 60.0; }
            },
            VedicPlanet::Jupiter => {
                 if (diff - 120.0).abs() < 15.0 { val = 60.0; }
                 if (diff - 240.0).abs() < 15.0 { val = 60.0; }
            },
            VedicPlanet::Saturn => {
                 if (diff - 60.0).abs() < 15.0 { val = 60.0; }
                 if (diff - 270.0).abs() < 15.0 { val = 60.0; }
            },
            _ => {}
        }

        val.max(0.0).min(60.0)
    }

    /// Naisargika Bala (Natural Strength)
    /// Fixed values from Sun (strongest) to Saturn (weakest)
    fn calculate_naisargika_bala(planet: VedicPlanet) -> f64 {
        match planet {
            VedicPlanet::Sun => 60.0,
            VedicPlanet::Moon => 51.43,
            VedicPlanet::Venus => 42.86,
            VedicPlanet::Jupiter => 34.29,
            VedicPlanet::Mercury => 25.71,
            VedicPlanet::Mars => 17.14,
            VedicPlanet::Saturn => 8.57,
            _ => 0.0,
        }
    }

    /// Simple Kala Bala (Diva-Ratri Bala)
    /// Day planets strong in Day, Night planets strong in Night.
    fn calculate_kala_bala(planet: VedicPlanet, is_day: bool) -> f64 {
        let is_day_planet = matches!(planet, VedicPlanet::Sun | VedicPlanet::Jupiter | VedicPlanet::Venus);
        let is_night_planet = matches!(planet, VedicPlanet::Moon | VedicPlanet::Mars | VedicPlanet::Saturn);
        
        if planet == VedicPlanet::Mercury { return 60.0; } // Mercury always strong in time
        
        if is_day {
            if is_day_planet { 60.0 } else { 0.0 }
        } else {
            if is_night_planet { 60.0 } else { 0.0 }
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
