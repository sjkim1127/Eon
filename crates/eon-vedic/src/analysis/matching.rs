use crate::core::chart::VedicChart;
use crate::planets::VedicPlanet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KootaScore {
    pub id: String,
    pub name: String,
    pub max_points: f64,
    pub earned_points: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompatibilityReport {
    pub total_score: f64,    // out of 36
    pub is_compatible: bool, // total >= 18 and no critical Nadi/Bhakoot dosha
    pub kootas: Vec<KootaScore>,
    pub male_mangal_dosha: bool,
    pub female_mangal_dosha: bool,
    pub mangal_dosha_cancelled: bool,
    pub explanation: String,
}

pub struct MatchingEngine;

impl MatchingEngine {
    pub fn calculate_compatibility(male: &VedicChart, female: &VedicChart) -> CompatibilityReport {
        let male_moon = male
            .planets
            .iter()
            .find(|p| p.planet == VedicPlanet::Moon)
            .cloned()
            .unwrap();
        let female_moon = female
            .planets
            .iter()
            .find(|p| p.planet == VedicPlanet::Moon)
            .cloned()
            .unwrap();

        // 1. Varna (1 Guna)
        let varna_score = calculate_varna(male_moon.rasi, female_moon.rasi);

        // 2. Vashya (2 Gunas)
        let vashya_score = calculate_vashya(male_moon.rasi, female_moon.rasi);

        // 3. Tara (3 Gunas)
        let tara_score = calculate_tara(male_moon.nakshatra, female_moon.nakshatra);

        // 4. Yoni (4 Gunas)
        let yoni_score = calculate_yoni(male_moon.nakshatra, female_moon.nakshatra);

        // 5. Graha Maitri (5 Gunas)
        let graha_maitri_score = calculate_graha_maitri(male_moon.rasi, female_moon.rasi);

        // 6. Gana (6 Gunas)
        let gana_score = calculate_gana(male_moon.nakshatra, female_moon.nakshatra);

        // 7. Bhakoot (7 Gunas)
        let bhakoot_score = calculate_bhakoot(male_moon.rasi, female_moon.rasi);

        // 8. Nadi (8 Gunas)
        let nadi_score = calculate_nadi(male_moon.nakshatra, female_moon.nakshatra);

        let total = varna_score
            + vashya_score
            + tara_score
            + yoni_score
            + graha_maitri_score
            + gana_score
            + bhakoot_score
            + nadi_score;

        let kootas = vec![
            KootaScore {
                id: "varna".to_string(),
                name: "Varna (Caste/Vocation)".to_string(),
                max_points: 1.0,
                earned_points: varna_score,
                description: if varna_score == 1.0 {
                    "Good work-profile alignment.".to_string()
                } else {
                    "Differing natural vocations.".to_string()
                },
            },
            KootaScore {
                id: "vashya".to_string(),
                name: "Vashya (Control/Attraction)".to_string(),
                max_points: 2.0,
                earned_points: vashya_score,
                description: format!("Mutual attraction rating: {}/2.", vashya_score),
            },
            KootaScore {
                id: "tara".to_string(),
                name: "Tara (Destiny/Health)".to_string(),
                max_points: 3.0,
                earned_points: tara_score,
                description: if tara_score == 3.0 {
                    "Excellent destiny and longevity alignment.".to_string()
                } else if tara_score == 1.5 {
                    "Moderate health compatibility.".to_string()
                } else {
                    "Challenging health compatibility (Tara Dosha).".to_string()
                },
            },
            KootaScore {
                id: "yoni".to_string(),
                name: "Yoni (Sensory/Sexual)".to_string(),
                max_points: 4.0,
                earned_points: yoni_score,
                description: format!("Biological compatibility: {}/4.", yoni_score),
            },
            KootaScore {
                id: "graha_maitri".to_string(),
                name: "Graha Maitri (Friendship)".to_string(),
                max_points: 5.0,
                earned_points: graha_maitri_score,
                description: if graha_maitri_score >= 4.0 {
                    "High mental harmony and friendship.".to_string()
                } else if graha_maitri_score >= 2.0 {
                    "Average mental harmony.".to_string()
                } else {
                    "Potential communication gaps.".to_string()
                },
            },
            KootaScore {
                id: "gana".to_string(),
                name: "Gana (Temperament)".to_string(),
                max_points: 6.0,
                earned_points: gana_score,
                description: if gana_score >= 5.0 {
                    "Compatible temperaments.".to_string()
                } else if gana_score >= 3.0 {
                    "Moderate temperament gaps.".to_string()
                } else {
                    "High temperamental friction (Gana Dosha).".to_string()
                },
            },
            KootaScore {
                id: "bhakoot".to_string(),
                name: "Bhakoot (Emotional Node)".to_string(),
                max_points: 7.0,
                earned_points: bhakoot_score,
                description: if bhakoot_score == 7.0 {
                    "Stable emotional bond.".to_string()
                } else {
                    "Challenging emotional/financial cycles (Bhakoot Dosha).".to_string()
                },
            },
            KootaScore {
                id: "nadi".to_string(),
                name: "Nadi (Health/Genetics)".to_string(),
                max_points: 8.0,
                earned_points: nadi_score,
                description: if nadi_score == 8.0 {
                    "Good genetic health & lineage compatibility.".to_string()
                } else {
                    "Excessive similar energy (Nadi Dosha). Possible genetic mismatches."
                        .to_string()
                },
            },
        ];

        // Mangal Dosha
        let male_mangal = check_mangal_dosha(male);
        let female_mangal = check_mangal_dosha(female);
        let mangal_dosha_cancelled = male_mangal && female_mangal; // Dosha Samya: Both having it cancels the negative effect

        let has_critical_dosha = (nadi_score == 0.0) || (bhakoot_score == 0.0);
        let is_compatible = total >= 18.0 && (!has_critical_dosha || mangal_dosha_cancelled);

        let explanation = format!(
            "총 {}점 획득 (36점 만점). {}",
            total,
            if is_compatible {
                "전반적으로 조화로운 매칭입니다. 추천합니다."
            } else if total >= 18.0 {
                "점수는 높으나 주요 살(Nadi/Bhakoot Dosha)의 영향으로 신중한 주의가 필요합니다."
            } else {
                "성향적 차이가 커 상호 조율과 깊은 이해가 요구되는 상성입니다."
            }
        );

        CompatibilityReport {
            total_score: total,
            is_compatible,
            kootas,
            male_mangal_dosha: male_mangal,
            female_mangal_dosha: female_mangal,
            mangal_dosha_cancelled,
            explanation,
        }
    }
}

// ── Koota Helpers ────────────────────────────────────────────────────────────

fn get_varna_caste(rasi: u8) -> u8 {
    // 4=Brahmin, 3=Kshatriya, 2=Vaishya, 1=Shudra
    match rasi {
        4 | 8 | 12 => 4, // Cancer, Scorpio, Pisces
        1 | 5 | 9 => 3,  // Aries, Leo, Sagittarius
        2 | 6 | 10 => 2, // Taurus, Virgo, Capricorn
        3 | 7 | 11 => 1, // Gemini, Libra, Aquarius
        _ => 1,
    }
}

fn calculate_varna(male_rasi: u8, female_rasi: u8) -> f64 {
    let male_caste = get_varna_caste(male_rasi);
    let female_caste = get_varna_caste(female_rasi);
    if male_caste >= female_caste {
        1.0
    } else {
        0.0
    }
}

fn get_vashya_type(rasi: u8) -> &'static str {
    match rasi {
        1 | 2 => "Chatushpada",
        9 => "Manushya",   // 1st half Manushya, simplified
        10 => "Jalachara", // 2nd half Jalachara, simplified
        3 | 6 | 7 | 11 => "Manushya",
        4 | 12 => "Jalachara",
        5 => "Vanachara",
        8 => "Keeta",
        _ => "Manushya",
    }
}

