use crate::dto::{
    AnalysisMeta, BirthTimePrecision, NumerologyAnalysisInput, NumerologyAnalysisOutput,
};
use crate::error::ServiceError;
use chrono::{Datelike, NaiveDate};

pub fn analyze(input: NumerologyAnalysisInput) -> Result<NumerologyAnalysisOutput, ServiceError> {
    NaiveDate::from_ymd_opt(input.base.year, input.base.month, input.base.day).ok_or_else(
        || {
            ServiceError::InvalidInput(format!(
                "유효하지 않은 양력 날짜입니다: {:04}-{:02}-{:02}",
                input.base.year, input.base.month, input.base.day
            ))
        },
    )?;

    let year = input.base.year as u32;
    let month = input.base.month;
    let day = input.base.day;
    let target_year = input
        .target_year
        .unwrap_or_else(|| chrono::Utc::now().year() as u32);

    let res = eon_numerology::calculate_numerology(
        year,
        month,
        day,
        input.name_latin.as_deref(),
        target_year,
    );

    let meta = AnalysisMeta {
        precision: BirthTimePrecision::Exact,
        input_time: format!("{:04}-{:02}-{:02}T00:00:00Z", year, month, day),
        corrected_time: format!("{:04}-{:02}-{:02}T00:00:00Z", year, month, day),
        is_dst: false,
        dst_offset_hours: None,
        analysis_timezone: input.base.timezone,
    };

    Ok(NumerologyAnalysisOutput { meta, result: res })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::{AnalysisInput, NumerologyAnalysisInput};

    #[test]
    fn analyze_preserves_name_target_year_and_reduced_cycles() {
        let input = NumerologyAnalysisInput::new(
            AnalysisInput {
                year: 1990,
                month: 5,
                day: 28,
                hour: 0,
                minute: 0,
                is_lunar: false,
                is_leap_month: false,
                lat: 37.5665,
                lon: 126.978,
                timezone: "Asia/Seoul".to_string(),
            },
            Some("John Doe".to_string()),
            Some(2026),
        );

        let output = analyze(input).expect("numerology service analysis should succeed");
        assert_eq!(output.result.personal_year, 7);
        assert_eq!(output.result.pinnacles[0].challenge_number, 4);
        assert_eq!(output.meta.input_time, "1990-05-28T00:00:00Z");
        assert_eq!(output.meta.analysis_timezone, "Asia/Seoul");
        assert_ne!(output.result.core.expression, 0);
    }

    #[test]
    fn analyze_rejects_invalid_gregorian_date() {
        let input = NumerologyAnalysisInput::new(
            AnalysisInput {
                year: 2026,
                month: 2,
                day: 30,
                hour: 0,
                minute: 0,
                is_lunar: false,
                is_leap_month: false,
                lat: 0.0,
                lon: 0.0,
                timezone: "UTC".to_string(),
            },
            None,
            Some(2026),
        );

        assert!(matches!(analyze(input), Err(ServiceError::InvalidInput(_))));
    }
}
