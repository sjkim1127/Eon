//! Birth Information (BirthInfo) - Historical Timezone & DST Support
//!
//! Provides common birth data structures used across all destiny systems.
//! Utilizes the IANA timezone database (via `chrono-tz`) to automatically
//! handle historical daylight saving time (DST) and timezone transitions.

use crate::error::CoreError;
use crate::location::Location;
use chrono::{DateTime, Datelike, LocalResult, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

/// Gender of the individual
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    /// Male
    Male,
    /// Female
    Female,
}

impl Gender {
    pub fn is_male(&self) -> bool {
        matches!(self, Gender::Male)
    }

    pub fn is_female(&self) -> bool {
        matches!(self, Gender::Female)
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
        }
    }
}

/// Calendar system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CalendarType {
    /// Solar (Gregorian Calendar)
    #[default]
    Solar,
    /// Lunar calendar
    Lunar {
        /// Whether the lunar month is a leap month
        is_leap_month: bool,
    },
}

impl CalendarType {
    pub fn is_solar(&self) -> bool {
        matches!(self, CalendarType::Solar)
    }

    pub fn is_lunar(&self) -> bool {
        matches!(self, CalendarType::Lunar { .. })
    }

    pub fn is_leap_month(&self) -> bool {
        matches!(
            self,
            CalendarType::Lunar {
                is_leap_month: true
            }
        )
    }
}

/// Comprehensive Birth Information
///
/// Used across all metaphysical engines (BaZi, Vedic, ZWDS, Western, etc.).
///
/// ## Timezone Support
/// Automatically handles historical daylight saving time (DST) via the IANA database.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BirthInfo {
    /// Year
    pub year: i32,
    /// Month (1-12)
    pub month: u32,
    /// Day (1-31)
    pub day: u32,
    /// Hour (0-23)
    pub hour: u32,
    /// Minute (0-59)
    pub minute: u32,
    /// Calendar type (Solar / Lunar)
    pub calendar: CalendarType,
    /// Geographical location (optional)
    pub location: Option<Location>,
    /// IANA timezone string (e.g. "Asia/Seoul", "America/New_York")
    pub timezone: Option<String>,
    /// Whether to apply longitude-based True Solar Time correction
    pub use_true_solar_time: bool,
    /// Gender (optional)
    pub gender: Option<Gender>,
}