fn calculate_vashya(male_rasi: u8, female_rasi: u8) -> f64 {
    let m_type = get_vashya_type(male_rasi);
    let f_type = get_vashya_type(female_rasi);
    if m_type == f_type {
        return 2.0;
    }
    match (m_type, f_type) {
        ("Manushya", "Chatushpada") => 1.0,
        ("Manushya", "Jalachara") => 0.5,
        ("Chatushpada", "Jalachara") => 1.0,
        _ => 0.0,
    }
}

fn calculate_tara(male_nak: u8, female_nak: u8) -> f64 {
    let dist_f_to_m = ((male_nak as i16 - female_nak as i16 + 27) % 9) + 1;
    let dist_m_to_f = ((female_nak as i16 - male_nak as i16 + 27) % 9) + 1;

    let inauspicious = [3, 5, 7]; // Vipat, Pratyak, Naidhana
    let m_ok = !inauspicious.contains(&dist_f_to_m);
    let f_ok = !inauspicious.contains(&dist_m_to_f);

    if m_ok && f_ok {
        3.0
    } else if m_ok || f_ok {
        1.5
    } else {
        0.0
    }
}

enum YoniAnimal {
    Horse,
    Elephant,
    Sheep,
    Serpent,
    Dog,
    Cat,
    Rat,
    Cow,
    Buffalo,
    Tiger,
    Deer,
    Monkey,
    Lion,
    Mongoose,
}

