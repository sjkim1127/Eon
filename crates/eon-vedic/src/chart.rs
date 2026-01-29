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
    
    // Varga Positions (Sign Index 1~12)
    pub hora_rasi: u8,         // D2
    pub drekkana_rasi: u8,     // D3
    pub chaturthamsha_rasi: u8,// D4
    pub saptamsa_rasi: u8,     // D7
    pub navamsa_rasi: u8,      // D9
    pub dasamsa_rasi: u8,      // D10
    pub dwadasamsa_rasi: u8,   // D12
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

    pub fn calculate(&self, time: DateTime<Utc>) -> Vec<VedicPosition> {
        let ayanamsa = get_lahiri_ayanamsa(&self.engine, time);
        let planets = [
            VedicPlanet::Sun, VedicPlanet::Moon, VedicPlanet::Mars,
            VedicPlanet::Mercury, VedicPlanet::Jupiter, VedicPlanet::Venus,
            VedicPlanet::Saturn, VedicPlanet::Rahu
        ];

        let mut positions = Vec::new();

        for p in planets {
            // SEFLG_SPEED (256) | SEFLG_SWIEPH (2)
            let mut flag = 256 | 2;
            let mut planet_id = p.se_id();

            // Override Node ID and flag base on config
            if p == VedicPlanet::Rahu {
                match self.config.node_calc {
                    NodeCalculation::MeanNode => {
                         // SE_MEAN_NODE = 10
                         planet_id = 10;
                    },
                    NodeCalculation::TrueNode => {
                         // SE_TRUE_NODE = 11
                         planet_id = 11;
                    }
                }
            }

            let trop_long = self.engine.get_planet_position(time, planet_id, flag).unwrap_or(0.0);
            
            let sidereal_long = (trop_long - ayanamsa + 360.0) % 360.0;
            
            // Nakshatra: 360 / 27 = 13.3333... deg per nakshatra
            let nak_pos = sidereal_long / (360.0 / 27.0);
            let nakshatra = (nak_pos.floor() as u8) + 1;
            
            let pada_pos = (sidereal_long % (360.0 / 27.0)) / (360.0 / 108.0);
            let pada = (pada_pos.floor() as u8) + 1;

                let rasi = (sidereal_long / 30.0).floor() as u8 + 1;
                
                // Varga Calculations
                let hora_rasi = crate::varga::VargaType::D2.calculate_rasi(sidereal_long);
                let drekkana_rasi = crate::varga::VargaType::D3.calculate_rasi(sidereal_long);
                let chaturthamsha_rasi = crate::varga::VargaType::D4.calculate_rasi(sidereal_long);
                let saptamsa_rasi = crate::varga::VargaType::D7.calculate_rasi(sidereal_long);
                let navamsa_rasi = crate::varga::VargaType::D9.calculate_rasi(sidereal_long);
                let dasamsa_rasi = crate::varga::VargaType::D10.calculate_rasi(sidereal_long);
                let dwadasamsa_rasi = crate::varga::VargaType::D12.calculate_rasi(sidereal_long);

                positions.push(VedicPosition {
                    planet: p,
                    tropical_deg: trop_long,
                    sidereal_deg: sidereal_long,
                    nakshatra,
                    pada,
                    rasi,
                    hora_rasi,
                    drekkana_rasi,
                    chaturthamsha_rasi,
                    saptamsa_rasi,
                    navamsa_rasi,
                    dasamsa_rasi,
                    dwadasamsa_rasi,
                });

                // Ketu is opposite Rahu
                if p == VedicPlanet::Rahu {
                    let ketu_long = (sidereal_long + 180.0) % 360.0;
                    let ketu_nak_pos = ketu_long / (360.0 / 27.0);
                    let ketu_nak = (ketu_nak_pos.floor() as u8) + 1;
                    let ketu_pada_pos = (ketu_long % (360.0 / 27.0)) / (360.0 / 108.0);
                    let ketu_pada = (ketu_pada_pos.floor() as u8) + 1;
                    let ketu_rasi = (ketu_long / 30.0).floor() as u8 + 1;
                    
                    // Varga for Ketu
                    let ketu_hora = crate::varga::VargaType::D2.calculate_rasi(ketu_long);
                    let ketu_drekkana = crate::varga::VargaType::D3.calculate_rasi(ketu_long);
                    let ketu_chatur = crate::varga::VargaType::D4.calculate_rasi(ketu_long);
                    let ketu_saptamsa = crate::varga::VargaType::D7.calculate_rasi(ketu_long);
                    let ketu_navamsa = crate::varga::VargaType::D9.calculate_rasi(ketu_long);
                    let ketu_dasamsa = crate::varga::VargaType::D10.calculate_rasi(ketu_long);
                    let ketu_dwadasamsa = crate::varga::VargaType::D12.calculate_rasi(ketu_long);

                    positions.push(VedicPosition {
                        planet: VedicPlanet::Ketu,
                        tropical_deg: (trop_long + 180.0) % 360.0,
                        sidereal_deg: ketu_long,
                        nakshatra: ketu_nak,
                        pada: ketu_pada,
                        rasi: ketu_rasi,
                        hora_rasi: ketu_hora,
                        drekkana_rasi: ketu_drekkana,
                        chaturthamsha_rasi: ketu_chatur,
                        saptamsa_rasi: ketu_saptamsa,
                        navamsa_rasi: ketu_navamsa,
                        dasamsa_rasi: ketu_dasamsa,
                        dwadasamsa_rasi: ketu_dwadasamsa,
                    });
                }
            }
        
        positions
    }
}
