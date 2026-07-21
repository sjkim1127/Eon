use crate::chart::VedicChart;
use crate::planets::VedicPlanet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TajikaAspectType {
    Mitra(bool),  // Friend (true = Very Friendly, false = Friendly)
    Shatru(bool), // Enemy (true = Very Hostile, false = Hostile)
    Sama,         // Neutral
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Saham {
    pub name: String,
    pub longitude: f64,
    pub rasi: u8,
}

pub struct TajikaEngine;

impl TajikaEngine {
    /// Calculate the Muntha (Annual Progressed Point)
    pub fn calculate_muntha(birth_lagna_rasi: u8, age_years: u32) -> u8 {
        let rasi = (birth_lagna_rasi as u32 + age_years - 1) % 12 + 1;
        rasi as u8
    }

    /// Tajika Aspects (Drishti)
    pub fn get_aspect_type(house_diff_1_indexed: u8) -> TajikaAspectType {
        match house_diff_1_indexed {
            3 | 11 => TajikaAspectType::Mitra(false),
            5 | 9 => TajikaAspectType::Mitra(true),
            1 | 7 => TajikaAspectType::Shatru(true),
            4 | 10 => TajikaAspectType::Shatru(false),
            _ => TajikaAspectType::Sama,
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
            chart
                .planets
                .iter()
                .find(|pos| pos.planet == p)
                .map(|pos| pos.sidereal_deg)
                .unwrap_or(0.0)
        };

        let sun = get_p(VedicPlanet::Sun);
        let moon = get_p(VedicPlanet::Moon);
        let jupiter = get_p(VedicPlanet::Jupiter);

        let is_day = chart.panchanga.is_day_birth;

        // Punya Saham (Fortune)
        let punya = if is_day {
            (moon - sun + lagna + 360.0) % 360.0
        } else {
            (sun - moon + lagna + 360.0) % 360.0
        };
        results.push(Saham {
            name: "Punya (Fortune)".to_string(),
            longitude: punya,
            rasi: (punya / 30.0).floor() as u8 + 1,
        });

        // Vidya Saham (Knowledge)
        let vidya = if is_day {
            (sun - moon + lagna + 360.0) % 360.0
        } else {
            (moon - sun + lagna + 360.0) % 360.0
        };
        results.push(Saham {
            name: "Vidya (Knowledge)".to_string(),
            longitude: vidya,
            rasi: (vidya / 30.0).floor() as u8 + 1,
        });

        // Yash Saham (Fame)
        let yash = (jupiter - sun + lagna + 360.0) % 360.0;
        results.push(Saham {
            name: "Yash (Fame)".to_string(),
            longitude: yash,
            rasi: (yash / 30.0).floor() as u8 + 1,
        });

        results
    }

    /// Tri-Rashi Pati Selection Table
    /// Returns the lord based on Annual Lagna sign and Day/Night birth.
    fn get_tri_rashi_pati(lagna_rasi: u8, is_day: bool) -> VedicPlanet {
        match (lagna_rasi, is_day) {
            (1, true) => VedicPlanet::Sun,
            (1, false) => VedicPlanet::Jupiter,
            (2, true) => VedicPlanet::Venus,
            (2, false) => VedicPlanet::Moon,
            (3, true) => VedicPlanet::Saturn,
            (3, false) => VedicPlanet::Mercury,
            (4, true) => VedicPlanet::Venus,
            (4, false) => VedicPlanet::Mars,
            (5, true) => VedicPlanet::Jupiter,
            (5, false) => VedicPlanet::Sun,
            (6, true) => VedicPlanet::Moon,
            (6, false) => VedicPlanet::Venus,
            (7, true) => VedicPlanet::Mercury,
            (7, false) => VedicPlanet::Saturn,
            (8, true) => VedicPlanet::Mars,
            (8, false) => VedicPlanet::Venus,
            (9, true) => VedicPlanet::Saturn,
            (9, false) => VedicPlanet::Mercury,
            (10, true) => VedicPlanet::Mars,
            (10, false) => VedicPlanet::Moon,
            (11, true) => VedicPlanet::Jupiter,
            (11, false) => VedicPlanet::Sun,
            (12, true) => VedicPlanet::Moon,
            (12, false) => VedicPlanet::Mars,
            _ => VedicPlanet::Sun,
        }
    }

    /// Selection of Year Lord (Varsheshwara) - Full Orthodox Tajika Implementation
    pub fn select_year_lord(
        chart: &VedicChart,
        birth_lagna_rasi: u8,
        age_years: u32,
    ) -> VedicPlanet {
        let annual_lagna_rasi = chart.ascendant.rasi;
        let muntha_rasi = Self::calculate_muntha(birth_lagna_rasi, age_years);
        let is_day = chart.panchanga.is_day_birth;

        // 1. Pancha Adhikaris (5 Candidates)
        let mut candidates = Vec::new();

        // 1.1 Muntha Lord
        candidates.push(VedicPlanet::get_ruler_of(muntha_rasi));

        // 1.2 Birth Lagna Lord
        candidates.push(VedicPlanet::get_ruler_of(birth_lagna_rasi));

        // 1.3 Varsha (Annual) Lagna Lord
        candidates.push(VedicPlanet::get_ruler_of(annual_lagna_rasi));

        // 1.4 Dina/Ratri Pati (Day/Night Lord)
        if is_day {
            if let Some(sun) = chart.planets.iter().find(|p| p.planet == VedicPlanet::Sun) {
                candidates.push(VedicPlanet::get_ruler_of(sun.rasi));
            }
        } else {
            if let Some(moon) = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon) {
                candidates.push(VedicPlanet::get_ruler_of(moon.rasi));
            }
        }

        // 1.5 Tri-Rashi Pati
        candidates.push(Self::get_tri_rashi_pati(annual_lagna_rasi, is_day));

        // 2. Filter candidates who aspect the Annual Lagna
        // (In Tajika, any aspect makes it eligible)
        let eligible: Vec<VedicPlanet> = candidates
            .into_iter()
            .filter(|&p| {
                if let Some(pos) = chart.planets.iter().find(|pos| pos.planet == p) {
                    let dist = (pos.rasi as i16 - annual_lagna_rasi as i16 + 12) % 12;
                    let aspect = Self::get_aspect_type((dist + 1) as u8);
                    !matches!(aspect, TajikaAspectType::Sama)
                        || p == VedicPlanet::get_ruler_of(annual_lagna_rasi)
                } else {
                    false
                }
            })
            .collect();

        // 3. Selection: Strongest among eligible by Harsha Bala
        if eligible.is_empty() {
            return VedicPlanet::get_ruler_of(muntha_rasi);
        }

        let mut strongest = eligible[0];
        let mut max_bala = 0;

        for p in eligible {
            let bala = TajikaBala::calculate_harsha_bala(chart, p);
            if bala >= max_bala {
                max_bala = bala;
                strongest = p;
            }
        }

        strongest
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
                VedicPlanet::Sun if h == 9 => {
                    score += 1;
                }
                VedicPlanet::Moon if h == 4 => {
                    score += 1;
                }
                VedicPlanet::Mars if h == 6 => {
                    score += 1;
                }
                VedicPlanet::Mercury if h == 1 => {
                    score += 1;
                }
                VedicPlanet::Jupiter if h == 11 => {
                    score += 1;
                }
                VedicPlanet::Venus if h == 5 => {
                    score += 1;
                }
                VedicPlanet::Saturn if h == 12 => {
                    score += 1;
                }
                _ => {}
            }

            // 2. Swavarga (Own/Exaltation Sign in Annual Chart)
            let lord = VedicPlanet::get_ruler_of(p.rasi);
            if lord == planet || p.rasi == planet.exaltation_rasi() {
                score += 1;
            }

            // 3. Stri-Purusha (Gender/Sect)
            let is_day = chart.panchanga.is_day_birth;
            match planet {
                VedicPlanet::Sun | VedicPlanet::Mars | VedicPlanet::Jupiter if is_day => {
                    score += 1;
                }
                VedicPlanet::Moon | VedicPlanet::Venus | VedicPlanet::Saturn if !is_day => {
                    score += 1;
                }
                _ => {}
            }

            // 4. Appearance (In Kendra)
            if [1, 4, 7, 10].contains(&h) {
                score += 1;
            }
        }
        score
    }
}
