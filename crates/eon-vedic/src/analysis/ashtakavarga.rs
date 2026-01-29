use crate::planets::VedicPlanet;
use crate::chart::VedicChart;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AshtakavargaPoints {
    pub planet: VedicPlanet,
    pub points: [u8; 12], // Points in each house 1-12
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sarvashtakavarga {
    pub points: [u8; 12], // Total points in each house 1-12
}

pub struct AshtakavargaEngine;

impl AshtakavargaEngine {
    /// Calculate Sarvashtakavarga (SAV) points for the chart
    pub fn calculate_sav(chart: &VedicChart) -> Sarvashtakavarga {
        let planets = [
            VedicPlanet::Sun, VedicPlanet::Moon, VedicPlanet::Mars,
            VedicPlanet::Mercury, VedicPlanet::Jupiter, VedicPlanet::Venus,
            VedicPlanet::Saturn
        ];

        let mut total_points = [0u8; 12];

        for &p in &planets {
            let bav = Self::calculate_bav(p, chart);
            for i in 0..12 {
                total_points[i] += bav.points[i];
            }
        }

        Sarvashtakavarga { points: total_points }
    }

    /// Bhinnashtakavarga (BAV) for a specific planet
    /// Note: This is a simplified MVP version. Full BAV rules are extensive.
    pub fn calculate_bav(planet: VedicPlanet, chart: &VedicChart) -> AshtakavargaPoints {
        // For MVP, we use a placeholder logic that distributes points based on dignity.
        // Full BPHS rules for all 7 planets + Lagna require a large matrix.
        // We will implement full rules in a subsequent update.
        
        let mut points = [0u8; 12];
        
        // Find planet position
        let planet_pos = chart.planets.iter().find(|p| p.planet == planet);
        
        if let Some(pos) = planet_pos {
            let own_house = pos.house_index;
            // Simplified logic: distribute 28 points around the chart based on distance
            // Real SAV sum is 337, avg per house is 28.
            for i in 0..12 {
                let dist = (i as i32 + 1 - own_house as i32).abs();
                let p = match dist {
                    0 => 5,
                    1 | 11 => 4,
                    2 | 10 => 3,
                    3 | 9 => 2,
                    _ => 4,
                };
                points[i] = p;
            }
        }

        AshtakavargaPoints { planet, points }
    }
}
