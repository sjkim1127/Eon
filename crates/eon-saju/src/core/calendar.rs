//! Solar Terms (節氣) and Calendar Utilities
//!
//! Uses 12 major solar terms among the 24 solar terms to determine month pillars.

use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// 12 Solar Terms (for month demarcation)
///
/// 12 JieQi mark the start of each month, while 12 ZhongQi mark the midpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SolarTerm {
    /// LiChun (立春) - Start of Yin month (~Feb 4)
    Lichun,
    /// JingZhe (驚蟄) - Start of Mao month (~Mar 6)
    Jingzhe,
    /// QingMing (清明) - Start of Chen month (~Apr 5)
    Qingming,
    /// LiXia (立夏) - Start of Si month (~May 6)
    Lixia,
    /// MangZhong (芒種) - Start of Wu month (~Jun 6)
    Mangzhong,
    /// XiaoShu (小暑) - Start of Wei month (~Jul 7)
    Xiaoshu,
    /// LiQiu (立秋) - Start of Shen month (~Aug 8)
    Liqiu,
    /// BaiLu (白露) - Start of You month (~Sep 8)
    Bailu,
    /// HanLu (寒露) - Start of Xu month (~Oct 8)
    Hanlu,
    /// LiDong (立冬) - Start of Hai month (~Nov 7)
    Lidong,
    /// DaXue (大雪) - Start of Zi month (~Dec 7)
    Daxue,
    /// XiaoHan (小寒) - Start of Chou month (~Jan 6)
    Xiaohan,
}

impl SolarTerm {
    /// All 12 terms array (starting from Yin month)
    pub const ALL: [SolarTerm; 12] = [
        Self::Lichun,    // Yin month
        Self::Jingzhe,   // Mao month
        Self::Qingming,  // Chen month
        Self::Lixia,     // Si month
        Self::Mangzhong, // Wu month
        Self::Xiaoshu,   // Wei month
        Self::Liqiu,     // Shen month
        Self::Bailu,     // You month
        Self::Hanlu,     // Xu month
        Self::Lidong,    // Hai month
        Self::Daxue,     // Zi month
        Self::Xiaohan,   // Chou month
    ];

    /// Hanja notation
    pub const HANJA: [&'static str; 12] = [
        "立春", "驚蟄", "清明", "立夏", "芒種", "小暑", "立秋", "白露", "寒露", "立冬", "大雪",
        "小寒",
    ];

    /// English name notation
    pub const HANGUL: [&'static str; 12] = [
        "LiChun",
        "JingZhe",
        "QingMing",
        "LiXia",
        "MangZhong",
        "XiaoShu",
        "LiQiu",
        "BaiLu",
        "HanLu",
        "LiDong",
        "DaXue",
        "XiaoHan",
    ];

    /// Index (0-11)
    #[inline]
    pub const fn index(self) -> u8 {
        match self {
            Self::Lichun => 0,
            Self::Jingzhe => 1,
            Self::Qingming => 2,
            Self::Lixia => 3,
            Self::Mangzhong => 4,
            Self::Xiaoshu => 5,
            Self::Liqiu => 6,
            Self::Bailu => 7,
            Self::Hanlu => 8,
            Self::Lidong => 9,
            Self::Daxue => 10,
            Self::Xiaohan => 11,
        }
    }

    /// Construct from index
    #[inline]
    pub const fn from_index(idx: i32) -> Self {
        Self::ALL[idx.rem_euclid(12) as usize]
    }

    /// Month branch index initiated by this solar term (Yin=2)
    #[inline]
    pub const fn month_branch_index(self) -> u8 {
        (self.index() + 2) % 12
    }

    /// Return starting solar term for branch index
    #[inline]
    pub const fn from_month_branch_index(branch_idx: u8) -> Self {
        Self::from_index((branch_idx as i32 - 2).rem_euclid(12))
    }

    /// Hanja notation
    #[inline]
    pub const fn hanja(self) -> &'static str {
        Self::HANJA[self.index() as usize]
    }

    /// English name notation
    #[inline]
    pub const fn hangul(self) -> &'static str {
        Self::HANGUL[self.index() as usize]
    }

    /// English name alias
    #[inline]
    pub const fn english(self) -> &'static str {
        Self::HANGUL[self.index() as usize]
    }
}

