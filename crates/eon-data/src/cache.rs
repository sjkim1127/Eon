use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Solar term timetable for a single year
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarTermTable {
    pub year: i32,
    /// List of (solar term index 0..=23, timestamp)
    pub terms: Vec<(u8, DateTime<Utc>)>,
}

/// Lunar month record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LunarMonthRecord {
    /// New moon date (in KST)
    pub new_moon_date: chrono::NaiveDate,
    pub lunar_year: i32,
    pub lunar_month: u32,
    pub is_leap: bool,
}

/// Manseryuk binary cache structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManseryukCache {
    /// Annual solar term data by year
    pub years: HashMap<i32, SolarTermTable>,
    /// Annual lunar month data by year
    pub lunar_months: HashMap<i32, Vec<LunarMonthRecord>>,
}

impl ManseryukCache {
    /// Load cache from binary bytes
    pub fn from_binary(bytes: &[u8]) -> Result<Self, crate::error::DataError> {
        bincode::serde::decode_from_slice(bytes, bincode::config::legacy())
            .map(|(cache, _)| cache)
            .map_err(|e| crate::error::DataError::Deserialization(e.to_string()))
    }

    /// Query timestamp of specific year and solar term index
    pub fn get_solar_term(&self, year: i32, term_idx: u8) -> Option<DateTime<Utc>> {
        self.years.get(&year).and_then(|table| {
            // Terms are stored in sorted order by index; use binary search for O(log n) lookup.
            table
                .terms
                .binary_search_by_key(&term_idx, |(idx, _)| *idx)
                .ok()
                .map(|i| table.terms[i].1)
        })
    }

    /// Lookup solar term index for a given timestamp
    pub fn get_term_index_at(&self, time: DateTime<Utc>) -> Option<u8> {
        let year = time.year();
        let table = self.years.get(&year)?;

        // Terms are in chronological order; find the latest term that started at or before `time`
        // using binary search via partition_point (O(log n) instead of O(n)).
        let pos = table
            .terms
            .partition_point(|(_, term_time)| *term_time <= time);
        if pos == 0 {
            // `time` is before all solar terms of this year. This edge case is rare in practice;
            // match the original behavior of returning index 0 (or the first term's index if stored
            // differently) rather than None, since callers expect a valid term index.
            Some(table.terms.first().map_or(0, |(idx, _)| *idx))
        } else {
            Some(table.terms[pos - 1].0)
        }
    }

    /// Query corresponding lunar date for a given solar date
    pub fn get_lunar_date(&self, solar_date: chrono::NaiveDate) -> Option<(i32, u32, u32, bool)> {
        let year = solar_date.year();
        // Current year and previous year data (lunar month 1 often starts in February).
        let prev_months = self
            .lunar_months
            .get(&(year - 1))
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        let curr_months = self
            .lunar_months
            .get(&year)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        // Each slice is sorted by new_moon_date; use binary search (partition_point) to find the
        // latest record whose new_moon_date <= solar_date in each slice, then pick the most recent.
        // This avoids allocating a combined Vec and runs in O(log n) per slice.
        let prev_pos = prev_months.partition_point(|r| r.new_moon_date <= solar_date);
        let curr_pos = curr_months.partition_point(|r| r.new_moon_date <= solar_date);

        let prev_best = if prev_pos > 0 {
            Some(&prev_months[prev_pos - 1])
        } else {
            None
        };
        let curr_best = if curr_pos > 0 {
            Some(&curr_months[curr_pos - 1])
        } else {
            None
        };

        let record = match (prev_best, curr_best) {
            (Some(a), Some(b)) => {
                if b.new_moon_date >= a.new_moon_date {
                    b
                } else {
                    a
                }
            }
            (Some(a), None) => a,
            (None, Some(b)) => b,
            (None, None) => return None,
        };

        let day = (solar_date - record.new_moon_date).num_days() as u32 + 1;
        Some((record.lunar_year, record.lunar_month, day, record.is_leap))
    }
}

/// Global shared cache instance (Lazy initialization)
static GLOBAL_CACHE_INTERNAL: OnceLock<ManseryukCache> = OnceLock::new();

pub struct GlobalCache;

impl GlobalCache {
    pub fn get() -> &'static ManseryukCache {
        GLOBAL_CACHE_INTERNAL.get_or_init(|| {
            // Load embedded binary cache data
            let bytes = include_bytes!("manseryuk.bin");
            ManseryukCache::from_binary(bytes).unwrap_or_else(|e| {
                eprintln!("Failed to load manseryuk cache: {}", e);
                ManseryukCache::default()
            })
        })
    }

    pub fn get_solar_term(year: i32, term_idx: u8) -> Option<DateTime<Utc>> {
        Self::get().get_solar_term(year, term_idx)
    }

    pub fn get_lunar_date(solar_date: chrono::NaiveDate) -> Option<(i32, u32, u32, bool)> {
        Self::get().get_lunar_date(solar_date)
    }
}
