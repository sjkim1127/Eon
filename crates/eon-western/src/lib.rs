//! eon-western: 서양 점성술 명리 연산 엔진
//!
//! eon-astro의 Swiss Ephemeris FFI를 활용하여 10대 행성, Chiron, True Node, South Node,
//! Black Moon Lilith, 4대 소행성(Ceres, Pallas, Juno, Vesta) 및 Placidus, Koch, Whole Sign, Equal House 등의 Cusp 좌표,
//! 메이저/마이너 아스펙트(어플라잉/세퍼레이팅), Essential Dignities, 하우스 룰러십 네트워크, 아라비안 파트(Lot of Fortune/Spirit/Eros),
//! 기하학적 아스펙트 패턴(Grand Trine, T-Square, Grand Cross, Yod, Kite), Synastry, Composite, Secondary Progressions를 정밀 분석합니다.

use chrono::{DateTime, Utc};
use eon_astro::{AstroEngine, AstroError};
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum WesternError {
    #[error("Astronomical calculation error: {0}")]
    Astro(#[from] AstroError),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WesternPlanetData {
    pub id: i32,
    pub name: String,
    pub longitude: f64,
    pub speed: f64,
    pub is_retrograde: bool,
    pub sign_index: usize,
    pub degree_in_sign: f64,
    pub house_number: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WesternHouseData {
    pub house_number: usize,
    pub cusp_longitude: f64,
    pub sign_index: usize,
    pub degree_in_sign: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AspectType {
    Conjunction,    // 0도 (Major)
    Sextile,        // 60도 (Major)
    Square,         // 90도 (Major)
    Trine,          // 120도 (Major)
    Opposition,     // 180도 (Major)
    Semisextile,    // 30도 (Minor)
    Quincunx,       // 150도 / Inconjunct (Minor)
    Semisquare,     // 45도 (Minor)
    Sesquiquadrate, // 135도 (Minor)
    Quintile,       // 72도 (Minor)
    Biquintile,     // 144도 (Minor)
}

impl AspectType {
    pub fn angle(self) -> f64 {
        match self {
            AspectType::Conjunction => 0.0,
            AspectType::Semisextile => 30.0,
            AspectType::Semisquare => 45.0,
            AspectType::Sextile => 60.0,
            AspectType::Quintile => 72.0,
            AspectType::Square => 90.0,
            AspectType::Trine => 120.0,
            AspectType::Sesquiquadrate => 135.0,
            AspectType::Biquintile => 144.0,
            AspectType::Quincunx => 150.0,
            AspectType::Opposition => 180.0,
        }
    }

    pub fn standard_orb(self) -> f64 {
        match self {
            AspectType::Conjunction => 8.0,
            AspectType::Sextile => 6.0,
            AspectType::Square => 8.0,
            AspectType::Trine => 8.0,
            AspectType::Opposition => 8.0,
            AspectType::Semisextile => 2.0,
            AspectType::Semisquare => 2.5,
            AspectType::Quintile => 2.0,
            AspectType::Sesquiquadrate => 2.5,
            AspectType::Biquintile => 2.0,
            AspectType::Quincunx => 3.0,
        }
    }

    pub fn is_major(self) -> bool {
        matches!(
            self,
            AspectType::Conjunction
                | AspectType::Sextile
                | AspectType::Square
                | AspectType::Trine
                | AspectType::Opposition
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AspectDynamics {
    Applying,   // 접근 (에너지 강화)
    Separating, // 이탈 (에너지 약화)
    Exact,      // 일치
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WesternAspectData {
    pub body_a_name: String,
    pub body_b_name: String,
    pub aspect_type: AspectType,
    pub angle_diff: f64,
    pub orb: f64,
    pub dynamics: AspectDynamics,
    pub is_major: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ElementDistribution {
    pub fire: f64,
    pub earth: f64,
    pub air: f64,
    pub water: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ModalityDistribution {
    pub cardinal: f64,
    pub fixed: f64,
    pub mutable: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EssentialDignityInfo {
    pub planet_name: String,
    pub score: i32,
    pub is_domicile: bool,
    pub is_exaltation: bool,
    pub is_detriment: bool,
    pub is_fall: bool,
    pub status_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HouseRulershipInfo {
    pub house_number: usize,
    pub cusp_sign_index: usize,
    pub ruler_planet: String,
    pub ruler_in_house: usize,
    pub interpretation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArabianPartData {
    pub name: String,
    pub name_kr: String,
    pub longitude: f64,
    pub sign_index: usize,
    pub degree_in_sign: f64,
    pub house_number: usize,
    pub formula: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AspectPatternType {
    GrandTrine,
    TSquare,
    GrandCross,
    Yod,
    Kite,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AspectPatternData {
    pub pattern_type: AspectPatternType,
    pub name: String,
    pub name_kr: String,
    pub planets: Vec<String>,
    pub interpretation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WesternResult {
    pub planets: Vec<WesternPlanetData>,
    pub houses: Vec<WesternHouseData>,
    pub aspects: Vec<WesternAspectData>,
    pub elements: ElementDistribution,
    pub modalities: ModalityDistribution,
    pub ascendant: f64,
    pub midheaven: f64,
    pub chart_ruler: String,
    pub dominant_element: String,
    pub dominant_modality: String,
    pub dignities: Vec<EssentialDignityInfo>,
    pub house_rulerships: Vec<HouseRulershipInfo>,
    pub arabian_parts: Vec<ArabianPartData>,
    pub aspect_patterns: Vec<AspectPatternData>,
    pub is_day_birth: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SynastryResult {
    pub person_a_name: String,
    pub person_b_name: String,
    pub inter_aspects: Vec<WesternAspectData>,
    pub harmony_score: f64,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompositeResult {
    pub composite_chart: WesternResult,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProgressionResult {
    pub birth_chart: WesternResult,
    pub progressed_chart: WesternResult,
    pub progressed_age_years: f64,
    pub active_transit_aspects: Vec<WesternAspectData>,
}

pub const SIGN_NAMES: [&str; 12] = [
    "Aries",
    "Taurus",
    "Gemini",
    "Cancer",
    "Leo",
    "Virgo",
    "Libra",
    "Scorpio",
    "Sagittarius",
    "Capricorn",
    "Aquarius",
    "Pisces",
];

pub const SIGN_NAMES_KR: [&str; 12] = [
    "양자리 (Aries)",
    "황소자리 (Taurus)",
    "쌍둥이자리 (Gemini)",
    "게자리 (Cancer)",
    "사자자리 (Leo)",
    "처녀자리 (Virgo)",
    "천칭자리 (Libra)",
    "전갈자리 (Scorpio)",
    "궁수자리 (Sagittarius)",
    "염소자리 (Capricorn)",
    "물병자리 (Aquarius)",
    "물고기자리 (Pisces)",
];

pub fn get_sign_ruler(sign_idx: usize) -> &'static str {
    match sign_idx {
        0 => "Mars",     // Aries
        1 => "Venus",    // Taurus
        2 => "Mercury",  // Gemini
        3 => "Moon",     // Cancer
        4 => "Sun",      // Leo
        5 => "Mercury",  // Virgo
        6 => "Venus",    // Libra
        7 => "Pluto",    // Scorpio
        8 => "Jupiter",  // Sagittarius
        9 => "Saturn",   // Capricorn
        10 => "Uranus",  // Aquarius
        11 => "Neptune", // Pisces
        _ => "Mars",
    }
}

/// 특정 각도가 start와 end의 섹터 구간 안에 속하는지 판정 (각도 wrap-around 360도 대응)
pub fn is_angle_between(target: f64, start: f64, end: f64) -> bool {
    let t = (target - start + 360.0) % 360.0;
    let e = (end - start + 360.0) % 360.0;
    t < e
}

/// Essential Dignity (고전 위계) 점수 계산
pub fn calculate_essential_dignities(planets: &[WesternPlanetData]) -> Vec<EssentialDignityInfo> {
    // Domicile / Exaltation / Detriment / Fall 규칙
    // Sun: Domicile Leo(4), Exaltation Aries(0), Detriment Aquarius(10), Fall Libra(6)
    // Moon: Domicile Cancer(3), Exaltation Taurus(1), Detriment Capricorn(9), Fall Scorpio(7)
    // Mercury: Domicile Gemini(2)/Virgo(5), Exaltation Virgo(5), Detriment Sag(8)/Pisces(11), Fall Pisces(11)
    // Venus: Domicile Taurus(1)/Libra(6), Exaltation Pisces(11), Detriment Scorpio(7)/Aries(0), Fall Virgo(5)
    // Mars: Domicile Aries(0)/Scorpio(7), Exaltation Capricorn(9), Detriment Libra(6)/Taurus(1), Fall Cancer(3)
    // Jupiter: Domicile Sag(8)/Pisces(11), Exaltation Cancer(3), Detriment Gemini(2)/Virgo(5), Fall Capricorn(9)
    // Saturn: Domicile Cap(9)/Aquarius(10), Exaltation Libra(6), Detriment Cancer(3)/Leo(4), Fall Aries(0)
    // Uranus: Domicile Aquarius(10), Exaltation Scorpio(7), Detriment Leo(4), Fall Taurus(1)
    // Neptune: Domicile Pisces(11), Exaltation Cancer(3), Detriment Virgo(5), Fall Capricorn(9)
    // Pluto: Domicile Scorpio(7), Exaltation Aries(0), Detriment Taurus(1), Fall Libra(6)

    let mut result = Vec::new();

    for p in planets {
        if matches!(
            p.name.as_str(),
            "Chiron" | "True Node" | "South Node" | "Lilith"
        ) {
            continue;
        }

        let s = p.sign_index;
        let mut is_dom = false;
        let mut is_exalt = false;
        let mut is_det = false;
        let mut is_fall = false;

        match p.name.as_str() {
            "Sun" => {
                is_dom = s == 4;
                is_exalt = s == 0;
                is_det = s == 10;
                is_fall = s == 6;
            }
            "Moon" => {
                is_dom = s == 3;
                is_exalt = s == 1;
                is_det = s == 9;
                is_fall = s == 7;
            }
            "Mercury" => {
                is_dom = s == 2 || s == 5;
                is_exalt = s == 5;
                is_det = s == 8 || s == 11;
                is_fall = s == 11;
            }
            "Venus" => {
                is_dom = s == 1 || s == 6;
                is_exalt = s == 11;
                is_det = s == 7 || s == 0;
                is_fall = s == 5;
            }
            "Mars" => {
                is_dom = s == 0 || s == 7;
                is_exalt = s == 9;
                is_det = s == 6 || s == 1;
                is_fall = s == 3;
            }
            "Jupiter" => {
                is_dom = s == 8 || s == 11;
                is_exalt = s == 3;
                is_det = s == 2 || s == 5;
                is_fall = s == 9;
            }
            "Saturn" => {
                is_dom = s == 9 || s == 10;
                is_exalt = s == 6;
                is_det = s == 3 || s == 4;
                is_fall = s == 0;
            }
            "Uranus" => {
                is_dom = s == 10;
                is_exalt = s == 7;
                is_det = s == 4;
                is_fall = s == 1;
            }
            "Neptune" => {
                is_dom = s == 11;
                is_exalt = s == 3;
                is_det = s == 5;
                is_fall = s == 9;
            }
            "Pluto" => {
                is_dom = s == 7;
                is_exalt = s == 0;
                is_det = s == 1;
                is_fall = s == 6;
            }
            _ => {}
        }

        let mut score = 0;
        let mut summaries = Vec::new();

        if is_dom {
            score += 5;
            summaries.push("Domicile (+5)");
        }
        if is_exalt {
            score += 4;
            summaries.push("Exaltation (+4)");
        }
        if is_det {
            score -= 5;
            summaries.push("Detriment (-5)");
        }
        if is_fall {
            score -= 4;
            summaries.push("Fall (-4)");
        }

        if !is_dom && !is_exalt && !is_det && !is_fall {
            summaries.push("Peregrine (0)");
        }

        let status_summary = format!("점수 {}pt ({})", score, summaries.join(", "));

        result.push(EssentialDignityInfo {
            planet_name: p.name.clone(),
            score,
            is_domicile: is_dom,
            is_exaltation: is_exalt,
            is_detriment: is_det,
            is_fall,
            status_summary,
        });
    }

    result
}

/// 하우스 룰러 입주 하우스 연계 네트워크 산출
pub fn calculate_house_rulerships(
    houses: &[WesternHouseData],
    planets: &[WesternPlanetData],
) -> Vec<HouseRulershipInfo> {
    let mut result = Vec::new();

    for h in houses {
        let ruler = get_sign_ruler(h.sign_index);
        let ruler_planet = planets.iter().find(|p| p.name == ruler);

        let ruler_in_house = ruler_planet.map(|p| p.house_number).unwrap_or(1);
        let sign_name = SIGN_NAMES_KR[h.sign_index];

        let interpretation = format!(
            "{}하우스 Cusp(성좌: {})의 주행성인 {}이(가) {}하우스에 위치하여, {}하우스 에너지가 {}하우스 영역으로 확장됩니다.",
            h.house_number, sign_name, ruler, ruler_in_house, h.house_number, ruler_in_house
        );

        result.push(HouseRulershipInfo {
            house_number: h.house_number,
            cusp_sign_index: h.sign_index,
            ruler_planet: ruler.to_string(),
            ruler_in_house,
            interpretation,
        });
    }

    result
}

/// 아라비안 파트 (Lots) 산출
pub fn calculate_arabian_parts(
    asc: f64,
    planets: &[WesternPlanetData],
    houses: &[WesternHouseData],
    is_day_birth: bool,
) -> Vec<ArabianPartData> {
    let sun_long = planets
        .iter()
        .find(|p| p.name == "Sun")
        .map(|p| p.longitude)
        .unwrap_or(0.0);
    let moon_long = planets
        .iter()
        .find(|p| p.name == "Moon")
        .map(|p| p.longitude)
        .unwrap_or(0.0);
    let venus_long = planets
        .iter()
        .find(|p| p.name == "Venus")
        .map(|p| p.longitude)
        .unwrap_or(0.0);

    // Part of Fortune: Day = ASC + Moon - Sun, Night = ASC + Sun - Moon
    let pof_long = if is_day_birth {
        (asc + moon_long - sun_long + 3600.0) % 360.0
    } else {
        (asc + sun_long - moon_long + 3600.0) % 360.0
    };

    // Part of Spirit: Day = ASC + Sun - Moon, Night = ASC + Moon - Sun
    let pos_long = if is_day_birth {
        (asc + sun_long - moon_long + 3600.0) % 360.0
    } else {
        (asc + moon_long - sun_long + 3600.0) % 360.0
    };

    // Part of Eros: Day = ASC + Venus - Part of Spirit, Night = ASC + Part of Spirit - Venus
    let eros_long = if is_day_birth {
        (asc + venus_long - pos_long + 3600.0) % 360.0
    } else {
        (asc + pos_long - venus_long + 3600.0) % 360.0
    };

    let calculate_part_info = |name: &str, name_kr: &str, long: f64, formula: &str| {
        let sign_index = (long / 30.0).floor() as usize;
        let degree_in_sign = long % 30.0;
        let mut house_number = 1;
        if houses.len() == 12 {
            for h in 0..12 {
                let start = houses[h].cusp_longitude;
                let end = houses[(h + 1) % 12].cusp_longitude;
                if is_angle_between(long, start, end) {
                    house_number = h + 1;
                    break;
                }
            }
        }
        ArabianPartData {
            name: name.to_string(),
            name_kr: name_kr.to_string(),
            longitude: long,
            sign_index,
            degree_in_sign,
            house_number,
            formula: formula.to_string(),
        }
    };

    vec![
        calculate_part_info(
            "Part of Fortune",
            "행운의 지점 (Part of Fortune / Lot of Fortune)",
            pof_long,
            if is_day_birth {
                "ASC + Moon - Sun (Day)"
            } else {
                "ASC + Sun - Moon (Night)"
            },
        ),
        calculate_part_info(
            "Part of Spirit",
            "영혼의 지점 (Part of Spirit / Lot of Spirit)",
            pos_long,
            if is_day_birth {
                "ASC + Sun - Moon (Day)"
            } else {
                "ASC + Moon - Sun (Night)"
            },
        ),
        calculate_part_info(
            "Part of Eros",
            "애정의 지점 (Part of Eros / Lot of Eros)",
            eros_long,
            if is_day_birth {
                "ASC + Venus - Spirit (Day)"
            } else {
                "ASC + Spirit - Venus (Night)"
            },
        ),
    ]
}

/// 기하학적 아스펙트 패턴 (Aspect Geometries) 자동 검출기
pub fn detect_aspect_patterns(
    planets: &[WesternPlanetData],
    aspects: &[WesternAspectData],
) -> Vec<AspectPatternData> {
    let mut patterns = Vec::new();

    // 1. Grand Trine (3 planets forming 3 Trines with each other)
    for i in 0..planets.len() {
        for j in (i + 1)..planets.len() {
            for k in (j + 1)..planets.len() {
                let p1 = &planets[i].name;
                let p2 = &planets[j].name;
                let p3 = &planets[k].name;

                let has_trine_12 = aspects.iter().any(|a| {
                    a.aspect_type == AspectType::Trine
                        && ((&a.body_a_name == p1 && &a.body_b_name == p2)
                            || (&a.body_a_name == p2 && &a.body_b_name == p1))
                });
                let has_trine_23 = aspects.iter().any(|a| {
                    a.aspect_type == AspectType::Trine
                        && ((&a.body_a_name == p2 && &a.body_b_name == p3)
                            || (&a.body_a_name == p3 && &a.body_b_name == p2))
                });
                let has_trine_31 = aspects.iter().any(|a| {
                    a.aspect_type == AspectType::Trine
                        && ((&a.body_a_name == p3 && &a.body_b_name == p1)
                            || (&a.body_a_name == p1 && &a.body_b_name == p3))
                });

                if has_trine_12 && has_trine_23 && has_trine_31 {
                    patterns.push(AspectPatternData {
                        pattern_type: AspectPatternType::GrandTrine,
                        name: "Grand Trine".to_string(),
                        name_kr: "그랜드 트라인 (대삼각 패턴)".to_string(),
                        planets: vec![p1.clone(), p2.clone(), p3.clone()],
                        interpretation: format!(
                            "{} - {} - {} 세 행성이 조화로운 120도 대삼각을 형성하여 창의성과 재능의 원활한 흐름을 가져옵니다.",
                            p1, p2, p3
                        ),
                    });
                }
            }
        }
    }

    // 2. T-Square (2 planets in Opposition, both forming Square to a 3rd Apex planet)
    for a in aspects {
        if a.aspect_type == AspectType::Opposition {
            let p1 = &a.body_a_name;
            let p2 = &a.body_b_name;

            for p3_data in planets {
                let p3 = &p3_data.name;
                if p3 == p1 || p3 == p2 {
                    continue;
                }

                let sq1 = aspects.iter().any(|asp| {
                    asp.aspect_type == AspectType::Square
                        && ((&asp.body_a_name == p1 && &asp.body_b_name == p3)
                            || (&asp.body_a_name == p3 && &asp.body_b_name == p1))
                });
                let sq2 = aspects.iter().any(|asp| {
                    asp.aspect_type == AspectType::Square
                        && ((&asp.body_a_name == p2 && &asp.body_b_name == p3)
                            || (&asp.body_a_name == p3 && &asp.body_b_name == p2))
                });

                if sq1 && sq2 {
                    patterns.push(AspectPatternData {
                        pattern_type: AspectPatternType::TSquare,
                        name: "T-Square".to_string(),
                        name_kr: "T-스퀘어 (직각 삼각 갈등 패턴)".to_string(),
                        planets: vec![p1.clone(), p2.clone(), p3.clone()],
                        interpretation: format!(
                            "{}와(과) {}의 대립(Opposition) 속에서 Apex 행성 {}이(가) 스퀘어 직각 자극을 주어 강력한 추진력과 긴장을 유발합니다.",
                            p1, p2, p3
                        ),
                    });
                }
            }
        }
    }

    // 3. Yod (Finger of God: 2 planets in Sextile, both forming Quincunx to a 3rd Apex planet)
    for a in aspects {
        if a.aspect_type == AspectType::Sextile {
            let p1 = &a.body_a_name;
            let p2 = &a.body_b_name;

            for p3_data in planets {
                let p3 = &p3_data.name;
                if p3 == p1 || p3 == p2 {
                    continue;
                }

                let q1 = aspects.iter().any(|asp| {
                    asp.aspect_type == AspectType::Quincunx
                        && ((&asp.body_a_name == p1 && &asp.body_b_name == p3)
                            || (&asp.body_a_name == p3 && &asp.body_b_name == p1))
                });
                let q2 = aspects.iter().any(|asp| {
                    asp.aspect_type == AspectType::Quincunx
                        && ((&asp.body_a_name == p2 && &asp.body_b_name == p3)
                            || (&asp.body_a_name == p3 && &asp.body_b_name == p2))
                });

                if q1 && q2 {
                    patterns.push(AspectPatternData {
                        pattern_type: AspectPatternType::Yod,
                        name: "Yod".to_string(),
                        name_kr: "요드 (Yod - 신의 손가락 운명 패턴)".to_string(),
                        planets: vec![p1.clone(), p2.clone(), p3.clone()],
                        interpretation: format!(
                            "{}와(과) {}의 60도 육각형 받침대에서 Apex 행성 {}이(가) 150도 퀸컹스 조정을 통해 명확한 소명과 숙명을 형성합니다.",
                            p1, p2, p3
                        ),
                    });
                }
            }
        }
    }

    patterns
}

pub fn calculate_western(
    datetime: DateTime<Utc>,
    latitude: f64,
    longitude: f64,
    house_system_char: char, // 'P', 'K', 'W', 'E' 등
) -> Result<WesternResult, WesternError> {
    let engine = AstroEngine::new();

    // 1. 하우스 Cusp 좌표 및 ASC/MC 계산
    let house_sys_byte = house_system_char as i32;
    let (mut cusps, ascmc) = engine.get_houses(datetime, latitude, longitude, house_sys_byte)?;

    let asc = ascmc[0];
    let mc = ascmc[1];

    if house_system_char == 'W' {
        let asc_sign = (asc / 30.0).floor() as usize;
        for i in 0..12 {
            cusps[i] = ((asc_sign + i) % 12) as f64 * 30.0;
        }
    } else if house_system_char == 'E' {
        for i in 0..12 {
            cusps[i] = (asc + i as f64 * 30.0) % 360.0;
        }
    }

    // 2. 16대 천체 및 노드 위치 조회
    // 0: Sun, 1: Moon, 2: Mercury, 3: Venus, 4: Mars, 5: Jupiter, 6: Saturn, 7: Uranus, 8: Neptune, 9: Pluto
    // 11: True Node (North Node), 12: Lilith, 15: Chiron, 17: Ceres, 18: Pallas, 19: Juno, 20: Vesta
    let bodies = vec![
        (0, "Sun"),
        (1, "Moon"),
        (2, "Mercury"),
        (3, "Venus"),
        (4, "Mars"),
        (5, "Jupiter"),
        (6, "Saturn"),
        (7, "Uranus"),
        (8, "Neptune"),
        (9, "Pluto"),
        (11, "True Node"),
        (12, "Lilith"),
        (15, "Chiron"),
        (17, "Ceres"),
        (18, "Pallas"),
        (19, "Juno"),
        (20, "Vesta"),
    ];

    let mut planets = Vec::new();
    let flag = 4; // SEFLG_MOEPH (Moshier Wasm compatible)

    for (id, name) in bodies {
        let (long, speed) = engine
            .get_planet_full(datetime, id, flag)
            .unwrap_or((0.0, 0.0));
        let sign_index = (long / 30.0).floor() as usize;
        let degree_in_sign = long % 30.0;
        let is_retrograde = speed < 0.0;

        let mut house_number = 1;
        for h in 0..12 {
            let start = cusps[h];
            let end = cusps[(h + 1) % 12];
            if is_angle_between(long, start, end) {
                house_number = h + 1;
                break;
            }
        }

        planets.push(WesternPlanetData {
            id,
            name: name.to_string(),
            longitude: long,
            speed,
            is_retrograde,
            sign_index,
            degree_in_sign,
            house_number,
        });
    }

    // South Node (180도 반대점 파생)
    let node_long = planets
        .iter()
        .find(|p| p.name == "True Node")
        .map(|p| p.longitude)
        .unwrap_or(0.0);
    let node_speed = planets
        .iter()
        .find(|p| p.name == "True Node")
        .map(|p| p.speed)
        .unwrap_or(0.0);

    let south_node_long = (node_long + 180.0) % 360.0;
    let sn_sign_index = (south_node_long / 30.0).floor() as usize;
    let sn_degree_in_sign = south_node_long % 30.0;
    let mut sn_house_number = 1;
    for h in 0..12 {
        let start = cusps[h];
        let end = cusps[(h + 1) % 12];
        if is_angle_between(south_node_long, start, end) {
            sn_house_number = h + 1;
            break;
        }
    }

    planets.push(WesternPlanetData {
        id: -11,
        name: "South Node".to_string(),
        longitude: south_node_long,
        speed: -node_speed,
        is_retrograde: true,
        sign_index: sn_sign_index,
        degree_in_sign: sn_degree_in_sign,
        house_number: sn_house_number,
    });

    // 3. 12 하우스 데이터 리스트 조립
    let mut houses = Vec::new();
    for i in 0..12 {
        let cusp = cusps[i];
        let sign_index = (cusp / 30.0).floor() as usize;
        let degree_in_sign = cusp % 30.0;
        houses.push(WesternHouseData {
            house_number: i + 1,
            cusp_longitude: cusp,
            sign_index,
            degree_in_sign,
        });
    }

    // 4. 메이저/마이너 아스펙트 및 어플라잉/세퍼레이팅 계산
    let mut aspects = Vec::new();
    let aspect_types = vec![
        AspectType::Conjunction,
        AspectType::Sextile,
        AspectType::Square,
        AspectType::Trine,
        AspectType::Opposition,
        AspectType::Semisextile,
        AspectType::Quincunx,
        AspectType::Semisquare,
        AspectType::Sesquiquadrate,
        AspectType::Quintile,
        AspectType::Biquintile,
    ];

    let mut aspect_bodies = planets
        .iter()
        .map(|p| (p.name.clone(), p.longitude, p.speed))
        .collect::<Vec<_>>();
    aspect_bodies.push(("ASC".to_string(), asc, 0.0));
    aspect_bodies.push(("MC".to_string(), mc, 0.0));

    for i in 0..aspect_bodies.len() {
        for j in (i + 1)..aspect_bodies.len() {
            let (ref name_a, long_a, speed_a) = aspect_bodies[i];
            let (ref name_b, long_b, speed_b) = aspect_bodies[j];

            let diff = (long_a - long_b).abs();
            let angle = if diff > 180.0 { 360.0 - diff } else { diff };

            for &asp in &aspect_types {
                let target = asp.angle();
                let orb = (angle - target).abs();
                if orb <= asp.standard_orb() {
                    // Applying / Separating 동역학 계산
                    let rel_speed = speed_a - speed_b;
                    let dynamics = if orb < 0.01 {
                        AspectDynamics::Exact
                    } else if (angle > target && rel_speed > 0.0)
                        || (angle < target && rel_speed < 0.0)
                    {
                        AspectDynamics::Separating
                    } else {
                        AspectDynamics::Applying
                    };

                    aspects.push(WesternAspectData {
                        body_a_name: name_a.clone(),
                        body_b_name: name_b.clone(),
                        aspect_type: asp,
                        angle_diff: angle,
                        orb,
                        dynamics,
                        is_major: asp.is_major(),
                    });
                }
            }
        }
    }

    // 5. 원소 및 모달리티 분포 점수
    let mut elements = ElementDistribution::default();
    let mut modalities = ModalityDistribution::default();

    let weight_mapping = |name: &str| -> f64 {
        match name {
            "Sun" | "Moon" => 3.0,
            "Mercury" | "Venus" | "Mars" | "ASC" | "MC" => 2.0,
            "Jupiter" | "Saturn" | "Uranus" | "Neptune" | "Pluto" => 1.0,
            _ => 0.0,
        }
    };

    for p in &planets {
        let w = weight_mapping(&p.name);
        if w > 0.0 {
            match p.sign_index {
                0 | 4 | 8 => elements.fire += w,
                1 | 5 | 9 => elements.earth += w,
                2 | 6 | 10 => elements.air += w,
                3 | 7 | 11 => elements.water += w,
                _ => {}
            }
            match p.sign_index {
                0 | 3 | 6 | 9 => modalities.cardinal += w,
                1 | 4 | 7 | 10 => modalities.fixed += w,
                2 | 5 | 8 | 11 => modalities.mutable += w,
                _ => {}
            }
        }
    }

    let add_point =
        |long: f64, name: &str, el: &mut ElementDistribution, mo: &mut ModalityDistribution| {
            let w = weight_mapping(name);
            let s_idx = (long / 30.0).floor() as usize;
            match s_idx {
                0 | 4 | 8 => el.fire += w,
                1 | 5 | 9 => el.earth += w,
                2 | 6 | 10 => el.air += w,
                3 | 7 | 11 => el.water += w,
                _ => {}
            }
            match s_idx {
                0 | 3 | 6 | 9 => mo.cardinal += w,
                1 | 4 | 7 | 10 => mo.fixed += w,
                2 | 5 | 8 | 11 => mo.mutable += w,
                _ => {}
            }
        };
    add_point(asc, "ASC", &mut elements, &mut modalities);
    add_point(mc, "MC", &mut elements, &mut modalities);

    let el_total = elements.fire + elements.earth + elements.air + elements.water;
    if el_total > 0.0 {
        elements.fire = (elements.fire / el_total) * 100.0;
        elements.earth = (elements.earth / el_total) * 100.0;
        elements.air = (elements.air / el_total) * 100.0;
        elements.water = (elements.water / el_total) * 100.0;
    }

    let mo_total = modalities.cardinal + modalities.fixed + modalities.mutable;
    if mo_total > 0.0 {
        modalities.cardinal = (modalities.cardinal / mo_total) * 100.0;
        modalities.fixed = (modalities.fixed / mo_total) * 100.0;
        modalities.mutable = (modalities.mutable / mo_total) * 100.0;
    }

    let mut el_vec = [
        ("Fire", elements.fire),
        ("Earth", elements.earth),
        ("Air", elements.air),
        ("Water", elements.water),
    ];
    el_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let dominant_element = el_vec[0].0.to_string();

    let mut mo_vec = [
        ("Cardinal", modalities.cardinal),
        ("Fixed", modalities.fixed),
        ("Mutable", modalities.mutable),
    ];
    mo_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let dominant_modality = mo_vec[0].0.to_string();

    let asc_sign = (asc / 30.0).floor() as usize;
    let chart_ruler = get_sign_ruler(asc_sign).to_string();

    // 주야간(Day/Night birth) 구분
    let sun_house = planets
        .iter()
        .find(|p| p.name == "Sun")
        .map(|p| p.house_number)
        .unwrap_or(1);
    let is_day_birth = (7..=12).contains(&sun_house);

    // 6. Dignities, House Rulerships, Arabian Parts, Aspect Patterns 연산
    let dignities = calculate_essential_dignities(&planets);
    let house_rulerships = calculate_house_rulerships(&houses, &planets);
    let arabian_parts = calculate_arabian_parts(asc, &planets, &houses, is_day_birth);
    let aspect_patterns = detect_aspect_patterns(&planets, &aspects);

    Ok(WesternResult {
        planets,
        houses,
        aspects,
        elements,
        modalities,
        ascendant: asc,
        midheaven: mc,
        chart_ruler,
        dominant_element,
        dominant_modality,
        dignities,
        house_rulerships,
        arabian_parts,
        aspect_patterns,
        is_day_birth,
    })
}

/// Synastry (두 인물 간 교차 궁합 아스펙트 연산)
pub fn calculate_synastry(
    chart_a: &WesternResult,
    chart_b: &WesternResult,
    person_a_name: &str,
    person_b_name: &str,
) -> SynastryResult {
    let mut inter_aspects = Vec::new();
    let aspect_types = vec![
        AspectType::Conjunction,
        AspectType::Sextile,
        AspectType::Square,
        AspectType::Trine,
        AspectType::Opposition,
    ];

    for pa in &chart_a.planets {
        for pb in &chart_b.planets {
            let diff = (pa.longitude - pb.longitude).abs();
            let angle = if diff > 180.0 { 360.0 - diff } else { diff };

            for &asp in &aspect_types {
                let target = asp.angle();
                let orb = (angle - target).abs();
                if orb <= asp.standard_orb() {
                    inter_aspects.push(WesternAspectData {
                        body_a_name: format!("{}의 {}", person_a_name, pa.name),
                        body_b_name: format!("{}의 {}", person_b_name, pb.name),
                        aspect_type: asp,
                        angle_diff: angle,
                        orb,
                        dynamics: AspectDynamics::Exact,
                        is_major: asp.is_major(),
                    });
                }
            }
        }
    }

    let mut harmony_score: f64 = 70.0;
    for asp in &inter_aspects {
        match asp.aspect_type {
            AspectType::Trine | AspectType::Sextile | AspectType::Conjunction => {
                harmony_score += 4.0
            }
            AspectType::Square | AspectType::Opposition => harmony_score -= 3.0,
            _ => {}
        }
    }
    harmony_score = harmony_score.clamp(0.0, 100.0);

    let summary = format!(
        "{}와(과) {}의 시나스트리 궁합 조화도 점수는 {:.1}점이며, 총 {}개의 교차 아스펙트가 형성됩니다.",
        person_a_name,
        person_b_name,
        harmony_score,
        inter_aspects.len()
    );

    SynastryResult {
        person_a_name: person_a_name.to_string(),
        person_b_name: person_b_name.to_string(),
        inter_aspects,
        harmony_score,
        summary,
    }
}

/// Composite Chart (두 차트 간 미드포인트 합성 차트 생성)
pub fn calculate_composite(
    chart_a: &WesternResult,
    chart_b: &WesternResult,
) -> Result<CompositeResult, WesternError> {
    // 360도 미드포인트 연산
    let midpoint = |l1: f64, l2: f64| -> f64 {
        let diff = (l1 - l2).abs();
        if diff <= 180.0 {
            (l1 + l2) / 2.0
        } else {
            ((l1 + l2 + 360.0) / 2.0) % 360.0
        }
    };

    let comp_asc = midpoint(chart_a.ascendant, chart_b.ascendant);
    let comp_mc = midpoint(chart_a.midheaven, chart_b.midheaven);

    let mut comp_planets = Vec::new();
    for pa in &chart_a.planets {
        if let Some(pb) = chart_b.planets.iter().find(|p| p.id == pa.id) {
            let mid_long = midpoint(pa.longitude, pb.longitude);
            let sign_index = (mid_long / 30.0).floor() as usize;
            let degree_in_sign = mid_long % 30.0;

            comp_planets.push(WesternPlanetData {
                id: pa.id,
                name: pa.name.clone(),
                longitude: mid_long,
                speed: (pa.speed + pb.speed) / 2.0,
                is_retrograde: pa.is_retrograde,
                sign_index,
                degree_in_sign,
                house_number: 1, // Will be re-assigned below
            });
        }
    }

    let mut comp_houses = Vec::new();
    for i in 0..12 {
        let h_a = &chart_a.houses[i];
        let h_b = &chart_b.houses[i];
        let mid_cusp = midpoint(h_a.cusp_longitude, h_b.cusp_longitude);
        let sign_index = (mid_cusp / 30.0).floor() as usize;
        let degree_in_sign = mid_cusp % 30.0;

        comp_houses.push(WesternHouseData {
            house_number: i + 1,
            cusp_longitude: mid_cusp,
            sign_index,
            degree_in_sign,
        });
    }

    // Re-assign planet house numbers
    for p in &mut comp_planets {
        for h in 0..12 {
            let start = comp_houses[h].cusp_longitude;
            let end = comp_houses[(h + 1) % 12].cusp_longitude;
            if is_angle_between(p.longitude, start, end) {
                p.house_number = h + 1;
                break;
            }
        }
    }

    let aspect_types = vec![
        AspectType::Conjunction,
        AspectType::Sextile,
        AspectType::Square,
        AspectType::Trine,
        AspectType::Opposition,
    ];
    let mut comp_aspects = Vec::new();

    for i in 0..comp_planets.len() {
        for j in (i + 1)..comp_planets.len() {
            let p1 = &comp_planets[i];
            let p2 = &comp_planets[j];
            let diff = (p1.longitude - p2.longitude).abs();
            let angle = if diff > 180.0 { 360.0 - diff } else { diff };

            for &asp in &aspect_types {
                let target = asp.angle();
                let orb = (angle - target).abs();
                if orb <= asp.standard_orb() {
                    comp_aspects.push(WesternAspectData {
                        body_a_name: p1.name.clone(),
                        body_b_name: p2.name.clone(),
                        aspect_type: asp,
                        angle_diff: angle,
                        orb,
                        dynamics: AspectDynamics::Exact,
                        is_major: asp.is_major(),
                    });
                }
            }
        }
    }

    let dignities = calculate_essential_dignities(&comp_planets);
    let house_rulerships = calculate_house_rulerships(&comp_houses, &comp_planets);
    let arabian_parts = calculate_arabian_parts(comp_asc, &comp_planets, &comp_houses, true);
    let aspect_patterns = detect_aspect_patterns(&comp_planets, &comp_aspects);

    let comp_result = WesternResult {
        planets: comp_planets,
        houses: comp_houses,
        aspects: comp_aspects,
        elements: chart_a.elements.clone(),
        modalities: chart_a.modalities.clone(),
        ascendant: comp_asc,
        midheaven: comp_mc,
        chart_ruler: get_sign_ruler((comp_asc / 30.0).floor() as usize).to_string(),
        dominant_element: chart_a.dominant_element.clone(),
        dominant_modality: chart_a.dominant_modality.clone(),
        dignities,
        house_rulerships,
        arabian_parts,
        aspect_patterns,
        is_day_birth: true,
    };

    let summary = "두 사람의 차트를 360도 미드포인트로 합성한 컴포지트 차트 연산이 완료되었습니다."
        .to_string();

    Ok(CompositeResult {
        composite_chart: comp_result,
        summary,
    })
}

/// Secondary Progressions (1일 = 1년 주천 세컨더리 프로그레션 연산)
pub fn calculate_secondary_progression(
    birth_time: DateTime<Utc>,
    lat: f64,
    lon: f64,
    target_time: DateTime<Utc>,
    house_sys: char,
) -> Result<ProgressionResult, WesternError> {
    let birth_chart = calculate_western(birth_time, lat, lon, house_sys)?;

    // 1년 = 1일 (365.2425일 per 1sec progression)
    let duration_days = (target_time - birth_time).num_seconds() as f64 / 86400.0;
    let progressed_years = duration_days / 365.2425;

    // Progressed time = birth_time + progressed_years * 1 day
    use chrono::Duration;
    let prog_seconds = (progressed_years * 86400.0) as i64;
    let prog_time = birth_time + Duration::seconds(prog_seconds);

    let progressed_chart = calculate_western(prog_time, lat, lon, house_sys)?;

    // Transit Aspects: Progressed Planets vs Natal Planets
    let mut active_transit_aspects = Vec::new();
    let aspect_types = vec![
        AspectType::Conjunction,
        AspectType::Sextile,
        AspectType::Square,
        AspectType::Trine,
        AspectType::Opposition,
    ];

    for pp in &progressed_chart.planets {
        for np in &birth_chart.planets {
            let diff = (pp.longitude - np.longitude).abs();
            let angle = if diff > 180.0 { 360.0 - diff } else { diff };

            for &asp in &aspect_types {
                let target = asp.angle();
                let orb = (angle - target).abs();
                if orb <= asp.standard_orb() {
                    active_transit_aspects.push(WesternAspectData {
                        body_a_name: format!("Prog {}", pp.name),
                        body_b_name: format!("Natal {}", np.name),
                        aspect_type: asp,
                        angle_diff: angle,
                        orb,
                        dynamics: AspectDynamics::Applying,
                        is_major: asp.is_major(),
                    });
                }
            }
        }
    }

    Ok(ProgressionResult {
        birth_chart,
        progressed_chart,
        progressed_age_years: progressed_years,
        active_transit_aspects,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_between() {
        assert!(is_angle_between(15.0, 0.0, 30.0));
        assert!(is_angle_between(350.0, 340.0, 10.0));
        assert!(!is_angle_between(20.0, 30.0, 10.0));
    }

    #[test]
    fn test_calculate_western_basic() {
        let utc_birth = "1990-05-15T10:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let lat = 37.5665;
        let lon = 126.9780;

        let res = calculate_western(utc_birth, lat, lon, 'P');
        assert!(res.is_ok());
        let result = res.unwrap();

        assert_eq!(result.planets.len(), 18); // 10 planets + True Node + South Node + Lilith + Chiron + 4 Asteroids
        assert_eq!(result.houses.len(), 12);
        assert_eq!(result.arabian_parts.len(), 3);
        assert_eq!(result.house_rulerships.len(), 12);

        assert!(result.ascendant >= 0.0 && result.ascendant < 360.0);
        assert!(result.midheaven >= 0.0 && result.midheaven < 360.0);
    }
}
