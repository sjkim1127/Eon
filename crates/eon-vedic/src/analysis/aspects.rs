use crate::chart::VedicChart;
use crate::planets::VedicPlanet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AspectRelation {
    pub aspecting_planet: VedicPlanet,
    pub aspected_houses: Vec<u8>, // 1-12
}

pub struct AspectEngine;

impl AspectEngine {
    /// Calculate aspects (Drishti) for all planets in the chart
    pub fn calculate_aspects(chart: &VedicChart) -> Vec<AspectRelation> {
        let mut relations = Vec::new();

        for pos in &chart.planets {
            let house = pos.house_index;
            let aspects = Self::get_planet_aspects(pos.planet, house);

            relations.push(AspectRelation {
                aspecting_planet: pos.planet,
                aspected_houses: aspects,
            });
        }

        relations
    }

    /// Returns the houses (1-12) aspected by a planet from a given house
    fn get_planet_aspects(planet: VedicPlanet, from_house: u8) -> Vec<u8> {
        let mut houses = Vec::new();

        // All planets aspect the 7th house from their position
        houses.push(Self::get_target_house(from_house, 7));

        // Special Aspects
        match planet {
            VedicPlanet::Mars => {
                // Mars aspects 4th and 8th as well
                houses.push(Self::get_target_house(from_house, 4));
                houses.push(Self::get_target_house(from_house, 8));
            }
            VedicPlanet::Jupiter | VedicPlanet::Rahu | VedicPlanet::Ketu => {
                // Jupiter (and nodes) aspect 5th and 9th
                houses.push(Self::get_target_house(from_house, 5));
                houses.push(Self::get_target_house(from_house, 9));
            }
            VedicPlanet::Saturn => {
                // Saturn aspects 3rd and 10th
                houses.push(Self::get_target_house(from_house, 3));
                houses.push(Self::get_target_house(from_house, 10));
            }
            _ => {}
        }

        houses.sort();
        houses.dedup();
        houses
    }

    fn get_target_house(start: u8, count: u8) -> u8 {
        // start is 1-12. count is distance (1=self, 7=opposite)
        let target = start as i32 + count as i32 - 1;
        let r = target % 12;
        if r == 0 {
            12
        } else {
            r as u8
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AspectEngine;

    #[test]
    fn wraps_to_twelve_when_modulo_is_zero() {
        assert_eq!(AspectEngine::get_target_house(6, 7), 12);
        assert_eq!(AspectEngine::get_target_house(3, 10), 12);
    }

    #[test]
    fn computes_general_wrapped_houses_correctly() {
        assert_eq!(AspectEngine::get_target_house(12, 7), 6);
        assert_eq!(AspectEngine::get_target_house(1, 7), 7);
        assert_eq!(AspectEngine::get_target_house(10, 4), 1);
    }
}
