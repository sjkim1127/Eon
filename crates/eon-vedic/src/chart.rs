use serde::{Deserialize, Serialize};
use crate::planets::VedicPlanet;
use crate::ayanamsa::get_lahiri_ayanamsa;
use crate::config::VedicConfig;
use eon_astro::AstroEngine;
use chrono::{DateTime, Utc};

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

    // Varga Positions (Sign Index 1~12)
    pub hora_rasi: u8,         // D2
    pub drekkana_rasi: u8,     // D3
    pub chaturthamsha_rasi: u8,// D4
    pub panchamsa_rasi: u8,     // D5
    pub saptamsa_rasi: u8,     // D7
    pub ashtamsa_rasi: u8,      // D8
    pub navamsa_rasi: u8,      // D9
    pub dasamsa_rasi: u8,      // D10
    pub rudramsa_rasi: u8,      // D11
    pub dwadasamsa_rasi: u8,   // D12
    pub shodashamsa_rasi: u8,   // D16
    pub vimsamsa_rasi: u8,      // D20
    pub chaturvimshamsa_rasi: u8, // D24
    pub saptavimsamsa_rasi: u8, // D27
    pub trimsamsa_rasi: u8,     // D30
    pub khavedamsa_rasi: u8,    // D40
    pub akshavedamsa_rasi: u8,  // D45
    pub shashtyamsa_rasi: u8,   // D60
    pub navanavamsa_rasi: u8,   // D81
    pub ashtottaramsa_rasi: u8, // D108
    pub dwadasdwadasamsa_rasi: u8, // D144
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VedicChart {
    pub ascendant: VedicPosition,
    pub planets: Vec<VedicPosition>,
    pub aspects: Vec<crate::analysis::aspects::AspectRelation>,
    pub sav: crate::analysis::ashtakavarga::Sarvashtakavarga,
}

pub struct VedicChartCalculator {
    engine: AstroEngine,
    config: VedicConfig,
}

impl VedicChartCalculator {
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
        
        let (_, ascmc) = self.engine.get_houses(time, latitude, longitude, 'W' as i32)
            .unwrap_or((vec![], [0.0; 10]));
            
        let asc_sidereal = (ascmc[0] - ayanamsa + 360.0) % 360.0;
        let asc_rasi = (asc_sidereal / 30.0).floor() as u8 + 1;
        
