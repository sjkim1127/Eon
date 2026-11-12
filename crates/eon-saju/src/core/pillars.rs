//! Four Pillars of Destiny (四柱八字) Calculation
//!
//! Calculates Year, Month, Day, and Hour pillars from date, time, and location.

use crate::core::branch::EarthlyBranch;
use crate::core::element::Element;
use crate::core::ganzi::GanZi;
use crate::core::stem::HeavenlyStem;
use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Timelike, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Saju Calculation Input
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SajuInput {
    /// Year (Solar)
    pub year: i32,
    /// Month (1-12)
    pub month: u32,
    /// Day (1-31)
    pub day: u32,
    /// Hour (0-23)
    pub hour: u32,
    /// Minute (0-59, optional)
    pub minute: u32,
    /// Whether date is Lunar
    pub is_lunar: bool,
    /// Whether leap month (only valid if is_lunar is true)
    pub is_leap_month: bool,
    /// Longitude offset in minutes (e.g., Ansan -33)
    pub longitude_offset_m: i32,
    /// Longitude in degrees (for True Solar Time calculation, e.g., Seoul 127.0)
    pub longitude: Option<f64>,
    /// Latitude in degrees (for climate adjustment)
    pub latitude: Option<f64>,
    /// Timezone offset in hours (e.g., KST = 9.0)
    pub timezone_offset_h: f32,
    /// Whether to apply Night Rat Hour (夜子時)
    /// - true: 23:00~24:00 treated as night rat hour of the current day (preserves Day pillar)
    /// - false: 23:00~24:00 treated as next day's early rat hour (advances Day pillar) - Default
    pub use_night_rat_hour: bool,
    /// Gender (used for Major Luck progression direction)
    pub gender: eon_core::Gender,
    /// Optional exact UTC birth moment for astronomical calculations (solar terms, etc.)
    pub birth_utc: Option<DateTime<Utc>>,
}

impl SajuInput {
    /// Creates solar date/time input
    pub fn new_solar(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            is_lunar: false,
            is_leap_month: false,
            longitude_offset_m: 0,
            longitude: None,
            latitude: None,
            timezone_offset_h: 9.0,         // Default KST
            use_night_rat_hour: false,      // Default next day early rat hour
            gender: eon_core::Gender::Male, // Default Male
            birth_utc: None,
        }
    }

    /// Creates input with geographical coordinates (for True Solar Time correction)
    pub fn new_solar_at(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        lon: f64,
        lat: f64,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            is_lunar: false,
            is_leap_month: false,
            longitude_offset_m: 0,
            longitude: Some(lon),
            latitude: Some(lat),
            timezone_offset_h: 9.0,
            use_night_rat_hour: false,
            gender: eon_core::Gender::Male,
            birth_utc: None,
        }
    }

    /// Creates solar input with regional time offset in minutes
    pub fn new_solar_with_offset(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        offset_m: i32,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            is_lunar: false,
            is_leap_month: false,
            longitude_offset_m: offset_m,
            longitude: None,
            latitude: None,
            timezone_offset_h: 9.0, // Default KST
            use_night_rat_hour: false,
            gender: eon_core::Gender::Male,
            birth_utc: None,
        }
    }

    /// Returns longitude-based time correction in minutes.
    /// Baseline Korea Standard Time (135°E), Seoul (127°E) is -8° -> -32 min.
    pub fn get_longitude_correction_minutes(&self) -> i32 {
        if let Some(lon) = self.longitude {
            let std_meridian = self.timezone_offset_h as f64 * 15.0;
            ((lon - std_meridian) * 4.0).round() as i32
        } else {
            self.longitude_offset_m
        }
    }

    /// Creates lunar date/time input
    pub fn new_lunar(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        is_leap: bool,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            is_lunar: true,
            is_leap_month: is_leap,
            longitude_offset_m: 0,
            longitude: None,
            latitude: None,
            timezone_offset_h: 9.0, // Default KST
            use_night_rat_hour: false,
            gender: eon_core::Gender::Male,
            birth_utc: None,
        }
    }

    /// Sets exact astronomical UTC timestamp
    pub fn with_birth_utc(mut self, birth_utc: DateTime<Utc>) -> Self {
        self.birth_utc = Some(birth_utc);
        self
    }

    /// Sets timezone offset in hours
    pub fn with_timezone_offset(mut self, offset_h: f32) -> Self {
        self.timezone_offset_h = offset_h;
        self
    }

    /// Sets gender
    pub fn with_gender(mut self, gender: eon_core::Gender) -> Self {
        self.gender = gender;
        self
    }

    /// Sets night rat hour option
    pub fn with_night_rat_hour(mut self, use_night_rat: bool) -> Self {
        self.use_night_rat_hour = use_night_rat;
        self
    }
}

