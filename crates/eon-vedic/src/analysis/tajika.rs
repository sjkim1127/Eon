use crate::chart::VedicChart;
use crate::planets::VedicPlanet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TajikaAspectType {
    Mitra(bool),  // Friend (true = Very Friendly, false = Friendly)
    Shatru(bool), // Enemy (true = Very Hostile, false = Hostile)
    Sama,         // Neutral
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Saham {
    pub name: String,
    pub longitude: f64,
    pub rasi: u8,
}

pub struct TajikaEngine;

impl TajikaEngine {
    /// Calculate the Muntha (Annual Progressed Point)
    /// Rule: Muntha starts at Lagna at birth and progresses 1 sign per year.
    /// Muntha = (Birth Lagna Sign + Years Elapsed - 1) % 12 + 1
    pub fn calculate_muntha(birth_lagna_rasi: u8, age_years: u32) -> u8 {
        let rasi = (birth_lagna_rasi as u32 + age_years - 1) % 12 + 1;
        rasi as u8
    }

    /// Tajika Aspects (Drishti)
    /// Friendly: 3, 5, 9, 11 houses apart
    /// Hostile: 1, 4, 7, 10 houses apart
    /// Neutral: 2, 6, 8, 12 houses apart
    pub fn get_aspect_type(house_diff_1_indexed: u8) -> TajikaAspectType {
        match house_diff_1_indexed {
            3 | 11 => TajikaAspectType::Mitra(false),  // Friendly
            5 | 9 => TajikaAspectType::Mitra(true),    // Very Friendly
            1 | 7 => TajikaAspectType::Shatru(true),   // Very Hostile
            4 | 10 => TajikaAspectType::Shatru(false), // Hostile
            _ => TajikaAspectType::Sama,               // Neutral
        }
    }

    /// Ithasala Yoga (Applying Aspect)
    pub fn check_ithasala(
        p1_long: f64,
        p1_speed: f64,
        p2_long: f64,
        p2_speed: f64,
        orb: f64,
    ) -> bool {
        let (faster_long, _faster_speed, slower_long, _slower_speed) = if p1_speed > p2_speed {
            (p1_long, p1_speed, p2_long, p2_speed)
        } else {
            (p2_long, p2_speed, p1_long, p1_speed)
        };

        let diff = (slower_long - faster_long + 360.0) % 360.0;
        diff > 0.0 && diff <= orb
    }

    /// Calculate Sahams (Sensitive Points in Annual Chart)
    pub fn calculate_sahams(chart: &VedicChart) -> Vec<Saham> {
        let mut results = Vec::new();
        let lagna = chart.ascendant.sidereal_deg;

        let get_p = |p: VedicPlanet| {
            chart.planets.iter().find(|pos| pos.planet == p).map(|pos| pos.sidereal_deg).unwrap_or(0.0)
        };

        let sun = get_p(VedicPlanet::Sun);
        let moon = get_p(VedicPlanet::Moon);
        let _mars = get_p(VedicPlanet::Mars);
        let _mercury = get_p(VedicPlanet::Mercury);
        let jupiter = get_p(VedicPlanet::Jupiter);
        let _venus = get_p(VedicPlanet::Venus);
        let _saturn = get_p(VedicPlanet::Saturn);

        let is_day = chart.panchanga.is_day_birth;

        // Punya Saham (Fortune)
        // Day: Moon - Sun + Lagna | Night: Sun - Moon + Lagna
        let punya = if is_day {
            (moon - sun + lagna + 360.0) % 360.0
        } else {
            (sun - moon + lagna + 360.0) % 360.0
        };
        results.push(Saham { name: "Punya (Fortune)".to_string(), longitude: punya, rasi: (punya / 30.0).floor() as u8 + 1 });

        // Vidya Saham (Education)
        // Day: Sun - Moon + Lagna | Night: Moon - Sun + Lagna
        let vidya = if is_day {
            (sun - moon + lagna + 360.0) % 360.0
        } else {
            (moon - sun + lagna + 360.0) % 360.0
        };
        results.push(Saham { name: "Vidya (Knowledge)".to_string(), longitude: vidya, rasi: (vidya / 30.0).floor() as u8 + 1 });

        // Yash Saham (Fame)
        // Jupiter - Sun + Lagna (Commonly used)
        let yash = (jupiter - sun + lagna + 360.0) % 360.0;
        results.push(Saham { name: "Yash (Fame)".to_string(), longitude: yash, rasi: (yash / 30.0).floor() as u8 + 1 });

        results
    }
}

pub struct TajikaBala;

impl TajikaBala {
    /// Harsha Bala (4 factors of Joy)
    pub fn calculate_harsha_bala(chart: &VedicChart, planet: VedicPlanet) -> u32 {
        let mut score = 0;
        let p_pos = chart.planets.iter().find(|p| p.planet == planet);

        if let Some(p) = p_pos {
            // 1. Sthana (House)
            let h = p.house_index;
            match planet {
                VedicPlanet::Sun => if h == 9 { score += 5; },
                VedicPlanet::Moon => if h == 4 { score += 5; },
                VedicPlanet::Mars => if h == 6 { score += 5; },
                VedicPlanet::Mercury => if h == 1 { score += 5; },
                VedicPlanet::Jupiter => if h == 11 { score += 5; },
                VedicPlanet::Venus => if h == 5 { score += 5; },
                VedicPlanet::Saturn => if h == 12 { score += 5; },
                _ => {}
            }

            // 2. Swavarga (Sign/Varga)
            // Simplified check

            // 3. Stri-Purusha (Gender/Sect)
            // Simplified: Day/Night birth joy
            let is_day = chart.panchanga.is_day_birth;
            match planet {
                VedicPlanet::Sun | VedicPlanet::Mars | VedicPlanet::Jupiter => if is_day { score += 5; },
                VedicPlanet::Moon | VedicPlanet::Venus | VedicPlanet::Saturn => if !is_day { score += 5; },
                _ => {}
            }
        }
        score
    }
}
