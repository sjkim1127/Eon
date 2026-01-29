use serde::{Deserialize, Serialize};
use crate::planets::VedicPlanet;
use chrono::{DateTime, Utc};
use crate::chart::VedicChart; // We might need Ayanamsa from chart or engine

/// Transit Result for a single planet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitPosition {
    pub planet: VedicPlanet,
    pub current_rasi: u8,
    pub house_from_moon: u8, // 1~12
    pub is_benefic_transit: bool, // Simple check based on Gochara rules
}

pub struct GocharaEngine;

impl GocharaEngine {
    /// Calculate current transit positions relative to Natal Moon
    /// We need an external way to calculate *current* planetary positions.
    /// Since GocharaEngine shouldn't hold the AstroEngine instance itself (usually),
    /// we expect the *current* chart to be passed in, or we simulate it.
    /// For this simplified version, let's assume we pass in a pre-calculated "Current Chart".
    /// 
    /// Usage: 
    /// let transit_chart = calculator.calculate(current_time, lat, lon);
    /// let transits = GocharaEngine::analyze(natal_chart.moon.rasi, &transit_chart);
    pub fn analyze(natal_moon_rasi: u8, current_chart: &VedicChart) -> Vec<TransitPosition> {
        let mut results = Vec::new();
        
        for pos in &current_chart.planets {
            let house_from_moon = if pos.rasi >= natal_moon_rasi {
                pos.rasi - natal_moon_rasi + 1
            } else {
                (12 - natal_moon_rasi) + pos.rasi + 1
            };
            
            let is_benefic = Self::check_benefic_transit(pos.planet, house_from_moon);
            
            results.push(TransitPosition {
                planet: pos.planet,
                current_rasi: pos.rasi,
                house_from_moon,
                is_benefic_transit: is_benefic,
            });
        }
        
        results
    }
    
    /// Standard Gochara Benefic Houses (from Moon)
    /// Sun: 3, 6, 10, 11
    /// Moon: 1, 3, 6, 7, 10, 11
    /// Mars: 3, 6, 11
    /// Mercury: 2, 4, 6, 8, 10, 11
    /// Jupiter: 2, 5, 7, 9, 11
    /// Venus: 1, 2, 3, 4, 5, 8, 9, 11, 12
    /// Saturn: 3, 6, 11
    /// Rahu: 3, 6, 11
    /// Ketu: 3, 6, 11
    fn check_benefic_transit(planet: VedicPlanet, house: u8) -> bool {
        match planet {
            VedicPlanet::Sun => [3, 6, 10, 11].contains(&house),
            VedicPlanet::Moon => [1, 3, 6, 7, 10, 11].contains(&house),
            VedicPlanet::Mars => [3, 6, 11].contains(&house),
            VedicPlanet::Mercury => [2, 4, 6, 8, 10, 11].contains(&house),
            VedicPlanet::Jupiter => [2, 5, 7, 9, 11].contains(&house),
            VedicPlanet::Venus => [1, 2, 3, 4, 5, 8, 9, 11, 12].contains(&house),
            VedicPlanet::Saturn => [3, 6, 11].contains(&house),
            VedicPlanet::Rahu => [3, 6, 11].contains(&house),
            VedicPlanet::Ketu => [3, 6, 11].contains(&house),
            _ => false,
        }
    }
}
