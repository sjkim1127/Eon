use crate::calc::ayanamsa::get_lahiri_ayanamsa;
use crate::core::config::{NodeCalculation, VedicConfig};
use crate::planets::VedicPlanet;
use chrono::{DateTime, Utc};
use eon_astro::AstroEngine;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VedicPosition {
    pub planet: VedicPlanet,
    pub tropical_deg: f64,
    pub sidereal_deg: f64,
    pub nakshatra: u8, // 1~27
    pub pada: u8,      // 1~4
    pub rasi: u8,      // 1~12 (Sign) - D1

    // Additional Info
    pub house_index: u8, // 1~12 (Bhava) - Calculated from Lagna
    pub speed: f64,      // Degrees per day
    pub is_retrograde: bool,
    pub is_combust: bool,
    pub declination: f64, // Equatorial declination

    // Varga Positions (Sign Index 1~12)
    pub hora_rasi: u8,             // D2
    pub drekkana_rasi: u8,         // D3
    pub chaturthamsha_rasi: u8,    // D4
    pub panchamsa_rasi: u8,        // D5
    pub saptamsa_rasi: u8,         // D7
    pub ashtamsa_rasi: u8,         // D8
    pub navamsa_rasi: u8,          // D9
    pub dasamsa_rasi: u8,          // D10
    pub rudramsa_rasi: u8,         // D11
    pub dwadasamsa_rasi: u8,       // D12
    pub shodashamsa_rasi: u8,      // D16
    pub vimsamsa_rasi: u8,         // D20
    pub chaturvimshamsa_rasi: u8,  // D24
    pub saptavimsamsa_rasi: u8,    // D27
    pub trimsamsa_rasi: u8,        // D30
    pub khavedamsa_rasi: u8,       // D40
    pub akshavedamsa_rasi: u8,     // D45
    pub shashtyamsa_rasi: u8,      // D60
    pub navanavamsa_rasi: u8,      // D81
    pub ashtottaramsa_rasi: u8,    // D108
    pub dwadasdwadasamsa_rasi: u8, // D144
}

