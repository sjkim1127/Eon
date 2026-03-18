use crate::dto::AnalysisInput;
use crate::error::ServiceError;
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
    pub corrected_time_string: String,
}

pub fn prepare_birth_context(
    input: &AnalysisInput,
    gender: Option<Gender>,
) -> Result<PreparedBirthContext, ServiceError> {
    let location = Location::new(
        "출생지",
        input.lat,
        input.lon,
        standard_meridian_from_tz(&input.timezone),
    );

    let mut birth_info = if input.is_lunar {
        BirthInfo::lunar(
            input.year,
            input.month,
            input.day,
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
        .with_true_solar_time(true);

    if let Some(g) = gender {
        birth_info = birth_info.with_gender(g);
    }

    let is_dst = birth_info.is_dst();
    let dst_offset_hours = birth_info.dst_offset_hours();
    let (cy, cm, cd, ch, cmin) = birth_info.corrected_datetime();
    let corrected_time_string = format!("{:04}-{:02}-{:02} {:02}:{:02}", cy, cm, cd, ch, cmin);

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
    })
}
