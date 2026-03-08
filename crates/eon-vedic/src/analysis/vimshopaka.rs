use crate::analysis::relationships::{RelationshipEngine, RelationshipType};
use crate::calc::varga::VargaType;
use crate::chart::{VedicChart, VedicPosition};
use crate::planets::VedicPlanet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VimshopakaScore {
    pub shadvarga_score: f64,           // Out of 20
    pub shodashavarga_score: f64,       // Out of 20 (Simple average for now)
    pub details: Vec<(VargaType, f64)>, // Score per Varga
}

pub struct VimshopakaEngine;

impl VimshopakaEngine {
    /// Calculate Vimshopaka Bala (Strength in Vargas)
    /// Focus on Shadvarga (6-fold) which is most commonly used.
    /// Weights: Rasi(6), Hora(2), Drekkana(4), Navamsa(5), Dwadasamsa(2), Trimsamsa(1). Total 20.
    pub fn calculate(pos: &VedicPosition, chart: &VedicChart) -> VimshopakaScore {
        let _total_weighted_score = 0.0;
        let mut details = Vec::new();

        // Shadvarga Weights
        let shadvarga_weights = [
            (VargaType::D1, 6.0),
            (VargaType::D2, 2.0),
            (VargaType::D3, 4.0),
            (VargaType::D9, 5.0),
            (VargaType::D12, 2.0),
            (VargaType::D30, 1.0),
        ];

        let mut shadvarga_sum = 0.0;

        // 1. Calculate Shadvarga
        for (varga, weight) in &shadvarga_weights {
            let rasi = Self::get_varga_rasi(pos, *varga);
            let point = Self::get_dignity_point(pos.planet, rasi, chart);
            shadvarga_sum += point * *weight;
            details.push((*varga, point));
        }

        // Normalize: The sum of weights is 20. The max point is 20.
        // So max weighted sum is 400. We want final score out of 20.
        // Formula: Sum(Point * Weight) / Sum(Weights)?? -> No, that gives average point (0-20).
        // BPHS Vimshopaka: The sum IS the score?? No.
        // Actually, if max point is 20, and weight is 6, max contribution is 120.
        // Total max contribution = 20*6 + 20*2 + ... = 20 * 20 = 400.
        // We usually scale it back to 20.
        let shadvarga_final = shadvarga_sum / 20.0;

        // 2. Shodashavarga (All 16) - Simplified Average for now
        // Or we can just use the Shadvarga score as the primary metric.
        // Let's calculate a simple average of dignity points across all implemented vargas for "Shodashavarga".
        let all_vargas = [
            VargaType::D1,
            VargaType::D2,
            VargaType::D3,
            VargaType::D4,
            VargaType::D7,
            VargaType::D9,
            VargaType::D10,
            VargaType::D12,
            VargaType::D16,
            VargaType::D20,
            VargaType::D24,
            VargaType::D27,
            VargaType::D30,
            VargaType::D40,
            VargaType::D45,
            VargaType::D60,
        ];

        let mut total_points = 0.0;
        for v in &all_vargas {
            let rasi = Self::get_varga_rasi(pos, *v);
            total_points += Self::get_dignity_point(pos.planet, rasi, chart);
        }
        let shodashavarga_final = total_points / all_vargas.len() as f64;

        VimshopakaScore {
            shadvarga_score: shadvarga_final,
            shodashavarga_score: shodashavarga_final,
            details,
        }
    }

    fn get_varga_rasi(pos: &VedicPosition, varga: VargaType) -> u8 {
        match varga {
            VargaType::D1 => pos.rasi,
            VargaType::D2 => pos.hora_rasi,
            VargaType::D3 => pos.drekkana_rasi,
            VargaType::D4 => pos.chaturthamsha_rasi,
            VargaType::D7 => pos.saptamsa_rasi,
            VargaType::D9 => pos.navamsa_rasi,
            VargaType::D10 => pos.dasamsa_rasi,
            VargaType::D12 => pos.dwadasamsa_rasi,
            VargaType::D16 => pos.shodashamsa_rasi,
            VargaType::D20 => pos.vimsamsa_rasi,
            VargaType::D24 => pos.chaturvimshamsa_rasi,
            VargaType::D27 => pos.saptavimsamsa_rasi,
            VargaType::D30 => pos.trimsamsa_rasi,
            VargaType::D40 => pos.khavedamsa_rasi,
            VargaType::D45 => pos.akshavedamsa_rasi,
            VargaType::D60 => pos.shashtyamsa_rasi,
            _ => pos.rasi, // Fallback
        }
    }

    /// Get Dignity Point (0-20)
    /// Exalted/Own: 20
    /// Great Friend: 18
    /// Friend: 15
    /// Neutral: 10
    /// Enemy: 7
    /// Great Enemy: 5
    /// Debilitated: 0
    fn get_dignity_point(planet: VedicPlanet, rasi: u8, chart: &VedicChart) -> f64 {
        // 1. Check Exaltation/Debilitation
        if planet.exaltation_rasi() == rasi {
            return 20.0;
        }
        if planet.debilitation_rasi() == rasi {
            return 0.0;
        } // Or 5? BPHS says Debilitated is lowest.

        // 2. Check Own Sign
        let lord = VedicPlanet::get_ruler_of(rasi);
        if lord == planet {
            return 20.0;
        }

        // 3. Check Relationship with Lord
        let rel = RelationshipEngine::get_relationship(planet, lord, chart);
        match rel {
            RelationshipType::GreatFriend => 18.0,
            RelationshipType::Friend => 15.0,
            RelationshipType::Neutral => 10.0,
            RelationshipType::Enemy => 7.0,
            RelationshipType::GreatEnemy => 5.0,
        }
    }
}
