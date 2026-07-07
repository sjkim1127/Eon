use crate::analysis::avasthas::AvasthaEngine;
use crate::analysis::nature::{FunctionalNature, FunctionalStatus};
use crate::analysis::strength::StrengthEngine;
use crate::chart::VedicChart;
use crate::planets::VedicPlanet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
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
    Parivartana,         // Exchange of house lords (generic)
    ParivartanaMaha,     // Great exchange (Kendra/Trikona with Kendra/Trikona)
    ParivartanaKhala,    // Mixed exchange (one dusthana)
    ParivartanaDainya,   // Difficult exchange (both dusthana 6,8,12)
    // Negative Yogas (Arishta)
    Kemadruma,         // No planets flanking Moon (poverty/hardship)
    VipareetaRajaYoga, // Lord of dusthana in dusthana (vice becomes virtue)
    // Dusthana Lord in Own House Yogas (Beneficial)
    Harsha,    // 6th lord in 6th house (victory over enemies)
    Sarala,    // 8th lord in 8th house (long life, fearlessness)
    Vimala,    // 12th lord in 12th house (spiritual liberation)
    KalaSarpa, // All planets hemmed between Rahu-Ketu axis (obstacles, delays)
    Adhi,      // Benefics in 6, 7, 8 from Moon/Lagna
    Vasumathi, // Benefics in Upachaya (3, 6, 10, 11) houses
    Sakata,    // Moon in 6, 8, 12 from Jupiter
    // ── Nabhasa Yogas (BPHS) ────────────────────────────────
    // Sankhya (분산 개수 기반) — 9행성이 점유한 라시 수로 판별
    NabhasaGola,   // 9행성 → 1 rasi: 극단적 집중, 매우 희귀
    NabhasaYuga,   // 9행성 → 2 rasi: 극단적 이원성, 도전적 삶
    NabhasaShoola, // 9행성 → 3 rasi: 삼위일체 균형, 고난과 강인함
    NabhasaKedara, // 9행성 → 4 rasi: 근면, 농업적 성향, 인내
    NabhasaPasha,  // 9행성 → 5 rasi: 결속력, 집중, 집착
    NabhasaDaama,  // 9행성 → 6 rasi: 관대함, 통솔력
    NabhasaVeena,  // 9행성 → 7 rasi: 음악/예술/조화의 재능
    // Akriti (사인 성질 기반) — 모든 행성이 같은 성질의 사인에만 위치
    NabhasaAshrita,     // Chara 사인만 (Aries/Cancer/Libra/Cap): 유동성, 적응력
    NabhasaSthira,      // Sthira 사인만 (Taurus/Leo/Scorpio/Aqu): 안정, 고집
    NabhasaDvisvabhava, // Dvisvabhava 사인만 (Gem/Vir/Sag/Pis): 유연, 이중성
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YogaResult {
    pub name: String,
    pub yoga_type: YogaType,
    pub description: String,
    pub planets_involved: Vec<VedicPlanet>,
    pub quality: YogaQuality,
    pub strength_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
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
    /// Special Check: Kemadruma Yoga (No planets flanking Moon)
    KemadrumaCheck,
    /// Special Check: Vipareeta Raja Yoga (Dusthana lord in dusthana)
    VipareetaRajaYogaCheck,
    /// Special Check: Harsha Yoga (6th lord in 6th house)
    HarshaYogaCheck,
    /// Special Check: Sarala Yoga (8th lord in 8th house)
    SaralaYogaCheck,
    /// Special Check: Vimala Yoga (12th lord in 12th house)
    VimalaYogaCheck,
    /// Special Check: Kala Sarpa Yoga (All planets between Rahu-Ketu axis)
    KalaSarpaCheck,
    /// Special Check: Adhi Yoga (Benefics in 6, 7, 8 from Moon)
    AdhiYogaCheck,
    /// Special Check: Vasumathi Yoga (Benefics in Upachaya houses)
    VasumathiYogaCheck,
    /// Special Check: Sakata Yoga (Moon in 6, 8, 12 from Jupiter)
    SakataYogaCheck,
    /// Special Check: Dhana Yoga (Wealth combinations)
    DhanaYogaCheck,
    /// Composite AND
    And(Vec<YogaCondition>),
    /// Special Check: Nabhasa Yoga (Planet distribution patterns, BPHS)
    NabhasaCheck,
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
            // Parivartana is handled separately through classification logic
            if rule.condition == YogaCondition::ParivartanaCheck {
                continue;
            }

            if let Some(mut planets) = Self::evaluate_condition(&rule.condition, chart) {
                // Determine Quality
                let mut quality = Self::assess_quality(chart, &planets);

                // [BPHS 고도화] Vipareeta Raja Yoga: Shadbala 기반 최종 품질 조정
                // evaluate_condition 에서 이미 score < 80 행성은 필터됐으므로
                // 여기서는 80~180 구간(약한 Yoga)을 Weak 으로 다운그레이드
                if rule.yoga_type == YogaType::VipareetaRajaYoga {
                    let avg_shadbala = planets
                        .iter()
                        .filter_map(|&pl| {
                            chart
                                .planets
                                .iter()
                                .find(|p| p.planet == pl)
                                .map(|pos| StrengthEngine::calculate(pos, chart).total_score)
                        })
                        .sum::<f64>()
                        / planets.len().max(1) as f64;

                    if avg_shadbala < 180.0 {
                        quality = YogaQuality::Weak(format!(
                            "Shadbala 강도 부족 ({:.0}/180 미만) — Yoga 발현 약함",
                            avg_shadbala
                        ));
                    }
                }

                if rule.yoga_type == YogaType::PanchaMahapurusha {
                    let avg_shadbala = planets
                        .iter()
                        .filter_map(|&pl| {
                            chart
                                .planets
                                .iter()
                                .find(|p| p.planet == pl)
                                .map(|pos| StrengthEngine::calculate(pos, chart).total_score)
                        })
                        .sum::<f64>()
                        / planets.len().max(1) as f64;

                    if avg_shadbala < 100.0 {
                        quality = YogaQuality::Weak(format!(
                            "Shadbala 강도 부족 ({:.0}/100 미만) — Mahapurusha 발현 불가",
                            avg_shadbala
                        ));
                    }
                }

                if rule.yoga_type == YogaType::NeechaBhanga {
                    let mut has_kendra_exaltation = false;
                    for pl in &planets {
                        if let Some(pos) = chart.planets.iter().find(|p| p.planet == *pl) {
                            if [1, 4, 7, 10].contains(&pos.house_index) {
                                has_kendra_exaltation = true;
                            }
                        }
                    }
                    if !has_kendra_exaltation {
                        quality = YogaQuality::Weak(
                            "고양성/지배성이 켄드라에 위치하지 않아 약한 니차방가".to_string(),
                        );
                    } else {
                        quality = YogaQuality::VeryHigh; // 확실한 역전
                    }
                }

                // Dedup planets
                planets.sort();
                planets.dedup();

                let mut total_avastha = 0.0;
                let mut valid_planets_count = 0;
                for pl in &planets {
                    if let Some(pos) = chart.planets.iter().find(|p| p.planet == *pl) {
                        let avastha = AvasthaEngine::calculate(pos, chart);
                        total_avastha += avastha.score;
                        valid_planets_count += 1;
                    }
                }

                let mut strength_percentage = if valid_planets_count > 0 {
                    total_avastha / valid_planets_count as f64
                } else {
                    0.0
                };

                // Adjust strength based on quality
                if let YogaQuality::Weak(_) = quality {
                    strength_percentage *= 0.5; // Halve strength if weak
                }

                results.push(YogaResult {
                    name: rule.name,
                    yoga_type: rule.yoga_type,
                    description: rule.description,
                    planets_involved: planets,
                    quality,
                    strength_percentage,
                });
            }
        }

        // Include Parivartana Yogas with proper classification (Maha, Khala, Dainya)
        results.extend(Self::find_parivartana_exchanges(chart));

        // Include Nabhasa Yogas (BPHS planet distribution patterns)
        results.extend(Self::evaluate_nabhasa_yogas(chart));

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

        // 9. Parivartana (with classification)
        // This will be handled specially in check_yogas
        rules.push(YogaRule {
            name: "Parivartana Yoga".to_string(),
            yoga_type: YogaType::Parivartana,
            description: "Exchange of House Lords.".to_string(),
            condition: YogaCondition::ParivartanaCheck,
        });

        // 10. Kemadruma Yoga (No planets flanking Moon)
        rules.push(YogaRule {
            name: "Kemadruma Yoga".to_string(),
            yoga_type: YogaType::Kemadruma,
            description: "Moon isolated with no planets in 2nd or 12th houses. Poverty, hardship."
                .to_string(),
            condition: YogaCondition::KemadrumaCheck,
        });

        // 11. Vipareeta Raja Yoga (Lord of dusthana in another dusthana)
        rules.push(YogaRule {
            name: "Vipareeta Raja Yoga".to_string(),
            yoga_type: YogaType::VipareetaRajaYoga,
            description:
                "Lord of 6th, 8th, or 12th house in another dusthana. Vice becomes virtue."
                    .to_string(),
            condition: YogaCondition::VipareetaRajaYogaCheck,
        });

        // 12. Harsha Yoga (6th lord in 6th house)
        rules.push(YogaRule {
            name: "Harsha Yoga".to_string(),
            yoga_type: YogaType::Harsha,
            description: "Lord of 6th house in 6th house. Victory over enemies, good health."
                .to_string(),
            condition: YogaCondition::HarshaYogaCheck,
        });

        // 13. Sarala Yoga (8th lord in 8th house)
        rules.push(YogaRule {
            name: "Sarala Yoga".to_string(),
            yoga_type: YogaType::Sarala,
            description: "Lord of 8th house in 8th house. Long life, fearlessness, prosperity."
                .to_string(),
            condition: YogaCondition::SaralaYogaCheck,
        });

        // 14. Vimala Yoga (12th lord in 12th house)
        rules.push(YogaRule {
            name: "Vimala Yoga".to_string(),
            yoga_type: YogaType::Vimala,
            description:
                "Lord of 12th house in 12th house. Spiritual liberation, charitable nature."
                    .to_string(),
            condition: YogaCondition::VimalaYogaCheck,
        });

        // 15. Kala Sarpa Yoga
        rules.push(YogaRule {
            name: "Kala Sarpa Yoga".to_string(),
            yoga_type: YogaType::KalaSarpa,
            description: "All planets hemmed between Rahu and Ketu. Intense destiny.".to_string(),
            condition: YogaCondition::KalaSarpaCheck,
        });

        // 16. Adhi Yoga
        rules.push(YogaRule {
            name: "Chandra Adhi Yoga".to_string(),
            yoga_type: YogaType::Adhi,
            description: "Benefics in 6th, 7th, 8th from Moon. Leadership and fame.".to_string(),
            condition: YogaCondition::AdhiYogaCheck,
        });

        // 17. Vasumathi Yoga
        rules.push(YogaRule {
            name: "Vasumathi Yoga".to_string(),
            yoga_type: YogaType::Vasumathi,
            description: "Benefics in Upachaya houses (3, 6, 10, 11). Prosperity.".to_string(),
            condition: YogaCondition::VasumathiYogaCheck,
        });

        // 18. Sakata Yoga
        rules.push(YogaRule {
            name: "Sakata Yoga".to_string(),
            yoga_type: YogaType::Sakata,
            description: "Moon in 6th, 8th, or 12th from Jupiter. Financial ups and downs."
                .to_string(),
            condition: YogaCondition::SakataYogaCheck,
        });

        rules
    }

    /// Find and classify all Parivartana Yoga exchanges
    fn find_parivartana_exchanges(chart: &VedicChart) -> Vec<YogaResult> {
        let mut results = Vec::new();
        let mut processed = std::collections::HashSet::new();

        // Pre-build a map from planet to its position for O(1) lookups instead of
        // repeated O(n) linear scans inside the nested loop.
        let planet_map: std::collections::HashMap<_, _> =
            chart.planets.iter().map(|p| (p.planet, p)).collect();

        for h1 in 1..=12 {
            let lord1 = Self::get_lord_of_house(chart.ascendant.rasi, h1);
            if let Some(pos1) = planet_map.get(&lord1) {
                let h2 = pos1.house_index;
                if h1 != h2 {
                    let lord2 = Self::get_lord_of_house(chart.ascendant.rasi, h2);
                    if let Some(pos2) = planet_map.get(&lord2) {
                        if pos2.house_index == h1 && !processed.contains(&(h1.min(h2), h1.max(h2)))
                        {
                            // This is a valid exchange
                            processed.insert((h1.min(h2), h1.max(h2)));

                            let (yoga_type, quality_desc) = Self::classify_parivartana(h1, h2);
                            let planets = vec![lord1, lord2];
                            let quality = Self::assess_quality(chart, &planets);

                            let mut total_avastha = 0.0;
                            let mut valid_planets_count = 0;
                            for pl in &planets {
                                if let Some(pos) = chart.planets.iter().find(|p| p.planet == *pl) {
                                    let avastha = AvasthaEngine::calculate(pos, chart);
                                    total_avastha += avastha.score;
                                    valid_planets_count += 1;
                                }
                            }
                            let mut strength_percentage = if valid_planets_count > 0 {
                                total_avastha / valid_planets_count as f64
                            } else {
                                0.0
                            };
                            if let YogaQuality::Weak(_) = quality {
                                strength_percentage *= 0.5;
                            }

                            results.push(YogaResult {
                                name: format!("Parivartana Yoga: House {} ↔ House {}", h1, h2),
                                yoga_type,
                                description: format!(
                                    "{} ({} exchange)",
                                    quality_desc,
                                    if h1 == h2 { "same house" } else { "cross" }
                                ),
                                planets_involved: planets,
                                quality,
                                strength_percentage,
                            });
                        }
                    }
                }
            }
        }

        results
    }

    /// Classify Parivartana Yoga type based on house types (BPHS Standard)
    fn classify_parivartana(h1: u8, h2: u8) -> (YogaType, String) {
        let is_kendra = |h: u8| matches!(h, 1 | 4 | 7 | 10);
        let is_trikona = |h: u8| matches!(h, 1 | 5 | 9);
        let is_dusthana = |h: u8| matches!(h, 6 | 8 | 12);

        let d1 = is_dusthana(h1);
        let d2 = is_dusthana(h2);

        if d1 && d2 {
            // Both dusthana - Dainya (Difficult)
            (
                YogaType::ParivartanaDainya,
                "Dainya Parivartana - Exchange of difficult houses".to_string(),
            )
        } else if d1 || d2 {
            // One dusthana - Khala (Mixed)
            (
                YogaType::ParivartanaKhala,
                "Khala Parivartana - Mixed exchange with one difficult house".to_string(),
            )
        } else if (is_kendra(h1) || is_trikona(h1)) && (is_kendra(h2) || is_trikona(h2)) {
            // Both auspicious - Maha (Great)
            (
                YogaType::ParivartanaMaha,
                "Maha Parivartana - Great exchange of auspicious houses".to_string(),
            )
        } else {
            // Mixed but not involving dusthana - Khala
            (
                YogaType::ParivartanaKhala,
                "Khala Parivartana - Mixed exchange".to_string(),
            )
        }
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

                // BPHS Neecha Bhanga Raja Yoga Cancellation Rules:
                // 1. Lord of the sign where planet is debilitated is in Kendra from Lagna/Moon.
                // 2. Lord of the exaltation sign of the debilitated planet is in Kendra from Lagna/Moon.
                // 3. Exalted planet aspects the debilitated planet.
                // 4. Debilitated planet is conjunct with an exalted planet.
                // 5. Lord of debilitation sign is conjunct/aspecting debilitated planet.

                let mut cancellation_planets = vec![*planet];
                let dispositor = VedicPlanet::get_ruler_of(pos.rasi);

                // Get Moon position for Moon-based Kendra check
                let moon_pos = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon);

                // Rule 1a: Dispositor (lord of debilitation sign) in Kendra from Lagna
                if let Some(disp_pos) = chart.planets.iter().find(|p| p.planet == dispositor) {
                    if [1, 4, 7, 10].contains(&disp_pos.house_index) {
                        cancellation_planets.push(dispositor);
                        return Some(cancellation_planets);
                    }

                    // Rule 1b: Dispositor in Kendra from Moon (BPHS extension)
                    if let Some(moon) = moon_pos {
                        let moon_rasi = moon.rasi;
                        let disp_rasi = disp_pos.rasi;
                        let diff = ((disp_rasi as i32 - moon_rasi as i32 + 12) % 12) as u8;
                        // Kendra from Moon: 0 (1st), 3 (4th), 6 (7th), 9 (10th) houses
                        if [0, 3, 6, 9].contains(&diff) {
                            cancellation_planets.push(dispositor);
                            return Some(cancellation_planets);
                        }
                    }

                    // Rule 5: Dispositor conjunct with debilitated planet
                    if disp_pos.rasi == pos.rasi {
                        cancellation_planets.push(dispositor);
                        return Some(cancellation_planets);
                    }
                }

                // Rule 2a: Lord of exaltation sign in Kendra from Lagna
                let exalt_sign = planet.exaltation_rasi();
                let exalt_lord = VedicPlanet::get_ruler_of(exalt_sign);
                if let Some(exalt_lord_pos) = chart.planets.iter().find(|p| p.planet == exalt_lord)
                {
                    if [1, 4, 7, 10].contains(&exalt_lord_pos.house_index) {
                        cancellation_planets.push(exalt_lord);
                        return Some(cancellation_planets);
                    }

                    // Rule 2b: Lord of exaltation sign in Kendra from Moon (BPHS extension)
                    if let Some(moon) = moon_pos {
                        let moon_rasi = moon.rasi;
                        let exalt_lord_rasi = exalt_lord_pos.rasi;
                        let diff = ((exalt_lord_rasi as i32 - moon_rasi as i32 + 12) % 12) as u8;
                        if [0, 3, 6, 9].contains(&diff) {
                            cancellation_planets.push(exalt_lord);
                            return Some(cancellation_planets);
                        }
                    }
                }

                // Rule 3 & 4: Check for exalted planets aspecting or conjunct
                for other in &chart.planets {
                    if other.planet == *planet {
                        continue;
                    }

                    let other_exalt = other.planet.exaltation_rasi();
                    if other.rasi == other_exalt {
                        // This planet is exalted

                        // Check conjunction
                        if other.rasi == pos.rasi {
                            cancellation_planets.push(other.planet);
                            return Some(cancellation_planets);
                        }

                        // Check aspect (7th house opposition as basic aspect)
                        let diff = (other.rasi as i32 - pos.rasi as i32).abs();
                        if diff == 6 {
                            // 7th house aspect
                            cancellation_planets.push(other.planet);
                            return Some(cancellation_planets);
                        }
                    }
                }

                // No cancellation found
                None
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
            YogaCondition::KemadrumaCheck => {
                // Kemadruma Yoga: Moon with no planets in 2nd or 12th house from it
                // Exception: If Moon is in Kendra or aspected by benefics (simplified check)

                let moon_pos = chart
                    .planets
                    .iter()
                    .find(|p| p.planet == VedicPlanet::Moon)?;
                let moon_rasi = moon_pos.rasi;

                // Calculate 2nd and 12th houses from Moon
                let second_from_moon = ((moon_rasi % 12) + 1).max(1);
                let twelfth_from_moon = if moon_rasi == 1 { 12 } else { moon_rasi - 1 };

                // Check if any planet (except Sun/Moon) is in 2nd or 12th from Moon
                let has_flanking_planets = chart.planets.iter().any(|p| {
                    if p.planet == VedicPlanet::Moon || p.planet == VedicPlanet::Sun {
                        return false;
                    }
                    p.rasi == second_from_moon || p.rasi == twelfth_from_moon
                });

                // If no flanking planets, Kemadruma Yoga may exist
                if !has_flanking_planets {
                    // BPHS Cancellation conditions:
                    // 1. Moon in Kendra (1,4,7,10) from Lagna
                    let is_moon_in_kendra = [1, 4, 7, 10].contains(&moon_pos.house_index);

                    // 2. Planets in Kendra from Moon (BPHS extension)
                    let has_kendra_from_moon = chart.planets.iter().any(|p| {
                        if p.planet == VedicPlanet::Moon || p.planet == VedicPlanet::Sun {
                            return false;
                        }
                        let p_rasi = p.rasi;
                        let diff = ((p_rasi as i32 - moon_rasi as i32 + 12) % 12) as u8;
                        // Kendra from Moon: 0 (1st), 3 (4th), 6 (7th), 9 (10th) houses
                        [0, 3, 6, 9].contains(&diff)
                    });

                    if !is_moon_in_kendra && !has_kendra_from_moon {
                        Some(vec![VedicPlanet::Moon])
                    } else {
                        None // Cancelled by Kendra position or Kendra planets
                    }
                } else {
                    None
                }
            }
            YogaCondition::VipareetaRajaYogaCheck => {
                // Vipareeta Raja Yoga: Lord of 6th, 8th, or 12th in another dusthana
                //
                // BPHS 고도화 조건:
                // 1. 길성 7th 어스펙트가 없어야 함 (있으면 Yoga 희석)
                // 2. 쇠약(debilitated) 행성은 제외
                // 3. [신규] Shadbala total_score 기반 강도 게이팅:
                //    - score < 80  → 행성이 너무 약해 Yoga 발현 불가
                //    - 80 ≤ score < 180 → 약한 Yoga (YogaQuality 계산에 페널티)
                //    - score ≥ 180 → 표준 Yoga 강도
                let lagna_rasi = chart.ascendant.rasi;
                let mut vipareeta_planets: Vec<(VedicPlanet, f64)> = Vec::new(); // (planet, shadbala_score)

                for dusthana_house in [6u8, 8, 12] {
                    let lord = Self::get_lord_of_house(lagna_rasi, dusthana_house);

                    if let Some(lord_pos) = chart.planets.iter().find(|p| p.planet == lord) {
                        let lord_house = lord_pos.house_index;

                        if [6, 8, 12].contains(&lord_house) && lord_house != dusthana_house {
                            // 1. 길성 7th 어스펙트 확인
                            let has_benefic_aspect = chart.planets.iter().any(|p| {
                                let is_benefic = matches!(
                                    p.planet,
                                    VedicPlanet::Jupiter
                                        | VedicPlanet::Venus
                                        | VedicPlanet::Mercury
                                );
                                if !is_benefic || p.planet == lord {
                                    return false;
                                }
                                let diff =
                                    ((p.rasi as i32 - lord_pos.rasi as i32).abs() % 12) as u8;
                                diff == 6
                            });

                            // 2. 쇠약 확인
                            let is_debilitated = lord_pos.rasi == lord.debilitation_rasi();

                            // 3. 흉성 영향 (Yoga 순수성 강화)
                            let has_malefic_influence = chart.planets.iter().any(|p| {
                                let is_malefic = matches!(
                                    p.planet,
                                    VedicPlanet::Mars | VedicPlanet::Saturn | VedicPlanet::Sun
                                );
                                if !is_malefic || p.planet == lord {
                                    return false;
                                }
                                let diff =
                                    ((p.rasi as i32 - lord_pos.rasi as i32).abs() % 12) as u8;
                                diff == 0 || diff == 6
                            });

                            // 4. [BPHS 고도화] Shadbala 강도 산출
                            let shadbala = StrengthEngine::calculate(lord_pos, chart);
                            let strength_score = shadbala.total_score;

                            // 너무 약한 행성은 Yoga 불발 (BPHS: 쇠약하면 악도 약해 길로 전환 불가)
                            if strength_score < 80.0 {
                                continue;
                            }

                            // 쇠약+강한 어스펙트 → Yoga 제외
                            if has_benefic_aspect && is_debilitated {
                                continue;
                            }

                            // 조건 충족
                            if !has_benefic_aspect && !is_debilitated {
                                vipareeta_planets.push((lord, strength_score));
                            } else if has_malefic_influence && !is_debilitated {
                                vipareeta_planets.push((lord, strength_score));
                            }
                        }
                    }
                }

                if !vipareeta_planets.is_empty() {
                    // Shadbala score가 높은 행성부터 정렬 (강도 순)
                    vipareeta_planets
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    vipareeta_planets.dedup_by_key(|p| p.0);
                    Some(vipareeta_planets.into_iter().map(|(p, _)| p).collect())
                } else {
                    None
                }
            }
            YogaCondition::HarshaYogaCheck => {
                // Harsha Yoga: 6th lord in 6th house
                let lagna_rasi = chart.ascendant.rasi;
                let lord_6 = Self::get_lord_of_house(lagna_rasi, 6);

                if let Some(lord_pos) = chart.planets.iter().find(|p| p.planet == lord_6) {
                    if lord_pos.house_index == 6 {
                        Some(vec![lord_6])
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            YogaCondition::SaralaYogaCheck => {
                // Sarala Yoga: 8th lord in 8th house
                let lagna_rasi = chart.ascendant.rasi;
                let lord_8 = Self::get_lord_of_house(lagna_rasi, 8);

                if let Some(lord_pos) = chart.planets.iter().find(|p| p.planet == lord_8) {
                    if lord_pos.house_index == 8 {
                        Some(vec![lord_8])
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            YogaCondition::VimalaYogaCheck => {
                // Vimala Yoga: 12th lord in 12th house
                let lagna_rasi = chart.ascendant.rasi;
                let lord_12 = Self::get_lord_of_house(lagna_rasi, 12);

                if let Some(lord_pos) = chart.planets.iter().find(|p| p.planet == lord_12) {
                    if lord_pos.house_index == 12 {
                        Some(vec![lord_12])
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            YogaCondition::KalaSarpaCheck => {
                // All planets between Rahu and Ketu
                let rahu = chart
                    .planets
                    .iter()
                    .find(|p| p.planet == VedicPlanet::Rahu)?;
                let ketu = chart
                    .planets
                    .iter()
                    .find(|p| p.planet == VedicPlanet::Ketu)?;

                let mut arc1_count = 0;
                let mut arc2_count = 0;
                let planets_to_check = [
                    VedicPlanet::Sun,
                    VedicPlanet::Moon,
                    VedicPlanet::Mars,
                    VedicPlanet::Mercury,
                    VedicPlanet::Jupiter,
                    VedicPlanet::Venus,
                    VedicPlanet::Saturn,
                ];

                for &pl in &planets_to_check {
                    let pos = chart.planets.iter().find(|p| p.planet == pl)?;
                    let deg = pos.sidereal_deg;
                    let start = rahu.sidereal_deg;
                    let end = ketu.sidereal_deg;

                    // Normalize to arc starting from Rahu
                    let normalized_deg = (deg - start + 360.0) % 360.0;
                    let normalized_end = (end - start + 360.0) % 360.0;

                    if normalized_deg <= normalized_end {
                        arc1_count += 1;
                    } else {
                        arc2_count += 1;
                    }
                }

                if arc1_count == 7 || arc2_count == 7 {
                    Some(vec![VedicPlanet::Rahu, VedicPlanet::Ketu])
                } else {
                    None
                }
            }
            YogaCondition::AdhiYogaCheck => {
                // Benefics (Mer, Jup, Ven) in 6, 7, 8 from Moon
                let moon = chart
                    .planets
                    .iter()
                    .find(|p| p.planet == VedicPlanet::Moon)?;
                let moon_house = moon.house_index;

                let mut involved = Vec::new();
                let benefics = [
                    VedicPlanet::Mercury,
                    VedicPlanet::Jupiter,
                    VedicPlanet::Venus,
                ];

                for &pl in &benefics {
                    let pos = chart.planets.iter().find(|p| p.planet == pl)?;
                    let rel_house = (pos.house_index as i32 - moon_house as i32 + 12) % 12 + 1;
                    if [6, 7, 8].contains(&(rel_house as u8)) {
                        involved.push(pl);
                    }
                }

                if !involved.is_empty() {
                    Some(involved)
                } else {
                    None
                }
            }
            YogaCondition::VasumathiYogaCheck => {
                // Benefics in Upachaya (3, 6, 10, 11) from Lagna (and/or Moon)
                let mut involved = Vec::new();
                let benefics = [
                    VedicPlanet::Mercury,
                    VedicPlanet::Jupiter,
                    VedicPlanet::Venus,
                ];

                for &pl in &benefics {
                    let pos = chart.planets.iter().find(|p| p.planet == pl)?;
                    if [3, 6, 10, 11].contains(&pos.house_index) {
                        involved.push(pl);
                    }
                }

                if !involved.is_empty() {
                    Some(involved)
                } else {
                    None
                }
            }
            YogaCondition::SakataYogaCheck => {
                // Moon in 6, 8, 12 from Jupiter
                let jupiter = chart
                    .planets
                    .iter()
                    .find(|p| p.planet == VedicPlanet::Jupiter)?;
                let moon = chart
                    .planets
                    .iter()
                    .find(|p| p.planet == VedicPlanet::Moon)?;

                let rel_house =
                    (moon.house_index as i32 - jupiter.house_index as i32 + 12) % 12 + 1;
                if [6, 8, 12].contains(&(rel_house as u8)) {
                    Some(vec![VedicPlanet::Moon, VedicPlanet::Jupiter])
                } else {
                    None
                }
            }

            YogaCondition::NabhasaCheck => None, // Handled separately via evaluate_nabhasa_yogas
            _ => None,
        }
    }

    // ── Nabhasa Yoga Engine (BPHS) ─────────────────────────────────────────────
    //
    // Nabhasa Yogas are based on the DISTRIBUTION of the 9 planets (Sun~Ketu)
    // across the 12 rasis. Ascendant is excluded from counting.
    // Two families:
    //   Sankhya: determined by the COUNT of unique rasis occupied.
    //   Akriti:  determined by the QUALITY (Chara/Sthira/Dvisvabhava) of all occupied rasis.
    //
    // BPHS precedence rule:
    //   If an Akriti yoga is present AND all rasis share the same quality,
    //   the Akriti yoga is reported alongside the Sankhya yoga.
    // ─────────────────────────────────────────────────────────────────────────

    /// Returns the unique rasi numbers occupied by the 9 planets (Ascendant excluded).
    fn occupied_rasis(chart: &VedicChart) -> std::collections::HashSet<u8> {
        use std::collections::HashSet;
        chart
            .planets
            .iter()
            .filter(|p| {
                // Exclude Ascendant pseudo-planet (planet == Ketu is the mock ascendant sentinel
                // in tests, but real charts use VedicPlanet::Ascendant).
                p.planet != VedicPlanet::Ascendant
            })
            .map(|p| p.rasi)
            .collect::<HashSet<u8>>()
    }

    /// Returns true when ALL 9 non-Ascendant planets fall within rasis of the given quality.
    /// quality: 0 = Chara (0-indexed rasi % 3 == 0),
    ///          1 = Sthira (rasi % 3 == 1),
    ///          2 = Dvisvabhava (rasi % 3 == 2).
    /// Rasi numbering: 1-Aries, 2-Taurus … 12-Pisces.
    /// Chara  (Cardinal): Aries(1), Cancer(4), Libra(7), Capricorn(10)  → (rasi-1) % 3 == 0
    /// Sthira (Fixed):    Taurus(2), Leo(5), Scorpio(8), Aquarius(11)   → (rasi-1) % 3 == 1
    /// Dvísva (Mutable):  Gemini(3), Virgo(6), Sagittarius(9), Pisces(12)→ (rasi-1) % 3 == 2
    fn all_planets_in_sign_quality(chart: &VedicChart, quality: u8) -> bool {
        chart
            .planets
            .iter()
            .filter(|p| p.planet != VedicPlanet::Ascendant)
            .all(|p| (p.rasi - 1) % 3 == quality)
    }

    /// Evaluate all Nabhasa Yogas and return the matching results.
    fn evaluate_nabhasa_yogas(chart: &VedicChart) -> Vec<YogaResult> {
        let mut results = Vec::new();

        // ── Sankhya Yoga ──────────────────────────────────────────────────────
        let rasi_count = Self::occupied_rasis(chart).len();

        // Collect all 9-planet list for planets_involved
        let all_planets: Vec<VedicPlanet> = chart
            .planets
            .iter()
            .filter(|p| p.planet != VedicPlanet::Ascendant)
            .map(|p| p.planet)
            .collect();

        let (sankhya_type, sankhya_name, sankhya_desc, sankhya_quality) = match rasi_count {
            1 => (
                Some(YogaType::NabhasaGola),
                "Gola Yoga",
                "All 9 planets in a single sign. Extreme single-directedness; highly rare. Life shaped by one dominant energy.",
                YogaQuality::High,
            ),
            2 => (
                Some(YogaType::NabhasaYuga),
                "Yuga Yoga",
                "All planets concentrated in 2 signs. Intense duality; life of extreme contrasts and sharp transitions.",
                YogaQuality::Medium,
            ),
            3 => (
                Some(YogaType::NabhasaShoola),
                "Shoola Yoga",
                "Planets span 3 signs. Trident pattern — challenges forge resilience; cycles of hardship and renewal.",
                YogaQuality::Medium,
            ),
            4 => (
                Some(YogaType::NabhasaKedara),
                "Kedara Yoga",
                "Planets in 4 signs. Diligent, agricultural temperament; patience, steadfastness, material grounding.",
                YogaQuality::High,
            ),
            5 => (
                Some(YogaType::NabhasaPasha),
                "Pasha Yoga",
                "Planets in 5 signs. Strong bonds and attachments; focus, loyalty, and sometimes obsession.",
                YogaQuality::Medium,
            ),
            6 => (
                Some(YogaType::NabhasaDaama),
                "Daama Yoga",
                "Planets in 6 signs. Generous leadership; broad influence, charity, and social responsibility.",
                YogaQuality::High,
            ),
            7 => (
                Some(YogaType::NabhasaVeena),
                "Veena Yoga",
                "Planets in 7 signs. Artistic, musical, and harmonious nature; balanced distribution of energy across life domains.",
                YogaQuality::VeryHigh,
            ),
            _ => (None, "", "", YogaQuality::Medium),
        };

        let mut total_avastha = 0.0;
        let mut valid_planets_count = 0;
        for pl in &all_planets {
            if let Some(pos) = chart.planets.iter().find(|p| p.planet == *pl) {
                let avastha = AvasthaEngine::calculate(pos, chart);
                total_avastha += avastha.score;
                valid_planets_count += 1;
            }
        }
        let strength_percentage = if valid_planets_count > 0 {
            total_avastha / valid_planets_count as f64
        } else {
            0.0
        };

        if let Some(yoga_type) = sankhya_type {
            let mut sankhya_strength = strength_percentage;
            if let YogaQuality::Weak(_) = sankhya_quality {
                sankhya_strength *= 0.5;
            }
            results.push(YogaResult {
                name: sankhya_name.to_string(),
                yoga_type,
                description: sankhya_desc.to_string(),
                planets_involved: all_planets.clone(),
                quality: sankhya_quality,
                strength_percentage: sankhya_strength,
            });
        }

        // ── Akriti Yoga ───────────────────────────────────────────────────────
        // Only evaluated when ALL planets fall within rasis of the same quality.
        if Self::all_planets_in_sign_quality(chart, 0) {
            results.push(YogaResult {
                name: "Ashrita Yoga".to_string(),
                yoga_type: YogaType::NabhasaAshrita,
                description: "All planets in Chara (Cardinal) signs. Restless, adaptable nature; perpetual movement and change.".to_string(),
                planets_involved: all_planets.clone(),
                quality: YogaQuality::High,
                strength_percentage,
            });
        } else if Self::all_planets_in_sign_quality(chart, 1) {
            results.push(YogaResult {
                name: "Sthira Yoga".to_string(),
                yoga_type: YogaType::NabhasaSthira,
                description: "All planets in Sthira (Fixed) signs. Stable, determined, and persistent; builds enduring structures.".to_string(),
                planets_involved: all_planets.clone(),
                quality: YogaQuality::High,
                strength_percentage,
            });
        } else if Self::all_planets_in_sign_quality(chart, 2) {
            results.push(YogaResult {
                name: "Dvisvabhava Yoga".to_string(),
                yoga_type: YogaType::NabhasaDvisvabhava,
                description: "All planets in Dvisvabhava (Mutable) signs. Dual nature, versatility, and intellectual flexibility.".to_string(),
                planets_involved: all_planets,
                quality: YogaQuality::High,
                strength_percentage,
            });
        }

        results
    }

    fn get_lord_of_house(lagna_rasi: u8, house: u8) -> VedicPlanet {
        let rasi_idx = ((lagna_rasi as i32 + house as i32 - 1 - 1) % 12 + 1) as u8;
        VedicPlanet::get_ruler_of(rasi_idx)
    }

    // Updated assess_quality to pass 'chart' to FunctionalNature::analyze
    fn assess_quality(chart: &VedicChart, planets: &[VedicPlanet]) -> YogaQuality {
        let mut score = 0;
        let mut reasons = Vec::new();
        let mut is_combust = false;

        for &pl in planets {
            if let Some(pos) = chart.planets.iter().find(|p| p.planet == pl) {
                if pos.is_combust {
                    is_combust = true;
                    reasons.push(format!("{:?} is Combust", pl));
                }

                // CHANGED: Passing 'chart' instead of just 'rasi'
                match FunctionalNature::analyze(chart, pl) {
                    FunctionalStatus::Yogakaraka => {
                        score += 3; // Powerful boost
                    }
                    FunctionalStatus::FunctionalBenefic => {
                        score += 1;
                    }
                    FunctionalStatus::FunctionalMalefic => {
                        score -= 2; // Significant penalty
                        reasons.push(format!("{:?} is Functional Malefic", pl));
                    }
                    FunctionalStatus::Maraka => {
                        score -= 1;
                        reasons.push(format!("{:?} is Maraka", pl));
                    }
                    FunctionalStatus::Neutral => {}
                }
            }
        }

        if is_combust {
            return YogaQuality::Weak(format!("Combustion: {}", reasons.join(", ")));
        }

        match score {
            s if s >= 3 => YogaQuality::VeryHigh,
            s if s >= 1 => YogaQuality::High,
            s if s >= -1 => YogaQuality::Medium,
            _ => YogaQuality::Weak(if reasons.is_empty() {
                "Malefic Influence".to_string()
            } else {
                reasons.join(", ")
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::panchanga::Panchanga;
    use crate::chart::VedicPosition;

    fn mock_chart(planets: Vec<VedicPosition>) -> VedicChart {
        VedicChart {
            ascendant: mock_pos(VedicPlanet::Ketu, 1, 1),
            planets,
            karakas: vec![],
            arudha_padas: vec![],
            special_lagnas: vec![],
            house_cusps: vec![],
            bhava_strengths: vec![],
            vimshopaka_scores: vec![],
            aspects: vec![],
            sav: crate::analysis::ashtakavarga::Sarvashtakavarga { points: [0; 12] },
            bav: vec![],
            avasthas: vec![],
            shadbalas: vec![],
            analysis_report: None,
            panchanga: Panchanga {
                vara: "Sunday".to_string(),
                tithi: 1,
                tithi_name: "Pratipada".to_string(),
                nakshatra: 1,
                karana: 1,
                karana_name: "Bava".to_string(),
                yoga: 1,
                day_lord: VedicPlanet::Sun,
                hour_lord: VedicPlanet::Sun,
                sunrise: chrono::DateTime::from_timestamp(21600, 0)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                sunset: chrono::DateTime::from_timestamp(64800, 0)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                next_sunrise: chrono::DateTime::from_timestamp(108000, 0)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                is_day_birth: true,
                is_night_birth: false,
                daily_parts: [VedicPlanet::Sun; 8],
                current_time: chrono::DateTime::from_timestamp(43200, 0)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                yogi_point: 0.0,
                yogi_planet: VedicPlanet::Sun,
                avayogi_planet: VedicPlanet::Saturn,
                dagdha_rashis: vec![],
                rahu_kalam: (
                    chrono::DateTime::from_timestamp(21600, 0)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    chrono::DateTime::from_timestamp(21600, 0)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                ),
                yamaganda: (
                    chrono::DateTime::from_timestamp(21600, 0)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    chrono::DateTime::from_timestamp(21600, 0)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                ),
                gulika: (
                    chrono::DateTime::from_timestamp(21600, 0)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    chrono::DateTime::from_timestamp(21600, 0)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                ),
            },
            ayanamsa: 23.0,
        }
    }

    fn mock_pos(planet: VedicPlanet, rasi: u8, house: u8) -> VedicPosition {
        VedicPosition {
            planet,
            tropical_deg: (rasi as f64 - 1.0) * 30.0 + 15.0,
            sidereal_deg: (rasi as f64 - 1.0) * 30.0 + 15.0,
            nakshatra: 1,
            pada: 1,
            rasi,
            house_index: house,
            speed: 1.0,
            is_retrograde: false,
            is_combust: false,
            declination: 0.0,
            hora_rasi: rasi,
            drekkana_rasi: rasi,
            chaturthamsha_rasi: rasi,
            panchamsa_rasi: rasi,
            saptamsa_rasi: rasi,
            ashtamsa_rasi: rasi,
            navamsa_rasi: rasi,
            dasamsa_rasi: rasi,
            shashtamsa_rasi: rasi,
            rudramsa_rasi: rasi,
            dwadasamsa_rasi: rasi,
            shodashamsa_rasi: rasi,
            vimsamsa_rasi: rasi,
            chaturvimshamsa_rasi: rasi,
            saptavimsamsa_rasi: rasi,
            trimsamsa_rasi: rasi,
            khavedamsa_rasi: rasi,
            akshavedamsa_rasi: rasi,
            shashtyamsa_rasi: rasi,
            navanavamsa_rasi: rasi,
            ashtottaramsa_rasi: rasi,
            dwadasdwadasamsa_rasi: rasi,
        }
    }

    #[test]
    fn test_neecha_bhanga_moon_kendra() {
        // Mars debilitated in Cancer (Rasi 4), which is house 4 for Aries ascendant
        // dispositor is Moon. Exaltation lord is Saturn.
        // If Moon is in Kendra from Ascendant (e.g. 7th house, Libra), cancellation occurs.
        let mars = mock_pos(VedicPlanet::Mars, 4, 4);
        let moon = mock_pos(VedicPlanet::Moon, 7, 7); // Moon in Kendra from lagna

        let chart = mock_chart(vec![mars, moon]);

        let yogas = YogaEngine::check_yogas(&chart);
        let nb_yoga = yogas.iter().find(|y| y.yoga_type == YogaType::NeechaBhanga);
        assert!(
            nb_yoga.is_some(),
            "Neecha Bhanga should be detected due to Dispositor Moon in Kendra"
        );

        let nb_yoga = nb_yoga.unwrap();
        assert!(nb_yoga.planets_involved.contains(&VedicPlanet::Mars));
        assert!(nb_yoga.planets_involved.contains(&VedicPlanet::Moon));
    }

    #[test]
    fn test_vipareeta_raja_yoga() {
        // Aries Lagna.
        // Debilitated planet usually doesn't give a strong Vipareeta Raja Yoga according to our refinement.
        // Let's use 8th lord instead. 8th lord is Mars (Scorpio). Mars in 12th (Pisces).
        let mars = mock_pos(VedicPlanet::Mars, 12, 12);

        let chart = mock_chart(vec![mars]);
        let yogas = YogaEngine::check_yogas(&chart);
        let vr_yoga = yogas
            .iter()
            .find(|y| y.yoga_type == YogaType::VipareetaRajaYoga);
        assert!(
            vr_yoga.is_some(),
            "Vipareeta Raja Yoga should be detected for 8th lord in 12th house"
        );

        let vr_yoga = vr_yoga.unwrap();
        assert!(vr_yoga.planets_involved.contains(&VedicPlanet::Mars));
    }

    // ── Nabhasa Yoga Tests ────────────────────────────────────────────────────

    #[test]
    fn test_nabhasa_veena_yoga_seven_rasi() {
        // 9 planets spread across 7 distinct rasis → Veena Yoga
        let planets = vec![
            mock_pos(VedicPlanet::Sun, 1, 1),
            mock_pos(VedicPlanet::Moon, 2, 2),
            mock_pos(VedicPlanet::Mars, 3, 3),
            mock_pos(VedicPlanet::Mercury, 4, 4),
            mock_pos(VedicPlanet::Jupiter, 5, 5),
            mock_pos(VedicPlanet::Venus, 6, 6),
            mock_pos(VedicPlanet::Saturn, 7, 7),
            mock_pos(VedicPlanet::Rahu, 7, 7), // same rasi as Saturn
            mock_pos(VedicPlanet::Ketu, 1, 1), // same rasi as Sun
        ];
        // Occupied rasis: {1,2,3,4,5,6,7} = 7 → Veena
        let chart = mock_chart(planets);
        let yogas = YogaEngine::check_yogas(&chart);
        assert!(
            yogas.iter().any(|y| y.yoga_type == YogaType::NabhasaVeena),
            "Veena Yoga should be detected when 9 planets are in 7 rasis"
        );
    }

    #[test]
    fn test_nabhasa_gola_yoga_one_rasi() {
        // All 9 planets in rasi 5 (Leo) → Gola Yoga
        let planets = vec![
            mock_pos(VedicPlanet::Sun, 5, 5),
            mock_pos(VedicPlanet::Moon, 5, 5),
            mock_pos(VedicPlanet::Mars, 5, 5),
            mock_pos(VedicPlanet::Mercury, 5, 5),
            mock_pos(VedicPlanet::Jupiter, 5, 5),
            mock_pos(VedicPlanet::Venus, 5, 5),
            mock_pos(VedicPlanet::Saturn, 5, 5),
            mock_pos(VedicPlanet::Rahu, 5, 5),
            mock_pos(VedicPlanet::Ketu, 5, 5),
        ];
        let chart = mock_chart(planets);
        let yogas = YogaEngine::check_yogas(&chart);
        assert!(
            yogas.iter().any(|y| y.yoga_type == YogaType::NabhasaGola),
            "Gola Yoga should be detected when all planets are in a single rasi"
        );
    }

    #[test]
    fn test_nabhasa_ashrita_yoga_all_chara() {
        // All planets in Chara (Cardinal) signs: Aries(1), Cancer(4), Libra(7), Capricorn(10)
        // (rasi - 1) % 3 == 0 for all
        let planets = vec![
            mock_pos(VedicPlanet::Sun, 1, 1),       // Aries
            mock_pos(VedicPlanet::Moon, 4, 4),      // Cancer
            mock_pos(VedicPlanet::Mars, 7, 7),      // Libra
            mock_pos(VedicPlanet::Mercury, 10, 10), // Capricorn
            mock_pos(VedicPlanet::Jupiter, 1, 1),
            mock_pos(VedicPlanet::Venus, 4, 4),
            mock_pos(VedicPlanet::Saturn, 7, 7),
            mock_pos(VedicPlanet::Rahu, 10, 10),
            mock_pos(VedicPlanet::Ketu, 1, 1),
        ];
        let chart = mock_chart(planets);
        let yogas = YogaEngine::check_yogas(&chart);
        assert!(
            yogas
                .iter()
                .any(|y| y.yoga_type == YogaType::NabhasaAshrita),
            "Ashrita Yoga should be detected when all planets are in Chara signs"
        );
    }

    #[test]
    fn test_nabhasa_none_if_mixed_signs() {
        // Planets in both Chara and Sthira signs → no Akriti Yoga
        let planets = vec![
            mock_pos(VedicPlanet::Sun, 1, 1),  // Aries (Chara)
            mock_pos(VedicPlanet::Moon, 2, 2), // Taurus (Sthira)
            mock_pos(VedicPlanet::Mars, 3, 3), // Gemini (Dvisvabhava)
            mock_pos(VedicPlanet::Mercury, 4, 4),
            mock_pos(VedicPlanet::Jupiter, 5, 5),
            mock_pos(VedicPlanet::Venus, 6, 6),
            mock_pos(VedicPlanet::Saturn, 7, 7),
            mock_pos(VedicPlanet::Rahu, 8, 8),
            mock_pos(VedicPlanet::Ketu, 9, 9),
        ];
        let chart = mock_chart(planets);
        let yogas = YogaEngine::check_yogas(&chart);
        let has_akriti = yogas.iter().any(|y| {
            matches!(
                y.yoga_type,
                YogaType::NabhasaAshrita | YogaType::NabhasaSthira | YogaType::NabhasaDvisvabhava
            )
        });
        assert!(
            !has_akriti,
            "No Akriti Yoga when planets span mixed sign qualities"
        );
        // But Sankhya (9 rasis) should produce no Sankhya yoga (only 1-7 generate one)
        let has_sankhya = yogas.iter().any(|y| {
            matches!(
                y.yoga_type,
                YogaType::NabhasaGola
                    | YogaType::NabhasaYuga
                    | YogaType::NabhasaShoola
                    | YogaType::NabhasaKedara
                    | YogaType::NabhasaPasha
                    | YogaType::NabhasaDaama
                    | YogaType::NabhasaVeena
            )
        });
        assert!(
            !has_sankhya,
            "No Sankhya Yoga when planets are spread across all 9 rasis"
        );
    }
}
