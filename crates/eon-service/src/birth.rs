use crate::dto::AnalysisInput;
use crate::error::ServiceError;
use chrono::Datelike;
use chrono::NaiveDate;
use chrono_tz::Tz;
use eon_core::{standard_meridian_from_tz, BirthInfo, Gender, Location};
use std::str::FromStr;

pub struct PreparedBirthContext {
    pub birth_info: BirthInfo,
    pub corrected_year: i32,
    pub corrected_month: u32,
    pub corrected_day: u32,
    pub corrected_hour: u32,
    pub corrected_minute: u32,
    pub is_dst: bool,
    pub dst_offset_hours: Option<i32>,
    pub input_time_string: String,
    pub corrected_time_string: String,
}

pub fn prepare_birth_context(
    input: &AnalysisInput,
    gender: Option<Gender>,
    apply_tst: bool,
) -> Result<PreparedBirthContext, ServiceError> {
    validate_analysis_input(input)?;

    let location = Location::new(
        "출생지",
        input.lat,
        input.lon,
        standard_meridian_from_tz(&input.timezone),
    );

    let mut birth_info = if input.is_lunar {
        // 음력인 경우 eon_data를 이용해 양력으로 변환한 뒤 BirthInfo에 등록합니다.
        use eon_data::LunarCalendar;
        let solar_date =
            LunarCalendar::to_solar(input.year, input.month, input.day, input.is_leap_month)
                .map_err(|e| {
                    ServiceError::BirthInfo(format!(
                        "음력 날짜를 양력으로 변환할 수 없습니다: {}",
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

fn validate_analysis_input(input: &AnalysisInput) -> Result<(), ServiceError> {
    if !input.lat.is_finite() || !(-90.0..=90.0).contains(&input.lat) {
        return Err(ServiceError::InvalidInput(format!(
            "위도는 -90도에서 90도 사이의 유한한 값이어야 합니다: {}",
            input.lat
        )));
    }
    if !input.lon.is_finite() || !(-180.0..=180.0).contains(&input.lon) {
        return Err(ServiceError::InvalidInput(format!(
            "경도는 -180도에서 180도 사이의 유한한 값이어야 합니다: {}",
            input.lon
        )));
    }
    if input.hour > 23 || input.minute > 59 {
        return Err(ServiceError::InvalidInput(format!(
            "출생 시각이 올바르지 않습니다: {:02}:{:02}",
            input.hour, input.minute
        )));
    }
    if !input.is_lunar && NaiveDate::from_ymd_opt(input.year, input.month, input.day).is_none() {
        return Err(ServiceError::InvalidInput(format!(
            "양력 날짜가 올바르지 않습니다: {:04}-{:02}-{:02}",
            input.year, input.month, input.day
        )));
    }
    Tz::from_str(&input.timezone).map_err(|_| {
        ServiceError::InvalidInput(format!(
            "유효하지 않은 IANA 타임존입니다: {}",
            input.timezone
        ))
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> AnalysisInput {
        AnalysisInput {
            year: 1990,
            month: 1,
            day: 1,
            hour: 12,
            minute: 0,
            is_lunar: false,
            is_leap_month: false,
            lat: 37.5665,
            lon: 126.978,
            timezone: "Asia/Seoul".into(),
        }
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
