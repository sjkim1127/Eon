//! eon-western: 서양 점성술 명리 연산 엔진
//!
//! eon-astro의 Swiss Ephemeris FFI를 활용하여 10대 행성 및 Chiron, True Node의 황경과
//! Placidus, Koch, Whole Sign, Equal House 등의 Cusp 좌표, 아스펙트를 산출하고 성향 지표를 분석합니다.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use eon_astro::{AstroEngine, AstroError};

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
    Conjunction, // 0도
    Sextile,     // 60도
    Square,      // 90도
    Trine,       // 120도
    Opposition,  // 180도
}

impl AspectType {
    pub fn angle(self) -> f64 {
        match self {
            AspectType::Conjunction => 0.0,
            AspectType::Sextile => 60.0,
            AspectType::Square => 90.0,
            AspectType::Trine => 120.0,
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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WesternAspectData {
    pub body_a_name: String,
    pub body_b_name: String,
    pub aspect_type: AspectType,
    pub angle_diff: f64,
    pub orb: f64,
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
}

pub const SIGN_NAMES: [&str; 12] = [
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"
];

pub fn get_sign_ruler(sign_idx: usize) -> &'static str {
    match sign_idx {
        0 => "Mars",        // Aries
        1 => "Venus",       // Taurus
        2 => "Mercury",     // Gemini
        3 => "Moon",        // Cancer
        4 => "Sun",         // Leo
        5 => "Mercury",     // Virgo
        6 => "Venus",       // Libra
        7 => "Pluto",       // Scorpio
        8 => "Jupiter",     // Sagittarius
        9 => "Saturn",      // Capricorn
        10 => "Uranus",     // Aquarius
        11 => "Neptune",    // Pisces
        _ => "Mars",
    }
}

/// 특정 각도가 start와 end의 섹터 구간 안에 속하는지 판정 (각도 wrap-around 360도 대응)
pub fn is_angle_between(target: f64, start: f64, end: f64) -> bool {
    let t = (target - start + 360.0) % 360.0;
    let e = (end - start + 360.0) % 360.0;
    t < e
}

pub fn calculate_western(
    datetime: DateTime<Utc>,
    latitude: f64,
    longitude: f64,
    house_system_char: char, // 'P', 'K', 'W', 'E' 등
) -> Result<WesternResult, WesternError> {
    let engine = AstroEngine::new();

    // 1. 하우스 Cusp 좌표 및 ASC/MC 계산
    // Swiss Ephemeris에 맞춤: 'W' (Whole Sign)이나 'E' (Equal)는 자체 보정 또는 char 전달
    let house_sys_byte = house_system_char as i32;
    let (mut cusps, ascmc) = engine.get_houses(datetime, latitude, longitude, house_sys_byte)?;
    
    let asc = ascmc[0];
    let mc = ascmc[1];

    // 만약 Whole Sign 시스템인 경우, ASC의 사인 시작점(0도)을 기준으로 균등하게 12사인 cusps를 재구축합니다.
    if house_system_char == 'W' {
        let asc_sign = (asc / 30.0).floor() as usize;
        for i in 0..12 {
            cusps[i] = ((asc_sign + i) % 12) as f64 * 30.0;
        }
    } else if house_system_char == 'E' {
        // Equal House: ASC를 시작점으로 30도씩 분할
        for i in 0..12 {
            cusps[i] = (asc + i as f64 * 30.0) % 360.0;
        }
    }

    // 2. 12대 주요 행성/요소 위치 조회
    // 0: Sun, 1: Moon, 2: Mercury, 3: Venus, 4: Mars, 5: Jupiter, 6: Saturn, 7: Uranus, 8: Neptune, 9: Pluto, 15: Chiron, 11: True Node
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
        (15, "Chiron"),
        (11, "True Node"),
    ];

    let mut planets = Vec::new();
    let flag = 2; // SEFLG_SWIEPH (Tropical)

    for (id, name) in bodies {
        let (long, speed) = match engine.get_planet_full(datetime, id, flag) {
            Ok(val) => val,
            Err(e) => {
                if id == 15 {
                    (0.0, 0.0) // Fallback for Chiron if ephemeris file is not available
                } else {
                    return Err(WesternError::Astro(e));
                }
            }
        };
        let sign_index = (long / 30.0).floor() as usize;
        let degree_in_sign = long % 30.0;
        let is_retrograde = speed < 0.0;

        // 해당 행성이 어느 하우스(1~12)에 속해 있는지 판정
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

    // 4. 행성 및 주요 각(ASC/MC) 간의 아스펙트 계산
    let mut aspects = Vec::new();
    let aspect_types = vec![
        AspectType::Conjunction,
        AspectType::Sextile,
        AspectType::Square,
        AspectType::Trine,
        AspectType::Opposition,
    ];

    // 아스펙트 비교 대상에 ASC, MC 추가
    let mut aspect_bodies = planets.iter()
        .map(|p| (p.name.clone(), p.longitude))
        .collect::<Vec<_>>();
    aspect_bodies.push(("ASC".to_string(), asc));
    aspect_bodies.push(("MC".to_string(), mc));

    for i in 0..aspect_bodies.len() {
        for j in (i + 1)..aspect_bodies.len() {
            let (ref name_a, long_a) = aspect_bodies[i];
            let (ref name_b, long_b) = aspect_bodies[j];

            let diff = (long_a - long_b).abs();
            let angle = if diff > 180.0 { 360.0 - diff } else { diff };

            for &asp in &aspect_types {
                let target = asp.angle();
                let orb = (angle - target).abs();
                if orb <= asp.standard_orb() {
                    aspects.push(WesternAspectData {
                        body_a_name: name_a.clone(),
                        body_b_name: name_b.clone(),
                        aspect_type: asp,
                        angle_diff: angle,
                        orb,
                    });
                }
            }
        }
    }

    // 5. 성향 지표 점수 가중치 산출 (Fire, Earth, Air, Water 및 Cardinal, Fixed, Mutable)
    let mut elements = ElementDistribution::default();
    let mut modalities = ModalityDistribution::default();
    
    // 가중치 매핑: Sun, Moon (3점), Mercury, Venus, Mars (2점), ASC, MC (2점), Jupiter, Saturn, Uranus, Neptune, Pluto (1점)
    let weight_mapping = |name: &str| -> f64 {
        match name {
            "Sun" | "Moon" => 3.0,
            "Mercury" | "Venus" | "Mars" | "ASC" | "MC" => 2.0,
            "Jupiter" | "Saturn" | "Uranus" | "Neptune" | "Pluto" => 1.0,
            _ => 0.0, // Chiron, Node 등은 계산 제외
        }
    };

    // 행성 분포 점수
    for p in &planets {
        let w = weight_mapping(&p.name);
        if w > 0.0 {
            // 원소 판정
            match p.sign_index {
                0 | 4 | 8 => elements.fire += w,
                1 | 5 | 9 => elements.earth += w,
                2 | 6 | 10 => elements.air += w,
                3 | 7 | 11 => elements.water += w,
                _ => {}
            }
            // 양태 판정
            match p.sign_index {
                0 | 3 | 6 | 9 => modalities.cardinal += w,
                1 | 4 | 7 | 10 => modalities.fixed += w,
                2 | 5 | 8 | 11 => modalities.mutable += w,
                _ => {}
            }
        }
    }

    // ASC, MC 분포 점수 가중치 추가
    let add_point = |long: f64, name: &str, el: &mut ElementDistribution, mo: &mut ModalityDistribution| {
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

    // 백분율 환산
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

    // 우세 원소/모달리티 문자열 도출
    let mut el_vec = vec![
        ("Fire", elements.fire),
        ("Earth", elements.earth),
        ("Air", elements.air),
        ("Water", elements.water),
    ];
    el_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let dominant_element = el_vec[0].0.to_string();

    let mut mo_vec = vec![
        ("Cardinal", modalities.cardinal),
        ("Fixed", modalities.fixed),
        ("Mutable", modalities.mutable),
    ];
    mo_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let dominant_modality = mo_vec[0].0.to_string();

    // Chart Ruler 구하기 (ASC 사인의 Ruler)
    let asc_sign = (asc / 30.0).floor() as usize;
    let chart_ruler = get_sign_ruler(asc_sign).to_string();

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
        // May 15, 1990 at 10:00 AM UTC (approx KST 19:00 or standard UTC)
        let utc_birth = "1990-05-15T10:00:00Z".parse::<DateTime<Utc>>().unwrap();
        // Seoul coordinates
        let lat = 37.5665;
        let lon = 126.9780;
        
        let res = calculate_western(utc_birth, lat, lon, 'P');
        assert!(res.is_ok());
        let result = res.unwrap();
        
        // Check standard outputs
        assert_eq!(result.planets.len(), 12);
        assert_eq!(result.houses.len(), 12);
        
        // Assert Ascendant and Midheaven coordinates are valid
        assert!(result.ascendant >= 0.0 && result.ascendant < 360.0);
        assert!(result.midheaven >= 0.0 && result.midheaven < 360.0);

        // Placidus house check: planet should belong to valid house range (1..=12)
        for p in &result.planets {
            assert!(p.house_number >= 1 && p.house_number <= 12);
        }
    }
}
