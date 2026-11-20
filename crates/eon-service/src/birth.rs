use crate::dto::AnalysisInput;
use crate::error::ServiceError;
use chrono::Datelike;
use chrono::NaiveDate;
use chrono_tz::Tz;
use eon_core::{BirthInfo, Gender, Location};
use std::str::FromStr;

pub struct PreparedBirthContext {
    pub birth_info: BirthInfo,
    pub corrected_year: i32,
    pub corrected_month: u32,
    pub corrected_day: u32,
    pub corrected_hour: u32,
    pub corrected_minute: u32,
    pub is_dst: bool,
    pub dst_offset_hours: Option<f64>,
    pub input_time_string: String,
    pub corrected_time_string: String,
}

pub fn prepare_birth_context(
    input: &AnalysisInput,
    gender: Option<Gender>,
    apply_tst: bool,
) -> Result<PreparedBirthContext, ServiceError> {
    validate_analysis_input(input)?;

    let location_name = input.city.as_deref().unwrap_or("Birth Location");
    let dt_naive = chrono::NaiveDate::from_ymd_opt(input.year, input.month, input.day)
        .and_then(|d| d.and_hms_opt(input.hour, input.minute, 0));
    let location = Location::new(
        location_name,
        input.lat,
        input.lon,
        eon_core::standard_meridian_from_tz_at(&input.timezone, dt_naive),
    );

    let mut birth_info = if input.is_lunar {
        // If lunar calendar, convert to solar date via eon_data and register in BirthInfo.
        use eon_data::LunarCalendar;
        let solar_date =
            LunarCalendar::to_solar(input.year, input.month, input.day, input.is_leap_month)
                .map_err(|e| {
                    ServiceError::BirthInfo(format!(
                        "Cannot convert lunar date to solar date: {}",
                        e
                    ))
                })?;

        BirthInfo::lunar(
            solar_date.year(),
            solar_date.month(),
            solar_date.day(),
            input.hour,
            input.minute,
            input.is_leap_month,
        )
    } else {
        BirthInfo::solar(input.year, input.month, input.day, input.hour, input.minute)
    };

    birth_info = birth_info
        .with_timezone(&input.timezone)
        .with_location(location)
        .with_true_solar_time(apply_tst);

    if let Some(g) = gender {
        birth_info = birth_info.with_gender(g);
    }

    let is_dst = birth_info.is_dst();
    let dst_offset_hours = birth_info.dst_offset_hours();
    let (cy, cm, cd, ch, cmin) = birth_info.corrected_datetime();
    let corrected_time_string = format!("{:04}-{:02}-{:02} {:02}:{:02}", cy, cm, cd, ch, cmin);
    let input_time_string = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        input.year, input.month, input.day, input.hour, input.minute
    );

    Ok(PreparedBirthContext {
        birth_info,
        corrected_year: cy,
        corrected_month: cm,
        corrected_day: cd,
        corrected_hour: ch,
        corrected_minute: cmin,
        is_dst,
        dst_offset_hours,
        corrected_time_string,
        input_time_string,
    })
}

/// Parses a user supplied point in time into UTC.
///
/// Accepts an RFC 3339 timestamp (`2026-06-01T09:00:00+09:00`), or a local date/time in
/// `timezone` (`2026-06-01`, `2026-06-01 09:30`, `2026-06-01T09:30`). A date alone means 12:00.
pub fn parse_instant(
    text: &str,
    timezone: &str,
) -> Result<chrono::DateTime<chrono::Utc>, ServiceError> {
    use chrono::{LocalResult, NaiveDateTime, TimeZone};

    let text = text.trim();
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(text) {
        return Ok(dt.with_timezone(&chrono::Utc));
    }

    let tz = Tz::from_str(timezone)
        .map_err(|_| ServiceError::InvalidInput(format!("Invalid IANA timezone: {timezone}")))?;

    let naive = ["%Y-%m-%d %H:%M", "%Y-%m-%dT%H:%M", "%Y-%m-%d %H:%M:%S"]
        .iter()
        .find_map(|fmt| NaiveDateTime::parse_from_str(text, fmt).ok())
        .or_else(|| {
            NaiveDate::parse_from_str(text, "%Y-%m-%d")
                .ok()
                .and_then(|d| d.and_hms_opt(12, 0, 0))
        })
        .ok_or_else(|| {
            ServiceError::InvalidInput(format!(
                "Invalid date/time '{text}'. Expected 'YYYY-MM-DD', 'YYYY-MM-DD HH:MM' or RFC 3339"
            ))
        })?;

    match tz.from_local_datetime(&naive) {
        LocalResult::Single(dt) => Ok(dt.with_timezone(&chrono::Utc)),
        // Autumn DST overlap: take the first occurrence.
        LocalResult::Ambiguous(first, _) => Ok(first.with_timezone(&chrono::Utc)),
        // Spring-forward gap: the local time does not exist.
        LocalResult::None => Err(ServiceError::InvalidInput(format!(
            "'{text}' does not exist in {timezone} (daylight saving gap)"
        ))),
    }
}

