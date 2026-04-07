use crate::planets::VedicPlanet;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashaPeriod {
    pub lord: VedicPlanet,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub sub_dashas: Vec<DashaPeriod>,
    pub name: Option<String>,
}

pub struct VimshottariDasha;

impl VimshottariDasha {
    const CYCLE: [(VedicPlanet, f64); 9] = [
        (VedicPlanet::Ketu, 7.0),
        (VedicPlanet::Venus, 20.0),
        (VedicPlanet::Sun, 6.0),
        (VedicPlanet::Moon, 10.0),
        (VedicPlanet::Mars, 7.0),
        (VedicPlanet::Rahu, 18.0),
        (VedicPlanet::Jupiter, 16.0),
        (VedicPlanet::Saturn, 19.0),
        (VedicPlanet::Mercury, 17.0),
    ];

    const TOTAL_YEARS: f64 = 120.0;

    /// Calculate the full list of Mahadashas for a given birth time and Moon's longitude
    pub fn calculate_timeline(
        birth_time: DateTime<Utc>,
        moon_long: f64,
        levels: u8,
    ) -> Vec<DashaPeriod> {
        // Moon Nakshatra range is 13°20' (13.3333...)
        let nak_duration = 360.0 / 27.0;
        let nak_index_0 = (moon_long / nak_duration).floor() as usize;
        let start_lord_index = nak_index_0 % 9;

        // Calculate elapsed portion of the current Nakshatra
        let elapsed_in_nak = moon_long % nak_duration;
        let remaining_in_nak = nak_duration - elapsed_in_nak;
        let portion_remaining = remaining_in_nak / nak_duration;

        let mut timeline = Vec::new();
        let mut current_start = birth_time;

        // Initial Dasha balance
        let (first_lord, full_years) = Self::CYCLE[start_lord_index];
        let remaining_years = full_years * portion_remaining;

        // Approximate years to seconds (365.2425 days per year)
        let years_to_secs = |y: f64| (y * 365.2425 * 24.0 * 60.0 * 60.0) as i64;

        let first_end = current_start + Duration::seconds(years_to_secs(remaining_years));

        let mut first_dasha = DashaPeriod {
            lord: first_lord,
            start_time: current_start,
            end_time: first_end,
            sub_dashas: Vec::new(),
            name: None,
        };

        if levels > 1 {
            first_dasha.sub_dashas = Self::calculate_sub_periods(
                first_lord,
                current_start,
                first_end,
                levels - 1,
                Some(portion_remaining),
            );
        }

        timeline.push(first_dasha);
        current_start = first_end;

        // Generate subsequent Mahadashas for the next 120 years
        for i in 1..9 {
            let idx = (start_lord_index + i) % 9;
            let (lord, years) = Self::CYCLE[idx];
            let end_time = current_start + Duration::seconds(years_to_secs(years));

            let mut dasha = DashaPeriod {
                lord,
                start_time: current_start,
                end_time,
                sub_dashas: Vec::new(),
                name: None,
            };

            if levels > 1 {
                dasha.sub_dashas =
                    Self::calculate_sub_periods(lord, current_start, end_time, levels - 1, None);
            }

            timeline.push(dasha);
            current_start = end_time;
        }

        timeline
    }

