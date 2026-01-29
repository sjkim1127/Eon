use serde::{Deserialize, Serialize};
use crate::planets::VedicPlanet;
use crate::ayanamsa::get_lahiri_ayanamsa;
use crate::config::{VedicConfig, NodeCalculation};
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

    // Varga Positions (Sign Index 1~12)
    pub hora_rasi: u8,         // D2
    pub drekkana_rasi: u8,     // D3
    pub chaturthamsha_rasi: u8,// D4
    pub saptamsa_rasi: u8,     // D7
    pub navamsa_rasi: u8,      // D9
    pub dasamsa_rasi: u8,      // D10
    pub dwadasamsa_rasi: u8,   // D12
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VedicChart {
    pub ascendant: VedicPosition,
    pub planets: Vec<VedicPosition>,
    pub houses: Vec<f64>, // Cusps (start degrees) - typically 12
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
        
        // Calculate Ascendant (Lagna)
        let (cusps, ascmc) = self.engine.get_houses(time, latitude, longitude, 'P' as i32)
            .unwrap_or((vec![], [0.0; 10]));
            
        let asc_trop = ascmc[0];
        let asc_sidereal = (asc_trop - ayanamsa + 360.0) % 360.0;
        
        // --- Output Chart Data ---
        let mut planets_vec = Vec::new();
        
        // Helper to Create Position
        let create_position = |planet: VedicPlanet, sidereal: f64, tropical: f64, _is_ascendant: bool, lagna_rasi: Option<u8>| -> VedicPosition {
             let nak_pos = sidereal / (360.0 / 27.0);
             let nakshatra = (nak_pos.floor() as u8) + 1;
             
             let pada_pos = (sidereal % (360.0 / 27.0)) / (360.0 / 108.0);
             let pada = (pada_pos.floor() as u8) + 1;
             let rasi = (sidereal / 30.0).floor() as u8 + 1;

             // House Index Calculation (Whole Sign)
             // If this IS Lagna, it is House 1.
             // If this is a planet, House = (Planet Rasi - Lagna Rasi + 1 + 12) % 12
             // Actually: (Planet Rasi - Lagna Rasi + 1)
             // Example: Lagna Aries(1), Sun Taurus(2) -> 2 - 1 + 1 = 2nd House
             // Example: Lagna Pisces(12), Sun Aries(1) -> 1 - 12 + 1 = -10 -> +12 = 2nd House
             let house_index = if let Some(l_rasi) = lagna_rasi {
                 let diff = (rasi as i32 - l_rasi as i32);
                 let h = if diff >= 0 { diff + 1 } else { diff + 13 };
                 h as u8
             } else {
                 1 // Default for Lagna itself (House 1)
             };

             VedicPosition {
                planet,
                tropical_deg: tropical,
                sidereal_deg: sidereal,
                nakshatra,
                pada,
                rasi,
                house_index,
                hora_rasi: crate::varga::VargaType::D2.calculate_rasi(sidereal),
                drekkana_rasi: crate::varga::VargaType::D3.calculate_rasi(sidereal),
                chaturthamsha_rasi: crate::varga::VargaType::D4.calculate_rasi(sidereal),
                saptamsa_rasi: crate::varga::VargaType::D7.calculate_rasi(sidereal),
                navamsa_rasi: crate::varga::VargaType::D9.calculate_rasi(sidereal),
                dasamsa_rasi: crate::varga::VargaType::D10.calculate_rasi(sidereal),
                dwadasamsa_rasi: crate::varga::VargaType::D12.calculate_rasi(sidereal),
             }
        };

        // 1. Create Ascendant Position
        let asc_position = create_position(VedicPlanet::Ascendant, asc_sidereal, asc_trop, true, None);
        let lagna_rasi = asc_position.rasi;

        // 2. Planets
        let planets_list = [
            VedicPlanet::Sun, VedicPlanet::Moon, VedicPlanet::Mars,
            VedicPlanet::Mercury, VedicPlanet::Jupiter, VedicPlanet::Venus,
            VedicPlanet::Saturn, VedicPlanet::Rahu
        ];

        for p in planets_list {
            let flag = 256 | 2;
            let mut planet_id = p.se_id();
            if p == VedicPlanet::Rahu {
                match self.config.node_calc {
                    NodeCalculation::MeanNode => planet_id = 10,
                    NodeCalculation::TrueNode => planet_id = 11,
                }
            }

            let trop_long = self.engine.get_planet_position(time, planet_id, flag).unwrap_or(0.0);
            let sidereal_long = (trop_long - ayanamsa + 360.0) % 360.0;
            
            planets_vec.push(create_position(p, sidereal_long, trop_long, false, Some(lagna_rasi)));

            // Ketu
            if p == VedicPlanet::Rahu {
                let ketu_long = (sidereal_long + 180.0) % 360.0;
                let ketu_trop = (trop_long + 180.0) % 360.0;
                planets_vec.push(create_position(VedicPlanet::Ketu, ketu_long, ketu_trop, false, Some(lagna_rasi)));
            }
        }
        
        VedicChart {
            ascendant: asc_position,
            planets: planets_vec,
            houses: cusps,
        }
    }
}