impl std::fmt::Display for SolarTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hanja())
    }
}

/// Solar Term Timestamp Data
#[derive(Debug, Clone, Copy)]
pub struct SolarTermTime {
    pub term: SolarTerm,
    pub datetime: DateTime<Utc>,
}

/// Calculate exact start timestamp of solar term using eon-astro
pub fn get_solar_term_time(
    dt: DateTime<Utc>,
    term: SolarTerm,
) -> Result<DateTime<Utc>, CalendarError> {
    use eon_astro::AstroEngine;
    let engine = AstroEngine::new();

    engine
        .find_solar_term_time(dt, term.index() * 2)
        .map_err(CalendarError::Astro)
}

/// Calculate month branch index from solar date/time and timezone offset
pub fn get_month_branch_index(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    tz_offset_minutes: i32,
) -> Result<u8, CalendarError> {
    let offset_secs = tz_offset_minutes * 60;
    let tz = FixedOffset::east_opt(offset_secs)
        .ok_or(CalendarError::InvalidTimezone(tz_offset_minutes))?;

    let dt_utc = tz
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .ok_or(CalendarError::InvalidDateTime {
            year,
            month,
            day,
            hour,
            minute,
        })?
        .with_timezone(&Utc);

    get_month_branch_index_from_dt(dt_utc)
}

/// Calculate month branch index from DateTime<Utc>
pub fn get_month_branch_index_from_dt(dt: DateTime<Utc>) -> Result<u8, CalendarError> {
    use eon_astro::AstroEngine;

    let engine = AstroEngine::new();
    let sun_long = engine.get_sun_longitude(dt)?;
    let adjusted = (sun_long - 315.0 + 360.0) % 360.0;
    let index = (adjusted / 15.0).floor() as u8;
    let term_24_idx = index % 24;
    let term_12_idx = term_24_idx / 2;

    Ok(SolarTerm::from_index(term_12_idx as i32).month_branch_index())
}

/// Calendar Errors
#[derive(Debug, Clone, thiserror::Error, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CalendarError {
    /// Invalid date/time
    #[error("Invalid datetime: {year}-{month:02}-{day:02} {hour:02}:{minute:02}")]
    InvalidDateTime {
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    },
    /// Invalid timezone offset
    #[error("Invalid timezone offset: {0} minutes")]
    InvalidTimezone(i32),
    /// Astronomical calculation error
    #[error("Astronomical calculation error: {0}")]
    Astro(#[from] eon_astro::AstroError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solar_term_month_branch() {
        assert_eq!(SolarTerm::Lichun.month_branch_index(), 2);
        assert_eq!(SolarTerm::Jingzhe.month_branch_index(), 3);
        assert_eq!(SolarTerm::Daxue.month_branch_index(), 0);
        assert_eq!(SolarTerm::Xiaohan.month_branch_index(), 1);
    }

    #[test]
    fn test_get_month_branch_index_kst() {
        let result = get_month_branch_index(2024, 2, 3, 12, 0, 540).unwrap();
        assert_eq!(result, 1);

        let result = get_month_branch_index(2024, 2, 5, 12, 0, 540).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_timezone_boundary() {
        let before = get_month_branch_index(2024, 2, 4, 17, 0, 540).unwrap();
        assert_eq!(before, 1, "Immediately before LiChun must be Chou month");

        let after = get_month_branch_index(2024, 2, 4, 18, 0, 540).unwrap();
        assert_eq!(after, 2, "Immediately after LiChun must be Yin month");

        let utc_before = get_month_branch_index(2024, 2, 4, 8, 0, 0).unwrap();
        assert_eq!(utc_before, 1, "UTC 08:00 must be Chou month");

        let utc_after = get_month_branch_index(2024, 2, 4, 9, 0, 0).unwrap();
        assert_eq!(utc_after, 2, "UTC 09:00 must be Yin month");
    }
}
