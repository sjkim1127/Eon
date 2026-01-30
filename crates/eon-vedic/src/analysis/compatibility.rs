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

    /// Yoni (Max 4) - Biological affinity based on Animal archetypes
    pub fn score_yoni(b_nak: u8, g_nak: u8) -> f64 {
        let b_animal = Self::get_yoni_animal(b_nak);
        let g_animal = Self::get_yoni_animal(g_nak);

        if b_animal == g_animal { return 4.0; }

        // Relationship matrix: Swantrata (4), Mitra (3), Sama (2), Shatru (1), Maha-Shatru (0)
        let matrix = [
            // Horse, Elephant, Sheep, Serpent, Dog, Cat, Rat, Cow, Buffalo, Tiger, Deer, Monkey, Lion, Mongoose
            [4, 3, 2, 2, 2, 2, 2, 2, 2, 1, 3, 3, 1, 2], // Horse
            [3, 4, 3, 3, 2, 2, 2, 3, 3, 2, 2, 2, 1, 2], // Elephant
            [2, 3, 4, 2, 1, 2, 1, 3, 3, 1, 2, 0, 1, 2], // Sheep
            [2, 3, 2, 4, 2, 1, 1, 1, 1, 2, 2, 2, 2, 0], // Serpent
            [2, 2, 1, 2, 4, 2, 1, 2, 2, 1, 2, 3, 1, 2], // Dog
            [2, 2, 2, 1, 2, 4, 0, 2, 2, 2, 3, 3, 2, 1], // Cat
            [2, 2, 1, 1, 1, 0, 4, 2, 2, 2, 2, 2, 2, 2], // Rat
            [2, 3, 3, 1, 2, 2, 2, 4, 3, 0, 2, 2, 1, 2], // Cow
            [2, 3, 3, 1, 2, 2, 2, 3, 4, 1, 2, 2, 1, 2], // Buffalo
            [1, 2, 1, 2, 1, 2, 2, 0, 1, 4, 1, 1, 1, 2], // Tiger
            [3, 2, 2, 2, 2, 3, 2, 2, 2, 1, 4, 2, 0, 2], // Deer
            [3, 2, 0, 2, 3, 3, 2, 2, 2, 1, 2, 4, 2, 2], // Monkey
            [1, 1, 1, 2, 1, 2, 2, 1, 1, 1, 0, 2, 4, 2], // Lion
            [2, 2, 2, 0, 2, 1, 2, 2, 2, 2, 2, 2, 2, 4], // Mongoose
        ];

        matrix[b_animal as usize][g_animal as usize] as f64
    }

    fn get_yoni_animal(nak: u8) -> u8 {
        match nak {
            1 | 24 => 0,  // Horse: Ashwini, Shatabhisha
            2 | 27 => 1,  // Elephant: Bharani, Revati
            3 | 10 => 2,  // Sheep: Krittika, Pushya (Wait, Krittika/Pushya=Sheep? BPHS says Kr/Pu=Sheep, others say Goat)
            4 | 5 => 3,   // Serpent: Rohini, Mrigashira
            6 | 19 => 4,  // Dog: Ardra, Mula
            7 | 8 => 5,   // Cat: Punarvasu, Ashlesha
            11 | 12 => 6, // Rat: Magha, Purva Phalguni
            9 | 21 => 7,  // Cow/Ox: Uttara Phalguni, Uttara Ashadha (Actually UtPhal/UtAsh)
            13 | 14 => 8, // Buffalo: Hasta, Swati
            15 | 16 => 9, // Tiger: Chitra, Vishakha
            17 | 18 => 10,// Deer: Anuradha, Jyeshtha
            20 | 25 => 11,// Monkey: Purva Ashadha, Purva Bhadra
            22 | 23 => 12,// Lion: Shravana, Dhanishta
            26 => 13,     // Mongoose: Uttara Bhadrapada
            _ => 0,
        }
    }

    /// Maitri (Max 5) - Planetary Lord friendship (Panchadha Maitri)
    fn score_maitri(b_rasi: u8, g_rasi: u8) -> f64 {
        let b_lord = VedicPlanet::get_ruler_of(b_rasi);
        let g_lord = VedicPlanet::get_ruler_of(g_rasi);
        
        if b_lord == g_lord { return 5.0; }

        // Combined relationship
        // In Ashta Kuta, usually we only look at Naisargika (Natural) or simplified Graha Maitri table.
        // But the user specifically requested Panchadha (Natural + Temporal).
        // For Synastry context, "Temporal" is often calculated from the distance between their Moons.
        let relation = b_lord.panchadha_relation(g_lord, b_rasi, g_rasi);

        match relation {
            2 => 5.0,  // Great Friend
            1 => 4.0,  // Friend
            0 => 3.0,  // Neutral
            -1 => 1.0, // Enemy
            -2 => 0.0, // Great Enemy
            _ => 3.0,
        }
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
        let p_dist = if dist == 0 { 0 } else { dist + 1 }; // 1-indexed distance

        match p_dist {
            1 | 7 => 7.0, // Same or 1/7
            2 | 12 | 5 | 9 | 6 | 8 => {
                // Potential 0 points, but check for exceptions (Bhakoot Dosha Parihara)
                let b_lord = VedicPlanet::get_ruler_of(b_rasi);
                let g_lord = VedicPlanet::get_ruler_of(g_rasi);
                
                // Exception 1: Same Lord
                // Exception 2: Lords are mutual friends
                if b_lord == g_lord || b_lord.naisargika_relation(g_lord) == 1 {
                    7.0 // Cancelled
                } else {
                    0.0
                }
            },
            _ => 4.0,
        }
    }

    /// Nadi (Max 8) - Pulse (Aadi, Madhya, Antya)
    fn score_nadi(b_nak: u8, g_nak: u8) -> f64 {
        let get_n = |n: u8| n % 3;
        if get_n(b_nak) != get_n(g_nak) { 8.0 } else { 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yoni_index_safety() {
        // Test all 27 nakshatras for Yoni animal mapping
        for nak in 1..=27 {
            let score = CompatibilityEngine::score_yoni(nak, nak);
            assert_eq!(score, 4.0, "Same nakshatra {} should have max yoni score", nak);
        }
        
        let ashwini = 1; // Horse
        let bharani = 2; // Elephant
        let score = CompatibilityEngine::score_yoni(ashwini, bharani);
        assert!(score >= 0.0 && score <= 4.0);
    }
}
