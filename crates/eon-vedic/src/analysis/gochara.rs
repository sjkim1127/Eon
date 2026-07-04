use crate::chart::VedicChart;
use crate::planets::VedicPlanet;
use serde::{Deserialize, Serialize}; // We might need Ayanamsa from chart or engine

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MurtiType {
    Gold,   // Suvarna
    Silver, // Rajata
    Copper, // Tamra
    Iron,   // Loha
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SadeSatiPhase {
    None,
    Rising,  // Saturn in 12th from Moon
    Peak,    // Saturn in 1st from Moon
    Setting, // Saturn in 2nd from Moon
}

/// Transit Result for a single planet
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitPosition {
    pub planet: VedicPlanet,
    pub current_rasi: u8,
    pub house_from_moon: u8,      // 1~12
    pub is_benefic_transit: bool, // Simple check based on Gochara rules
    pub is_blocked: bool,         // Blocked by Vedha (obstruction)
    pub murti: MurtiType,         // Murti Nirnaya
    pub summary: String,
    pub description: String,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GocharaSummary {
    pub transits: Vec<TransitPosition>,
    pub sade_sati: SadeSatiPhase,
}

pub struct GocharaEngine;

impl GocharaEngine {
    /// Calculate current transit positions relative to Natal Moon
    /// We need an external way to calculate *current* planetary positions.
    /// Since GocharaEngine shouldn't hold the AstroEngine instance itself (usually),
    /// we expect the *current* chart to be passed in, or we simulate it.
    /// For this simplified version, let's assume we pass in a pre-calculated "Current Chart".
    ///
    /// Usage:
    /// let transit_chart = calculator.calculate(current_time, lat, lon);
    /// let transits = GocharaEngine::analyze(natal_chart.moon.rasi, &transit_chart);
    pub fn analyze(natal_moon_rasi: u8, current_chart: &VedicChart) -> GocharaSummary {
        let mut transits = Vec::new();
        let mut sade_sati = SadeSatiPhase::None;

        for pos in &current_chart.planets {
            let house_from_moon = if pos.rasi >= natal_moon_rasi {
                pos.rasi - natal_moon_rasi + 1
            } else {
                (12 - natal_moon_rasi) + pos.rasi + 1
            };

            let is_benefic = Self::check_benefic_transit(pos.planet, house_from_moon);

            // 1. Murti Nirnaya (Form)
            // Based on Moon's position (in the current chart) relative to Natal Moon.
            let current_moon = current_chart
                .planets
                .iter()
                .find(|p| p.planet == VedicPlanet::Moon);
            let murti = if let Some(m) = current_moon {
                let moon_house = if m.rasi >= natal_moon_rasi {
                    m.rasi - natal_moon_rasi + 1
                } else {
                    (12 - natal_moon_rasi) + m.rasi + 1
                };
                match moon_house {
                    1 | 6 | 11 => MurtiType::Gold,
                    2 | 5 | 9 => MurtiType::Silver,
                    3 | 7 | 10 => MurtiType::Copper,
                    4 | 8 | 12 => MurtiType::Iron,
                    _ => MurtiType::Unknown,
                }
            } else {
                MurtiType::Unknown
            };

            let mut is_blocked = false;
            if let Some(vedha_house) = Self::get_vedha_house(pos.planet, house_from_moon) {
                // Check if any planet is in the vedha_house (relative to moon)
                for other in &current_chart.planets {
                    let other_house = if other.rasi >= natal_moon_rasi {
                        other.rasi - natal_moon_rasi + 1
                    } else {
                        (12 - natal_moon_rasi) + other.rasi + 1
                    };

                    if other_house == vedha_house {
                        // Check exceptions
                        let is_exception = match (pos.planet, other.planet) {
                            (VedicPlanet::Sun, VedicPlanet::Saturn)
                            | (VedicPlanet::Saturn, VedicPlanet::Sun) => true,
                            (VedicPlanet::Moon, VedicPlanet::Mercury)
                            | (VedicPlanet::Mercury, VedicPlanet::Moon) => true,
                            _ => false,
                        };

                        if !is_exception {
                            is_blocked = true;
                            break;
                        }
                    }
                }
            }

            if pos.planet == VedicPlanet::Saturn {
                sade_sati = Self::calculate_sade_sati(natal_moon_rasi, pos.rasi);
            }

            let mut reasons = Vec::new();
            if is_benefic {
                reasons.push(format!(
                    "{:?} is in a natural benefic house ({}) from Moon.",
                    pos.planet, house_from_moon
                ));
            } else {
                reasons.push(format!(
                    "{:?} is in a challenging house ({}) from Moon.",
                    pos.planet, house_from_moon
                ));
            }

            if is_blocked {
                reasons.push("This transit's influence is obstructed by Vedha.".to_string());
            }

            let murti_desc = match murti {
                MurtiType::Gold => "Suvarna (Gold) - Highly Auspicious",
                MurtiType::Silver => "Rajata (Silver) - Auspicious",
                MurtiType::Copper => "Tamra (Copper) - Moderate",
                MurtiType::Iron => "Loha (Iron) - Challenging",
                MurtiType::Unknown => "Stable",
            };
            reasons.push(format!("Murti Status: {}", murti_desc));

            let summary = if is_benefic && !is_blocked {
                format!("{:?} Auspicious Transit", pos.planet)
            } else if is_blocked {
                format!("{:?} Blocked Transit", pos.planet)
            } else {
                format!("{:?} Challenging Transit", pos.planet)
            };

            let description = format!(
                "{:?} is transiting {} house from natal Moon in {:?} form.",
                pos.planet, house_from_moon, murti
            );

            transits.push(TransitPosition {
                planet: pos.planet,
                current_rasi: pos.rasi,
                house_from_moon,
                is_benefic_transit: is_benefic,
                is_blocked,
                murti,
                summary,
                description,
                reasons,
            });
        }

        GocharaSummary {
            transits,
            sade_sati,
        }
    }

    pub fn calculate_sade_sati(natal_moon: u8, saturn_transit: u8) -> SadeSatiPhase {
        // Moon's 12th is (Moon - 1)
        // Sign index 1-12
        let prev = if natal_moon == 1 { 12 } else { natal_moon - 1 };
        let next = if natal_moon == 12 { 1 } else { natal_moon + 1 };

        if saturn_transit == prev {
            SadeSatiPhase::Rising
        } else if saturn_transit == natal_moon {
            SadeSatiPhase::Peak
        } else if saturn_transit == next {
            SadeSatiPhase::Setting
        } else {
            SadeSatiPhase::None
        }
    }

    /// Standard Gochara Benefic Houses (from Moon)
    /// Sun: 3, 6, 10, 11
    /// Moon: 1, 3, 6, 7, 10, 11
    /// Mars: 3, 6, 11
    /// Mercury: 2, 4, 6, 8, 10, 11
    /// Jupiter: 2, 5, 7, 9, 11
    /// Venus: 1, 2, 3, 4, 5, 8, 9, 11, 12
    /// Saturn: 3, 6, 11
    /// Rahu: 3, 6, 11
    /// Ketu: 3, 6, 11
    fn check_benefic_transit(planet: VedicPlanet, house: u8) -> bool {
        match planet {
            VedicPlanet::Sun => [3, 6, 10, 11].contains(&house),
            VedicPlanet::Moon => [1, 3, 6, 7, 10, 11].contains(&house),
            VedicPlanet::Mars => [3, 6, 11].contains(&house),
            VedicPlanet::Mercury => [2, 4, 6, 8, 10, 11].contains(&house),
            VedicPlanet::Jupiter => [2, 5, 7, 9, 11].contains(&house),
            VedicPlanet::Venus => [1, 2, 3, 4, 5, 8, 9, 11, 12].contains(&house),
            VedicPlanet::Saturn => [3, 6, 11].contains(&house),
            VedicPlanet::Rahu => [3, 6, 11].contains(&house),
            VedicPlanet::Ketu => [3, 6, 11].contains(&house),
            _ => false,
        }
    }

    /// Get Vedha House for a specific planet in a house
    /// Now supports bidirectional mapping for Vipreet Vedha
    fn get_vedha_house(planet: VedicPlanet, house: u8) -> Option<u8> {
        match planet {
            VedicPlanet::Sun => match house {
                3 => Some(9),
                9 => Some(3),
                6 => Some(12),
                12 => Some(6),
                10 => Some(4),
                4 => Some(10),
                11 => Some(5),
                5 => Some(11),
                _ => None,
            },
            VedicPlanet::Moon => match house {
                1 => Some(5),
                5 => Some(1),
                3 => Some(9),
                9 => Some(3),
                6 => Some(12),
                12 => Some(6),
                7 => Some(2),
                2 => Some(7),
                10 => Some(4),
                4 => Some(10),
                11 => Some(8),
                8 => Some(11),
                _ => None,
            },
            VedicPlanet::Mars => match house {
                3 => Some(12),
                12 => Some(3),
                6 => Some(9),
                9 => Some(6),
                11 => Some(5),
                5 => Some(11),
                _ => None,
            },
            VedicPlanet::Mercury => match house {
                2 => Some(5),
                5 => Some(2),
                4 => Some(3),
                3 => Some(4),
                6 => Some(9),
                9 => Some(6),
                8 => Some(1),
                1 => Some(8),
                10 => Some(7),
                7 => Some(10),
                11 => Some(12),
                12 => Some(11),
                _ => None,
            },
            VedicPlanet::Jupiter => match house {
                2 => Some(12),
                12 => Some(2),
                5 => Some(4),
                4 => Some(5),
                7 => Some(3),
                3 => Some(7),
                9 => Some(10),
                10 => Some(9),
                11 => Some(8),
                8 => Some(11),
                _ => None,
            },
            VedicPlanet::Venus => match house {
                1 => Some(8),
                8 => Some(1),
                2 => Some(7),
                7 => Some(2),
                3 => Some(1),
                4 => Some(10),
                10 => Some(4),
                5 => Some(9),
                9 => Some(5),
                11 => Some(6),
                6 => Some(11),
                12 => Some(3),
                _ => None,
            },
            VedicPlanet::Saturn | VedicPlanet::Rahu | VedicPlanet::Ketu => match house {
                3 => Some(12),
                12 => Some(3),
                6 => Some(9),
                9 => Some(6),
                11 => Some(5),
                5 => Some(11),
                _ => None,
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sade_sati_boundaries() {
        // Moon in Pisces (12)
        let moon_pisces = 12;

        // Saturn in Aquarius (11) -> Rising
        assert_eq!(
            GocharaEngine::calculate_sade_sati(moon_pisces, 11),
            SadeSatiPhase::Rising
        );

        // Saturn in Pisces (12) -> Peak
        assert_eq!(
            GocharaEngine::calculate_sade_sati(moon_pisces, 12),
            SadeSatiPhase::Peak
        );

        // Saturn in Aries (1) -> Setting
        assert_eq!(
            GocharaEngine::calculate_sade_sati(moon_pisces, 1),
            SadeSatiPhase::Setting
        );

        // Saturn in Taurus (2) -> None
        assert_eq!(
            GocharaEngine::calculate_sade_sati(moon_pisces, 2),
            SadeSatiPhase::None
        );
    }
}
