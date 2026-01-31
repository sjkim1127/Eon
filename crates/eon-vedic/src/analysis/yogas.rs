use crate::analysis::nature::{FunctionalNature, FunctionalStatus};
use crate::chart::VedicChart;
use crate::planets::VedicPlanet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum YogaType {
    RajaYoga,            // Generic Raja Yoga
    GajaKesari,          // Jupiter + Moon
    Sunapha,             // Planet in 2nd from Moon
    Anapha,              // Planet in 12th from Moon
    DhanaYoga,           // Wealth Yoga
    Budhaditya,          // Sun + Mercury
    DharmaKarmaAdhipati, // 9th + 10th Lord
    PanchaMahapurusha,   // Special position for Mars, Mercury, Jupiter, Venus, or Saturn
    NeechaBhanga,        // Cancellation of debilitation
    Parivartana,         // Exchange of house lords
                         // Add more types as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YogaResult {
    pub name: String,
    pub yoga_type: YogaType,
    pub description: String,
    pub planets_involved: Vec<VedicPlanet>,
    pub quality: YogaQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum YogaQuality {
    VeryHigh,
    High,
    Medium,
    Weak(String), // Reason for weakness
}

#[derive(Debug, Clone, PartialEq)]
pub enum YogaCondition {
    /// Planets are in the same sign
    Conjunction(VedicPlanet, VedicPlanet),
    /// Lord of H1 and Lord of H2 are conjunct or mutually aspecting
    LordConnection(u8, u8),
    /// Planet is in House H (from Lagna)
    Placement(VedicPlanet, u8),
    /// Lord of H1 is in House H2
    LordPlacement(u8, u8),
    /// Planet A is in Kendra (1,4,7,10) from Planet B
    KendraRelationship(VedicPlanet, VedicPlanet),
    /// Planet A is in Houses [H...] from Planet B
    RelativePlacement {
        p: VedicPlanet,
        from: VedicPlanet,
        houses: Vec<u8>,
    },
    /// Planets (excluding `exclude`) are in 2nd from `from`
    SunaphaLogic {
        from: VedicPlanet,
        exclude: Vec<VedicPlanet>,
    },
    /// Planets (excluding `exclude`) are in 12th from `from`
    AnaphaLogic {
        from: VedicPlanet,
        exclude: Vec<VedicPlanet>,
    },
    /// Special Check: Pancha Mahapurusha Yoga (Own/Exalted in Kendra)
    PanchaMahapurushaCheck(VedicPlanet),
    /// Special Check: Neecha Bhanga (Cancellation of Debilitation) for Planet
    NeechaBhangaCheck(VedicPlanet),
    /// Special Check: Parivartana (Exchange of Lords)
    ParivartanaCheck,
    /// Composite AND
    And(Vec<YogaCondition>),
}

pub struct YogaRule {
    pub name: String,
    pub yoga_type: YogaType,
    pub description: String,
    pub condition: YogaCondition,
}

pub struct YogaEngine;

impl YogaEngine {
    pub fn check_yogas(chart: &VedicChart) -> Vec<YogaResult> {
        let rules = Self::get_rules();
        let mut results = Vec::new();

        for rule in rules {
            if let Some(mut planets) = Self::evaluate_condition(&rule.condition, chart) {
                // Determine Quality
                let quality = Self::assess_quality(chart, &planets);

                // Dedup planets
                planets.sort();
                planets.dedup();

                results.push(YogaResult {
                    name: rule.name,
                    yoga_type: rule.yoga_type,
                    description: rule.description,
                    planets_involved: planets,
                    quality,
                });
            }
        }

        results
    }

    fn get_rules() -> Vec<YogaRule> {
        let mut rules = Vec::new();

        // 1. Gaja Kesari
        rules.push(YogaRule {
            name: "Gaja Kesari Yoga".to_string(),
            yoga_type: YogaType::GajaKesari,
            description: "Jupiter in Kendra from Moon. Wisdom, virtue, reputation.".to_string(),
            condition: YogaCondition::KendraRelationship(VedicPlanet::Jupiter, VedicPlanet::Moon),
        });

        // 2. Budhaditya
        rules.push(YogaRule {
            name: "Budhaditya Yoga".to_string(),
            yoga_type: YogaType::Budhaditya,
            description: "Sun and Mercury conjunction. Intelligence and communication skills."
                .to_string(),
            condition: YogaCondition::Conjunction(VedicPlanet::Sun, VedicPlanet::Mercury),
        });

        // 3. Dharma-Karma Adhipati (9th and 10th Lords)
        rules.push(YogaRule {
            name: "Dharma-Karma Adhipati Yoga".to_string(),
            yoga_type: YogaType::DharmaKarmaAdhipati,
            description: "Connection of 9th and 10th Lords. Success.".to_string(),
            condition: YogaCondition::LordConnection(9, 10),
        });

        // 4. Dhana Yogas
        rules.push(YogaRule {
            name: "Dhana Yoga (2 in 11)".to_string(),
            yoga_type: YogaType::DhanaYoga,
            description: "2nd Lord in 11th House. Wealth.".to_string(),
            condition: YogaCondition::LordPlacement(2, 11),
        });

        rules.push(YogaRule {
            name: "Dhana Yoga (11 in 2)".to_string(),
            yoga_type: YogaType::DhanaYoga,
            description: "11th Lord in 2nd House. Wealth.".to_string(),
            condition: YogaCondition::LordPlacement(11, 2),
        });

        // 5. Raja Yogas (Kendra + Trikona Lords)
        let kendras = [1, 4, 7, 10];
        let trikonas = [1, 5, 9];
        for &k in &kendras {
            for &t in &trikonas {
                if k == t {
                    continue;
                } // Skip 1-1
                rules.push(YogaRule {
                    name: format!("Raja Yoga (L{} & L{})", k, t),
                    yoga_type: YogaType::RajaYoga,
                    description: format!(
                        "Connection of Kendra Lord ({}) and Trikona Lord ({}).",
                        k, t
                    ),
                    condition: YogaCondition::LordConnection(k, t),
                });
            }
        }

        // 6. Sunapha / Anapha
        rules.push(YogaRule {
            name: "Sunapha Yoga".to_string(),
            yoga_type: YogaType::Sunapha,
            description: "Planets in 2nd from Moon (excluding Sun/Nodes).".to_string(),
            condition: YogaCondition::SunaphaLogic {
                from: VedicPlanet::Moon,
                exclude: vec![VedicPlanet::Sun, VedicPlanet::Rahu, VedicPlanet::Ketu],
            },
        });

        rules.push(YogaRule {
            name: "Anapha Yoga".to_string(),
            yoga_type: YogaType::Anapha,
            description: "Planets in 12th from Moon (excluding Sun/Nodes).".to_string(),
            condition: YogaCondition::AnaphaLogic {
                from: VedicPlanet::Moon,
                exclude: vec![VedicPlanet::Sun, VedicPlanet::Rahu, VedicPlanet::Ketu],
            },
        });

        // 7. Pancha Mahapurusha Yogas
        let mahapurusha_planets = [
            (VedicPlanet::Mars, "Ruchaka Yoga", "Courage and strength"),
            (VedicPlanet::Mercury, "Bhadra Yoga", "Intelligence"),
            (VedicPlanet::Jupiter, "Hamsa Yoga", "Wisdom"),
            (VedicPlanet::Venus, "Malavya Yoga", "Comforts"),
            (VedicPlanet::Saturn, "Sasa Yoga", "Authority"),
        ];
        for (p, name, desc) in mahapurusha_planets {
            rules.push(YogaRule {
                name: name.to_string(),
                yoga_type: YogaType::PanchaMahapurusha,
                description: desc.to_string(),
                condition: YogaCondition::PanchaMahapurushaCheck(p),
            });
        }

        // 8. Neecha Bhanga
        // We add rules for each planet potentially
        let planets = [
            VedicPlanet::Sun,
            VedicPlanet::Moon,
            VedicPlanet::Mars,
            VedicPlanet::Mercury,
            VedicPlanet::Jupiter,
            VedicPlanet::Venus,
            VedicPlanet::Saturn,
        ];
        for p in planets {
            rules.push(YogaRule {
                name: format!("Neecha Bhanga ({:?})", p),
                yoga_type: YogaType::NeechaBhanga,
                description: "Cancellation of debilitation.".to_string(),
                condition: YogaCondition::NeechaBhangaCheck(p),
            });
        }

        // 9. Parivartana
        rules.push(YogaRule {
            name: "Parivartana Yoga".to_string(),
            yoga_type: YogaType::Parivartana,
            description: "Exchange of House Lords.".to_string(),
            condition: YogaCondition::ParivartanaCheck,
        });

        rules
    }

    fn evaluate_condition(cond: &YogaCondition, chart: &VedicChart) -> Option<Vec<VedicPlanet>> {
        match cond {
            YogaCondition::Conjunction(p1, p2) => {
                let pos1 = chart.planets.iter().find(|p| p.planet == *p1)?;
                let pos2 = chart.planets.iter().find(|p| p.planet == *p2)?;
                if pos1.rasi == pos2.rasi {
                    Some(vec![*p1, *p2])
                } else {
                    None
                }
            }
            YogaCondition::KendraRelationship(p1, p2) => {
                let pos1 = chart.planets.iter().find(|p| p.planet == *p1)?;
                let pos2 = chart.planets.iter().find(|p| p.planet == *p2)?;

                // House from P2
                let diff = pos1.rasi as i32 - pos2.rasi as i32;
                let dist = if diff >= 0 { diff + 1 } else { diff + 13 };

                if [1, 4, 7, 10].contains(&dist) {
                    Some(vec![*p1, *p2])
                } else {
                    None
                }
            }
            YogaCondition::Placement(planet, house) => {
                let pos = chart.planets.iter().find(|p| p.planet == *planet)?;
                if pos.house_index == *house {
                    Some(vec![*planet])
                } else {
                    None
                }
            }
            YogaCondition::LordPlacement(lord_h, in_h) => {
                let lord = Self::get_lord_of_house(chart.ascendant.rasi, *lord_h);
                let pos = chart.planets.iter().find(|p| p.planet == lord)?;
                if pos.house_index == *in_h {
                    Some(vec![lord])
                } else {
                    None
                }
            }
            YogaCondition::LordConnection(h1, h2) => {
                let l1 = Self::get_lord_of_house(chart.ascendant.rasi, *h1);
                let l2 = Self::get_lord_of_house(chart.ascendant.rasi, *h2);

                if l1 == l2 {
                    return None;
                } // Same planet (e.g. for some Lagnas)

                let p1 = chart.planets.iter().find(|p| p.planet == l1)?;
                let p2 = chart.planets.iter().find(|p| p.planet == l2)?;

                // Conjunction check
                let conj = p1.rasi == p2.rasi;

                // Mutual Aspect check (7th house)
                let diff = (p1.rasi as i32 - p2.rasi as i32).abs();
                let aspect = diff == 6; // 1-7 axis

                if conj || aspect {
                    Some(vec![l1, l2])
                } else {
                    None
                }
            }
            YogaCondition::SunaphaLogic { from, exclude } => {
                let f_pos = chart.planets.iter().find(|p| p.planet == *from)?;
                let check_rasi = if f_pos.rasi == 12 { 1 } else { f_pos.rasi + 1 };

                let mut found_planets = Vec::new();
                for p in &chart.planets {
                    if p.rasi == check_rasi && !exclude.contains(&p.planet) {
                        found_planets.push(p.planet);
                    }
                }

                if !found_planets.is_empty() {
                    found_planets.push(*from);
                    Some(found_planets)
                } else {
                    None
                }
            }
            YogaCondition::AnaphaLogic { from, exclude } => {
                let f_pos = chart.planets.iter().find(|p| p.planet == *from)?;
                let check_rasi = if f_pos.rasi == 1 { 12 } else { f_pos.rasi - 1 };

                let mut found_planets = Vec::new();
                for p in &chart.planets {
                    if p.rasi == check_rasi && !exclude.contains(&p.planet) {
                        found_planets.push(p.planet);
                    }
                }
                if !found_planets.is_empty() {
                    found_planets.push(*from);
                    Some(found_planets)
                } else {
                    None
                }
            }
            YogaCondition::PanchaMahapurushaCheck(planet) => {
                let pos = chart.planets.iter().find(|p| p.planet == *planet)?;
                // Must be in Kendra from Lagna
                if [1, 4, 7, 10].contains(&pos.house_index) {
                    let rasi = pos.rasi;
                    let ruler = VedicPlanet::get_ruler_of(rasi);
                    let exalt = planet.exaltation_rasi();

                    if ruler == *planet || rasi == exalt {
                        Some(vec![*planet])
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            YogaCondition::NeechaBhangaCheck(planet) => {
                let pos = chart.planets.iter().find(|p| p.planet == *planet)?;
                if planet.debilitation_rasi() != pos.rasi {
                    return None; // Not debilitated
                }

                // Cancellation Rules (Simplified):
                // 1. Lord of the sign where planet is debilitated is in Kendra from Lagna/Moon.
                // 2. Lord of the exaltation sign of the debilitated planet is in Kendra from Lagna/Moon.
                // We will check just Lagna Kendra for Dispositor for now as per previous logic

                let dispositor = VedicPlanet::get_ruler_of(pos.rasi);
                let disp_pos = chart.planets.iter().find(|p| p.planet == dispositor)?;

                // Check if Dispositor is in Kendra (1,4,7,10)
                if [1, 4, 7, 10].contains(&disp_pos.house_index) {
                    Some(vec![*planet, dispositor])
                } else {
                    None
                }
            }
            YogaCondition::ParivartanaCheck => {
                // Return generic success if ANY exchange found?
                // The issue: Rule engine expects one result per rule.
                // But Parivartana can happen multiple times.
                // For now, let's find the FIRST one (limitation of single rule entry)
                // OR we return all planets involved in exchanges.

                let mut exchanged_planets = Vec::new();
                for h in 1..=12 {
                    let lord = Self::get_lord_of_house(chart.ascendant.rasi, h);
                    if let Some(pos) = chart.planets.iter().find(|p| p.planet == lord) {
                        let occ_h = pos.house_index;
                        if occ_h != h {
                            let lord_of_occ = Self::get_lord_of_house(chart.ascendant.rasi, occ_h);
                            if let Some(pos2) =
                                chart.planets.iter().find(|p| p.planet == lord_of_occ)
                            {
                                if pos2.house_index == h {
                                    // Exchange Match!
                                    // To avoid duplicates, we can check h < occ_h
                                    if h < occ_h {
                                        exchanged_planets.push(lord);
                                        exchanged_planets.push(lord_of_occ);
                                    }
                                }
                            }
                        }
                    }
                }

                if !exchanged_planets.is_empty() {
                    exchanged_planets.sort();
                    exchanged_planets.dedup();
                    Some(exchanged_planets)
                } else {
                    None
                }
            }
            _ => None, // Implement others if needed
        }
    }

    fn get_lord_of_house(lagna_rasi: u8, house: u8) -> VedicPlanet {
        let rasi_idx = ((lagna_rasi as i32 + house as i32 - 1 - 1) % 12 + 1) as u8;
        VedicPlanet::get_ruler_of(rasi_idx)
    }

    fn assess_quality(chart: &VedicChart, planets: &[VedicPlanet]) -> YogaQuality {
        let mut has_combust = false;
        let mut has_yogakaraka = false;
        let mut has_benefic = false;

        for &pl in planets {
            if let Some(pos) = chart.planets.iter().find(|p| p.planet == pl) {
                if pos.is_combust {
                    has_combust = true;
                }

                let nature = FunctionalNature::analyze(chart.ascendant.rasi, pl);
                if nature == FunctionalStatus::Yogakaraka {
                    has_yogakaraka = true;
                }
                if nature == FunctionalStatus::FunctionalBenefic {
                    has_benefic = true;
                }
            }
        }

        if has_combust {
            YogaQuality::Weak("Combustion".to_string())
        } else if has_yogakaraka {
            YogaQuality::VeryHigh
        } else if has_benefic {
            YogaQuality::High
        } else {
            YogaQuality::Medium
        }
    }
}