        // Helper to Create Position
        let create_position = |planet: VedicPlanet, sidereal: f64, tropical: f64, speed: f64, sun_sidereal: Option<f64>| -> VedicPosition {
             let nak_pos = sidereal / (360.0 / 27.0);
             let nak_pos = sidereal / (360.0 / 27.0);
             let nak_pos = sidereal / (360.0 / 27.0);
             let nakshatra = (nak_pos.floor() as u8) + 1;
             
             let pada_pos = (sidereal % (360.0 / 27.0)) / (360.0 / 108.0);
             let pada = (pada_pos.floor() as u8) + 1;
             let rasi = (sidereal / 30.0).floor() as u8 + 1;
             
             // Whole Sign House Index
             let house_index = if rasi >= asc_rasi {
                 rasi - asc_rasi + 1
             } else {
                 (12 - asc_rasi) + rasi + 1
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
                    if planet == VedicPlanet::Sun { false }
                    else {
                        let diff = (sidereal - sun_long).abs();
                        let d = if diff > 180.0 { 360.0 - diff } else { diff };
                        let limit = match planet {
                            VedicPlanet::Moon => 12.0,
                            VedicPlanet::Mars => 17.0,
                            VedicPlanet::Mercury => if speed < 0.0 { 12.0 } else { 14.0 },
                            VedicPlanet::Jupiter => 11.0,
                            VedicPlanet::Venus => 10.0,
                            VedicPlanet::Saturn => 15.0,
                            _ => 0.0,
                        };
                        d < limit
                    }
                } else { false },
                hora_rasi: crate::varga::VargaType::D2.calculate_rasi(sidereal),
                drekkana_rasi: crate::varga::VargaType::D3.calculate_rasi(sidereal),
                chaturthamsha_rasi: crate::varga::VargaType::D4.calculate_rasi(sidereal),
                panchamsa_rasi: crate::varga::VargaType::D5.calculate_rasi(sidereal),
                saptamsa_rasi: crate::varga::VargaType::D7.calculate_rasi(sidereal),
                ashtamsa_rasi: crate::varga::VargaType::D8.calculate_rasi(sidereal),
                navamsa_rasi: crate::varga::VargaType::D9.calculate_rasi(sidereal),
                dasamsa_rasi: crate::varga::VargaType::D10.calculate_rasi(sidereal),
                rudramsa_rasi: crate::varga::VargaType::D11.calculate_rasi(sidereal),
                dwadasamsa_rasi: crate::varga::VargaType::D12.calculate_rasi(sidereal),
                shodashamsa_rasi: crate::varga::VargaType::D16.calculate_rasi(sidereal),
                vimsamsa_rasi: crate::varga::VargaType::D20.calculate_rasi(sidereal),
                chaturvimshamsa_rasi: crate::varga::VargaType::D24.calculate_rasi(sidereal),
                saptavimsamsa_rasi: crate::varga::VargaType::D27.calculate_rasi(sidereal),
                trimsamsa_rasi: crate::varga::VargaType::D30.calculate_rasi(sidereal),
                khavedamsa_rasi: crate::varga::VargaType::D40.calculate_rasi(sidereal),
                akshavedamsa_rasi: crate::varga::VargaType::D45.calculate_rasi(sidereal),
                shashtyamsa_rasi: crate::varga::VargaType::D60.calculate_rasi(sidereal),
                navanavamsa_rasi: crate::varga::VargaType::D81.calculate_rasi(sidereal),
                ashtottaramsa_rasi: crate::varga::VargaType::D108.calculate_rasi(sidereal),
                dwadasdwadasamsa_rasi: crate::varga::VargaType::D144.calculate_rasi(sidereal),
             }
        };

        // 0. Find Sun's Sidereal Longitude first for combustion check
        let (sun_trop, _) = self.engine.get_planet_full(time, VedicPlanet::Sun.se_id(), 256 | 2).unwrap_or((0.0, 0.0));
        let sun_sidereal = (sun_trop - ayanamsa + 360.0) % 360.0;

        // 1. Create Ascendant Position
        let asc_position = create_position(VedicPlanet::Ascendant, asc_sidereal, ascmc[0], 0.0, Some(sun_sidereal));

        let planets_names = [
            VedicPlanet::Sun, VedicPlanet::Moon, VedicPlanet::Mars,
            VedicPlanet::Mercury, VedicPlanet::Jupiter, VedicPlanet::Venus,
            VedicPlanet::Saturn, VedicPlanet::Rahu
        ];

        let mut planets = Vec::new();
        for p in &planets_names {
            let flag = 256 | 2; // SEFLG_SPEED | SEFLG_SIDEREAL (or just standard)
            let (trop, speed) = self.engine.get_planet_full(time, p.se_id(), flag).unwrap_or((0.0, 0.0));
            let sidereal = (trop - ayanamsa + 360.0) % 360.0;
            planets.push(create_position(*p, sidereal, trop, speed, Some(sun_sidereal)));

            // Add Ketu opposite to Rahu
            if *p == VedicPlanet::Rahu {
                let ketu_sidereal = (sidereal + 180.0) % 360.0;
                let ketu_tropical = (trop + 180.0) % 360.0;
                // Ketu's speed is the same as Rahu's (since it's exactly opposite)
                planets.push(create_position(VedicPlanet::Ketu, ketu_sidereal, ketu_tropical, speed, Some(sun_sidereal)));
            }
        }

        let mut chart = VedicChart {
            ascendant: asc_position,
            planets,
            aspects: Vec::new(),
            sav: crate::analysis::ashtakavarga::Sarvashtakavarga { points: [0; 12] },
        };

        // Post-calculation analysis
        chart.aspects = crate::analysis::aspects::AspectEngine::calculate_aspects(&chart);
        chart.sav = crate::analysis::ashtakavarga::AshtakavargaEngine::calculate_sav(&chart);

        chart
    }
}