/// Rejects engines whose result is intrinsically determined by the birth time of day
/// (ascendant/houses, hour-based charts) when only a date was supplied.
pub fn require_known_birth_time(input: &AnalysisInput, engine: &str) -> Result<(), ServiceError> {
    if input.unknown_time {
        return Err(ServiceError::BirthTimeRequired(engine.to_string()));
    }
    Ok(())
}

fn validate_analysis_input(input: &AnalysisInput) -> Result<(), ServiceError> {
    if !input.lat.is_finite() || !(-90.0..=90.0).contains(&input.lat) {
        return Err(ServiceError::InvalidInput(format!(
            "Latitude must be a finite number between -90 and 90 degrees: {}",
            input.lat
        )));
    }
    if !input.lon.is_finite() || !(-180.0..=180.0).contains(&input.lon) {
        return Err(ServiceError::InvalidInput(format!(
            "Longitude must be a finite number between -180 and 180 degrees: {}",
            input.lon
        )));
    }
    if input.hour > 23 || input.minute > 59 {
        return Err(ServiceError::InvalidInput(format!(
            "Invalid birth time: {:02}:{:02}",
            input.hour, input.minute
        )));
    }
    if !input.is_lunar && NaiveDate::from_ymd_opt(input.year, input.month, input.day).is_none() {
        return Err(ServiceError::InvalidInput(format!(
            "Invalid solar calendar date: {:04}-{:02}-{:02}",
            input.year, input.month, input.day
        )));
    }
    Tz::from_str(&input.timezone).map_err(|_| {
        ServiceError::InvalidInput(format!("Invalid IANA timezone: {}", input.timezone))
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instant_handles_local_and_rfc3339_inputs() {
        use chrono::{TimeZone, Utc};
        let seoul = |s| parse_instant(s, "Asia/Seoul").unwrap();

        assert_eq!(
            seoul("2026-06-01 09:30"),
            Utc.with_ymd_and_hms(2026, 6, 1, 0, 30, 0).unwrap()
        );
        assert_eq!(
            seoul("2026-06-01T09:30"),
            Utc.with_ymd_and_hms(2026, 6, 1, 0, 30, 0).unwrap()
        );
        // A bare date means local noon.
        assert_eq!(
            seoul("2026-06-01"),
            Utc.with_ymd_and_hms(2026, 6, 1, 3, 0, 0).unwrap()
        );
        // RFC 3339 carries its own offset and ignores the timezone argument.
        assert_eq!(
            parse_instant("2026-06-01T09:00:00-05:00", "Asia/Seoul").unwrap(),
            Utc.with_ymd_and_hms(2026, 6, 1, 14, 0, 0).unwrap()
        );
        assert!(parse_instant("tomorrow", "Asia/Seoul").is_err());
        assert!(parse_instant("2026-06-01", "Mars/Base").is_err());
        // New York spring-forward gap 2026-03-08 02:30 does not exist.
        assert!(parse_instant("2026-03-08 02:30", "America/New_York").is_err());
    }

    fn input() -> AnalysisInput {
        AnalysisInput::new(1990, 1, 1, 12, 0, "Asia/Seoul", 37.5665, 126.978)
    }

    #[test]
    fn rejects_coordinates_outside_swiss_contract() {
        let mut bad = input();
        bad.lat = 90.0001;
        assert!(matches!(
            prepare_birth_context(&bad, None, false),
            Err(ServiceError::InvalidInput(_))
        ));

        bad = input();
        bad.lon = f64::NAN;
        assert!(matches!(
            prepare_birth_context(&bad, None, false),
            Err(ServiceError::InvalidInput(_))
        ));
    }

    #[test]
    fn rejects_invalid_calendar_time_and_timezone() {
        let mut bad = input();
        bad.day = 31;
        bad.month = 2;
        assert!(prepare_birth_context(&bad, None, false).is_err());

        bad = input();
        bad.hour = 24;
        assert!(prepare_birth_context(&bad, None, false).is_err());

        bad = input();
        bad.timezone = "Not/A_Timezone".into();
        assert!(prepare_birth_context(&bad, None, false).is_err());
    }
}