impl VedicPosition {
    /// Get rasi (1..12) for the given varga chart type
    pub fn varga_rasi(&self, vt: crate::calc::varga::VargaType) -> u8 {
        use crate::calc::varga::VargaType;
        match vt {
            VargaType::D1 => self.rasi,
            VargaType::D2 => self.hora_rasi,
            VargaType::D3 => self.drekkana_rasi,
            VargaType::D4 => self.chaturthamsha_rasi,
            VargaType::D5 => self.panchamsa_rasi,
            VargaType::D7 => self.saptamsa_rasi,
            VargaType::D8 => self.ashtamsa_rasi,
            VargaType::D9 => self.navamsa_rasi,
            VargaType::D10 => self.dasamsa_rasi,
            VargaType::D11 => self.rudramsa_rasi,
            VargaType::D12 => self.dwadasamsa_rasi,
            VargaType::D16 => self.shodashamsa_rasi,
            VargaType::D20 => self.vimsamsa_rasi,
            VargaType::D24 => self.chaturvimshamsa_rasi,
            VargaType::D27 => self.saptavimsamsa_rasi,
            VargaType::D30 => self.trimsamsa_rasi,
            VargaType::D40 => self.khavedamsa_rasi,
            VargaType::D45 => self.akshavedamsa_rasi,
            VargaType::D60 => self.shashtyamsa_rasi,
            VargaType::D81 => self.navanavamsa_rasi,
            VargaType::D108 => self.ashtottaramsa_rasi,
            VargaType::D144 => self.dwadasdwadasamsa_rasi,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VedicChart {
    pub ascendant: VedicPosition,
    pub planets: Vec<VedicPosition>,
    pub aspects: Vec<crate::analysis::aspects::AspectRelation>,
    pub sav: crate::analysis::ashtakavarga::Sarvashtakavarga,
    pub bav: Vec<crate::analysis::ashtakavarga::AshtakavargaPoints>, // 행성별 BAV
    pub house_cusps: Vec<f64>,
    pub karakas: Vec<crate::analysis::jaimini::KarakaAssignment>,
    pub bhava_strengths: Vec<crate::analysis::bhava::BhavaStrength>,
    pub vimshopaka_scores: Vec<(VedicPlanet, crate::analysis::vimshopaka::VimshopakaScore)>,
    pub avasthas: Vec<crate::analysis::avasthas::PlanetAvastha>,
    pub panchanga: crate::panchanga::Panchanga,
    pub analysis_report: Option<crate::analysis::report::VedicAnalysisReport>,
}

pub struct VedicChartCalculator {
    engine: AstroEngine,
    config: VedicConfig,
}

impl Default for VedicChartCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl VedicChartCalculator {
    fn calculate_nakshatra_and_pada(sidereal: f64) -> (u8, u8) {
        let nak_pos = sidereal / (360.0 / 27.0);
        let nakshatra = (nak_pos.floor() as u8) + 1;

        let pada_pos = (sidereal % (360.0 / 27.0)) / (360.0 / 108.0);
        let pada = (pada_pos.floor() as u8) + 1;

        (nakshatra, pada)
    }

    pub fn new() -> Self {
        Self {
            engine: AstroEngine::new(),
            config: VedicConfig::default(),
        }
    }

    pub fn with_config(config: VedicConfig) -> Self {
        Self {
            engine: AstroEngine::new(),
            config,
        }
    }

    pub fn calculate(&self, time: DateTime<Utc>, latitude: f64, longitude: f64) -> VedicChart {
        let ayanamsa = get_lahiri_ayanamsa(&self.engine, time);

        let hsys = match self.config.house_system {
            crate::core::config::HouseSystem::WholeSign => b'W' as i32,
            crate::core::config::HouseSystem::Sripati => b'S' as i32,
        };

        let (cusps, ascmc) = self
            .engine
            .get_houses(time, latitude, longitude, hsys)
            .unwrap_or((vec![], [0.0; 10]));

        let sidereal_cusps: Vec<f64> = cusps
            .iter()
            .map(|c| (c - ayanamsa + 360.0) % 360.0)
            .collect();
        let asc_sidereal = (ascmc[0] - ayanamsa + 360.0) % 360.0;

        // Calculate Sandhis (Junctions) for Bhava Chalit
        let mut sandhis = Vec::new();
        if self.config.house_system == crate::core::config::HouseSystem::Sripati
            && !sidereal_cusps.is_empty()
        {
            for i in 0..12 {
                let c1 = sidereal_cusps[i];
                let c2 = sidereal_cusps[(i + 1) % 12];
                let mut diff = c2 - c1;
                if diff < 0.0 {
                    diff += 360.0;
                }
                sandhis.push((c1 + diff / 2.0) % 360.0);
            }
        }

        let asc_rasi = (asc_sidereal / 30.0).floor() as u8 + 1;

        // Helper to Create Position
        let create_position = |planet: VedicPlanet,
                               sidereal: f64,
                               tropical: f64,
                               speed: f64,
                               declination: f64,
                               sun_sidereal: Option<f64>|
         -> VedicPosition {
            let (nakshatra, pada) = Self::calculate_nakshatra_and_pada(sidereal);
            let rasi = (sidereal / 30.0).floor() as u8 + 1;

            // House Index Calculation
            let house_index = match self.config.house_system {
                crate::core::config::HouseSystem::WholeSign => {
                    // Simplified Modulo Arithmetic for Whole Sign
                    ((rasi as i32 - asc_rasi as i32 + 12) % 12) as u8 + 1
                }
                crate::core::config::HouseSystem::Sripati => {
                    let mut h = 0;
                    for i in 0..12 {
                        let s_start = sandhis[(i + 11) % 12];
                        let s_end = sandhis[i];

                        let in_house = if s_start < s_end {
                            sidereal >= s_start && sidereal < s_end
                        } else {
                            sidereal >= s_start || sidereal < s_end
                        };

                        if in_house {
                            h = i + 1;
                            break;
                        }
                    }
                    h as u8
                }
            };

            VedicPosition {
                planet,
                tropical_deg: tropical,
                sidereal_deg: sidereal,
                nakshatra,
                pada,
                rasi,
                house_index,
                speed,
                is_retrograde: speed < 0.0,
                is_combust: if let Some(sun_long) = sun_sidereal {
                    if planet == VedicPlanet::Sun {
                        false
                    } else {
                        let diff = (sidereal - sun_long).abs();
                        let d = if diff > 180.0 { 360.0 - diff } else { diff };
                        let limit = match planet {
                            VedicPlanet::Moon => 12.0,
                            VedicPlanet::Mars => 17.0,
                            VedicPlanet::Mercury => {
                                if speed < 0.0 {
                                    12.0
                                } else {
                                    14.0
                                }
                            }
                            VedicPlanet::Jupiter => 11.0,
                            VedicPlanet::Venus => 10.0,
                            VedicPlanet::Saturn => 15.0,
                            _ => 0.0,
                        };
                        d < limit
                    }
                } else {
                    false
                },
                declination,
                hora_rasi: crate::calc::varga::VargaType::D2.calculate_rasi(sidereal),
                drekkana_rasi: crate::calc::varga::VargaType::D3.calculate_rasi(sidereal),
                chaturthamsha_rasi: crate::calc::varga::VargaType::D4.calculate_rasi(sidereal),
                panchamsa_rasi: crate::calc::varga::VargaType::D5.calculate_rasi(sidereal),
                saptamsa_rasi: crate::calc::varga::VargaType::D7.calculate_rasi(sidereal),
                ashtamsa_rasi: crate::calc::varga::VargaType::D8.calculate_rasi(sidereal),
                navamsa_rasi: crate::calc::varga::VargaType::D9.calculate_rasi(sidereal),
                dasamsa_rasi: crate::calc::varga::VargaType::D10.calculate_rasi(sidereal),
                rudramsa_rasi: crate::calc::varga::VargaType::D11.calculate_rasi(sidereal),
                dwadasamsa_rasi: crate::calc::varga::VargaType::D12.calculate_rasi(sidereal),
                shodashamsa_rasi: crate::calc::varga::VargaType::D16.calculate_rasi(sidereal),
                vimsamsa_rasi: crate::calc::varga::VargaType::D20.calculate_rasi(sidereal),
                chaturvimshamsa_rasi: crate::calc::varga::VargaType::D24.calculate_rasi(sidereal),
                saptavimsamsa_rasi: crate::calc::varga::VargaType::D27.calculate_rasi(sidereal),
                trimsamsa_rasi: crate::calc::varga::VargaType::D30.calculate_rasi(sidereal),
                khavedamsa_rasi: crate::calc::varga::VargaType::D40.calculate_rasi(sidereal),
                akshavedamsa_rasi: crate::calc::varga::VargaType::D45.calculate_rasi(sidereal),
                shashtyamsa_rasi: crate::calc::varga::VargaType::D60.calculate_rasi(sidereal),
                navanavamsa_rasi: crate::calc::varga::VargaType::D81.calculate_rasi(sidereal),
                ashtottaramsa_rasi: crate::calc::varga::VargaType::D108.calculate_rasi(sidereal),
                dwadasdwadasamsa_rasi: crate::calc::varga::VargaType::D144.calculate_rasi(sidereal),
            }
        };

        // 0. Find Sun's Sidereal Longitude first for combustion check
        let (sun_trop, _) = self
            .engine
            .get_planet_full(time, VedicPlanet::Sun.se_id(), 256 | 2)
            .unwrap_or((0.0, 0.0));
        let (_, _sun_dec) = self
            .engine
            .get_planet_equatorial(time, VedicPlanet::Sun.se_id())
            .unwrap_or((0.0, 0.0));
        let sun_sidereal = (sun_trop - ayanamsa + 360.0) % 360.0;

        // 1. Create Ascendant Position
        // Ascendant declination is approximation or 0.0 (not usually used for strength)
        let asc_position = create_position(
            VedicPlanet::Ascendant,
            asc_sidereal,
            ascmc[0],
            0.0,
            0.0,
            Some(sun_sidereal),
        );

        let planets_names = [
            VedicPlanet::Sun,
            VedicPlanet::Moon,
            VedicPlanet::Mars,
            VedicPlanet::Mercury,
            VedicPlanet::Jupiter,
            VedicPlanet::Venus,
            VedicPlanet::Saturn,
            VedicPlanet::Rahu,
        ];

        let mut planets = Vec::new();
        for p in &planets_names {
            // Use config to decide Mean vs True Node for Rahu/Ketu
            let se_id = if *p == VedicPlanet::Rahu {
                match self.config.node_calc {
                    NodeCalculation::MeanNode => 10, // SE_MEAN_NODE
                    NodeCalculation::TrueNode => 11, // SE_TRUE_NODE
                }
            } else {
                p.se_id()
            };
            let flag = 256 | 2; // SEFLG_SPEED | SEFLG_SIDEREAL (or just standard)
            let (trop, speed) = self
                .engine
                .get_planet_full(time, se_id, flag)
                .unwrap_or((0.0, 0.0));
            let (_, dec) = self
                .engine
                .get_planet_equatorial(time, p.se_id())
                .unwrap_or((0.0, 0.0));
            let sidereal = (trop - ayanamsa + 360.0) % 360.0;
            planets.push(create_position(
                *p,
                sidereal,
                trop,
                speed,
                dec,
                Some(sun_sidereal),
            ));

            // Add Ketu opposite to Rahu
            if *p == VedicPlanet::Rahu {
                let ketu_sidereal = (sidereal + 180.0) % 360.0;
                let ketu_tropical = (trop + 180.0) % 360.0;
                let ketu_dec = -dec; // Ketu dec is opposite Rahu
                                     // Ketu's speed is the same as Rahu's (since it's exactly opposite)
                planets.push(create_position(
                    VedicPlanet::Ketu,
                    ketu_sidereal,
                    ketu_tropical,
                    speed,
                    ketu_dec,
                    Some(sun_sidereal),
                ));
            }
        }

        // Initialize Report option
        let analysis_report = None;

        // Calculate Panchanga
        let sun_deg = planets
            .iter()
            .find(|p| p.planet == VedicPlanet::Sun)
            .map(|p| p.sidereal_deg)
            .unwrap_or(0.0);
        let moon_deg = planets
            .iter()
            .find(|p| p.planet == VedicPlanet::Moon)
            .map(|p| p.sidereal_deg)
            .unwrap_or(0.0);

        let panchanga = crate::panchanga::PanchangaEngine::calculate(
            sun_deg, moon_deg, time, latitude, longitude,
        );

        let mut chart = VedicChart {
            ascendant: asc_position,
            planets,
            aspects: Vec::new(),
            sav: crate::analysis::ashtakavarga::Sarvashtakavarga { points: [0; 12] },
            bav: Vec::new(),
            house_cusps: sidereal_cusps,
            karakas: Vec::new(),
            bhava_strengths: Vec::new(),
            vimshopaka_scores: Vec::new(),
            avasthas: Vec::new(),
            panchanga,
            analysis_report,
        };

        // Post-calculation analysis
        chart.aspects = crate::analysis::aspects::AspectEngine::calculate_aspects(&chart);
        chart.sav = crate::analysis::ashtakavarga::AshtakavargaEngine::calculate_sav(&chart);
        // 행성별 BAV (Bhinnashtakavarga) — Sun/Moon/Mars/Mercury/Jupiter/Venus/Saturn
        let bav_planets = [
            crate::planets::VedicPlanet::Sun,
            crate::planets::VedicPlanet::Moon,
            crate::planets::VedicPlanet::Mars,
            crate::planets::VedicPlanet::Mercury,
            crate::planets::VedicPlanet::Jupiter,
            crate::planets::VedicPlanet::Venus,
            crate::planets::VedicPlanet::Saturn,
        ];
        chart.bav = bav_planets
            .iter()
            .map(|&p| crate::analysis::ashtakavarga::AshtakavargaEngine::calculate_bav(p, &chart))
            .collect();
        chart.karakas = crate::analysis::jaimini::JaiminiEngine::calculate_karakas(&chart, true); // Default 8-karaka
        chart.bhava_strengths = crate::analysis::bhava::BhavaEngine::calculate_all(&chart);
        chart.avasthas = chart
            .planets
            .iter()
            .map(crate::analysis::avasthas::AvasthaEngine::calculate)
            .collect();

        let mut v_scores = Vec::new();
        for p in &chart.planets {
            v_scores.push((
                p.planet,
                crate::analysis::vimshopaka::VimshopakaEngine::calculate(p, &chart),
            ));
        }
        chart.vimshopaka_scores = v_scores;

        // Final High-level Report
        chart.analysis_report = Some(crate::analysis::report::VedicAnalysisReport::generate(
            &chart, time,
        ));

        chart
    }
}

#[cfg(test)]
mod tests {
    use super::VedicChartCalculator;

    #[test]
    fn nakshatra_and_pada_start_boundary_is_correct() {
        let (nak, pada) = VedicChartCalculator::calculate_nakshatra_and_pada(0.0);
        assert_eq!(nak, 1);
        assert_eq!(pada, 1);
    }

    #[test]
    fn nakshatra_and_pada_transition_boundary_is_correct() {
        let segment = 360.0 / 27.0;

        let (nak_before, pada_before) =
            VedicChartCalculator::calculate_nakshatra_and_pada(segment - 1e-9);
        assert_eq!(nak_before, 1);
        assert_eq!(pada_before, 4);

        let (nak_after, pada_after) = VedicChartCalculator::calculate_nakshatra_and_pada(segment);
        assert_eq!(nak_after, 2);
        assert_eq!(pada_after, 1);
    }
}
