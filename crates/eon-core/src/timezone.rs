//! Timezone utilities
//!
//! Calculates standard meridian longitude from IANA timezone strings.

use chrono::TimeZone;
use chrono_tz::Tz;

/// Calculates the standard meridian (longitude in degrees) from an IANA timezone string
/// at an optional historical reference datetime.
///
/// Example:
/// - "Asia/Seoul" at 2024 -> 135.0 (UTC+9)
/// - "Asia/Seoul" at 1955 -> 127.5 (UTC+8:30, used historically between 1954-03-21 and 1961-08-09)
pub fn standard_meridian_from_tz_at(timezone: &str, dt_opt: Option<chrono::NaiveDateTime>) -> f64 {
    if let Ok(tz) = timezone.parse::<Tz>() {
        let ref_dt = match dt_opt {
            Some(dt) => tz
                .from_local_datetime(&dt)
                .earliest()
                .or_else(|| tz.from_local_datetime(&dt).latest()),
            None => tz.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).single(),
        };
        if let Some(dt) = ref_dt {
            use chrono_tz::OffsetComponents;
            let base_offset_secs = dt.offset().base_utc_offset().num_seconds() as f64;
            return (base_offset_secs / 3600.0) * 15.0;
        }
    }
    135.0 // Fallback: Standard Meridian
}

/// Calculates the standard meridian (longitude in degrees) from an IANA timezone string (defaulting to modern 2024 reference).
pub fn standard_meridian_from_tz(timezone: &str) -> f64 {
    standard_meridian_from_tz_at(timezone, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asia_seoul() {
        let m = standard_meridian_from_tz("Asia/Seoul");
        assert!((m - 135.0).abs() < 0.01);
    }

    #[test]
    fn test_asia_seoul_historical_1955() {
        use chrono::NaiveDate;
        let dt = NaiveDate::from_ymd_opt(1955, 6, 15)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();
        let m = standard_meridian_from_tz_at("Asia/Seoul", Some(dt));
        assert!(
            (m - 127.5).abs() < 0.01,
            "1955 Korea must use UTC+8:30 (127.5 deg meridian), got {}",
            m
        );
    }

    #[test]
    fn test_america_new_york() {
        let m = standard_meridian_from_tz("America/New_York");
        assert!((m - (-75.0)).abs() < 0.01);
    }

    #[test]
    fn test_invalid_fallback() {
        let m = standard_meridian_from_tz("invalid/tz");
        assert!((m - 135.0).abs() < 0.01);
    }
}