fn get_nak_yoni(nak: u8) -> YoniAnimal {
    match nak {
        1 | 24 => YoniAnimal::Horse,
        2 | 27 => YoniAnimal::Elephant,
        3 | 8 => YoniAnimal::Sheep,
        4 | 5 => YoniAnimal::Serpent,
        6 | 19 => YoniAnimal::Dog,
        7 | 9 => YoniAnimal::Cat,
        10 | 11 => YoniAnimal::Rat,
        12 | 26 => YoniAnimal::Cow,
        13 | 15 => YoniAnimal::Buffalo,
        14 | 16 => YoniAnimal::Tiger,
        17 | 18 => YoniAnimal::Deer,
        20 | 22 => YoniAnimal::Monkey,
        23 | 25 => YoniAnimal::Lion,
        21 | _ => YoniAnimal::Mongoose,
    }
}

fn calculate_yoni(male_nak: u8, female_nak: u8) -> f64 {
    let m_yoni = get_nak_yoni(male_nak);
    let f_yoni = get_nak_yoni(female_nak);

    use YoniAnimal::*;
    let is_enemy = match (&m_yoni, &f_yoni) {
        (Horse, Buffalo) | (Buffalo, Horse) => true,
        (Elephant, Lion) | (Lion, Elephant) => true,
        (Sheep, Monkey) | (Monkey, Sheep) => true,
        (Serpent, Mongoose) | (Mongoose, Serpent) => true,
        (Dog, Deer) | (Deer, Dog) => true,
        (Cat, Rat) | (Rat, Cat) => true,
        (Cow, Tiger) | (Tiger, Cow) => true,
        _ => false,
    };

    if is_enemy {
        return 0.0;
    }

    // Same animal type = 4. Otherwise we simplify friendly vs neutral
    let m_val = m_yoni as u8;
    let f_val = f_yoni as u8;
    if m_val == f_val {
        4.0
    } else if (m_val + f_val).is_multiple_of(2) {
        3.0 // Friendly
    } else {
        2.0 // Neutral
    }
}

fn calculate_graha_maitri(male_rasi: u8, female_rasi: u8) -> f64 {
    use crate::analysis::relationships::RelationshipEngine;

    let m_lord = get_sign_lord(male_rasi);
    let f_lord = get_sign_lord(female_rasi);

    if m_lord == f_lord {
        return 5.0;
    }

    let rel_m = RelationshipEngine::get_natural_relationship(m_lord, f_lord);
    let rel_f = RelationshipEngine::get_natural_relationship(f_lord, m_lord);

    use crate::analysis::relationships::NaturalRelationship::*;
    match (rel_m, rel_f) {
        (Friend, Friend) => 5.0,
        (Friend, Neutral) | (Neutral, Friend) => 4.0,
        (Neutral, Neutral) => 3.0,
        (Friend, Enemy) | (Enemy, Friend) => 2.0,
        (Neutral, Enemy) | (Enemy, Neutral) => 1.0,
        (Enemy, Enemy) => 0.0,
    }
}

