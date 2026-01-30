use serde::{Deserialize, Serialize};
use crate::planets::VedicPlanet;
use crate::chart::{VedicChart, VedicPosition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JaiminiKarakaRole {
    Atmakaraka,      // AK - Soul
    Amatyakaraka,    // AmK - Career/Minister
    Bhratrukaraka,   // BK - Siblings
    Matrukaraka,     // MK - Mother
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
    /// Calculate 7 Chara Karakas (exclude Rahu/Ketu)
    pub fn calculate_karakas(chart: &VedicChart) -> Vec<KarakaAssignment> {
        let mut planets: Vec<&VedicPosition> = chart.planets.iter()
            .filter(|p| matches!(p.planet, 
                VedicPlanet::Sun | 
                VedicPlanet::Moon | 
                VedicPlanet::Mars | 
                VedicPlanet::Mercury | 
                VedicPlanet::Jupiter | 
                VedicPlanet::Venus | 
                VedicPlanet::Saturn
            ))
            .collect();

        // Sort by degree within the Rasi (ascending for easier role assignment)
        // Degree in Rasi = sidereal_deg % 30.0
        planets.sort_by(|a, b| {
            let deg_a = a.sidereal_deg % 30.0;
            let deg_b = b.sidereal_deg % 30.0;
            deg_b.partial_cmp(&deg_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        let roles = [
            JaiminiKarakaRole::Atmakaraka,
            JaiminiKarakaRole::Amatyakaraka,
            JaiminiKarakaRole::Bhratrukaraka,
            JaiminiKarakaRole::Matrukaraka,
            JaiminiKarakaRole::Putrakaraka,
            JaiminiKarakaRole::Gnatikaraka,
            JaiminiKarakaRole::Darakaraka,
        ];

        let mut assignments = Vec::new();
        for (idx, pos) in planets.iter().enumerate() {
            if idx < roles.len() {
                assignments.push(KarakaAssignment {
                    planet: pos.planet,
                    role: roles[idx].clone(),
                    degree_in_rasi: pos.sidereal_deg % 30.0,
                });
            }
        }

        assignments
    }
}
