use crate::chart::VedicChart;
use crate::planets::VedicPlanet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipType {
    GreatFriend, // Adhi Mitra
    Friend,      // Mitra
    Neutral,     // Sama
    Enemy,       // Shatru
    GreatEnemy,  // Adhi Shatru
}

pub struct RelationshipEngine;

impl RelationshipEngine {
    /// Calculate Panchadha Maitri (Compound Relationship)
    /// Combines Natural (Naisargika) and Temporal (Tatkalika) relationships.
    pub fn get_relationship(
        planet: VedicPlanet,
        target: VedicPlanet,
        chart: &VedicChart,
    ) -> RelationshipType {
        let natural = Self::get_natural_relationship(planet, target);
        let temporal = Self::get_temporal_relationship(planet, target, chart);

        match (natural, temporal) {
            (NaturalRelationship::Friend, TemporalRelationship::Friend) => {
                RelationshipType::GreatFriend
            }
            (NaturalRelationship::Friend, TemporalRelationship::Enemy) => RelationshipType::Neutral,
            (NaturalRelationship::Neutral, TemporalRelationship::Friend) => {
                RelationshipType::Friend
            }
            (NaturalRelationship::Neutral, TemporalRelationship::Enemy) => RelationshipType::Enemy,
            (NaturalRelationship::Enemy, TemporalRelationship::Friend) => RelationshipType::Neutral,
            (NaturalRelationship::Enemy, TemporalRelationship::Enemy) => {
                RelationshipType::GreatEnemy
            }
        }
    }

    /// Naisargika Maitri (Natural Relationship) based on BPHS
    pub fn get_natural_relationship(
        planet: VedicPlanet,
        target: VedicPlanet,
    ) -> NaturalRelationship {
        if planet == target {
            return NaturalRelationship::Neutral;
        }

        match planet {
            VedicPlanet::Sun => match target {
                VedicPlanet::Moon | VedicPlanet::Mars | VedicPlanet::Jupiter => {
                    NaturalRelationship::Friend
                }
                VedicPlanet::Venus | VedicPlanet::Saturn => NaturalRelationship::Enemy,
                _ => NaturalRelationship::Neutral,
            },
            VedicPlanet::Moon => match target {
                VedicPlanet::Sun | VedicPlanet::Mercury => NaturalRelationship::Friend,
                _ => NaturalRelationship::Neutral, // Moon has no enemies
            },
            VedicPlanet::Mars => match target {
                VedicPlanet::Sun | VedicPlanet::Moon | VedicPlanet::Jupiter => {
                    NaturalRelationship::Friend
                }
                VedicPlanet::Mercury => NaturalRelationship::Enemy,
                _ => NaturalRelationship::Neutral,
            },
            VedicPlanet::Mercury => match target {
                VedicPlanet::Sun | VedicPlanet::Venus => NaturalRelationship::Friend,
                VedicPlanet::Moon => NaturalRelationship::Enemy,
                _ => NaturalRelationship::Neutral,
            },
            VedicPlanet::Jupiter => match target {
                VedicPlanet::Sun | VedicPlanet::Moon | VedicPlanet::Mars => {
                    NaturalRelationship::Friend
                }
                VedicPlanet::Mercury | VedicPlanet::Venus => NaturalRelationship::Enemy,
                _ => NaturalRelationship::Neutral,
            },
            VedicPlanet::Venus => match target {
                VedicPlanet::Mercury | VedicPlanet::Saturn => NaturalRelationship::Friend,
                VedicPlanet::Sun | VedicPlanet::Moon => NaturalRelationship::Enemy,
                _ => NaturalRelationship::Neutral,
            },
            VedicPlanet::Saturn => match target {
                VedicPlanet::Mercury | VedicPlanet::Venus => NaturalRelationship::Friend,
                VedicPlanet::Sun | VedicPlanet::Moon | VedicPlanet::Mars => {
                    NaturalRelationship::Enemy
                }
                _ => NaturalRelationship::Neutral,
            },
            _ => NaturalRelationship::Neutral,
        }
    }

    /// Tatkalika Maitri (Temporal Relationship)
    /// Planets in 2, 3, 4, 10, 11, 12 from each other are temporary friends.
    /// Others (1, 5, 6, 7, 8, 9) are enemies.
    pub fn get_temporal_relationship(
        planet: VedicPlanet,
        target: VedicPlanet,
        chart: &VedicChart,
    ) -> TemporalRelationship {
        if planet == target {
            return TemporalRelationship::Enemy;
        } // Self is not friend in temporal logic usually, but irrelevant

        let p_pos = chart.planets.iter().find(|p| p.planet == planet);
        let t_pos = chart.planets.iter().find(|p| p.planet == target);

        if let (Some(p), Some(t)) = (p_pos, t_pos) {
            // Count from planet to target
            let mut diff = t.rasi as i32 - p.rasi as i32;
            if diff < 0 {
                diff += 12;
            }
            let count = diff + 1; // 1-based count

            if [2, 3, 4, 10, 11, 12].contains(&count) {
                TemporalRelationship::Friend
            } else {
                TemporalRelationship::Enemy
            }
        } else {
            TemporalRelationship::Enemy
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NaturalRelationship {
    Friend,
    Neutral,
    Enemy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemporalRelationship {
    Friend,
    Enemy,
}
