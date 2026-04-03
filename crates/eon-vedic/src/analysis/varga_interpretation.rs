use crate::chart::VedicChart;
use crate::planets::VedicPlanet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VargaInterpretation {
    pub planet: crate::planets::VedicPlanet,
    pub is_vargottama: bool,
    pub is_pushkar_navamsa: bool,
    pub d9_rasi: u8,
    pub d10_rasi: u8,
    pub d60_rasi: u8,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub reasons: Vec<String>,
}

pub struct VargaInterpretationEngine;

impl VargaInterpretationEngine {
    pub fn interpret_planet(chart: &VedicChart, planet: VedicPlanet) -> VargaInterpretation {
        let p_pos = chart.planets.iter().find(|p| p.planet == planet);
        
        if let Some(p) = p_pos {
            let is_vargottama = p.rasi == p.navamsa_rasi;
            let is_pushkar_navamsa = Self::is_pushkar_navamsa(p.rasi, p.sidereal_deg % 30.0);
            
            VargaInterpretation {
                planet,
                is_vargottama,
                is_pushkar_navamsa,
                d9_rasi: p.navamsa_rasi,
                d10_rasi: p.dasamsa_rasi,
                d60_rasi: p.shashtyamsa_rasi,
                summary: if is_vargottama { "Stable and Strong".to_string() } else { "Standard".to_string() },
                description: format!("{:?} occupies the same sign in D1 and D9 charts.", planet),
                reasons: {
                    let mut r = Vec::new();
                    if is_vargottama { r.push("Vargottama (Sign-identical)".to_string()); }
                    if is_pushkar_navamsa { r.push("Pushkar Navamsa (Spiritual Strength)".to_string()); }
                    r
                },
            }
        } else {
            // Fallback for ASC or others
            VargaInterpretation {
                planet,
                is_vargottama: false,
                is_pushkar_navamsa: false,
                d9_rasi: 0,
                d10_rasi: 0,
                d60_rasi: 0,
                summary: "N/A".to_string(),
                description: "Planet position missing".to_string(),
                reasons: Vec::new(),
            }
        }
    }

    /// Check if a position is in Pushkar Navamsa
    fn is_pushkar_navamsa(d1_rasi: u8, deg_in_sign: f64) -> bool {
        let nav_idx = (deg_in_sign / (30.0 / 9.0)).floor() as u8 + 1; // 1..9
        match d1_rasi {
            1 | 5 | 9 => nav_idx == 7 || nav_idx == 9, // Fire: 7th (Lib), 9th (Sag)
            2 | 6 | 10 => nav_idx == 3 || nav_idx == 5, // Earth: 3rd (Pis), 5th (Tau)
            3 | 7 | 11 => nav_idx == 6 || nav_idx == 8, // Air: 6th (Pis), 8th (Tau)
            4 | 8 | 12 => nav_idx == 1 || nav_idx == 3, // Water: 1st (Can), 3rd (Vir)
            _ => false,
        }
    }

    /// Analyze Marriage (D9)
    pub fn analyze_marriage(chart: &VedicChart) -> String {
        let d9_lagna = chart.ascendant.navamsa_rasi;
        let d9_7th = (d9_lagna + 5) % 12 + 1;
        let d9_7th_lord = VedicPlanet::get_ruler_of(d9_7th);
        
        format!("D9 Lagna: {}, 7th House Lord in D9: {:?}", d9_lagna, d9_7th_lord)
    }

    /// Analyze Career (D10)
    pub fn analyze_career(chart: &VedicChart) -> String {
        let d10_lagna = chart.ascendant.dasamsa_rasi;
        let d10_10th = (d10_lagna + 8) % 12 + 1;
        let d10_10th_lord = VedicPlanet::get_ruler_of(d10_10th);
        
        format!("D10 Lagna: {}, 10th House Lord in D10: {:?}", d10_lagna, d10_10th_lord)
    }
}
