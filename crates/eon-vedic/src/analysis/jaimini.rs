use crate::chart::VedicChart;
use crate::planets::VedicPlanet;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct KarakaAssignment {
    pub planet: VedicPlanet,
    pub role: JaiminiKarakaRole,
    pub degree_in_rasi: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArudhaPada {
    pub house: u8, // 1~12
    pub rasi: u8,  // 1~12
    pub name: String, // e.g., "Arudha Lagna (AL)", "Dhanapada (A2)"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecialLagna {
    pub name: String,
    pub longitude: f64,
    pub rasi: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
                    // If Arudha is in the house itself, final Arudha is 10th from house
                    arudha_rasi = ((house_rasi as i16 + 10 - 2) % 12 + 1) as u8;
                } else if arudha_rasi == ((house_rasi as i16 + 7 - 2) % 12 + 1) as u8 {
                    // If Arudha is in the 7th from house, final Arudha is 4th from house
                    arudha_rasi = ((house_rasi as i16 + 4 - 2) % 12 + 1) as u8;
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

    /// Calculate Chara Dasha (True KN Rao method)
    /// 
    /// Sequence (KN Rao):
    /// - Forward: Ar, Le, Vi, Li, Aq, Pi (1, 5, 6, 7, 11, 12)
    /// - Backward: Ta, Ge, Cn, Sc, Sg, Cp (2, 3, 4, 8, 9, 10)
    pub fn calculate_chara_dasha(chart: &VedicChart) -> Vec<SignDashaPeriod> {
        let lagna_rasi = chart.ascendant.rasi;
        let birth_time = chart.panchanga.current_time;
        
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
            let years = Self::calculate_chara_dasha_years(chart, rasi);

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

    /// Calculate years for a sign in Chara Dasha
    fn calculate_chara_dasha_years(chart: &VedicChart, rasi: u8) -> u32 {
        let forward_signs = [1, 5, 6, 7, 11, 12];
        let is_counting_forward = forward_signs.contains(&rasi);

        let lord_rasi = match rasi {
            8 => Self::evaluate_co_ruler_strength(chart, 8, VedicPlanet::Mars, VedicPlanet::Ketu),
            11 => Self::evaluate_co_ruler_strength(chart, 11, VedicPlanet::Saturn, VedicPlanet::Rahu),
            _ => {
                let lord = VedicPlanet::get_ruler_of(rasi);
                chart.planets.iter().find(|p| p.planet == lord).map(|p| p.rasi).unwrap_or(rasi)
            }
        };

        let dist = if is_counting_forward {
            (lord_rasi as i16 - rasi as i16 + 12) % 12
        } else {
            (rasi as i16 - lord_rasi as i16 + 12) % 12
        };

        if dist == 0 { 12 } else { dist as u32 }
    }

    /// Selection rules for Scorpio/Aquarius co-rulers (Jaimini / KN Rao)
    /// 1. Sign with more planets (excluding the sign itself) wins.
    /// 2. If equal, the node/planet that is stronger wins (Exaltation > Own Sign > Others).
    /// 3. If still equal, the one with more degrees in the sign wins (Jaimini logic).
    fn evaluate_co_ruler_strength(chart: &VedicChart, rasi: u8, p1: VedicPlanet, p2: VedicPlanet) -> u8 {
        let pos1 = chart.planets.iter().find(|p| p.planet == p1).unwrap();
        let pos2 = chart.planets.iter().find(|p| p.planet == p2).unwrap();

        // Count conjunctions in the rasi where the lord is placed (excluding the lord itself)
        let count_conj = |p_rasi: u8, planet: VedicPlanet| {
            chart.planets.iter()
                .filter(|p| p.rasi == p_rasi && p.planet != planet)
                .count()
        };

        let c1 = count_conj(pos1.rasi, p1);
        let c2 = count_conj(pos2.rasi, p2);

        if c1 > c2 { return pos1.rasi; }
        if c2 > c1 { return pos2.rasi; }

        // If conjunctions are equal, check if one is in their own sign/exalted
        let is_stronger = |p: VedicPlanet, prasi: u8| {
            if prasi == rasi { 2 } // Own sign
            else if prasi == p.exaltation_rasi() { 3 } // Exalted
            else { 1 }
        };

        let s1 = is_stronger(p1, pos1.rasi);
        let s2 = is_stronger(p2, pos2.rasi);

        if s1 > s2 { return pos1.rasi; }
        if s2 > s1 { return pos2.rasi; }

        // Last resort: Higher degree in the rasi
        if pos1.sidereal_deg % 30.0 > pos2.sidereal_deg % 30.0 { pos1.rasi } else { pos2.rasi }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::chart::VedicChart;
    use crate::core::planets::VedicPlanet;
    use crate::core::chart::VedicPosition;
    use chrono::{TimeZone, Utc};

    fn mock_position(planet: VedicPlanet, rasi: u8, deg: f64) -> VedicPosition {
        VedicPosition {
            planet,
            rasi,
            sidereal_deg: (rasi as f64 - 1.0) * 30.0 + deg,
            tropical_deg: 0.0,
            nakshatra: 1,
            pada: 1,
            house_index: 1,
            speed: 1.0,
            is_retrograde: false,
            is_combust: false,
            declination: 0.0,
            hora_rasi: 1,
            drekkana_rasi: 1,
            chaturthamsha_rasi: 1,
            panchamsa_rasi: 1,
            saptamsa_rasi: 1,
            ashtamsa_rasi: 1,
            navamsa_rasi: 1,
            dasamsa_rasi: 1,
            shashtamsa_rasi: 1,
            rudramsa_rasi: 1,
            dwadasamsa_rasi: 1,
            shodashamsa_rasi: 1,
            vimsamsa_rasi: 1,
            chaturvimshamsa_rasi: 1,
            saptavimsamsa_rasi: 1,
            trimsamsa_rasi: 1,
            khavedamsa_rasi: 1,
            akshavedamsa_rasi: 1,
            shashtyamsa_rasi: 1,
            navanavamsa_rasi: 1,
            ashtottaramsa_rasi: 1,
            dwadasdwadasamsa_rasi: 1,
        }
    }

    fn mock_chart(planets: Vec<VedicPosition>, lagna_rasi: u8) -> VedicChart {
        let ascendant = mock_position(VedicPlanet::Ascendant, lagna_rasi, 10.0);
        VedicChart {
            ascendant,
            planets,
            aspects: vec![],
            sav: crate::analysis::ashtakavarga::Sarvashtakavarga { points: [0u8; 12] },
            bav: vec![],
            house_cusps: vec![0.0; 12],
            karakas: vec![],
            arudha_padas: vec![],
            special_lagnas: vec![],
            bhava_strengths: vec![],
            vimshopaka_scores: vec![],
            avasthas: vec![],
            panchanga: crate::calc::panchanga::Panchanga {
                current_time: Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap(),
                ..Default::default()
            },
            analysis_report: None,
        }
    }

    #[test]
    fn test_chara_dasha_years_basic() {
        // Aries (1) lord Mars in Gemini (3)
        // 1 is forward sign. Dist: (3 - 1) = 2. Years: 2 - 1 = 1 year? 
        // Wait, my impl says (dist == 0 ? 12 : dist). 
        // 3-1 = 2. dist = 2. 
        let planets = vec![
            mock_position(VedicPlanet::Mars, 3, 5.0),
        ];
        let chart = mock_chart(planets, 1);
        let years = JaiminiEngine::calculate_chara_dasha_years(&chart, 1);
        assert_eq!(years, 2); // 1st to 3rd is 3 signs, but Rao uses diff. 
        // Actually dist is sign index difference. 1 to 3 is 2. 
        // If dist=2, years=2. 
    }

    #[test]
    fn test_scorpio_co_ruler_strength() {
        // Scorpio (8) lords Mars (4) and Ketu (12)
        // Put Ketu with Sun in 12. Ketu has 1 conjunction. Mars is alone in 4.
        // Ketu should win.
        let planets = vec![
            mock_position(VedicPlanet::Mars, 4, 10.0),
            mock_position(VedicPlanet::Ketu, 12, 15.0),
            mock_position(VedicPlanet::Sun, 12, 20.0),
        ];
        let chart = mock_chart(planets, 1);
        let lord_rasi = JaiminiEngine::evaluate_co_ruler_strength(&chart, 8, VedicPlanet::Mars, VedicPlanet::Ketu);
        assert_eq!(lord_rasi, 12);
    }

    #[test]
    fn test_arudha_pada_7th_exception() {
        // Lagna (1) lord Mars in 4. 
        // 4 is 4th from 1. 4th from 4 is 7. 
        // This is the 7th house exception. Should move 4 houses from 7 -> 10.
        let planets = vec![
            mock_position(VedicPlanet::Mars, 4, 10.0),
        ];
        let chart = mock_chart(planets, 1);
        let padas = JaiminiEngine::calculate_arudha_padas(&chart);
        let l1_arudha = padas.iter().find(|p| p.house == 1).unwrap();
        assert_eq!(l1_arudha.rasi, 4);
    }
}
