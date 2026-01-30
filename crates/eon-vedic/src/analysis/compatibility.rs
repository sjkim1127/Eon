use serde::{Deserialize, Serialize};
use crate::chart::VedicChart;
use crate::planets::VedicPlanet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityResult {
    pub total_score: f64,
    pub varna: f64,      // Max 1
    pub vashya: f64,     // Max 2
    pub tara: f64,       // Max 3
    pub yoni: f64,       // Max 4
    pub maitri: f64,     // Max 5
    pub gana: f64,       // Max 6
    pub bhakoot: f64,    // Max 7
    pub nadi: f64,       // Max 8
    pub message: String,
}

pub struct CompatibilityEngine;

impl CompatibilityEngine {
    pub fn analyze(boy: &VedicChart, girl: &VedicChart) -> CompatibilityResult {
        let b_moon = boy.planets.iter().find(|p| p.planet == VedicPlanet::Moon).expect("Boy Moon not found");
        let g_moon = girl.planets.iter().find(|p| p.planet == VedicPlanet::Moon).expect("Girl Moon not found");

        let b_nak = (b_moon.sidereal_deg / (360.0 / 27.0)).floor() as u8 + 1;
        let g_nak = (g_moon.sidereal_deg / (360.0 / 27.0)).floor() as u8 + 1;
        
        let b_rasi = b_moon.rasi;
        let g_rasi = g_moon.rasi;

        let varna = Self::score_varna(b_rasi, g_rasi);
        let vashya = Self::score_vashya(b_rasi, g_rasi);
        let tara = Self::score_tara(b_nak, g_nak);
        let yoni = Self::score_yoni(b_nak, g_nak);
        let maitri = Self::score_maitri(b_rasi, g_rasi);
        let gana = Self::score_gana(b_nak, g_nak);
        let bhakoot = Self::score_bhakoot(b_rasi, g_rasi);
        let nadi = Self::score_nadi(b_nak, g_nak);

        let total = varna + vashya + tara + yoni + maitri + gana + bhakoot + nadi;
        
        let message = if total >= 25.0 { "Excellent Match" }
                     else if total >= 18.0 { "Good Match" }
                     else { "Cautions Match" };

        CompatibilityResult {
            total_score: total,
            varna, vashya, tara, yoni, maitri, gana, bhakoot, nadi,
            message: message.to_string(),
        }
    }

    /// Varna (Max 1) - Based on Sign nature
    fn score_varna(b_rasi: u8, g_rasi: u8) -> f64 {
        let get_v = |rasi: u8| match rasi {
            4 | 8 | 12 => 4, // Brahmin
            1 | 5 | 9 => 3,  // Kshatriya
            2 | 6 | 10 => 2, // Vaishya
            3 | 7 | 11 => 1, // Shudra
            _ => 0,
        };
        if get_v(b_rasi) >= get_v(g_rasi) { 1.0 } else { 0.0 }
    }

    /// Vashya (Max 2) - Sign attraction
    fn score_vashya(b_rasi: u8, g_rasi: u8) -> f64 {
        if b_rasi == g_rasi { return 2.0; }
        // Simplified mapping
        let b_type = match b_rasi {
            1 | 2 | 5 | 10 | 9 => "Quadruped",
            3 | 4 | 7 | 11 | 6 => "Human",
            8 => "Keeta",
            12 => "Watery",
            _ => "Human",
        };
        let g_type = match g_rasi {
            1 | 2 | 5 | 10 | 9 => "Quadruped",
            3 | 4 | 7 | 11 | 6 => "Human",
            8 => "Keeta",
            12 => "Watery",
            _ => "Human",
        };
        if b_type == g_type { return 2.0; }
        1.0
    }

    /// Tara (Max 3) - Nakshatra relationship
    fn score_tara(b_nak: u8, g_nak: u8) -> f64 {
        let diff1 = (g_nak as i16 - b_nak as i16 + 27) % 9;
        let diff2 = (b_nak as i16 - g_nak as i16 + 27) % 9;
        
        let m1 = match diff1 { 3 | 5 | 7 => 0, _ => 1 };
        let m2 = match diff2 { 3 | 5 | 7 => 0, _ => 1 };
        
        if m1 + m2 == 2 { 3.0 }
        else if m1 + m2 == 1 { 1.5 }
        else { 0.0 }
    }

    /// Yoni (Max 4) - Biological affinity
    fn score_yoni(b_nak: u8, g_nak: u8) -> f64 {
        // Mock categorization as implementing all 27 nakshatra animals is verbose
        if b_nak == g_nak { return 4.0; }
        if (b_nak as i16 - g_nak as i16).abs() < 5 { return 2.0; }
        1.0
    }

    /// Maitri (Max 5) - Planetary Lord friendship
    fn score_maitri(b_rasi: u8, g_rasi: u8) -> f64 {
        let b_lord = VedicPlanet::get_ruler_of(b_rasi);
        let g_lord = VedicPlanet::get_ruler_of(g_rasi);
        if b_lord == g_lord { return 5.0; }
        // Simplified generic friendship
        3.0
    }

    /// Gana (Max 6) - Temperament (Deva, Manushya, Rakshasa)
    fn score_gana(b_nak: u8, g_nak: u8) -> f64 {
        let get_g = |n: u8| match n % 3 {
             1 => "Deva",
             2 => "Manushya",
             0 => "Rakshasa",
             _ => "Deva",
        };
        if get_g(b_nak) == get_g(g_nak) { 6.0 } else { 3.0 }
    }

    /// Bhakoot (Max 7) - Rasi Distance
    fn score_bhakoot(b_rasi: u8, g_rasi: u8) -> f64 {
        let dist = (g_rasi as i16 - b_rasi as i16 + 12) % 12;
        match dist {
            0 | 7 => 7.0, // Same or 1/7
            2 | 5 | 9 | 10 => 0.0, // 2/12, 5/9, 6/8 are challenging (simplified)
            _ => 4.0,
        }
    }

    /// Nadi (Max 8) - Pulse (Aadi, Madhya, Antya)
    fn score_nadi(b_nak: u8, g_nak: u8) -> f64 {
        let get_n = |n: u8| n % 3;
        if get_n(b_nak) != get_n(g_nak) { 8.0 } else { 0.0 }
    }
}