/// Four Pillars of Destiny (四柱八字)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FourPillars {
    /// Year Pillar (年柱)
    pub year: GanZi,
    /// Month Pillar (月柱)
    pub month: GanZi,
    /// Day Pillar (日柱)
    pub day: GanZi,
    /// Hour Pillar (時柱)
    pub hour: GanZi,
    /// Reference UTC timestamp
    pub birth_time: DateTime<Utc>,
    /// Gender
    pub gender: eon_core::Gender,
    /// Raw input data (for recomputation)
    pub raw_input: SajuInput,
    /// Supplementary pillars analysis
    pub supplementary_pillars: crate::analysis::supplementary_pillars::SupplementaryPillars,
}

/// Reference year for Year pillar calculation (Year 4 = Jia-Zi year)
const GANZI_BASE_YEAR: i32 = 4;

impl FourPillars {
    /// Calculates Four Pillars from Saju input
    pub fn calculate(input: &SajuInput) -> Result<Self, SajuError> {
        // Convert lunar to solar if required
        let (solar_year, solar_month, solar_day) = if input.is_lunar {
            use eon_data::LunarCalendar;
            let solar_date =
                LunarCalendar::to_solar(input.year, input.month, input.day, input.is_leap_month)
                    .map_err(|e| {
                        SajuError::CalculationError(format!("Lunar date conversion failed: {}", e))
                    })?;
            (solar_date.year(), solar_date.month(), solar_date.day())
        } else {
            (input.year, input.month, input.day)
        };

        // Validate solar month
        if !(1..=12).contains(&solar_month) {
            return Err(SajuError::InvalidDateTime(format!(
                "Invalid month: {}",
                solar_month
            )));
        }

        // 1. UTC timestamp for solar term calculation (astronomical absolute time)
        let dt_absolute_utc = if let Some(utc) = input.birth_utc {
            utc
        } else {
            NaiveDate::from_ymd_opt(solar_year, solar_month, solar_day)
                .and_then(|d| d.and_hms_opt(input.hour, input.minute, 0))
                .map(|dt| {
                    let offset_ms = (input.timezone_offset_h * 3600.0 * 1000.0) as i64;
                    Utc.from_utc_datetime(&(dt - Duration::milliseconds(offset_ms)))
                })
                .ok_or_else(|| {
                    SajuError::InvalidDateTime(format!(
                        "Absolute DT error: {}-{}-{}",
                        solar_year, solar_month, solar_day
                    ))
                })?
        };

        // 2. Regional time correction (True Solar Time calculation) - for Day/Hour pillars
        let lon_correction = input.get_longitude_correction_minutes();
        let (adj_year, adj_month, adj_day, adj_hour, _adj_minute) = if lon_correction != 0 {
            let dt = NaiveDate::from_ymd_opt(solar_year, solar_month, solar_day)
                .and_then(|d| d.and_hms_opt(input.hour, input.minute, 0))
                .ok_or_else(|| {
                    SajuError::InvalidDateTime(format!(
                        "{}-{}-{} {}:{}",
                        solar_year, solar_month, solar_day, input.hour, input.minute
                    ))
                })?;

            let adjusted_dt = dt + Duration::minutes(lon_correction as i64);
            (
                adjusted_dt.year(),
                adjusted_dt.month(),
                adjusted_dt.day(),
                adjusted_dt.hour(),
                adjusted_dt.minute(),
            )
        } else {
            (solar_year, solar_month, solar_day, input.hour, input.minute)
        };

        // Pillar calculations
        // Year and Month pillars use astronomical absolute UTC time
        let year_pillar = Self::calculate_year_pillar(dt_absolute_utc)?;
        let month_pillar = Self::calculate_month_pillar(dt_absolute_utc)?;

        // Day and Hour pillars use adjusted date/time (True Solar Time)
        // [Night Rat Hour handling]
        // If use_night_rat_hour == false and adj_hour >= 23, treat as next day early rat hour
        let (calc_year, calc_month, calc_day) = if !input.use_night_rat_hour && adj_hour >= 23 {
            let base_date =
                NaiveDate::from_ymd_opt(adj_year, adj_month, adj_day).ok_or_else(|| {
                    SajuError::InvalidDateTime(format!(
                        "Cannot create date: {}-{}-{}",
                        adj_year, adj_month, adj_day
                    ))
                })?;
            let next_day = base_date + Duration::days(1);
            (next_day.year(), next_day.month(), next_day.day())
        } else {
            (adj_year, adj_month, adj_day)
        };

        let day_pillar = Self::calculate_day_pillar(calc_year, calc_month, calc_day)?;
        let hour_pillar = Self::calculate_hour_pillar(&day_pillar, adj_hour);

        Ok(Self {
            year: year_pillar,
            month: month_pillar,
            day: day_pillar,
            hour: hour_pillar,
            birth_time: dt_absolute_utc,
            gender: input.gender,
            raw_input: input.clone(),
            supplementary_pillars:
                crate::analysis::supplementary_pillars::SupplementaryPillars::calculate_partial(
                    &year_pillar,
                    &month_pillar,
                    &day_pillar,
                    &hour_pillar,
                ),
        })
    }

