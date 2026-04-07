use serde::{Deserialize, Serialize};
use crate::planets::VedicPlanet;
use crate::chart::{VedicPosition, VedicChart};
use crate::analysis::relationships::{RelationshipEngine, RelationshipType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BaladiAvastha {
    Bala,    // Infant (0-6)
    Kumara,  // Youthful (6-12)
    Yuva,    // Young Adult (12-18)
    Vriddha, // Old (18-24)
    Mrita,   // Dead (24-30)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JagradadiAvastha {
    Jagrat,   // Awake (0-10)
    Swapna,   // Dream (10-20)
    Sushupti, // Sleep (20-30)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeeptaadiAvastha {
    Deepta,   // Exalted
    Svastha,  // Own Sign
    Mudita,   // Great Friend
    Shanta,   // Friend
    Deena,    // Neutral
    Dukhita,  // Enemy
    Vikala,   // Great Enemy
    Khala,    // Debilitated
    Kopita,   // Combust
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanetAvastha {
    pub planet: VedicPlanet,
    pub baladi: BaladiAvastha,
    pub jagradadi: JagradadiAvastha,
    pub deeptaadi: DeeptaadiAvastha,
}

pub struct AvasthaEngine;

impl AvasthaEngine {
    pub fn calculate(pos: &VedicPosition, chart: &VedicChart) -> PlanetAvastha {
        let deg_in_sign = pos.sidereal_deg % 30.0;
        let rasi = pos.rasi;
        let is_odd = rasi % 2 != 0;

        let baladi = if is_odd {
            if deg_in_sign < 6.0 { BaladiAvastha::Bala }
            else if deg_in_sign < 12.0 { BaladiAvastha::Kumara }
            else if deg_in_sign < 18.0 { BaladiAvastha::Yuva }
            else if deg_in_sign < 24.0 { BaladiAvastha::Vriddha }
            else { BaladiAvastha::Mrita }
        } else if deg_in_sign < 6.0 { BaladiAvastha::Mrita }
        else if deg_in_sign < 12.0 { BaladiAvastha::Vriddha }
        else if deg_in_sign < 18.0 { BaladiAvastha::Yuva }
        else if deg_in_sign < 24.0 { BaladiAvastha::Kumara }
        else { BaladiAvastha::Bala };

        let jagradadi = if is_odd {
            if deg_in_sign < 10.0 { JagradadiAvastha::Jagrat }
            else if deg_in_sign < 20.0 { JagradadiAvastha::Swapna }
            else { JagradadiAvastha::Sushupti }
        } else if deg_in_sign < 10.0 { JagradadiAvastha::Sushupti }
        else if deg_in_sign < 20.0 { JagradadiAvastha::Swapna }
        else { JagradadiAvastha::Jagrat };

        // Deeptaadi Avastha
        let deeptaadi = if pos.is_combust {
            DeeptaadiAvastha::Kopita
        } else if rasi == pos.planet.exaltation_rasi() {
            DeeptaadiAvastha::Deepta
        } else if rasi == pos.planet.debilitation_rasi() {
            DeeptaadiAvastha::Khala
        } else {
            let lord = VedicPlanet::get_ruler_of(rasi);
            if lord == pos.planet {
                DeeptaadiAvastha::Svastha
            } else {
                let rel = RelationshipEngine::get_relationship(pos.planet, lord, chart);
                match rel {
                    RelationshipType::GreatFriend => DeeptaadiAvastha::Mudita,
                    RelationshipType::Friend => DeeptaadiAvastha::Shanta,
                    RelationshipType::Neutral => DeeptaadiAvastha::Deena,
                    RelationshipType::Enemy => DeeptaadiAvastha::Dukhita,
                    RelationshipType::GreatEnemy => DeeptaadiAvastha::Vikala,
                }
            }
        };

        PlanetAvastha {
            planet: pos.planet,
            baladi,
            jagradadi,
            deeptaadi,
        }
    }
}
