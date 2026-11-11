//! Timezone utilities
//!
//! Calculates standard meridian longitude from IANA timezone strings.

use chrono::TimeZone;
use chrono_tz::Tz;

/// Calculates the standard meridian (longitude in degrees) from an IANA timezone string.
///
/// Example: "Asia/Seoul" (UTC+9) -> 135.0, "America/New_York" (UTC-5) -> -75.0
///
/// Falls back to 135.0 (Korean Standard Meridian) on parsing failure.
pub fn standard_meridian_from_tz(timezone: &str) -> f64 {
    if let Ok(tz) = timezone.parse::<Tz>() {
        let ref_dt = tz.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).single();
        if let Some(dt) = ref_dt {
            use chrono_tz::OffsetComponents;
            let base_offset_secs = dt.offset().base_utc_offset().num_seconds() as f64;
            return (base_offset_secs / 3600.0) * 15.0;
        }
    }
    135.0 // Fallback: Standard Meridian
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
