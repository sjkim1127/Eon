use crate::config::VedicYearType;
use crate::constants::*;
use crate::planets::VedicPlanet;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashaPeriod {
    pub planet: VedicPlanet,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub duration_years: f64,
    pub level: u8,            // 1=Mahadasha, 2=Antardasha
    pub name: Option<String>, // For Yogini Dasha names
    pub sub_periods: Vec<DashaPeriod>,
}

pub struct Vimshottari;

impl Vimshottari {
    // Planetary periods in years (Sun to Venus sequence as per Vimshottari)
    // Sequence: Ketu, Venus, Sun, Moon, Mars, Rahu, Jupiter, Saturn, Mercury
    // Years:    7,    20,    6,   10,   7,    18,   16,      19,     17

    fn get_dasha_sequence() -> [(VedicPlanet, f64); 9] {
        [
            (VedicPlanet::Ketu, DASHA_YEARS_KETU),
            (VedicPlanet::Venus, DASHA_YEARS_VENUS),
            (VedicPlanet::Sun, DASHA_YEARS_SUN),
            (VedicPlanet::Moon, DASHA_YEARS_MOON),
            (VedicPlanet::Mars, DASHA_YEARS_MARS),
            (VedicPlanet::Rahu, DASHA_YEARS_RAHU),
            (VedicPlanet::Jupiter, DASHA_YEARS_JUPITER),
            (VedicPlanet::Saturn, DASHA_YEARS_SATURN),
            (VedicPlanet::Mercury, DASHA_YEARS_MERCURY),
        ]
    }

    /// Calculate nakshatra ruler index (0..9) based on nakshatra index (1..27)
    /// Nakshatra 1 (Ashwini) -> Ketu (0)
    fn get_ruler_index(nakshatra: u8) -> usize {
        // (nakshatra - 1) % 9
        (nakshatra as usize - 1) % 9
    }

