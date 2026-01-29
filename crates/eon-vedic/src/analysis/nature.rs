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
             return FunctionalStatus::Neutral; // Should not happen for main planets
        }

        // --- Classification Logic ---
        // 1. Yogakaraka: Owns Kendra (1,4,7,10) AND Trikona (1,5,9)
        // Note: 1st house is both Kendra and Trikona.
        let is_kendra_lord = owned_houses.iter().any(|&h| [1, 4, 7, 10].contains(&h));
        let is_trikona_lord = owned_houses.iter().any(|&h| [1, 5, 9].contains(&h));
        
        // If owns ONLY 1 sign (Sun/Moon), logic is simpler.
        // If owns 2 signs, check combination.
        
        // Special Case: Yogakaraka (e.g. Mars for Cancer/Leo Lagna)
        if is_kendra_lord && is_trikona_lord {
             // Exception: If also owns bad house? 
             // Ideally Yogakaraka overrides.
             return FunctionalStatus::Yogakaraka;
        }

        // 2. Benefic: Lord of Trikona (5, 9) is always auspicious.
        // (Lord of 1 is Neutral/Benefic, usually Benefic)
        if owned_houses.iter().any(|&h| [1, 5, 9].contains(&h)) {
             // Provide benefit. 
             // But if also owns 6, 8, 12? Moolatrikona matters (not impl yet).
             // Simplified: Trikona lordship makes it benefic.
             return FunctionalStatus::FunctionalBenefic;
        }
        
        // 3. Maraka: Lord of 2 and 7.
        // Note: 7 is Kendra. If 7th lord does not own Trikona/Upachaya?
        // Maraka rules are complex. Primary Marakas are 2nd and 7th lords.
        // If a planet owns 2 or 7 and is NOT a Trikona lord, it has Maraka potential.
        // Especially if it is a natural malefic.
        let is_maraka_house = owned_houses.iter().any(|&h| [2, 7].contains(&h));
        if is_maraka_house {
             // Check if it also owns good houses?
             // Simple version: Tag as Maraka if primarily associated with death houses and no Trikona.
             return FunctionalStatus::Maraka;
        }

        // 4. Malefic (Dusthana): Lord of 3, 6, 8, 11, 12.
        // 3, 6, 11 are Upachaya (Improve w/ time) but functional malefics.
        // 6, 8, 12 are Dusthana (Suffering).
        // 8th is most malefic.
        let is_dusthana = owned_houses.iter().any(|&h| [6, 8, 12].contains(&h));
        let is_upachaya_bad = owned_houses.iter().any(|&h| [3, 11].contains(&h));
        
        if is_dusthana || is_upachaya_bad {
            return FunctionalStatus::FunctionalMalefic;
        }

        // Default
        FunctionalStatus::Neutral
    }
}