    /// Year pillar transitions on Li Chun (Start of Spring, 315° ecliptic longitude).
    pub(crate) fn calculate_year_pillar(dt: DateTime<Utc>) -> Result<GanZi, SajuError> {
        use eon_astro::AstroEngine;
        let engine = AstroEngine::new();
        let year = dt.year();
        let month = dt.month();

        let sun_long = engine
            .get_sun_longitude(dt)
            .map_err(|e| SajuError::CalculationError(format!("Sun longitude error: {}", e)))?;

        let effective_year = if month <= 2 {
            if month == 1 || sun_long < 315.0 {
                year - 1
            } else {
                year
            }
        } else {
            year
        };

        let idx = (effective_year - GANZI_BASE_YEAR).rem_euclid(60);
        Ok(GanZi::from_index(idx))
    }

    /// Month pillar calculation
    pub(crate) fn calculate_month_pillar(dt: DateTime<Utc>) -> Result<GanZi, SajuError> {
        use crate::core::calendar::get_month_branch_index_from_dt;

        let branch_idx = get_month_branch_index_from_dt(dt)?;
        let branch = EarthlyBranch::from_index(branch_idx as i32);

        let year_pillar = Self::calculate_year_pillar(dt)?;
        let year_stem = year_pillar.stem;

        let zhi_idx = branch.index();
        let yi_stem_idx = (year_stem.index() as i32 % 5) * 2 + 2;
        let month_stem_idx = (yi_stem_idx + (zhi_idx as i32 - 2).rem_euclid(12)) % 10;

        Ok(GanZi::new(HeavenlyStem::from_index(month_stem_idx), branch))
    }

    /// Day pillar calculation
    pub(crate) fn calculate_day_pillar(
        year: i32,
        month: u32,
        day: u32,
    ) -> Result<GanZi, SajuError> {
        let date = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| SajuError::InvalidDateTime(format!("{}-{}-{}", year, month, day)))?;

        let idx = eon_data::manseryuk::get_day_ganzi_index(date);
        Ok(GanZi::from_index(idx as i32))
    }

    /// Hour pillar calculation
    pub(crate) fn calculate_hour_pillar(day_pillar: &GanZi, hour: u32) -> GanZi {
        let branch_idx = hour.div_ceil(2) % 12;
        let branch = EarthlyBranch::from_index(branch_idx as i32);

        let day_stem = day_pillar.stem;
        let zi_stem_idx = (day_stem.index() as i32 % 5) * 2;
        let hour_stem_idx = (zi_stem_idx + branch.index() as i32) % 10;

        GanZi::new(HeavenlyStem::from_index(hour_stem_idx), branch)
    }

    /// Five elements distribution
    pub fn element_distribution(&self) -> [(Element, u32); 5] {
        let elements = [
            self.year.stem.element(),
            self.year.branch.element(),
            self.month.stem.element(),
            self.month.branch.element(),
            self.day.stem.element(),
            self.day.branch.element(),
            self.hour.stem.element(),
            self.hour.branch.element(),
        ];

        let mut counts = [0u32; 5];
        for el in elements {
            counts[el as usize] += 1;
        }

        [
            (Element::Wood, counts[0]),
            (Element::Fire, counts[1]),
            (Element::Earth, counts[2]),
            (Element::Metal, counts[3]),
            (Element::Water, counts[4]),
        ]
    }

    /// Day Master (日干)
    pub fn day_master(&self) -> HeavenlyStem {
        self.day.stem
    }

    /// Day Master element
    pub fn day_master_element(&self) -> Element {
        self.day.stem.element()
    }

    /// Element counts (Alias of element_distribution)
    pub fn element_counts(&self) -> [(Element, u32); 5] {
        self.element_distribution()
    }

    /// Hanja notation
    pub fn hanja(&self) -> String {
        format!(
            "{} {} {} {}",
            self.hour.hanja(),
            self.day.hanja(),
            self.month.hanja(),
            self.year.hanja()
        )
    }

    /// Romanized/English notation
    pub fn hangul(&self) -> String {
        format!(
            "{} {} {} {}",
            self.hour.hangul(),
            self.day.hangul(),
            self.month.hangul(),
            self.year.hangul()
        )
    }
}