impl BirthInfo {
    /// Create new solar birth info
    pub fn solar(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            calendar: CalendarType::Solar,
            location: None,
            timezone: None,
            use_true_solar_time: false,
            gender: None,
        }
    }

    /// Create new lunar birth info
    pub fn lunar(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        is_leap_month: bool,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            calendar: CalendarType::Lunar { is_leap_month },
            location: None,
            timezone: None,
            use_true_solar_time: false,
            gender: None,
        }
    }

    /// Set geographical location
    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    /// Set IANA timezone
    pub fn with_timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = Some(timezone.into());
        self
    }

    /// Set Korea timezone (convenience function)
    pub fn with_korea_timezone(self) -> Self {
        self.with_timezone(timezones::KOREA)
    }

    /// Set True Solar Time toggle
    pub fn with_true_solar_time(mut self, use_it: bool) -> Self {
        self.use_true_solar_time = use_it;
        self
    }

    /// Set gender
    pub fn with_gender(mut self, gender: Gender) -> Self {
        self.gender = Some(gender);
        self
    }

    /// Parse IANA timezone
    pub fn parse_timezone(&self) -> Result<Tz, CoreError> {
        let tz_str = self.timezone.as_deref().unwrap_or(timezones::KOREA);
        tz_str
            .parse::<Tz>()
            .map_err(|_| CoreError::InvalidTimezone(tz_str.to_string()))
    }

    /// Build local NaiveDateTime
    pub fn local_naive_datetime(&self) -> Result<NaiveDateTime, CoreError> {
        let date = NaiveDate::from_ymd_opt(self.year, self.month, self.day)
            .ok_or(CoreError::InvalidDateTime)?;
        let time =
            NaiveTime::from_hms_opt(self.hour, self.minute, 0).ok_or(CoreError::InvalidDateTime)?;
        Ok(NaiveDateTime::new(date, time))
    }

    /// Convert to UTC DateTime, accounting for historical DST
    pub fn to_utc(&self) -> Result<DateTime<Utc>, CoreError> {
        let naive = self.local_naive_datetime()?;
        let tz = self.parse_timezone()?;

        use chrono::TimeZone;
        match tz.from_local_datetime(&naive) {
            LocalResult::Single(dt) => Ok(dt.with_timezone(&Utc)),
            LocalResult::Ambiguous(dt1, _dt2) => {
                // When ambiguous due to DST fallback, default to earlier daylight time
                Ok(dt1.with_timezone(&Utc))
            }
            LocalResult::None => {
                // Gap hour due to spring forward: advance by 1 hour
                let adjusted = naive + chrono::Duration::hours(1);
                match tz.from_local_datetime(&adjusted) {
                    LocalResult::Single(dt) => Ok(dt.with_timezone(&Utc)),
                    LocalResult::Ambiguous(dt1, _) => Ok(dt1.with_timezone(&Utc)),
                    LocalResult::None => Err(CoreError::NonExistentLocalTime),
                }
            }
        }
    }

    /// Check whether DST was active at birth time
    pub fn is_dst(&self) -> bool {
        let naive = match self.local_naive_datetime() {
            Ok(n) => n,
            Err(_) => return false,
        };
        let tz = match self.parse_timezone() {
            Ok(t) => t,
            Err(_) => return false,
        };

        use chrono::TimeZone;
        use chrono_tz::OffsetComponents;

        match tz.from_local_datetime(&naive) {
            LocalResult::Single(dt) => dt.offset().dst_offset().num_seconds() > 0,
            LocalResult::Ambiguous(dt1, _) => dt1.offset().dst_offset().num_seconds() > 0,
            LocalResult::None => false,
        }
    }

    /// DST shift in minutes in effect at the local birth time (0 when none or timezone unknown).
    ///
    /// True solar time is derived from the *standard* (non-DST) clock, so this shift must be
    /// removed from the local clock time before applying the longitude correction.
    pub fn dst_offset_minutes(&self) -> i32 {
        let (Ok(naive), Ok(tz)) = (self.local_naive_datetime(), self.parse_timezone()) else {
            return 0;
        };

        use chrono::TimeZone;
        use chrono_tz::OffsetComponents;

        let secs = match tz.from_local_datetime(&naive) {
            LocalResult::Single(dt) => dt.offset().dst_offset().num_seconds(),
            LocalResult::Ambiguous(dt1, _) => dt1.offset().dst_offset().num_seconds(),
            LocalResult::None => 0,
        };
        (secs / 60) as i32
    }

    /// Birth clock time on the standard (non-DST) clock, without any longitude correction.
    ///
    /// Equals the local clock time outside DST; during DST it is one DST shift earlier.
    pub fn standard_clock_datetime(&self) -> (i32, u32, u32, u32, u32) {
        match self.local_naive_datetime() {
            Ok(naive) => {
                let t = naive - chrono::Duration::minutes(self.dst_offset_minutes() as i64);
                (t.year(), t.month(), t.day(), t.hour(), t.minute())
            }
            Err(_) => (self.year, self.month, self.day, self.hour, self.minute),
        }
    }

    /// Returns total UTC offset in hours (base + DST)
    pub fn dst_offset_hours(&self) -> Option<f64> {
        let naive = self.local_naive_datetime().ok()?;
        let tz = self.parse_timezone().ok()?;

        use chrono::TimeZone;
        use chrono_tz::OffsetComponents;

        match tz.from_local_datetime(&naive) {
            LocalResult::Single(dt) => {
                let total = dt.offset().base_utc_offset() + dt.offset().dst_offset();
                Some(total.num_seconds() as f64 / 3600.0)
            }
            LocalResult::Ambiguous(dt1, _) => {
                let total = dt1.offset().base_utc_offset() + dt1.offset().dst_offset();
                Some(total.num_seconds() as f64 / 3600.0)
            }
            LocalResult::None => None,
        }
    }

    /// Local solar time offset in minutes based on longitude
    pub fn longitude_offset_minutes(&self) -> i32 {
        if let Some(ref loc) = self.location {
            loc.time_offset_minutes()
        } else {
            0
        }
    }

    /// Returns corrected time (hour, minute) if True Solar Time is active
    pub fn corrected_time(&self) -> (u32, u32) {
        if !self.use_true_solar_time {
            return (self.hour, self.minute);
        }

        let offset = self.longitude_offset_minutes() - self.dst_offset_minutes();
        let total_minutes = self.hour as i32 * 60 + self.minute as i32 + offset;

        let normalized = ((total_minutes % 1440) + 1440) % 1440;
        let hour = (normalized / 60) as u32;
        let minute = (normalized % 60) as u32;

        (hour, minute)
    }

    /// Returns corrected date and time if True Solar Time is active
    pub fn corrected_datetime(&self) -> (i32, u32, u32, u32, u32) {
        if !self.use_true_solar_time {
            return (self.year, self.month, self.day, self.hour, self.minute);
        }

        let offset = self.longitude_offset_minutes() - self.dst_offset_minutes();
        if let Ok(naive) = self.local_naive_datetime() {
            let corrected = naive + chrono::Duration::minutes(offset as i64);
            (
                corrected.year(),
                corrected.month(),
                corrected.day(),
                corrected.hour(),
                corrected.minute(),
            )
        } else {
            let (h, m) = self.corrected_time();
            (self.year, self.month, self.day, h, m)
        }
    }

    /// Returns (year, month, day, hour) for BaZi calculation
    pub fn for_saju(&self) -> (i32, u32, u32, u32) {
        let (year, month, day, hour, _) = self.corrected_datetime();
        (year, month, day, hour)
    }
}

