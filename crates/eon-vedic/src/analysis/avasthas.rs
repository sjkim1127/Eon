use serde::{Deserialize, Serialize};
use crate::planets::VedicPlanet;
use crate::chart::VedicPosition;

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
pub struct PlanetAvastha {
    pub planet: VedicPlanet,
    pub baladi: BaladiAvastha,
    pub jagradadi: JagradadiAvastha,
}

pub struct AvasthaEngine;

impl AvasthaEngine {
    pub fn calculate(pos: &VedicPosition) -> PlanetAvastha {
        let deg_in_sign = pos.sidereal_deg % 30.0;
        let is_odd = !pos.rasi.is_multiple_of(2);

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

        PlanetAvastha {
            planet: pos.planet,
            baladi,
            jagradadi,
        }
    }
}