impl std::fmt::Display for FourPillars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    時    日    月    年")?;
        writeln!(
            f,
            "  {:>2}   {:>2}   {:>2}   {:>2}",
            self.hour.stem.hanja(),
            self.day.stem.hanja(),
            self.month.stem.hanja(),
            self.year.hanja()
        )?;
        writeln!(f, "  ---   ---   ---   ---")?;
        writeln!(
            f,
            "  {}   {}   {}   {}",
            self.hour.hangul(),
            self.day.hangul(),
            self.month.hangul(),
            self.year.hangul()
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Error, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SajuError {
    #[error("Invalid date/time: {0}")]
    InvalidDateTime(String),
    #[error("Lunar conversion not supported")]
    LunarNotSupported,
    #[error("Calculation error: {0}")]
    CalculationError(String),
    #[error("Calendar error: {0}")]
    Calendar(#[from] crate::core::calendar::CalendarError),
    #[error("Astronomical error: {0}")]
    Astro(#[from] eon_astro::AstroError),
    #[error("Data error: {0}")]
    Data(#[from] eon_data::DataError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year_pillar_basic() {
        let dt = Utc.with_ymd_and_hms(2024, 3, 20, 10, 0, 0).unwrap();
        let pillar = FourPillars::calculate_year_pillar(dt).unwrap();
        assert_eq!(pillar.stem, HeavenlyStem::Jia);
        assert_eq!(pillar.branch, EarthlyBranch::Chen);
    }

    #[test]
    fn test_rat_hour_options() {
        // 2024-03-20 23:30 (Boundary of night rat hour / early rat hour)
        let input_base = SajuInput::new_solar(2024, 3, 20, 23, 30);

        // 1. Without night rat hour (default) -> advances to next day pillar
        let pillars_no_night = FourPillars::calculate(&input_base).unwrap();
        // March 20 is Gui-Mao, 21 is Jia-Chen. After 23:00, becomes Jia-Chen.
        assert_eq!(pillars_no_night.day.stem.index(), 0); // Jia (甲)
        assert_eq!(pillars_no_night.hour.branch.index(), 0); // Zi (子)

        // 2. With night rat hour -> preserves current day pillar
        let input_night = input_base.with_night_rat_hour(true);
        let pillars_night = FourPillars::calculate(&input_night).unwrap();
        assert_eq!(pillars_night.day.stem.index(), 9); // Gui (癸)
        assert_eq!(pillars_night.hour.branch.index(), 0); // Zi (子)
    }

    #[test]
    fn test_month_pillar_zi_chou() {
        // 2014 (Jia-Wu year) December (Bing-Zi month) - after Da Xue (12/7)
        let input = SajuInput::new_solar(2014, 12, 10, 12, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        assert_eq!(pillars.year.stem.index(), 0); // Jia
        assert_eq!(pillars.month.stem.index(), 2); // Bing (Zi month of Jia year is Bing Zi)
        assert_eq!(pillars.month.branch.index(), 0); // Zi

        // 2015 New Year (still Jia-Wu year until Li Chun) January (Ding-Chou month) - after Xiao Han (1/6)
        let input2 = SajuInput::new_solar(2015, 1, 10, 12, 0);
        let pillars2 = FourPillars::calculate(&input2).unwrap();

        assert_eq!(pillars2.year.stem.index(), 0); // Jia (still Jia Wu until Feb)
        assert_eq!(pillars2.month.stem.index(), 3); // Ding (Chou month of Jia year is Ding Chou)
        assert_eq!(pillars2.month.branch.index(), 1); // Chou
    }
}