fn get_sign_lord(rasi: u8) -> VedicPlanet {
    match rasi {
        1 | 8 => VedicPlanet::Mars,
        2 | 7 => VedicPlanet::Venus,
        3 | 6 => VedicPlanet::Mercury,
        4 => VedicPlanet::Moon,
        5 => VedicPlanet::Sun,
        9 | 12 => VedicPlanet::Jupiter,
        10 | 11 => VedicPlanet::Saturn,
        _ => VedicPlanet::Sun,
    }
}

enum GanaType {
    Deva,
    Manushya,
    Rakshasa,
}

fn get_nak_gana(nak: u8) -> GanaType {
    match nak {
        1 | 5 | 7 | 8 | 13 | 15 | 17 | 22 | 27 => GanaType::Deva,
        2 | 4 | 6 | 11 | 12 | 20 | 21 | 25 | 26 => GanaType::Manushya,
        _ => GanaType::Rakshasa,
    }
}

fn calculate_gana(male_nak: u8, female_nak: u8) -> f64 {
    let m_gana = get_nak_gana(male_nak);
    let f_gana = get_nak_gana(female_nak);

    use GanaType::*;
    match (m_gana, f_gana) {
        (Deva, Deva) => 6.0,
        (Manushya, Manushya) => 6.0,
        (Rakshasa, Rakshasa) => 6.0,
        (Deva, Manushya) | (Manushya, Deva) => 5.0,
        (Manushya, Rakshasa) | (Rakshasa, Manushya) => 3.0,
        (Deva, Rakshasa) | (Rakshasa, Deva) => 1.0,
    }
}

fn calculate_bhakoot(male_rasi: u8, female_rasi: u8) -> f64 {
    if male_rasi == female_rasi {
        return 7.0;
    }
    let diff = ((male_rasi as i16 - female_rasi as i16 + 12) % 12) + 1;
    // 1-based relative position: 1, 7, 3, 11, 4, 10 are auspicious
    match diff {
        1 | 7 | 3 | 11 | 4 | 10 => 7.0,
        _ => 0.0, // 2/12, 5/9, 6/8 are zero points
    }
}

enum NadiType {
    Adi,
    Madhya,
    Antya,
}

fn get_nak_nadi(nak: u8) -> NadiType {
    match nak {
        1 | 6 | 7 | 12 | 13 | 18 | 19 | 24 | 25 => NadiType::Adi,
        2 | 5 | 8 | 11 | 14 | 17 | 20 | 23 | 26 => NadiType::Madhya,
        _ => NadiType::Antya,
    }
}

fn calculate_nadi(male_nak: u8, female_nak: u8) -> f64 {
    let m_nadi = get_nak_nadi(male_nak);
    let f_nadi = get_nak_nadi(female_nak);

    let m_val = m_nadi as u8;
    let f_val = f_nadi as u8;

    if m_val != f_val {
        8.0
    } else {
        0.0 // Nadi Dosha
    }
}

// ── Mangal Dosha helper ──────────────────────────────────────────────────────

fn check_mangal_dosha(chart: &VedicChart) -> bool {
    let mars = chart.planets.iter().find(|p| p.planet == VedicPlanet::Mars);
    if let Some(m) = mars {
        // From Lagna (house_index)
        let is_mangal_lagna = [1, 2, 4, 7, 8, 12].contains(&m.house_index);

        // From Moon
        let moon = chart
            .planets
            .iter()
            .find(|p| p.planet == VedicPlanet::Moon)
            .unwrap();
        let mut diff = m.rasi as i16 - moon.rasi as i16;
        if diff < 0 {
            diff += 12;
        }
        let house_from_moon = (diff + 1) as u8;
        let is_mangal_moon = [1, 2, 4, 7, 8, 12].contains(&house_from_moon);

        is_mangal_lagna || is_mangal_moon
    } else {
        false
    }
}
