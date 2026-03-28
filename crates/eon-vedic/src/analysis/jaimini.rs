use crate::chart::VedicChart;
use crate::planets::VedicPlanet;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JaiminiKarakaRole {
    Atmakaraka,    // AK - Soul
    Amatyakaraka,  // AmK - Career/Minister
    Bhratrukaraka, // BK - Siblings
    Matrukaraka,   // MK - Mother
    Pitrikaraka,   // PiK - Father (Used in 8-Karaka)
    Putrakaraka,   // PK - Children
    Gnatikaraka,   // GK - Rivals/Cousins
    Darakaraka,    // DK - Spouse
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KarakaAssignment {
    pub planet: VedicPlanet,
    pub role: JaiminiKarakaRole,
    pub degree_in_rasi: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArudhaPada {
    pub house: u8, // 1~12
    pub rasi: u8,  // 1~12
    pub name: String, // e.g., "Arudha Lagna (AL)", "Dhanapada (A2)"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialLagna {
    pub name: String,
    pub longitude: f64,
    pub rasi: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignDashaPeriod {
    pub rasi: u8,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

pub struct JaiminiEngine;

impl JaiminiEngine {
    /// Calculate 7 or 8 Chara Karakas
    pub fn calculate_karakas(chart: &VedicChart, use_8_karakas: bool) -> Vec<KarakaAssignment> {
        let mut planets_data: Vec<(VedicPlanet, f64)> = chart
            .planets
            .iter()
            .filter(|p| {
                let is_base = matches!(
                    p.planet,
                    VedicPlanet::Sun
                        | VedicPlanet::Moon
                        | VedicPlanet::Mars
                        | VedicPlanet::Mercury
                        | VedicPlanet::Jupiter
                        | VedicPlanet::Venus
                        | VedicPlanet::Saturn
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
        planets_data.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

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

    /// Calculate Arudha Padas for all 12 houses
    pub fn calculate_arudha_padas(chart: &VedicChart) -> Vec<ArudhaPada> {
        let mut results = Vec::new();
        let lagna_rasi = chart.ascendant.rasi;

        let names = [
            "Arudha Lagna (AL)", "Dhanapada (A2)", "Vikramapada (A3)", "Matrupada (A4)",
            "Putrapada (A5)", "Shatrupada (A6)", "Darapada (A7)", "Mrityupada (A8)",
            "Bhagyapada (A9)", "Rajyapada (A10)", "Labhapada (A11)", "Upapada Lagna (UL/A12)"
        ];

        for house in 1..=12 {
            let house_rasi = ((lagna_rasi as i16 + house as i16 - 2) % 12 + 1) as u8;
            let lord = VedicPlanet::get_ruler_of(house_rasi);
            
            // Find lord's position in D1
            if let Some(lord_pos) = chart.planets.iter().find(|p| p.planet == lord) {
                let lord_rasi = lord_pos.rasi;
                
                // Distance from house to lord
                let dist = (lord_rasi as i16 - house_rasi as i16 + 12) % 12;
                
                // Arudha = Lord + Distance
                let mut arudha_rasi = ((lord_rasi as i16 + dist - 1) % 12 + 1) as u8;
                
                if arudha_rasi == house_rasi {
                    // If Arudha is in the house itself, move 10 houses (Parashara)
                    arudha_rasi = ((arudha_rasi as i16 + 9 - 1) % 12 + 1) as u8;
                } else if arudha_rasi == ((house_rasi as i16 + 6 - 1) % 12 + 1) as u8 {
                    // If Arudha is in the 7th from house, move 4 houses from Arudha (which is 10th from house)
                    arudha_rasi = ((arudha_rasi as i16 + 4 - 1) % 12 + 1) as u8;
                }

                results.push(ArudhaPada {
                    house,
                    rasi: arudha_rasi,
                    name: names[house as usize - 1].to_string(),
                });
            }
        }

        results
    }

    /// Calculate Special Lagnas from BPHS
    pub fn calculate_special_lagnas(chart: &VedicChart) -> Vec<SpecialLagna> {
        let mut results = Vec::new();
        
        let sun_pos = chart.planets.iter().find(|p| p.planet == VedicPlanet::Sun);
        if let (Some(sun), sunrise) = (sun_pos, chart.panchanga.sunrise) {
            let birth_time = chart.panchanga.current_time;
            let diff_mins = birth_time.signed_duration_since(sunrise).num_minutes() as f64;
            
            // Bhava Lagna (BL): 1 Rasi (30 deg) per 24 mins (1 Ghati)
            let bl_long = (sun.sidereal_deg + diff_mins * (30.0 / 24.0)) % 360.0;
            results.push(SpecialLagna {
                name: "Bhava Lagna (BL)".to_string(),
                longitude: bl_long,
                rasi: (bl_long / 30.0).floor() as u8 + 1,
            });

            // Hora Lagna (HL): 1 Rasi (30 deg) per 60 mins (2.5 Ghati)
            let hl_long = (sun.sidereal_deg + diff_mins * (30.0 / 60.0)) % 360.0;
            results.push(SpecialLagna {
                name: "Hora Lagna (HL)".to_string(),
                longitude: hl_long,
                rasi: (hl_long / 30.0).floor() as u8 + 1,
            });

            // Ghati Lagna (GL): Rate of 1.25 signs per Ghati (24 mins)
            // This is equivalent to 1 sign per 19.2 mins.
            // BPHS standard: 1.25 * (30 deg) / 24 mins = 1.5625 deg / min.
            let gl_long = (sun.sidereal_deg + diff_mins * 1.5625) % 360.0;
            results.push(SpecialLagna {
                name: "Ghati Lagna (GL)".to_string(),
                longitude: gl_long,
                rasi: (gl_long / 30.0).floor() as u8 + 1,
            });
        }
        
        results
    }

    /// Calculate Chara Dasha (Simplified KN Rao method version 1)
    /// NOTE: This is an foundational implementation. 
    /// Traditional systems use special cases for Scorpio (Ketu/Mars) and Aquarius (Saturn/Rahu),
    /// which are not yet fully implemented here.
    pub fn calculate_chara_dasha(chart: &VedicChart) -> Vec<SignDashaPeriod> {
        let lagna_rasi = chart.ascendant.rasi;
        let birth_time = chart.panchanga.current_time;
        
        // Sequence logic (KN Rao):
        // Forward: 1, 5, 6, 7, 11, 12
        // Backward: 2, 3, 4, 8, 9, 10
        let forward_signs = [1, 5, 6, 7, 11, 12];
        let is_forward_seq = forward_signs.contains(&lagna_rasi);
        
        let mut sequence = Vec::new();
        for i in 0..12 {
            let rasi = if is_forward_seq {
                ((lagna_rasi as i16 + i - 1) % 12 + 1) as u8
            } else {
                ((lagna_rasi as i16 - i + 11) % 12 + 1) as u8
            };
            sequence.push(rasi);
        }

        let mut timeline = Vec::new();
        let mut current_start = birth_time;

        for rasi in sequence {
            let lord = VedicPlanet::get_ruler_of(rasi);
            let lord_pos = chart.planets.iter().find(|p| p.planet == lord);
            
            let years = if let Some(lp) = lord_pos {
                let lord_rasi = lp.rasi;
                let is_forward_count = forward_signs.contains(&rasi);
                
                let dist = if is_forward_count {
                    (lord_rasi as i16 - rasi as i16 + 12) % 12
                } else {
                    (rasi as i16 - lord_rasi as i16 + 12) % 12
                };
                
                if dist == 0 { 12 } else { dist as u32 }
            } else {
                7 // Fallback
            };

            let end_time = current_start + Duration::seconds((years as f64 * 365.2425 * 24.0 * 60.0 * 60.0) as i64);
            
            timeline.push(SignDashaPeriod {
                rasi,
                start_time: current_start,
                end_time,
            });
            
            current_start = end_time;
        }

        timeline
    }

    /// Jaimini Rashi Drishti (Sign Aspects)
    /// Rule: 
    /// - Movable (Ar, Cn, Li, Cp) aspects all Fixed (Ta, Le, Sc, Aq) except the adjacent one.
    /// - Fixed aspects all Movable except the adjacent one.
    /// - Dual (Ge, Vi, Sg, Pi) aspects all other Dual signs.
    pub fn get_rashi_drishti(rasi: u8) -> Vec<u8> {
        match rasi {
            1 => vec![5, 8, 11], // Ar (M) -> Le, Sc, Aq (not Ta)
            2 => vec![4, 7, 10], // Ta (F) -> Cn, Li, Cp (not Ar)
            3 => vec![6, 9, 12], // Ge (D) -> Vi, Sg, Pi (not self)
            4 => vec![2, 8, 11], // Cn (M) -> Ta, Sc, Aq (not Le)
            5 => vec![1, 7, 10], // Le (F) -> Ar, Li, Cp (not Cn)
            6 => vec![3, 9, 12], // Vi (D) -> Ge, Sg, Pi
            7 => vec![2, 5, 11], // Li (M) -> Ta, Le, Aq (not Sc)
            8 => vec![1, 4, 10], // Sc (F) -> Ar, Cn, Cp (not Li)
            9 => vec![3, 6, 12], // Sg (D) -> Ge, Vi, Pi
            10 => vec![2, 5, 8], // Cp (M) -> Ta, Le, Sc (not Aq)
            11 => vec![1, 4, 7], // Aq (F) -> Ar, Cn, Li (not Cp)
            12 => vec![3, 6, 9], // Pi (D) -> Ge, Vi, Sg
            _ => vec![],
        }
    }

    /// Calculate Argala (Intervention) for a sign/planet position
    pub fn get_argala(rasi: u8) -> Vec<(u8, String)> {
        let mut results = Vec::new();
        
        // Primary Argala: 2, 4, 11
        // Obstruction (Virodhargala): 12, 10, 3
        let primary = [(2, "Wealth/Speech"), (4, "Happiness/Home"), (11, "Gains")];
        for (dist, desc) in primary {
            let target = ((rasi as i16 + dist - 2) % 12 + 1) as u8;
            results.push((target, format!("Primary Argala ({})", desc)));
        }

        // Secondary Argala: 5 (Obstruction: 9)
        let secondary = (5, "Knowledge/Children");
        let target = ((rasi as i16 + secondary.0 - 2) % 12 + 1) as u8;
        results.push((target, format!("Secondary Argala ({})", secondary.1)));

        results
    }
}
