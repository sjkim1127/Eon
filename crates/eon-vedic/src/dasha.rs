use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use crate::planets::VedicPlanet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashaPeriod {
    pub planet: VedicPlanet,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub duration_years: f64,
    pub level: u8, // 1=Mahadasha, 2=Antardasha
    pub sub_periods: Vec<DashaPeriod>,
}

pub struct Vimshottari;

impl Vimshottari {
    // Planetary periods in years (Sun to Venus sequence as per Vimshottari)
    // Sequence: Ketu, Venus, Sun, Moon, Mars, Rahu, Jupiter, Saturn, Mercury
    // Years:    7,    20,    6,   10,   7,    18,   16,      19,     17
    
    fn get_dasha_sequence() -> [(VedicPlanet, f64); 9] {
        [
            (VedicPlanet::Ketu, 7.0),
            (VedicPlanet::Venus, 20.0),
            (VedicPlanet::Sun, 6.0),
            (VedicPlanet::Moon, 10.0),
            (VedicPlanet::Mars, 7.0),
            (VedicPlanet::Rahu, 18.0),
            (VedicPlanet::Jupiter, 16.0),
            (VedicPlanet::Saturn, 19.0),
            (VedicPlanet::Mercury, 17.0),
        ]
    }

    /// Calculate nakshatra ruler index (0..9) based on nakshatra index (1..27)
    /// Nakshatra 1 (Ashwini) -> Ketu (0)
    fn get_ruler_index(nakshatra: u8) -> usize {
        // (nakshatra - 1) % 9
        ((nakshatra as usize - 1) % 9)
    }

    pub fn calculate(
        moon_longitude: f64,
        birth_date: DateTime<Utc>,
        depth: u8 // 1 for Mahadasha, 2 for Antardasha
    ) -> Vec<DashaPeriod> {
        let nak_len = 360.0 / 27.0; // 13.3333...
        let nak_pos_val = moon_longitude / nak_len;
        let nak_idx = nak_pos_val.floor() as usize; // 0..26
        let nakshatra = (nak_idx + 1) as u8;
        
        // Progression within the nakshatra (0.0 to 1.0)
        let progression = nak_pos_val - nak_idx as f64;
        let remaining_fraction = 1.0 - progression;
        
        let sequence = Self::get_dasha_sequence();
        let start_ruler_idx = Self::get_ruler_index(nakshatra);
        
        let mut dashas = Vec::new();
        let mut current_date = birth_date;
        
        // 1. First Dasha (Balance)
        let (first_planet, full_duration) = sequence[start_ruler_idx];
        let balance_years = full_duration * remaining_fraction;
        let first_end_date = Self::add_years(current_date, balance_years);
        
        dashas.push(DashaPeriod {
            planet: first_planet,
            start_date: current_date,
            end_date: first_end_date,
            duration_years: balance_years,
            level: 1,
            sub_periods: if depth > 1 {
                Self::calculate_antardasha(first_planet, current_date, full_duration, Some(remaining_fraction))
            } else {
                Vec::new()
            },
        });
        
        current_date = first_end_date;
        
        // 2. Subsequent Dashas (for 120 years total coverage usually, or just one full cycle)
        // Let's generate for ~100 years lifespan
        let mut years_covered = balance_years;
        let mut idx = (start_ruler_idx + 1) % 9;
        
        while years_covered < 100.0 {
            let (planet, duration) = sequence[idx];
            let end_date = Self::add_years(current_date, duration);
            
            dashas.push(DashaPeriod {
                planet,
                start_date: current_date,
                end_date,
                duration_years: duration,
                level: 1,
                sub_periods: if depth > 1 {
                    Self::calculate_antardasha(planet, current_date, duration, None)
                } else {
                    Vec::new()
                },
            });
            
            current_date = end_date;
            years_covered += duration;
            idx = (idx + 1) % 9;
        }
        
        dashas
    }
    
    fn calculate_antardasha(
        lord: VedicPlanet, 
        start_date: DateTime<Utc>, 
        mahadasha_duration: f64,
        balance_fraction: Option<f64>
    ) -> Vec<DashaPeriod> {
        let sequence = Self::get_dasha_sequence();
        // Antardasha starts with the Mahadasha lord itself
        let start_idx = sequence.iter().position(|(p, _)| *p == lord).unwrap();
        
        let mut periods = Vec::new();
        let mut current = start_date;
        
        let _loop_start = if balance_fraction.is_some() {
             // If this is a balance dasha, we need to find where in the sub-period cycle we are.
             // This is complex. Simplified: If balance, generate sub-periods strictly proportional 
             // to the *remaining* balance.
             // But technically, the sub-periods passed are passed.
             // Implementation for precise balance sub-periods requires calculating which sub-period dominates.
             // For implementation MVP, we will simplify: 
             // Calculate full sub-periods and filter/clip based on start_date.
             0 // Placeholder for full re-calc
        } else {
            0
        };

        // Standard Antardasha Calculation
        // Sub-period duration = (Mahadasha Years * Antardasha Lord Years) / 120
        for i in 0..9 {
            let idx = (start_idx + i) % 9;
            let (sub_planet, sub_base_years) = sequence[idx];
            let sub_duration = (mahadasha_duration * sub_base_years) / 120.0;
            
            if let Some(_fraction) = balance_fraction {
                 // Simplified handling for balance: 
                 // If we are in the middle of a Mahadasha, we might be in the middle of an Antardasha.
                 // This requires locating the exact sub-period.
                 // Let's defer complex balance logic for now and just handle full cycles for non-balance dashas.
            }

            let end = Self::add_years(current, sub_duration);
            periods.push(DashaPeriod {
                planet: sub_planet,
                start_date: current,
                end_date: end,
                duration_years: sub_duration,
                level: 2,
                sub_periods: Vec::new(),
            });
            current = end;
        }
        
        // Fix for balance case: Filter out periods that happened before start_date (strictly speaking logic above needs adjustment)
        // For MVP, we only calculate proper Antar for full Mahadashas.
        if balance_fraction.is_some() {
            return Vec::new(); // Return empty for balance block to avoid confusion
        }

        periods
    }

    fn add_years(date: DateTime<Utc>, years: f64) -> DateTime<Utc> {
        let seconds = years * 365.2425 * 86400.0;
        date + Duration::seconds(seconds as i64)
    }
}
