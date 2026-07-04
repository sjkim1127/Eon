use crate::dto::AnalysisInput;
use crate::error::ServiceError;
use chrono::Datelike;
use eon_core::{standard_meridian_from_tz, BirthInfo, Gender, Location};

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