    fn calculate_sub_periods(
        lord: VedicPlanet,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        levels_remaining: u8,
        initial_portion: Option<f64>,
    ) -> Vec<DashaPeriod> {
        let total_duration = end.signed_duration_since(start).num_seconds() as f64;

        // Find index of the main lord in the cycle
        let lord_idx = Self::CYCLE
            .iter()
            .position(|&(p, _)| p == lord)
            .unwrap_or(0);

        let mut sub_periods = Vec::new();
        let mut current_start = start;

        // If it's the first dasha, we only show the remaining sub-dashas
        let start_offset = if let Some(portion) = initial_portion {
            let elapsed_portion = 1.0 - portion;
            let mut cumulative_portion = 0.0;
            let mut start_sub_idx = 0;
            let mut sub_elapsed_in_period = 0.0;

            for i in 0..9 {
                let idx = (lord_idx + i) % 9;
                let (_, years) = Self::CYCLE[idx];
                let p = years / Self::TOTAL_YEARS;
                if cumulative_portion + p > elapsed_portion {
                    start_sub_idx = i;
                    sub_elapsed_in_period = elapsed_portion - cumulative_portion;
                    break;
                }
                cumulative_portion += p;
            }
            Some((start_sub_idx, sub_elapsed_in_period))
        } else {
            None
        };

        if let Some((start_sub_idx, sub_elapsed)) = start_offset {
            let idx = (lord_idx + start_sub_idx) % 9;
            let (p_lord, years) = Self::CYCLE[idx];
            let full_sub_portion = years / Self::TOTAL_YEARS;
            let remaining_sub_portion = full_sub_portion - sub_elapsed;
            let full_mahadasha_duration = total_duration / initial_portion.unwrap();
            let sub_duration = full_mahadasha_duration * remaining_sub_portion;
            let sub_end = current_start + Duration::seconds(sub_duration as i64);

            let mut sub = DashaPeriod {
                lord: p_lord,
                start_time: current_start,
                end_time: sub_end,
                sub_dashas: Vec::new(),
                name: None,
            };

            if levels_remaining > 1 {
                sub.sub_dashas = Self::calculate_sub_periods(
                    p_lord,
                    current_start,
                    sub_end,
                    levels_remaining - 1,
                    Some(sub_elapsed / full_sub_portion),
                );
            }

            sub_periods.push(sub);
            current_start = sub_end;

            for i in (start_sub_idx + 1)..9 {
                let idx = (lord_idx + i) % 9;
                let (p_lord, years) = Self::CYCLE[idx];
                let sub_duration = full_mahadasha_duration * (years / Self::TOTAL_YEARS);
                let sub_end = current_start + Duration::seconds(sub_duration as i64);

                let mut sub = DashaPeriod {
                    lord: p_lord,
                    start_time: current_start,
                    end_time: sub_end,
                    sub_dashas: Vec::new(),
                    name: None,
                };

                if levels_remaining > 1 {
                    sub.sub_dashas = Self::calculate_sub_periods(
                        p_lord,
                        current_start,
                        sub_end,
                        levels_remaining - 1,
                        None,
                    );
                }

                sub_periods.push(sub);
                current_start = sub_end;
            }
        } else {
            for i in 0..9 {
                let idx = (lord_idx + i) % 9;
                let (p_lord, years) = Self::CYCLE[idx];
                let sub_duration = total_duration * (years / Self::TOTAL_YEARS);
                let sub_end = current_start + Duration::seconds(sub_duration as i64);

                let mut sub = DashaPeriod {
                    lord: p_lord,
                    start_time: current_start,
                    end_time: sub_end,
                    sub_dashas: Vec::new(),
                    name: None,
                };

                if levels_remaining > 1 {
                    sub.sub_dashas = Self::calculate_sub_periods(
                        p_lord,
                        current_start,
                        sub_end,
                        levels_remaining - 1,
                        None,
                    );
                }

                sub_periods.push(sub);
                current_start = sub_end;
            }
        }

        sub_periods
    }
}

pub struct YoginiDasha;

impl YoginiDasha {
    const CYCLE: [(&'static str, VedicPlanet, f64); 8] = [
        ("Mangala", VedicPlanet::Moon, 1.0),
        ("Pingala", VedicPlanet::Sun, 2.0),
        ("Dhanya", VedicPlanet::Jupiter, 3.0),
        ("Bhramari", VedicPlanet::Mars, 4.0),
        ("Bhadrika", VedicPlanet::Mercury, 5.0),
        ("Ulka", VedicPlanet::Saturn, 6.0),
        ("Siddha", VedicPlanet::Venus, 7.0),
        ("Sankata", VedicPlanet::Rahu, 8.0),
    ];

    pub fn calculate_timeline(
        birth_time: DateTime<Utc>,
        moon_long: f64,
    ) -> Vec<DashaPeriod> {
        let nak_duration = 360.0 / 27.0;
        let nak_index_1 = (moon_long / nak_duration).floor() as usize + 1;
        let mut start_idx = (nak_index_1 + 3) % 8;
        if start_idx == 0 { start_idx = 8; }
        let start_idx_0 = start_idx - 1;

        let elapsed_in_nak = moon_long % nak_duration;
        let remaining_in_nak = nak_duration - elapsed_in_nak;
        let portion_remaining = remaining_in_nak / nak_duration;

        let mut timeline = Vec::new();
        let mut current_start = birth_time;
        let years_to_secs = |y: f64| (y * 365.2425 * 24.0 * 60.0 * 60.0) as i64;

        for cycle_num in 0..3 {
            for i in 0..8 {
                let idx = (start_idx_0 + i) % 8;
                let (name, lord, full_years) = Self::CYCLE[idx];
                let actual_years = if cycle_num == 0 && i == 0 {
                    full_years * portion_remaining
                } else {
                    full_years
                };
                let end_time = current_start + Duration::seconds(years_to_secs(actual_years));
                timeline.push(DashaPeriod {
                    lord,
                    start_time: current_start,
                    end_time,
                    sub_dashas: Vec::new(),
                    name: Some(name.to_string()),
                });
                current_start = end_time;
            }
        }
        timeline
    }
}