impl std::fmt::Display for BirthInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cal = match self.calendar {
            CalendarType::Solar => "Solar",
            CalendarType::Lunar {
                is_leap_month: false,
            } => "Lunar",
            CalendarType::Lunar {
                is_leap_month: true,
            } => "Lunar (Leap)",
        };

        write!(
            f,
            "{} {:04}-{:02}-{:02} {:02}:{:02}",
            cal, self.year, self.month, self.day, self.hour, self.minute
        )?;

        if let Some(ref loc) = self.location {
            write!(f, " ({})", loc.name)?;
        }

        if let Some(ref tz) = self.timezone {
            write!(f, " [{}]", tz)?;
        }

        if let Some(gender) = self.gender {
            write!(f, " {}", gender)?;
        }

        if self.is_dst() {
            write!(f, " (DST)")?;
        }

        if self.use_true_solar_time {
            let offset = self.longitude_offset_minutes();
            if offset != 0 {
                write!(f, " [Solar Offset {:+}m]", offset)?;
            }
        }

        Ok(())
    }
}

/// Predefined IANA timezone constants
pub mod timezones {
    pub const KOREA: &str = "Asia/Seoul";
    pub const JAPAN: &str = "Asia/Tokyo";
    pub const CHINA: &str = "Asia/Shanghai";
    pub const TAIWAN: &str = "Asia/Taipei";
    pub const HONG_KONG: &str = "Asia/Hong_Kong";
    pub const INDIA: &str = "Asia/Kolkata";
    pub const US_EAST: &str = "America/New_York";
    pub const US_WEST: &str = "America/Los_Angeles";
    pub const UK: &str = "Europe/London";
    pub const GERMANY: &str = "Europe/Berlin";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birth_info_creation() {
        let info = BirthInfo::solar(2004, 11, 27, 22, 0)
            .with_location(Location::ansan())
            .with_korea_timezone()
            .with_true_solar_time(true)
            .with_gender(Gender::Male);

        assert_eq!(info.year, 2004);
        assert_eq!(info.month, 11);
        assert_eq!(info.day, 27);
        assert_eq!(info.hour, 22);
        assert_eq!(info.gender, Some(Gender::Male));
        assert_eq!(info.timezone, Some("Asia/Seoul".to_string()));
    }

