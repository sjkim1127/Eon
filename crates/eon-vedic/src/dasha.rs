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
        max_level: u8 
    ) -> Vec<DashaPeriod> {
        let nak_len = 360.0 / 27.0;
        let nak_pos_val = moon_longitude / nak_len;
        let nak_idx = nak_pos_val.floor() as usize; // 0..26
        let nakshatra = (nak_idx + 1) as u8;
        
        let progression = nak_pos_val - nak_idx as f64;
        let remaining_fraction = 1.0 - progression;
        
        let sequence = Self::get_dasha_sequence();
        let start_ruler_idx = Self::get_ruler_index(nakshatra);
        
        let mut dashas = Vec::new();
        let mut current_date = birth_date;
        
        // 1. First Mahadasha (Balance)
        let (first_planet, full_duration) = sequence[start_ruler_idx];
        let balance_years = full_duration * remaining_fraction;
        let first_end_date = Self::add_years(current_date, balance_years);
        
        // Theoretical start of this Mahadasha
        let theoretical_start = Self::add_years(current_date, -(full_duration * (1.0 - remaining_fraction)));

        dashas.push(DashaPeriod {
            planet: first_planet,
            start_date: current_date,
            end_date: first_end_date,
            duration_years: balance_years,
            level: 1,
            sub_periods: if max_level > 1 {
                Self::calculate_sub_periods(first_planet, theoretical_start, full_duration, 2, max_level, Some(birth_date))
            } else {
                Vec::new()
            },
        });
        
        current_date = first_end_date;
        
        // 2. Subsequent Mahadashas (for ~120 years)
        let mut years_covered = balance_years;
        let mut idx = (start_ruler_idx + 1) % 9;
        
        while years_covered < 120.0 {
            let (planet, duration) = sequence[idx];
            let end_date = Self::add_years(current_date, duration);
            
            dashas.push(DashaPeriod {
                planet,
                start_date: current_date,
                end_date,
                duration_years: duration,
                level: 1,
                sub_periods: if max_level > 1 {
                    Self::calculate_sub_periods(planet, current_date, duration, 2, max_level, None)
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
    
    fn calculate_sub_periods(
        lord: VedicPlanet, 
        theoretical_start: DateTime<Utc>, 
        parent_duration: f64,
        level: u8,
        max_level: u8,
        clip_start: Option<DateTime<Utc>>
    ) -> Vec<DashaPeriod> {
        let sequence = Self::get_dasha_sequence();
        let start_idx = sequence.iter().position(|(p, _)| *p == lord).unwrap();
        
        let mut periods = Vec::new();
        let mut current_start = theoretical_start;
        
        for i in 0..9 {
            let idx = (start_idx + i) % 9;
            let (sub_planet, sub_base_years) = sequence[idx];
            // Vimshottari sub-period formula: (Parent Years * Sub Lord Years) / 120
            let sub_duration = (parent_duration * sub_base_years) / 120.0;
            let current_end = Self::add_years(current_start, sub_duration);

            // Check if this sub-period is within the clip range
            let actual_start = if let Some(clip) = clip_start {
                if current_end <= clip {
                    current_start = current_end;
                    continue; // Already passed
                }
                if current_start < clip { clip } else { current_start }
            } else {
                current_start
            };

            let actual_duration = if actual_start > current_start {
                sub_duration - (actual_start.signed_duration_since(current_start).num_seconds() as f64 / (365.2425 * 86400.0))
            } else {
                sub_duration
            };

            periods.push(DashaPeriod {
                planet: sub_planet,
                start_date: actual_start,
                end_date: current_end,
                duration_years: actual_duration,
                level,
                sub_periods: if level < max_level {
                    Self::calculate_sub_periods(sub_planet, current_start, sub_duration, level + 1, max_level, clip_start)
                } else {
                    Vec::new()
                },
            });

            current_start = current_end;
        }

        periods
    }

    fn add_years(date: DateTime<Utc>, years: f64) -> DateTime<Utc> {
        let seconds = (years * 365.2425 * 86400.0) as i64;
        date + Duration::seconds(seconds)
    }
}
