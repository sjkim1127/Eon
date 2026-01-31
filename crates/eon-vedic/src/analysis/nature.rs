use crate::chart::VedicChart;
use crate::planets::VedicPlanet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FunctionalStatus {
    Yogakaraka,        // Best benefic (Kendra + Trikona Lord)
    FunctionalBenefic, // Good (Trikona Lord)
    Neutral,           // Mixed or Neutral
    FunctionalMalefic, // Bad (Dusthana Lord)
    Maraka,            // Killer (2nd/7th Lord)
}

pub struct FunctionalNature;

impl FunctionalNature {
    /// Get Moolatrikona sign for a planet
    pub fn moolatrikona_rasi(planet: VedicPlanet) -> Option<u8> {
        match planet {
            VedicPlanet::Sun => Some(5),     // Leo
            VedicPlanet::Moon => Some(2),    // Taurus
            VedicPlanet::Mars => Some(1),    // Aries
            VedicPlanet::Mercury => Some(6), // Virgo
            VedicPlanet::Jupiter => Some(9), // Sagittarius
            VedicPlanet::Venus => Some(7),   // Libra
            VedicPlanet::Saturn => Some(11), // Aquarius
            _ => None,
        }
    }

    /// Analyze a planet's functional nature for a given Chart
    /// Updated to accept the full Chart to handle Nodes (Rahu/Ketu) correctly.
    pub fn analyze(chart: &VedicChart, planet: VedicPlanet) -> FunctionalStatus {
        let lagna_rasi = chart.ascendant.rasi;

        // Special Logic for Nodes (Rahu/Ketu)
        if planet == VedicPlanet::Rahu || planet == VedicPlanet::Ketu {
            return Self::analyze_node(chart, planet);
        }

        Self::analyze_regular_planet(lagna_rasi, planet)
    }

    /// Internal helper for regular planets (Sun..Saturn)
    fn analyze_regular_planet(lagna_rasi: u8, planet: VedicPlanet) -> FunctionalStatus {
        let mut owned_houses = Vec::new();

        // Scan all 12 signs to see which are owned by this planet
        for rasi in 1..=12 {
            if VedicPlanet::get_ruler_of(rasi) == planet {
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

        let is_kendra_lord = owned_houses.iter().any(|&h| [1, 4, 7, 10].contains(&h));
        let is_trikona_lord = owned_houses.iter().any(|&h| [1, 5, 9].contains(&h));

        // 1. Yogakaraka
        if is_kendra_lord && is_trikona_lord {
            return FunctionalStatus::Yogakaraka;
        }

        // 2. Benefic
        if is_trikona_lord {
            if let Some(h) = mt_house {
                if [1, 4, 5, 7, 9, 10].contains(&h) {
                    return FunctionalStatus::FunctionalBenefic;
                }
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

    /// Dynamic analysis for Rahu/Ketu based on Conjunctions and Dispositor
    fn analyze_node(chart: &VedicChart, node: VedicPlanet) -> FunctionalStatus {
        // 1. Find node position
        let pos = match chart.planets.iter().find(|p| p.planet == node) {
            Some(p) => p,
            None => return FunctionalStatus::FunctionalMalefic, // Should not happen
        };

        // 2. Check Conjunctions (Planets in the same sign)
        let mut conjunct_benefics = false;
        let mut conjunct_malefics = false;
        let mut conjunct_yogakaraka = false;

        for other in &chart.planets {
            if other.planet == node || matches!(other.planet, VedicPlanet::Rahu | VedicPlanet::Ketu)
            {
                continue;
            }

            if other.rasi == pos.rasi {
                // Determine the nature of the conjoined planet (Recursive check prevention: call regular analyze)
                let nature = Self::analyze_regular_planet(chart.ascendant.rasi, other.planet);
                match nature {
                    FunctionalStatus::Yogakaraka => conjunct_yogakaraka = true,
                    FunctionalStatus::FunctionalBenefic => conjunct_benefics = true,
                    FunctionalStatus::FunctionalMalefic | FunctionalStatus::Maraka => {
                        conjunct_malefics = true
                    }
                    _ => {}
                }
            }
        }

        // Rules of thumb for Node Modification:
        // - If conjunct Yogakaraka, becomes Yogakaraka.
        // - If conjunct Benefic and no Malefic, becomes Benefic.
        // - If conjunct Malefic, becomes Malefic.
        if conjunct_yogakaraka {
            return FunctionalStatus::Yogakaraka;
        }
        if conjunct_benefics && !conjunct_malefics {
            return FunctionalStatus::FunctionalBenefic;
        }
        if conjunct_malefics {
            return FunctionalStatus::FunctionalMalefic;
        }

        // 3. If no conjunctions, look at Dispositor (Lord of the sign)
        let dispositor = VedicPlanet::get_ruler_of(pos.rasi);
        let disp_nature = Self::analyze_regular_planet(chart.ascendant.rasi, dispositor);

        // Nodes mimic their dispositor
        match disp_nature {
            FunctionalStatus::Yogakaraka => FunctionalStatus::Yogakaraka,
            FunctionalStatus::FunctionalBenefic => FunctionalStatus::FunctionalBenefic,
            FunctionalStatus::Neutral => FunctionalStatus::Neutral,
            _ => FunctionalStatus::FunctionalMalefic,
        }
    }
}
