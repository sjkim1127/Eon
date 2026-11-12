use crate::core::chart::VedicChart;
use crate::planets::VedicPlanet;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

fn normalize_moon_longitude(longitude: f64) -> f64 {
    if longitude.is_finite() {
        longitude.rem_euclid(360.0)
    } else {
        0.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashaPeriod {
    pub lord: VedicPlanet,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub sub_dashas: Vec<DashaPeriod>,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorable: Option<bool>,
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
        Self::calculate_timeline_with_year_days(birth_time, moon_long, levels, 365.2425)
    }

    pub fn calculate_timeline_with_year_days(
        birth_time: DateTime<Utc>,
        moon_long: f64,
        levels: u8,
        year_days: f64,
    ) -> Vec<DashaPeriod> {
        // Moon Nakshatra range is 13°20' (13.3333...)
        let nak_duration = 360.0 / 27.0;
        let moon_long = normalize_moon_longitude(moon_long);
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

        let year_days = if year_days.is_finite() && year_days > 0.0 {
            year_days
        } else {
            365.2425
        };
        let years_to_secs = |y: f64| (y * year_days * 24.0 * 60.0 * 60.0) as i64;

        let first_end = current_start + Duration::seconds(years_to_secs(remaining_years));

        let mut first_dasha = DashaPeriod {
            lord: first_lord,
            start_time: current_start,
            end_time: first_end,
            sub_dashas: Vec::new(),
            name: None,
            interpretation: None,
            is_favorable: None,
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
                interpretation: None,
                is_favorable: None,
            };

            if levels > 1 {
                dasha.sub_dashas =
                    Self::calculate_sub_periods(lord, current_start, end_time, levels - 1, None);
            }

            timeline.push(dasha);
            current_start = end_time;
        }

        // Guarantee 120-year complete cycle coverage even when starting mid-dasha
        let elapsed_portion = 1.0 - portion_remaining;
        if elapsed_portion > 0.0001 {
            let remainder_years = full_years * elapsed_portion;
            let end_time = current_start + Duration::seconds(years_to_secs(remainder_years));

            let mut dasha = DashaPeriod {
                lord: first_lord,
                start_time: current_start,
                end_time,
                sub_dashas: Vec::new(),
                name: None,
                interpretation: None,
                is_favorable: None,
            };

            if levels > 1 {
                dasha.sub_dashas = Self::calculate_sub_periods_prefix(
                    first_lord,
                    current_start,
                    end_time,
                    levels - 1,
                    elapsed_portion,
                );
            }

            timeline.push(dasha);
        }

        timeline
    }

    fn calculate_sub_periods_prefix(
        lord: VedicPlanet,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        levels_remaining: u8,
        prefix_fraction: f64,
    ) -> Vec<DashaPeriod> {
        let total_duration = end.signed_duration_since(start).num_seconds() as f64;
        let lord_idx = Self::CYCLE
            .iter()
            .position(|&(p, _)| p == lord)
            .unwrap_or(0);

        let full_mahadasha_duration = if prefix_fraction > 0.0 {
            total_duration / prefix_fraction
        } else {
            total_duration
        };

        let mut sub_periods = Vec::new();
        let mut current_start = start;
        let mut cumulative_fraction = 0.0;

        for i in 0..9 {
            let idx = (lord_idx + i) % 9;
            let (p_lord, years) = Self::CYCLE[idx];
            let sub_fraction = years / Self::TOTAL_YEARS;

            if cumulative_fraction >= prefix_fraction - 1e-6 {
                break;
            }

            let portion_to_take = (prefix_fraction - cumulative_fraction).min(sub_fraction);
            let sub_duration = full_mahadasha_duration * portion_to_take;
            let sub_end = current_start + Duration::seconds(sub_duration as i64);

            let mut sub = DashaPeriod {
                lord: p_lord,
                start_time: current_start,
                end_time: sub_end,
                sub_dashas: Vec::new(),
                name: None,
                interpretation: None,
                is_favorable: None,
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
            cumulative_fraction += portion_to_take;
        }

        sub_periods
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
            let full_mahadasha_duration = total_duration / initial_portion.unwrap_or(1.0);
            let sub_duration = full_mahadasha_duration * remaining_sub_portion;
            let sub_end = current_start + Duration::seconds(sub_duration as i64);

            let mut sub = DashaPeriod {
                lord: p_lord,
                start_time: current_start,
                end_time: sub_end,
                sub_dashas: Vec::new(),
                name: None,
                interpretation: None,
                is_favorable: None,
            };

            if levels_remaining > 1 {
                sub.sub_dashas = Self::calculate_sub_periods(
                    p_lord,
                    current_start,
                    sub_end,
                    levels_remaining - 1,
                    Some(remaining_sub_portion / full_sub_portion),
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
                    interpretation: None,
                    is_favorable: None,
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
                    interpretation: None,
                    is_favorable: None,
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

    pub fn attach_interpretations(timeline: &mut [DashaPeriod], chart: &VedicChart) {
        for maha in timeline.iter_mut() {
            let maha_lord = maha.lord;
            let maha_pos = chart.planets.iter().find(|p| p.planet == maha_lord);

            if let Some(pos) = maha_pos {
                let house = pos.house_index;
                let is_exalted = pos.rasi == maha_lord.exaltation_rasi();
                let is_debilitated = pos.rasi == maha_lord.debilitation_rasi();
                let is_own = VedicPlanet::get_ruler_of(pos.rasi) == maha_lord;

                let (favorable, dignity_desc) = if is_exalted {
                    (
                        true,
                        "Exalted, expressing exceptionally powerful auspicious energy.",
                    )
                } else if is_own {
                    (
                        true,
                        "In its own sign, granting stability and positive growth.",
                    )
                } else if is_debilitated {
                    (
                        false,
                        "Debilitated, weakening planetary strength and introducing life challenges.",
                    )
                } else {
                    (
                        true,
                        "In a normal disposition, steadily fulfilling its natural significations.",
                    )
                };

                let theme = match house {
                    1 => "Self-development, vitality, new beginnings",
                    2 => "Wealth, family assets, speech, core values",
                    3 => "Courage, initiative, siblings, short journeys",
                    4 => "Inner peace, mother, real estate, emotional security",
                    5 => "Creativity, progeny, intellect, strategic investments",
                    6 => "Overcoming competition, health discipline, service",
                    7 => "Spouse, partnership, public relations",
                    8 => "Transformation, occult studies, unexpected changes",
                    9 => "Fortune, higher wisdom, dharma, spiritual guidance",
                    10 => "Career achievements, public standing, authority",
                    11 => "Income gains, influential networks, fulfillment of goals",
                    12 => "Spiritual liberation, foreign travel, expenditures, solitude",
                    _ => "House domain",
                };

                let lord_desc = match maha_lord {
                    VedicPlanet::Sun => "Era of confidence, vitality, and executive leadership",
                    VedicPlanet::Moon => "Era of emotional maturation and inner evolution",
                    VedicPlanet::Mars => "Era of decisive courage, ambition, and dynamic action",
                    VedicPlanet::Mercury => {
                        "Era of intellect, commerce, and analytical communication"
                    }
                    VedicPlanet::Jupiter => "Era of wisdom, higher expansion, and divine blessings",
                    VedicPlanet::Venus => {
                        "Era of harmony, artistic creation, and material prosperity"
                    }
                    VedicPlanet::Saturn => "Era of perseverance, discipline, and karmic harvest",
                    VedicPlanet::Rahu => {
                        "Era of worldly ambition and intense unconventional expansion"
                    }
                    VedicPlanet::Ketu => {
                        "Era of spiritual discernment, detachment, and karmic release"
                    }
                    _ => "",
                };

                maha.is_favorable = Some(favorable);
                maha.interpretation = Some(
                    format!(
                        "[{}]: Focuses on House {} themes ({}). The planet is {}{}",
                        lord_desc,
                        house,
                        theme,
                        dignity_desc,
                        if !favorable {
                            " Requires conscious patience and strategic resilience."
                        } else {
                            ""
                        }
                    )
                    .trim()
                    .to_string(),
                );

                for antar in maha.sub_dashas.iter_mut() {
                    let antar_lord = antar.lord;
                    let antar_pos = chart.planets.iter().find(|p| p.planet == antar_lord);

                    if let Some(a_pos) = antar_pos {
                        let diff =
                            (a_pos.house_index as i32 - pos.house_index as i32).rem_euclid(12) + 1;

                        let (a_favorable, rel_desc) = match diff {
                            1 | 5 | 9 => (true, "Major and sub-period lords form a harmonious 1/5/9 Trikona relationship, fostering growth, fortune, and alignment."),
                            3 | 11 => (true, "Efforts yield tangible accomplishments and expansive social connections."),
                            4 | 10 => (true, "A Kendra period providing stability, public achievement, and solid foundational progress."),
                            2 | 12 => (false, "A mixed period highlighting income vs. expenditure cycles and family dynamics."),
                            6 | 8 => (false, "Forms a difficult 6/8 Shadashtaka axis with the major lord, bringing unexpected shifts, conflicts, and endurance tests."),
                            7 => (true, "A pivotal turning point unfolding through partnerships and interpersonal connections."),
                            _ => (true, "A steady, balanced developmental flow."),
                        };

                        let a_lord_desc = match antar_lord {
                            VedicPlanet::Sun => "Self-actualization and authority",
                            VedicPlanet::Moon => "Emotional receptivity and mind",
                            VedicPlanet::Mars => "Drive, courage, and initiative",
                            VedicPlanet::Mercury => "Intellectual inquiry and commerce",
                            VedicPlanet::Jupiter => "Expansive wisdom and benefic guidance",
                            VedicPlanet::Venus => "Creative harmony, romance, and luxury",
                            VedicPlanet::Saturn => "Discipline, perseverance, and structure",
                            VedicPlanet::Rahu => "Material ambition and breakthrough exploration",
                            VedicPlanet::Ketu => "Spiritual introspection and detachment",
                            _ => "",
                        };

                        antar.is_favorable = Some(favorable && a_favorable);
                        antar.interpretation = Some(
                            format!(
                                "Focusing on {}: {}. {}",
                                a_lord_desc,
                                rel_desc,
                                if !a_favorable {
                                    "Caution advised against over-expansion and unexpected turbulence."
                                } else {
                                    ""
                                }
                            )
                            .trim()
                            .to_string(),
                        );
                    }
                }
            }
        }
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

    pub fn calculate_timeline(birth_time: DateTime<Utc>, moon_long: f64) -> Vec<DashaPeriod> {
        let nak_duration = 360.0 / 27.0;
        let moon_long = normalize_moon_longitude(moon_long);
        let nak_index_1 = (moon_long / nak_duration).floor() as usize + 1;
        let mut start_idx = (nak_index_1 + 3) % 8;
        if start_idx == 0 {
            start_idx = 8;
        }
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
                    interpretation: None,
                    is_favorable: None,
                });
                current_start = end_time;
            }
        }
        timeline
    }
}

#[cfg(test)]
mod tests {
    use super::{VimshottariDasha, YoginiDasha};
    use crate::planets::VedicPlanet;
    use chrono::{TimeZone, Utc};

    #[test]
    fn dasha_timelines_normalize_boundary_longitudes() {
        let birth = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        for longitude in [0.0, 360.0, -360.0, f64::NAN, f64::INFINITY] {
            let vimshottari = VimshottariDasha::calculate_timeline(birth, longitude, 1);
            let yogini = YoginiDasha::calculate_timeline(birth, longitude);
            assert!(!vimshottari.is_empty());
            assert!(!yogini.is_empty());
            assert_eq!(vimshottari[0].lord, VedicPlanet::Ketu);
        }
    }

    #[test]
    fn test_vimshottari_guarantees_120_year_coverage() {
        let birth = Utc.with_ymd_and_hms(1990, 5, 15, 10, 30, 0).unwrap();
        // Moon in middle of a nakshatra (say 50% elapsed)
        let moon_long = 125.5;
        let timeline = VimshottariDasha::calculate_timeline(birth, moon_long, 2);

        // First dasha starts at birth
        assert_eq!(timeline.first().unwrap().start_time, birth);

        // Last dasha ends ~120 years later
        let end_time = timeline.last().unwrap().end_time;
        let total_days = (end_time - birth).num_days();
        let expected_days = (120.0 * 365.2425) as i64;
        assert!(
            (total_days - expected_days).abs() <= 2,
            "Total days covered {} must match 120 years {}",
            total_days,
            expected_days
        );
    }
}
