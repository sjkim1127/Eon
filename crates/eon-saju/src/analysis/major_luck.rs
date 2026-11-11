//! Major Luck Cycles (大運) Calculation
//!
//! Calculates 10-year major luck cycle flows.
//!
//! ## Calculation Principles
//!
//! 1. **Forward / Reverse Progression**:
//!    - Yang Male or Yin Female -> Forward (advances along 60 JiaZi)
//!    - Yin Male or Yang Female -> Reverse (retreats along 60 JiaZi)
//!
//! 2. **Start Age Calculation**:
//!    - Days from birth to adjacent solar term divided by 3 = start age
//!    - Forward: days to next solar term
//!    - Reverse: days to previous solar term
//!
//! 3. **Pillars Sequence**:
//!    - Sequences sequentially from the Month Pillar GanZi.
//!

use crate::core::element::Polarity;
use crate::core::ganzi::GanZi;
use crate::core::pillars::FourPillars;
use crate::core::pillars::SajuError;
use crate::core::stem::HeavenlyStem;
use crate::core::ten_gods::TenGod;
use chrono::{Datelike, NaiveDate, TimeZone, Utc};
use eon_core::Gender;
use serde::{Deserialize, Serialize};

/// Major Luck Progression Direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LuckDirection {
    /// Forward (advance from month pillar)
    Forward,
    /// Reverse (recede from month pillar)
    Reverse,
}

impl LuckDirection {
    /// Determine major luck direction from year stem polarity and gender
    ///
    /// -   or   →
    /// -   or   →
    pub fn from_year_and_gender(year_stem: HeavenlyStem, gender: Gender) -> Self {
        let is_yang_year = year_stem.polarity() == Polarity::Yang;
        let is_male = gender.is_male();

        if (is_yang_year && is_male) || (!is_yang_year && !is_male) {
            Self::Forward
        } else {
            Self::Reverse
        }
    }
}

impl std::fmt::Display for LuckDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Forward => write!(f, "Forward"),
            Self::Reverse => write!(f, "Reverse"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MajorLuck {
    /// Major Luck GanZi
    pub ganzi: GanZi,
    /// Start age
    pub start_age: u32,
    /// End age
    pub end_age: u32,
    /// Stem Ten God
    pub stem_god: TenGod,
    /// Branch Ten God (Primary Stem)
    pub branch_god: TenGod,
    /// Exact major luck start timestamp
    pub start_date: chrono::DateTime<chrono::Utc>,
}

impl MajorLuck {
    /// Construct new Major Luck
    pub fn new(
        ganzi: GanZi,
        start_age: u32,
        day_master: HeavenlyStem,
        start_date: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            ganzi,
            start_age,
            end_age: start_age + 9, // 10-year cycle
            stem_god: TenGod::from_stems(day_master, ganzi.stem),
            branch_god: TenGod::from_stem_and_branch(day_master, ganzi.branch),
            start_date,
        }
    }

    /// Check if given age falls within this cycle
    pub fn contains_age(&self, age: u32) -> bool {
        age >= self.start_age && age <= self.end_age
    }

    /// Korean nominal age
    pub fn korean_age(&self, birth_year: i32) -> u32 {
        //     -   + 1
        (self.start_date.naive_utc().year() - birth_year + 1) as u32
    }
}

impl std::fmt::Display for MajorLuck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ages {:2}~{:2}: {} ({}/{}) | Start: {}",
            self.start_age,
            self.end_age,
            self.ganzi,
            self.stem_god.hangul(),
            self.branch_god.hangul(),
            self.start_date.format("%Y-%m-%d %H:%M")
        )
    }
}

/// Major Luck Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MajorLuckAnalysis {
    /// Direction
    pub direction: LuckDirection,
    /// Start age (years)
    pub start_age: u32,
    /// Start age (months)
    pub start_months: u32,
    /// Start age (days)
    pub start_days: u32,
    /// Exact start date/time
    pub start_date: chrono::DateTime<chrono::Utc>,
    /// List of major luck cycles (8-10 cycles)
    pub cycles: Vec<MajorLuck>,
    /// Day Master
    pub day_master: HeavenlyStem,
}