    #[test]
    fn test_to_utc_normal() {
        // 2004-11-27 22:00 KST (No DST)
        let info = BirthInfo::solar(2004, 11, 27, 22, 0).with_korea_timezone();

        let utc = info.to_utc().unwrap();
        // KST = UTC+9, 22:00 KST = 13:00 UTC
        assert_eq!(utc.hour(), 13);
    }

    #[test]
    fn test_to_utc_with_dst_1988() {
        // 1988-05-15 10:00 KDT (Korean Summer Time before 1988 Seoul Olympics)
        let info = BirthInfo::solar(1988, 5, 15, 10, 0).with_korea_timezone();

        let utc = info.to_utc().unwrap();
        // KDT = UTC+10, 10:00 KDT = 00:00 UTC
        assert_eq!(utc.hour(), 0);

        // Verify DST
        assert!(info.is_dst());
        assert_eq!(info.dst_offset_hours(), Some(10.0));
    }

    #[test]
    fn test_no_dst_winter() {
        // 2004-01-15 (Winter, no DST)
        let info = BirthInfo::solar(2004, 1, 15, 10, 0).with_korea_timezone();

        assert!(!info.is_dst());
        assert_eq!(info.dst_offset_hours(), Some(9.0));
    }

    #[test]
    fn test_corrected_time_with_location() {
        // Ansan, 22:00 -> 21:27 (-33m)
        let info = BirthInfo::solar(2004, 11, 27, 22, 0)
            .with_location(Location::ansan())
            .with_true_solar_time(true);

        let (hour, minute) = info.corrected_time();
        assert_eq!(hour, 21);
        assert_eq!(minute, 27);
    }

    #[test]
    fn test_display_with_dst() {
        let info = BirthInfo::solar(1988, 5, 15, 10, 0)
            .with_korea_timezone()
            .with_gender(Gender::Male);

        let display = format!("{}", info);
        assert!(display.contains("DST"));
    }

    #[test]
    fn test_corrected_time_removes_dst_hour() {
        // Seoul 1988-07-01 12:00 is DST (UTC+10): standard 11:00, minus 32m longitude => 10:28
        let info = BirthInfo::solar(1988, 7, 1, 12, 0)
            .with_korea_timezone()
            .with_location(Location::seoul())
            .with_true_solar_time(true);
        assert_eq!(info.dst_offset_minutes(), 60);
        assert_eq!(info.corrected_datetime(), (1988, 7, 1, 10, 28));

        // Outside DST nothing changes
        let info = BirthInfo::solar(1990, 5, 15, 10, 30)
            .with_korea_timezone()
            .with_location(Location::seoul())
            .with_true_solar_time(true);
        assert_eq!(info.dst_offset_minutes(), 0);
        assert_eq!(info.corrected_datetime(), (1990, 5, 15, 9, 58));
    }

    #[test]
    fn test_standard_clock_datetime_removes_only_dst() {
        // 1988-07-01 01:10 Seoul is DST => standard clock 00:10 (no longitude shift)
        let info = BirthInfo::solar(1988, 7, 1, 1, 10).with_korea_timezone();
        assert_eq!(info.standard_clock_datetime(), (1988, 7, 1, 0, 10));
        // DST crossing midnight: 00:30 DST => previous day 23:30
        let info = BirthInfo::solar(1988, 7, 1, 0, 30).with_korea_timezone();
        assert_eq!(info.standard_clock_datetime(), (1988, 6, 30, 23, 30));
        // No DST: unchanged
        let info = BirthInfo::solar(1990, 5, 15, 10, 30).with_korea_timezone();
        assert_eq!(info.standard_clock_datetime(), (1990, 5, 15, 10, 30));
    }
}
