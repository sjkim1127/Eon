use crate::planets::VedicPlanet;
use crate::chart::VedicChart;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AshtakavargaPoints {
    pub planet: VedicPlanet,
    pub points: [u8; 12],            // Raw points
    pub trikona_points: [u8; 12],    // After Trikona Shodhana
    pub shodhana_points: [u8; 12],   // After Ekadhipatya Shodhana
    pub sodya_pinda: u32,            // Final Pinda score
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

        let trikona_points = Self::apply_triangular_reduction(points);
        let shodhana_points = Self::apply_ekadhipatya_reduction(trikona_points, chart);
        let sodya_pinda = Self::calculate_pinda(target_planet, shodhana_points, chart);

        AshtakavargaPoints { 
            planet: target_planet, 
            points, 
            trikona_points,
            shodhana_points,
            sodya_pinda
        }
    }

    /// Trikona Shodhana (Triangular Reduction)
    /// Reduces points based on triplicities (Elements)
    pub fn apply_triangular_reduction(points: [u8; 12]) -> [u8; 12] {
        let mut reduced = points;
        let triplicities = [
            [0, 4, 8],  // Fire (1, 5, 9)
            [1, 5, 9],  // Earth (2, 6, 10)
            [2, 6, 10], // Air (3, 7, 11)
            [3, 7, 11], // Water (4, 8, 12)
        ];

        for trip in &triplicities {
            let p1 = reduced[trip[0]];
            let p2 = reduced[trip[1]];
            let p3 = reduced[trip[2]];

            let zeros = (if p1 == 0 { 1 } else { 0 }) + 
                        (if p2 == 0 { 1 } else { 0 }) + 
                        (if p3 == 0 { 1 } else { 0 });

            if zeros == 0 {
                // All have points: subtract minimum
                let min_val = p1.min(p2).min(p3);
                reduced[trip[0]] -= min_val;
                reduced[trip[1]] -= min_val;
                reduced[trip[2]] -= min_val;
            } else if zeros == 1 {
                // One is zero: no reduction
            } else if zeros == 2 {
                // Two are zero: third becomes zero too
                reduced[trip[0]] = 0;
                reduced[trip[1]] = 0;
                reduced[trip[2]] = 0;
            } else {
                // All are zero: already 0
            }
        }
        reduced
    }

    /// Ekadhipatya Shodhana (Reduction for planets owning two signs)
    pub fn apply_ekadhipatya_reduction(points: [u8; 12], chart: &VedicChart) -> [u8; 12] {
        let mut reduced = points;
        let pairs = [
            (0, 7),  // Mars: Aries (1) & Scorpio (8)
            (1, 6),  // Venus: Taurus (2) & Libra (7)
            (2, 5),  // Mercury: Gemini (3) & Virgo (6)
            (8, 11), // Jupiter: Sag (9) & Pisces (12)
            (9, 10), // Saturn: Cap (10) & Aqua (11)
        ];

        let is_occupied = |rasi_idx: usize| -> bool {
            chart.planets.iter().any(|p| p.rasi == (rasi_idx as u8 + 1))
        };

        for (r1, r2) in pairs {
            let p1 = reduced[r1];
            let p2 = reduced[r2];
            let occ1 = is_occupied(r1);
            let occ2 = is_occupied(r2);

            if p1 == 0 || p2 == 0 {
                if p1 == 0 && p2 == 0 { continue; }
                let (has_pts_idx, other_idx, other_occ) = if p1 == 0 { (r2, r1, occ1) } else { (r1, r2, occ2) };
                if !is_occupied(has_pts_idx) {
                    reduced[has_pts_idx] = 0;
                }
            } else {
                // Both have points
                if occ1 && occ2 {
                    // Both occupied, no reduction
                } else if !occ1 && !occ2 {
                    // Neither occupied, both to lower value
                    let min_val = p1.min(p2);
                    reduced[r1] = min_val;
                    reduced[r2] = min_val;
                } else {
                    // One occupied
                    let (occ_idx, unocc_idx) = if occ1 { (r1, r2) } else { (r2, r1) };
                    if reduced[unocc_idx] > reduced[occ_idx] {
                        reduced[unocc_idx] = reduced[occ_idx];
                    }
                }
            }
        }
        reduced
    }

    /// Calculate Sodya Pinda (Final sum of reduced points multiplied by rasi/planet factors)
    pub fn calculate_pinda(_target: VedicPlanet, points: [u8; 12], chart: &VedicChart) -> u32 {
        let rasi_multipliers = [7, 10, 8, 4, 10, 5, 7, 8, 9, 5, 11, 12];
        let graha_multipliers = [
            (VedicPlanet::Sun, 5),
            (VedicPlanet::Moon, 5),
            (VedicPlanet::Mars, 8),
            (VedicPlanet::Mercury, 5),
            (VedicPlanet::Jupiter, 10),
            (VedicPlanet::Venus, 7),
            (VedicPlanet::Saturn, 5),
        ];

        // 1. Rasi Pinda: Points in each rasi * Rasi multiplier
        let mut rasi_pinda = 0u32;
        for i in 0..12 {
            rasi_pinda += (points[i] as u32) * rasi_multipliers[i];
        }

        // 2. Graha Pinda: Reduced points in rasis where planets are * Graha multiplier
        let mut graha_pinda = 0u32;
        for (p, mult) in graha_multipliers {
            if let Some(pos) = chart.planets.iter().find(|pos| pos.planet == p) {
                let rasi_idx = (pos.rasi as usize).wrapping_sub(1);
                if rasi_idx < 12 {
                    graha_pinda += (points[rasi_idx] as u32) * mult;
                }
            }
        }

        rasi_pinda + graha_pinda
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