impl MajorLuckAnalysis {
    /// Generate major luck analysis using astronomical engine
    pub fn calculate_astro(
        pillars: &FourPillars,
        gender: Gender,
        birth_year: i32,
        birth_month: u32,
        birth_day: u32,
        birth_hour: u32,
        birth_min: u32,
    ) -> Result<Self, SajuError> {
        use eon_astro::AstroEngine;

        let day_master = pillars.day_master();
        let direction = LuckDirection::from_year_and_gender(pillars.year.stem, gender);
        let astro = AstroEngine::new();

        // KST 9   UTC
        let dt_local = NaiveDate::from_ymd_opt(birth_year, birth_month, birth_day)
            .and_then(|d| d.and_hms_opt(birth_hour, birth_min, 0))
            .ok_or_else(|| {
                SajuError::InvalidDateTime(format!(
                    "Invalid date for major luck calculation: {}-{}-{} {}:{}",
                    birth_year, birth_month, birth_day, birth_hour, birth_min
                ))
            })?;

        let tz_offset =
            chrono::Duration::minutes((pillars.raw_input.timezone_offset_h * 60.0) as i64);
        let birth_time = Utc.from_utc_datetime(&(dt_local - tz_offset));

        //  24   (0:  ~ 23: )
        let term_idx = astro.get_solar_term_index(birth_time)?;

        //  '(Jieqi,  )'
        let target_term_idx = match direction {
            LuckDirection::Forward => {
                // :    (     )
                if term_idx.is_multiple_of(2) {
                    (term_idx + 2) % 24
                } else {
                    (term_idx + 1) % 24
                }
            }
            LuckDirection::Reverse => {
                // :    (      )
                if term_idx.is_multiple_of(2) {
                    term_idx
                } else {
                    term_idx - 1
                }
            }
        };

        let target_term_time = astro
            .find_solar_term_time(birth_time, target_term_idx)
            .map_err(|e| {
                SajuError::CalculationError(format!("Failed to calculate solar term time: {}", e))
            })?;

        //     (, ,  )
        let (start_age, start_months, start_days) =
            Self::calculate_precise_start_with_times(birth_time, target_term_time, direction);

        //     ( )
        // 3 = 1 : ()  (10 diff = 1217  )
        let diff = if direction == LuckDirection::Forward {
            target_term_time - birth_time
        } else {
            birth_time - target_term_time
        };
        let offset_seconds = (diff.num_seconds() as f64 * (365.2425 / 3.0)) as i64;
        let start_date = birth_time + chrono::Duration::seconds(offset_seconds);

        //    (  /)
        let mut cycles = Vec::new();
        let mut current_ganzi = pillars.month;

        for i in 0..10 {
            let age = start_age + (i * 10);
            let cycle_start_date = start_date
                + chrono::Duration::seconds((i as f64 * 10.0 * 365.2425 * 86400.0) as i64);

            current_ganzi = match direction {
                LuckDirection::Forward => current_ganzi.next(),
                LuckDirection::Reverse => current_ganzi.prev(),
            };
            cycles.push(MajorLuck::new(
                current_ganzi,
                age,
                day_master,
                cycle_start_date,
            ));
        }

        Ok(Self {
            direction,
            start_age,
            start_months,
            start_days,
            start_date,
            cycles,
            day_master,
        })
    }

    pub fn calculate(
        pillars: &FourPillars,
        gender: Gender,
        birth_year: i32,
        birth_month: u32,
        birth_day: u32,
        birth_hour: u32,
        birth_min: u32,
        term_year: i32,
        term_month: u32,
        term_day: u32,
        term_hour: u32,
        term_min: u32,
    ) -> Result<Self, SajuError> {
        let day_master = pillars.day_master();
        let direction = LuckDirection::from_year_and_gender(pillars.year.stem, gender);

        let birth_time = Utc
            .with_ymd_and_hms(birth_year, birth_month, birth_day, birth_hour, birth_min, 0)
            .earliest()
            .ok_or_else(|| {
                SajuError::InvalidDateTime(format!(
                    "Invalid birth date/time: {}-{}-{} {}:{}",
                    birth_year, birth_month, birth_day, birth_hour, birth_min
                ))
            })?;
        let term_time = Utc
            .with_ymd_and_hms(term_year, term_month, term_day, term_hour, term_min, 0)
            .earliest()
            .ok_or_else(|| {
                SajuError::InvalidDateTime(format!(
                    "Invalid solar term date/time: {}-{}-{} {}:{}",
                    term_year, term_month, term_day, term_hour, term_min
                ))
            })?;

        //     (, ,  )
        let (start_age, start_months, start_days) =
            Self::calculate_precise_start_with_times(birth_time, term_time, direction);

        let diff = if direction == LuckDirection::Forward {
            term_time - birth_time
        } else {
            birth_time - term_time
        };
        let offset_seconds = (diff.num_seconds() as f64 * (365.2425 / 3.0)) as i64;
        let start_date = birth_time + chrono::Duration::seconds(offset_seconds);

        //    (  /)
        let mut cycles = Vec::new();
        let mut current_ganzi = pillars.month;

        for i in 0..10 {
            let age = start_age + (i * 10);
            let cycle_start_date = start_date
                + chrono::Duration::seconds((i as f64 * 10.0 * 365.2425 * 86400.0) as i64);

            current_ganzi = match direction {
                LuckDirection::Forward => current_ganzi.next(),
                LuckDirection::Reverse => current_ganzi.prev(),
            };

            cycles.push(MajorLuck::new(
                current_ganzi,
                age,
                day_master,
                cycle_start_date,
            ));
        }

        Ok(Self {
            direction,
            start_age,
            start_months,
            start_days,
            start_date,
            cycles,
            day_master,
        })
    }

