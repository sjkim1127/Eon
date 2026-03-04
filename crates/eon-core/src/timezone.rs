//! 타임존 유틸리티
//!
//! IANA 타임존 문자열에서 표준 자오선(경도)을 계산합니다.

use chrono::TimeZone;
use chrono_tz::Tz;

/// IANA 타임존 문자열로부터 표준 자오선(경도, 도)을 계산합니다.
///
/// 예: "Asia/Seoul" (UTC+9) → 135.0, "America/New_York" (UTC-5) → -75.0
///
/// 파싱 실패 시 한국 표준경도 135.0을 반환합니다.
pub fn standard_meridian_from_tz(timezone: &str) -> f64 {
    if let Ok(tz) = timezone.parse::<Tz>() {
        let ref_dt = tz.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).single();
        if let Some(dt) = ref_dt {
            use chrono_tz::OffsetComponents;
            let base_offset_secs = dt.offset().base_utc_offset().num_seconds() as f64;
            return (base_offset_secs / 3600.0) * 15.0;
        }
    }
    135.0 // Fallback: Korean Standard Meridian
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
