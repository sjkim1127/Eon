use serde::{Deserialize, Serialize};
use crate::planets::VedicPlanet;
use crate::chart::{VedicChart, VedicPosition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JaiminiKarakaRole {
    Atmakaraka,      // AK - Soul
    Amatyakaraka,    // AmK - Career/Minister
    Bhratrukaraka,   // BK - Siblings
    Matrukaraka,     // MK - Mother
    Pitrikaraka,     // PiK - Father (Used in 8-Karaka)
    Putrakaraka,     // PK - Children
    Gnatikaraka,     // GK - Rivals/Cousins
    Darakaraka,      // DK - Spouse
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KarakaAssignment {
    pub planet: VedicPlanet,
    pub role: JaiminiKarakaRole,
    pub degree_in_rasi: f64,
}

pub struct JaiminiEngine;

impl JaiminiEngine {
    /// Calculate 7 or 8 Chara Karakas
    pub fn calculate_karakas(chart: &VedicChart, use_8_karakas: bool) -> Vec<KarakaAssignment> {
        let mut planets_data: Vec<(VedicPlanet, f64)> = chart.planets.iter()
            .filter(|p| {
                let is_base = matches!(p.planet, 
                    VedicPlanet::Sun | 
                    VedicPlanet::Moon | 
                    VedicPlanet::Mars | 
                    VedicPlanet::Mercury | 
                    VedicPlanet::Jupiter | 
                    VedicPlanet::Venus | 
                    VedicPlanet::Saturn
                );
                if use_8_karakas {
                    is_base || p.planet == VedicPlanet::Rahu
                } else {
                    is_base
                }
            })
            .map(|p| {
                let mut deg = p.sidereal_deg % 30.0;
                // Special Rule for Rahu in Jaimini: Reverse degree because Node is retrograde
                if p.planet == VedicPlanet::Rahu {
                    deg = 30.0 - deg;
                }
                (p.planet, deg)
            })
            .collect();

        // Sort by degree within the Rasi (descending: AK first)
        planets_data.sort_by(|a, b| {
            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut roles = vec![
            JaiminiKarakaRole::Atmakaraka,
            JaiminiKarakaRole::Amatyakaraka,
            JaiminiKarakaRole::Bhratrukaraka,
            JaiminiKarakaRole::Matrukaraka,
        ];

        if use_8_karakas {
            roles.push(JaiminiKarakaRole::Pitrikaraka);
        }

        roles.extend([
            JaiminiKarakaRole::Putrakaraka,
            JaiminiKarakaRole::Gnatikaraka,
            JaiminiKarakaRole::Darakaraka,
        ]);

        let mut assignments = Vec::new();
        for (idx, (planet, deg)) in planets_data.iter().enumerate() {
            if idx < roles.len() {
                assignments.push(KarakaAssignment {
                    planet: *planet,
                    role: roles[idx].clone(),
                    degree_in_rasi: *deg,
                });
            }
        }

        assignments
    }
}