    fn calculate_precise_start_with_times(
        birth_time: chrono::DateTime<chrono::Utc>,
        term_time: chrono::DateTime<chrono::Utc>,
        direction: LuckDirection,
    ) -> (u32, u32, u32) {
        let diff = if direction == LuckDirection::Forward {
            term_time - birth_time
        } else {
            birth_time - term_time
        };

        let total_minutes = diff.num_minutes().abs();

        // 3(4320) = 1
        let years = total_minutes / 4320;
        let rem_years = total_minutes % 4320;

        // 6(360) = 1
        let months = rem_years / 360;
        let rem_months = rem_years % 360;

        // 12 = 1
        let days = rem_months / 12;

        (years as u32, months as u32, days as u32)
    }

    pub fn at_age(&self, age: u32) -> Option<&MajorLuck> {
        self.cycles.iter().find(|luck| luck.contains_age(age))
    }

    pub fn current(&self, current_age: u32) -> Option<&MajorLuck> {
        self.at_age(current_age)
    }
}

impl std::fmt::Display for MajorLuckAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Major Luck ({}, starts after {}y {}m {}d)",
            self.direction, self.start_age, self.start_months, self.start_days
        )?;
        writeln!(f, "──────────────────────────────────────")?;

        for (i, luck) in self.cycles.iter().enumerate() {
            write!(f, "{:2}. ", i + 1)?;
            writeln!(f, "{}", luck)?;
        }

        Ok(())
    }
}

impl FourPillars {
    pub fn major_luck(
        &self,
        gender: Gender,
        b_year: i32,
        b_month: u32,
        b_day: u32,
        b_hour: u32,
        b_min: u32,
    ) -> Result<MajorLuckAnalysis, SajuError> {
        MajorLuckAnalysis::calculate_astro(self, gender, b_year, b_month, b_day, b_hour, b_min)
    }

    pub fn major_luck_precise(
        &self,
        gender: Gender,
        b_year: i32,
        b_month: u32,
        b_day: u32,
        b_hour: u32,
        b_min: u32,
        t_year: i32,
        t_month: u32,
        t_day: u32,
        t_hour: u32,
        t_min: u32,
    ) -> Result<MajorLuckAnalysis, SajuError> {
        MajorLuckAnalysis::calculate(
            self, gender, b_year, b_month, b_day, b_hour, b_min, t_year, t_month, t_day, t_hour,
            t_min,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pillars::SajuInput;

    #[test]
    fn test_luck_direction_yang_male() {
        let direction = LuckDirection::from_year_and_gender(HeavenlyStem::Jia, Gender::Male);
        assert_eq!(direction, LuckDirection::Forward);
    }

    #[test]
    fn test_luck_direction_yang_female() {
        let direction = LuckDirection::from_year_and_gender(HeavenlyStem::Jia, Gender::Female);
        assert_eq!(direction, LuckDirection::Reverse);
    }

    #[test]
    fn test_luck_direction_yin_male() {
        let direction = LuckDirection::from_year_and_gender(HeavenlyStem::Yi, Gender::Male);
        assert_eq!(direction, LuckDirection::Reverse);
    }

    #[test]
    fn test_luck_direction_yin_female() {
        let direction = LuckDirection::from_year_and_gender(HeavenlyStem::Yi, Gender::Female);
        assert_eq!(direction, LuckDirection::Forward);
    }

    #[test]
    fn test_user_major_luck() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        let luck = pillars
            .major_luck_precise(
                Gender::Male,
                2004,
                11,
                27,
                22,
                0, // Birth
                2004,
                12,
                7,
                3,
                48, // DaXue
            )
            .unwrap();

        assert_eq!(luck.direction, LuckDirection::Forward);
        assert!(luck.cycles.len() >= 8);
    }

    #[test]
    fn test_major_luck_at_age() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        let luck = pillars
            .major_luck_precise(
                Gender::Male,
                2004,
                11,
                27,
                22,
                0, // Birth
                2004,
                12,
                7,
                3,
                48, // DaXue
            )
            .unwrap();

        let current = luck.at_age(22);
        assert!(current.is_some());
    }

    #[test]
    fn test_major_luck_astro() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        // KST 22:00 -> UTC 13:00 (calculate_astro  )
        let luck = pillars
            .major_luck(Gender::Male, 2004, 11, 27, 22, 0)
            .unwrap();

        // 11 27 22:00 KST ~ 12 7 03:48 KST ()
        //  9 5 -> 3
        assert_eq!(luck.start_age, 3);
    }
}
