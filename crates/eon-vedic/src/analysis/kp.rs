use serde::{Deserialize, Serialize};
use crate::planets::VedicPlanet;
use crate::core::chart::VedicPosition;
use chrono::{DateTime, Utc};
use eon_astro::AstroEngine;

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
pub struct KpAnalysis {
    pub cusps: Vec<KpPoint>,
    pub planets: Vec<KpPoint>,
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
        // 1. Calculate Placidus unequal house cusps
        let (cusps, _) = engine
            .get_houses(time, latitude, longitude, b'P' as i32)
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

        Ok(Self {
            cusps: kp_cusps,
            planets: kp_planets,
        })
    }
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
