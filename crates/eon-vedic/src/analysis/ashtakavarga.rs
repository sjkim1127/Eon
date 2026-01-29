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

    /// Bhinnashtakavarga (BAV) for a specific planet using BPHS tables.
    pub fn calculate_bav(target_planet: VedicPlanet, chart: &VedicChart) -> AshtakavargaPoints {
        let mut points = [0u8; 12];
        
        let sun_pos = chart.planets.iter().find(|p| p.planet == VedicPlanet::Sun).unwrap().rasi;
        let moon_pos = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon).unwrap().rasi;
        let mars_pos = chart.planets.iter().find(|p| p.planet == VedicPlanet::Mars).unwrap().rasi;
        let merc_pos = chart.planets.iter().find(|p| p.planet == VedicPlanet::Mercury).unwrap().rasi;
        let jup_pos = chart.planets.iter().find(|p| p.planet == VedicPlanet::Jupiter).unwrap().rasi;
        let ven_pos = chart.planets.iter().find(|p| p.planet == VedicPlanet::Venus).unwrap().rasi;
        let sat_pos = chart.planets.iter().find(|p| p.planet == VedicPlanet::Saturn).unwrap().rasi;
        let lagna_pos = chart.ascendant.rasi;

        let refs = [
            (VedicPlanet::Sun, sun_pos),
            (VedicPlanet::Moon, moon_pos),
            (VedicPlanet::Mars, mars_pos),
            (VedicPlanet::Mercury, merc_pos),
            (VedicPlanet::Jupiter, jup_pos),
            (VedicPlanet::Venus, ven_pos),
            (VedicPlanet::Saturn, sat_pos),
            (VedicPlanet::Ascendant, lagna_pos),
        ];

        for (ref_planet, ref_rasi) in refs {
            let offsets = Self::get_offsets(target_planet, ref_planet);
            for &offset in offsets {
                // target_rasi = (ref_rasi + offset - 1) % 12 + 1
                let target_rasi = (ref_rasi + offset - 2) % 12 + 1;
                points[target_rasi as usize - 1] += 1;
            }
        }

        AshtakavargaPoints { planet: target_planet, points }
    }

    fn get_offsets(target: VedicPlanet, from: VedicPlanet) -> &'static [u8] {
        match target {
            VedicPlanet::Sun => match from {
                VedicPlanet::Sun => &[1, 2, 4, 7, 8, 9, 10, 11],
                VedicPlanet::Moon => &[3, 6, 10, 11],
                VedicPlanet::Mars => &[1, 2, 4, 7, 8, 9, 10, 11],
                VedicPlanet::Mercury => &[3, 5, 6, 9, 10, 11, 12],
                VedicPlanet::Jupiter => &[5, 6, 9, 11],
                VedicPlanet::Venus => &[6, 7, 12],
                VedicPlanet::Saturn => &[1, 2, 4, 7, 8, 9, 10, 11],
                VedicPlanet::Ascendant => &[3, 4, 6, 10, 11, 12],
                _ => &[],
            },
            VedicPlanet::Moon => match from {
                VedicPlanet::Sun => &[3, 6, 7, 8, 10, 11],
                VedicPlanet::Moon => &[1, 3, 6, 7, 10, 11],
                VedicPlanet::Mars => &[2, 3, 5, 6, 9, 10, 11],
                VedicPlanet::Mercury => &[1, 3, 4, 5, 7, 8, 10, 11],
                VedicPlanet::Jupiter => &[1, 4, 7, 8, 10, 11, 12],
                VedicPlanet::Venus => &[3, 4, 5, 7, 9, 10, 11],
                VedicPlanet::Saturn => &[3, 5, 6, 11],
                VedicPlanet::Ascendant => &[3, 6, 10, 11],
                _ => &[],
            },
            VedicPlanet::Mars => match from {
                VedicPlanet::Sun => &[3, 5, 6, 10, 11],
                VedicPlanet::Moon => &[3, 6, 11],
                VedicPlanet::Mars => &[1, 2, 4, 7, 8, 10, 11],
                VedicPlanet::Mercury => &[3, 5, 6, 11],
                VedicPlanet::Jupiter => &[6, 10, 11, 12],
                VedicPlanet::Venus => &[6, 8, 11, 12],
                VedicPlanet::Saturn => &[1, 4, 7, 8, 9, 10, 11],
                VedicPlanet::Ascendant => &[1, 3, 6, 10, 11],
                _ => &[],
            },
            VedicPlanet::Mercury => match from {
                VedicPlanet::Sun => &[5, 6, 9, 11, 12],
                VedicPlanet::Moon => &[2, 4, 6, 8, 10, 11],
                VedicPlanet::Mars => &[1, 2, 4, 7, 8, 9, 10, 11],
                VedicPlanet::Mercury => &[1, 3, 5, 6, 9, 10, 11, 12],
                VedicPlanet::Jupiter => &[6, 8, 11, 12],
                VedicPlanet::Venus => &[1, 2, 3, 4, 5, 8, 9, 11],
                VedicPlanet::Saturn => &[1, 2, 4, 7, 8, 9, 10, 11],
                VedicPlanet::Ascendant => &[1, 2, 4, 6, 8, 10, 11],
                _ => &[],
            },
            VedicPlanet::Jupiter => match from {
                VedicPlanet::Sun => &[1, 2, 3, 4, 7, 8, 9, 10, 11],
                VedicPlanet::Moon => &[2, 5, 7, 9, 11],
                VedicPlanet::Mars => &[1, 2, 4, 7, 8, 10, 11],
                VedicPlanet::Mercury => &[1, 2, 4, 5, 6, 9, 10, 11],
                VedicPlanet::Jupiter => &[1, 2, 3, 4, 7, 8, 10, 11],
                VedicPlanet::Venus => &[2, 5, 6, 9, 10, 11],
                VedicPlanet::Saturn => &[3, 5, 6, 12],
                VedicPlanet::Ascendant => &[1, 2, 4, 5, 6, 7, 9, 10, 11],
                _ => &[],
            },
            VedicPlanet::Venus => match from {
                VedicPlanet::Sun => &[8, 11, 12],
                VedicPlanet::Moon => &[1, 2, 3, 4, 5, 8, 9, 11, 12],
                VedicPlanet::Mars => &[3, 5, 6, 9, 11, 12],
                VedicPlanet::Mercury => &[3, 5, 6, 9, 11],
                VedicPlanet::Jupiter => &[5, 8, 9, 10, 11],
                VedicPlanet::Venus => &[1, 2, 3, 4, 5, 8, 9, 10, 11],
                VedicPlanet::Saturn => &[3, 4, 5, 8, 9, 10, 11],
                VedicPlanet::Ascendant => &[1, 2, 3, 4, 5, 8, 9, 11],
                _ => &[],
            },
            VedicPlanet::Saturn => match from {
                VedicPlanet::Sun => &[1, 2, 4, 7, 8, 10, 11],
                VedicPlanet::Moon => &[3, 6, 11],
                VedicPlanet::Mars => &[3, 5, 6, 10, 11, 12],
                VedicPlanet::Mercury => &[6, 8, 9, 10, 11, 12],
                VedicPlanet::Jupiter => &[5, 6, 11, 12],
                VedicPlanet::Venus => &[6, 11, 12],
                VedicPlanet::Saturn => &[3, 5, 6, 11],
                VedicPlanet::Ascendant => &[1, 3, 4, 6, 10, 11],
                _ => &[],
            },
            _ => &[],
        }
    }
}
