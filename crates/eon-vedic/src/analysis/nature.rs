use serde::{Deserialize, Serialize};
use crate::planets::VedicPlanet;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FunctionalStatus {
    Yogakaraka,       // Best benefic (Kendra + Trikona Lord)
    FunctionalBenefic,// Good (Trikona Lord)
    Neutral,          // Mixed or Neutral
    FunctionalMalefic,// Bad (Dusthana Lord)
    Maraka,           // Killer (2nd/7th Lord)
}

pub struct FunctionalNature;

impl FunctionalNature {
    /// Get Moolatrikona sign for a planet
    pub fn moolatrikona_rasi(planet: VedicPlanet) -> Option<u8> {
        match planet {
            VedicPlanet::Sun => Some(5),      // Leo
            VedicPlanet::Moon => Some(2),     // Taurus
            VedicPlanet::Mars => Some(1),     // Aries
            VedicPlanet::Mercury => Some(6),  // Virgo
            VedicPlanet::Jupiter => Some(9),  // Sagittarius
            VedicPlanet::Venus => Some(7),    // Libra
            VedicPlanet::Saturn => Some(11),  // Aquarius
            _ => None,
        }
    }

    /// Analyze a planet's functional nature for a given Lagna (Ascendant Sign)
    pub fn analyze(lagna_rasi: u8, planet: VedicPlanet) -> FunctionalStatus {
        // Sun/Moon own 1 sign, others own 2.
        // Rahu/Ketu are generally malefic unless in specific houses/conjunct benefics.
        // Identifying houses owned by this planet relative to Lagna.
        
        if planet == VedicPlanet::Rahu || planet == VedicPlanet::Ketu {
             // Simplified: Nodes are generally malefic unless yoga forming.
             return FunctionalStatus::FunctionalMalefic; 
        }

        let mut owned_houses = Vec::new();
        
        // Scan all 12 signs to see which are owned by this planet
        for rasi in 1..=12 {
            if VedicPlanet::get_ruler_of(rasi) == planet {
                // Calculate House Index logic (Same as in chart.rs)
                // House = (Rasi - Lagna + 1 + 12) % 12 ... 
                // Careful with math.
                // My formula: let diff = rasi - lagna; house = if diff>=0 {diff+1} else {diff+13} 
                let diff = rasi as i32 - lagna_rasi as i32;
                let house = if diff >= 0 { diff + 1 } else { diff + 13 };
                owned_houses.push(house);
            }
        }

        if owned_houses.is_empty() {
             return FunctionalStatus::Neutral;
        }

        // Moolatrikona house
        let mt_house = if let Some(mt_rasi) = Self::moolatrikona_rasi(planet) {
             let diff = mt_rasi as i32 - lagna_rasi as i32;
             Some(if diff >= 0 { diff + 1 } else { diff + 13 })
        } else {
             None
        };

        // --- Classification Logic ---
        // 1. Yogakaraka: Owns Kendra (1,4,7,10) AND Trikona (1,5,9)
        // Note: 1st house is both Kendra and Trikona.
        let is_kendra_lord = owned_houses.iter().any(|&h| [1, 4, 7, 10].contains(&h));
        let is_trikona_lord = owned_houses.iter().any(|&h| [1, 5, 9].contains(&h));
        
        // If owns ONLY 1 sign (Sun/Moon), logic is simpler.
        // If owns 2 signs, check combination.
        
        // Special Case: Yogakaraka (e.g. Mars for Cancer/Leo Lagna)
        // Classification Logic refined by Moolatrikona
        // 1. Yogakaraka
        if is_kendra_lord && is_trikona_lord {
             return FunctionalStatus::Yogakaraka;
        }

        // 2. Benefic
        if is_trikona_lord {
             // If Moolatrikona is in a good house (1, 5, 9 or 4, 7, 10)
             if let Some(h) = mt_house {
                 if [1, 4, 5, 7, 9, 10].contains(&h) {
                     return FunctionalStatus::FunctionalBenefic;
                 }
                 // If MT in 3, 6, 8, 11, 12, it might be neutral or slightly malefic
             } else {
                 return FunctionalStatus::FunctionalBenefic;
             }
        }

        // 3. Malefic / Dusthana
        if let Some(h) = mt_house {
             if [3, 6, 8, 11, 12].contains(&h) {
                 return FunctionalStatus::FunctionalMalefic;
             }
        }

        // 4. Maraka
        let is_maraka_house = owned_houses.iter().any(|&h| [2, 7].contains(&h));
        if is_maraka_house && !is_trikona_lord {
             return FunctionalStatus::Maraka;
        }

        FunctionalStatus::Neutral
    }
}