    pub fn calculate(
        moon_longitude: f64,
        birth_date: DateTime<Utc>,
        max_level: u8,
        year_type: VedicYearType,
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
        let first_end_date = Self::add_years(current_date, balance_years, year_type);

        // Theoretical start of this Mahadasha
        let theoretical_start = Self::add_years(
            current_date,
            -(full_duration * (1.0 - remaining_fraction)),
            year_type,
        );

        dashas.push(DashaPeriod {
            planet: first_planet,
            start_date: current_date,
            end_date: first_end_date,
            duration_years: balance_years,
            level: 1,
            name: None,
            sub_periods: if max_level > 1 {
                Self::calculate_sub_periods(
                    first_planet,
                    theoretical_start,
                    full_duration,
                    2,
                    max_level,
                    Some(birth_date),
                    year_type,
                )
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
            let end_date = Self::add_years(current_date, duration, year_type);

            dashas.push(DashaPeriod {
                planet,
                start_date: current_date,
                end_date,
                duration_years: duration,
                level: 1,
                name: None,
                sub_periods: if max_level > 1 {
                    Self::calculate_sub_periods(
                        planet,
                        current_date,
                        duration,
                        2,
                        max_level,
                        None,
                        year_type,
                    )
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
        clip_start: Option<DateTime<Utc>>,
        year_type: VedicYearType,
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
            let current_end = Self::add_years(current_start, sub_duration, year_type);

            // Check if this sub-period is within the clip range
            let actual_start = if let Some(clip) = clip_start {
                if current_end <= clip {
                    current_start = current_end;
                    continue; // Already passed
                }
                if current_start < clip {
                    clip
                } else {
                    current_start
                }
            } else {
                current_start
            };

            let actual_duration = if actual_start > current_start {
                let days_per_year = match year_type {
                    VedicYearType::Savana => SAVANA_YEAR_DAYS,
                    VedicYearType::Sidereal => SIDEREAL_YEAR_DAYS,
                    VedicYearType::Gregorian => GREGORIAN_YEAR_DAYS,
                };
                sub_duration
                    - (actual_start
                        .signed_duration_since(current_start)
                        .num_seconds() as f64
                        / (days_per_year * 86400.0))
            } else {
                sub_duration
            };

            periods.push(DashaPeriod {
                planet: sub_planet,
                start_date: actual_start,
                end_date: current_end,
                duration_years: actual_duration,
                level,
                name: None,
                sub_periods: if level < max_level {
                    Self::calculate_sub_periods(
                        sub_planet,
                        current_start,
                        sub_duration,
                        level + 1,
                        max_level,
                        clip_start,
                        year_type,
                    )
                } else {
                    Vec::new()
                },
            });

            current_start = current_end;
        }

        periods
    }

    fn add_years(date: DateTime<Utc>, years: f64, year_type: VedicYearType) -> DateTime<Utc> {
        let days_per_year = match year_type {
            VedicYearType::Savana => SAVANA_YEAR_DAYS,
            VedicYearType::Sidereal => SIDEREAL_YEAR_DAYS,
            VedicYearType::Gregorian => GREGORIAN_YEAR_DAYS,
        };
        let seconds = (years * days_per_year * SECONDS_PER_DAY) as i64;
        date + Duration::seconds(seconds)
    }
}

pub struct Yogini;

impl Yogini {
    /// Planetary periods in years for Yogini (Mangala to Sankata)
    fn get_sequence() -> [(VedicPlanet, f64, &'static str); 8] {
        [
            (VedicPlanet::Moon, 1.0, "Mangala"),
            (VedicPlanet::Sun, 2.0, "Pingala"),
            (VedicPlanet::Jupiter, 3.0, "Dhanya"),
            (VedicPlanet::Mars, 4.0, "Bhramari"),
            (VedicPlanet::Mercury, 5.0, "Bhadrika"),
            (VedicPlanet::Saturn, 6.0, "Ulka"),
            (VedicPlanet::Venus, 7.0, "Siddha"),
            (VedicPlanet::Rahu, 8.0, "Sankata"),
        ]
    }

    pub fn calculate(
        moon_longitude: f64,
        birth_date: DateTime<Utc>,
        max_level: u8,
        year_type: VedicYearType,
    ) -> Vec<DashaPeriod> {
        let nak_len = 360.0 / 27.0;
        let nak_pos_val = moon_longitude / nak_len;
        let nak_idx = nak_pos_val.floor() as usize; // 0..26
        let nakshatra = (nak_idx + 1) as u8;

        let progression = nak_pos_val - nak_idx as f64;
        let remaining_fraction = 1.0 - progression;

        // Starting Yogini: (Nakshatra + 3) / 8. Remainder 1=Mangala, ..., 0=Sankata.
        let mut start_idx = ((nakshatra + 3) % 8) as usize;
        if start_idx == 0 {
            start_idx = 8;
        }
        start_idx -= 1; // 0-indexed

        let sequence = Self::get_sequence();
        let mut dashas = Vec::new();
        let mut current_date = birth_date;

        let mut years_covered = 0.0;
        let mut idx = start_idx;

        // Calculate for at least 100 years or 3 cycles
        while years_covered < 100.0 {
            let (planet, full_duration, name) = sequence[idx];

            let duration = if years_covered == 0.0 {
                full_duration * remaining_fraction
            } else {
                full_duration
            };

            let theoretical_start = if years_covered == 0.0 {
                Vimshottari::add_years(
                    current_date,
                    -(full_duration * (1.0 - remaining_fraction)),
                    year_type,
                )
            } else {
                current_date
            };

            let end_date = Vimshottari::add_years(current_date, duration, year_type);

            dashas.push(DashaPeriod {
                planet,
                start_date: current_date,
                end_date,
                duration_years: duration,
                level: 1,
                name: Some(name.to_string()),
                sub_periods: if max_level > 1 {
                    Self::calculate_sub_periods(
                        idx,
                        theoretical_start,
                        full_duration,
                        2,
                        max_level,
                        if years_covered == 0.0 {
                            Some(birth_date)
                        } else {
                            None
                        },
                        year_type,
                    )
                } else {
                    Vec::new()
                },
            });

            current_date = end_date;
            years_covered += duration;
            idx = (idx + 1) % 8;
        }

        dashas
    }

    fn calculate_sub_periods(
        md_idx: usize,
        theoretical_start: DateTime<Utc>,
        parent_duration: f64,
        level: u8,
        max_level: u8,
        clip_start: Option<DateTime<Utc>>,
        year_type: VedicYearType,
    ) -> Vec<DashaPeriod> {
        let sequence = Self::get_sequence();
        let mut periods = Vec::new();
        let mut current_start = theoretical_start;

        for i in 0..8 {
            let idx = (md_idx + i) % 8;
            let (sub_planet, sub_base_years, name) = sequence[idx];
            // Yogini sub-period formula: (MD Lord Years * AD Lord Years) / 36
            let sub_duration = (parent_duration * sub_base_years) / 36.0;
            let current_end = Vimshottari::add_years(current_start, sub_duration, year_type);

            let actual_start = if let Some(clip) = clip_start {
                if current_end <= clip {
                    current_start = current_end;
                    continue;
                }
                if current_start < clip {
                    clip
                } else {
                    current_start
                }
            } else {
                current_start
            };

            let actual_duration = if actual_start > current_start {
                let days_per_year = match year_type {
                    VedicYearType::Savana => 360.0,
                    VedicYearType::Sidereal => 365.256363,
                    VedicYearType::Gregorian => 365.2425,
                };
                sub_duration
                    - (actual_start
                        .signed_duration_since(current_start)
                        .num_seconds() as f64
                        / (days_per_year * 86400.0))
            } else {
                sub_duration
            };

            periods.push(DashaPeriod {
                planet: sub_planet,
                start_date: actual_start,
                end_date: current_end,
                duration_years: actual_duration,
                level,
                name: Some(name.to_string()),
                sub_periods: if level < max_level {
                    Self::calculate_sub_periods(
                        idx,
                        current_start,
                        sub_duration,
                        level + 1,
                        max_level,
                        clip_start,
                        year_type,
                    )
                } else {
                    Vec::new()
                },
            });

            current_start = current_end;
        }
        periods
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::VedicYearType;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_dasha_year_length() {
        let birth = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();

        // 10 years in Savana (360 days each) = 3600 days
        let end_savana = Vimshottari::add_years(birth, 10.0, VedicYearType::Savana);
        let diff_savana = end_savana.signed_duration_since(birth).num_days();
        assert_eq!(diff_savana, 3600);

        // 10 years in Gregorian (~365.2425 days) = 3652 days (approx)
        let end_greg = Vimshottari::add_years(birth, 10.0, VedicYearType::Gregorian);
        let diff_greg = end_greg.signed_duration_since(birth).num_days();
        assert!(diff_greg >= 3652 && diff_greg <= 3653);
    }
}
