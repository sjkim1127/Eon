use crate::core::chart::VedicPosition;
use crate::planets::VedicPlanet;
use chrono::{DateTime, Utc};
use eon_astro::AstroEngine;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KpPoint {
    pub name: String,
    pub longitude: f64,
    pub rasi: u8,
    pub nakshatra: u8,
    pub pada: u8,
    pub sign_lord: VedicPlanet,
    pub star_lord: VedicPlanet,
    pub sub_lord: VedicPlanet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KpSignificator {
    pub planet: VedicPlanet,
    pub occupied_house: u8,
    pub owned_houses: Vec<u8>,
    pub star_lord_occupied: u8,
    pub star_lord_owned: Vec<u8>,
    pub level1: Vec<u8>,
    pub level2: Vec<u8>,
    pub level3: Vec<u8>,
    pub level4: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KpAnalysis {
    pub cusps: Vec<KpPoint>,
    pub planets: Vec<KpPoint>,
    pub significators: Vec<KpSignificator>,
}

impl KpAnalysis {
    pub fn calculate(
        time: DateTime<Utc>,
        latitude: f64,
        longitude: f64,
        ayanamsa: f64,
        natal_planets: &[VedicPosition],
        engine: &AstroEngine,
    ) -> Result<Self, String> {
        // 1. Calculate Placidus unequal house cusps (fallback to Koch, Porphyry, or Equal if Placidus fails at polar latitudes)
        let (cusps, _) = engine
            .get_houses(time, latitude, longitude, b'P' as i32)
            .or_else(|_| engine.get_houses(time, latitude, longitude, b'K' as i32))
            .or_else(|_| engine.get_houses(time, latitude, longitude, b'O' as i32))
            .or_else(|_| engine.get_houses(time, latitude, longitude, b'E' as i32))
            .map_err(|e| e.to_string())?;

        let mut kp_cusps = Vec::new();
        for (i, &raw_c) in cusps.iter().enumerate() {
            let sidereal = (raw_c - ayanamsa + 360.0) % 360.0;
            let (rasi, nak, pada, sign_l, star_l, sub_l) = calculate_lords(sidereal);
            kp_cusps.push(KpPoint {
                name: format!("Cusp {}", i + 1),
                longitude: sidereal,
                rasi,
                nakshatra: nak,
                pada,
                sign_lord: sign_l,
                star_lord: star_l,
                sub_lord: sub_l,
            });
        }

        // 2. Map natal planets to KP points
        let mut kp_planets = Vec::new();
        for p in natal_planets {
            let (rasi, nak, pada, sign_l, star_l, sub_l) = calculate_lords(p.sidereal_deg);
            kp_planets.push(KpPoint {
                name: get_planet_name(p.planet),
                longitude: p.sidereal_deg,
                rasi,
                nakshatra: nak,
                pada,
                sign_lord: sign_l,
                star_lord: star_l,
                sub_lord: sub_l,
            });
        }

        // 3. Calculate Significators
        let cusp_longs: Vec<f64> = kp_cusps.iter().map(|c| c.longitude).collect();
        let name_to_planet = |name: &str| -> Option<VedicPlanet> {
            match name {
                "Sun" => Some(VedicPlanet::Sun),
                "Moon" => Some(VedicPlanet::Moon),
                "Mars" => Some(VedicPlanet::Mars),
                "Mercury" => Some(VedicPlanet::Mercury),
                "Jupiter" => Some(VedicPlanet::Jupiter),
                "Venus" => Some(VedicPlanet::Venus),
                "Saturn" => Some(VedicPlanet::Saturn),
                "Rahu" => Some(VedicPlanet::Rahu),
                "Ketu" => Some(VedicPlanet::Ketu),
                "Ascendant" => Some(VedicPlanet::Ascendant),
                _ => None,
            }
        };

        let get_planet_long = |planet: VedicPlanet, planets_list: &[KpPoint]| -> f64 {
            planets_list
                .iter()
                .find(|kp_p| name_to_planet(&kp_p.name) == Some(planet))
                .map(|kp_p| kp_p.longitude)
                .unwrap_or(0.0)
        };

        let owned_houses_of = |planet: VedicPlanet, cusps_list: &[KpPoint]| -> Vec<u8> {
            let mut houses = Vec::new();
            for (idx, cusp) in cusps_list.iter().enumerate() {
                if cusp.sign_lord == planet {
                    houses.push((idx + 1) as u8);
                }
            }
            houses
        };

        let mut significators = Vec::new();
        let target_planets = [
            VedicPlanet::Sun,
            VedicPlanet::Moon,
            VedicPlanet::Mars,
            VedicPlanet::Mercury,
            VedicPlanet::Jupiter,
            VedicPlanet::Venus,
            VedicPlanet::Saturn,
            VedicPlanet::Rahu,
            VedicPlanet::Ketu,
        ];

        for &p in &target_planets {
            let p_long = get_planet_long(p, &kp_planets);
            let occupied_house = get_kp_house(p_long, &cusp_longs);
            let owned_houses = owned_houses_of(p, &kp_cusps);

            // Find planet's star lord
            let star_lord = kp_planets
                .iter()
                .find(|kp_p| name_to_planet(&kp_p.name) == Some(p))
                .map(|kp_p| kp_p.star_lord)
                .unwrap_or(VedicPlanet::Sun);

            let star_lord_long = get_planet_long(star_lord, &kp_planets);
            let star_lord_occupied = get_kp_house(star_lord_long, &cusp_longs);
            let star_lord_owned = owned_houses_of(star_lord, &kp_cusps);

            let level1 = vec![star_lord_occupied];
            let level2 = vec![occupied_house];
            let level3 = star_lord_owned.clone();
            let level4 = owned_houses.clone();

            significators.push(KpSignificator {
                planet: p,
                occupied_house,
                owned_houses,
                star_lord_occupied,
                star_lord_owned,
                level1,
                level2,
                level3,
                level4,
            });
        }

        Ok(Self {
            cusps: kp_cusps,
            planets: kp_planets,
            significators,
        })
    }
}

fn get_kp_house(deg: f64, cusps: &[f64]) -> u8 {
    if cusps.len() < 12 {
        return 1;
    }
    for i in 0..11 {
        let c1 = cusps[i];
        let c2 = cusps[i + 1];
        if c2 > c1 {
            if deg >= c1 && deg < c2 {
                return (i + 1) as u8;
            }
        } else {
            if deg >= c1 || deg < c2 {
                return (i + 1) as u8;
            }
        }
    }
    let c12 = cusps[11];
    let c1 = cusps[0];
    if c1 > c12 {
        if deg >= c12 && deg < c1 {
            return 12;
        }
    } else {
        if deg >= c12 || deg < c1 {
            return 12;
        }
    }
    12
}

fn calculate_lords(sidereal: f64) -> (u8, u8, u8, VedicPlanet, VedicPlanet, VedicPlanet) {
    let rasi = ((sidereal / 30.0).floor() as u8 % 12) + 1;

    let nak_pos = sidereal / (360.0 / 27.0);
    let nak = (nak_pos.floor() as u8) + 1;

    let pada_pos = (sidereal % (360.0 / 27.0)) / (360.0 / 108.0);
    let pada = (pada_pos.floor() as u8) + 1;

    let sign_lord = get_sign_lord(rasi);
    let star_lord = get_nakshatra_lord(nak);

    // Compute Sub-Lord
    let deg_within_nak = sidereal % (360.0 / 27.0);
    let sub_lord = get_sub_lord(nak, deg_within_nak);

    (rasi, nak, pada, sign_lord, star_lord, sub_lord)
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
        _ => VedicPlanet::Sun, // Fallback
    }
}

fn get_nakshatra_lord(nakshatra: u8) -> VedicPlanet {
    let lords = [
        VedicPlanet::Ketu,
        VedicPlanet::Venus,
        VedicPlanet::Sun,
        VedicPlanet::Moon,
        VedicPlanet::Mars,
        VedicPlanet::Rahu,
        VedicPlanet::Jupiter,
        VedicPlanet::Saturn,
        VedicPlanet::Mercury,
    ];
    lords[((nakshatra - 1) % 9) as usize]
}

fn get_sub_lord(nakshatra: u8, deg_within_nakshatra: f64) -> VedicPlanet {
    let sequence = [
        (VedicPlanet::Ketu, 7.0),
        (VedicPlanet::Venus, 20.0),
        (VedicPlanet::Sun, 6.0),
        (VedicPlanet::Moon, 10.0),
        (VedicPlanet::Mars, 7.0),
        (VedicPlanet::Rahu, 18.0),
        (VedicPlanet::Jupiter, 16.0),
        (VedicPlanet::Saturn, 19.0),
        (VedicPlanet::Mercury, 17.0),
    ];
    let start_idx = ((nakshatra - 1) % 9) as usize;
    let mut current_offset = 0.0;
    let width_factor = 360.0 / 27.0; // 13.3333333

    for i in 0..9 {
        let idx = (start_idx + i) % 9;
        let (planet, years) = sequence[idx];
        let width = width_factor * (years / 120.0);
        if deg_within_nakshatra >= current_offset && deg_within_nakshatra < current_offset + width {
            return planet;
        }
        current_offset += width;
    }
    sequence[start_idx].0
}

fn get_planet_name(planet: VedicPlanet) -> String {
    match planet {
        VedicPlanet::Sun => "Sun".to_string(),
        VedicPlanet::Moon => "Moon".to_string(),
        VedicPlanet::Mars => "Mars".to_string(),
        VedicPlanet::Mercury => "Mercury".to_string(),
        VedicPlanet::Jupiter => "Jupiter".to_string(),
        VedicPlanet::Venus => "Venus".to_string(),
        VedicPlanet::Saturn => "Saturn".to_string(),
        VedicPlanet::Rahu => "Rahu".to_string(),
        VedicPlanet::Ketu => "Ketu".to_string(),
        VedicPlanet::Ascendant => "Ascendant".to_string(),
    }
}
