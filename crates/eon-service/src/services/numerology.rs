use crate::dto::{
    AnalysisMeta, BirthTimePrecision, NumerologyAnalysisInput, NumerologyAnalysisOutput,
};
use crate::error::ServiceError;
use chrono::{Datelike, NaiveDate};

pub fn analyze(input: NumerologyAnalysisInput) -> Result<NumerologyAnalysisOutput, ServiceError> {
    if input.base.is_lunar {
        return Err(ServiceError::InvalidInput(
            "Numerology analysis only supports Gregorian calendar solar dates".to_string(),
        ));
    }

    if input.base.year <= 0 {
        return Err(ServiceError::InvalidInput(format!(
            "Birth year must be positive: {}",
            input.base.year
        )));
    }

    if matches!(input.target_year, Some(0)) {
        return Err(ServiceError::InvalidInput(
            "Target year must be positive".to_string(),
        ));
    }

    NaiveDate::from_ymd_opt(input.base.year, input.base.month, input.base.day).ok_or_else(
        || {
            ServiceError::InvalidInput(format!(
                "Invalid solar calendar date: {:04}-{:02}-{:02}",
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
            AnalysisInput::new(1990, 5, 28, 0, 0, "Asia/Seoul", 37.5665, 126.978),
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
        let base = AnalysisInput::new(2026, 2, 30, 0, 0, "UTC", 0.0, 0.0);
        let input = NumerologyAnalysisInput::new(base, None, Some(2026));

        assert!(matches!(analyze(input), Err(ServiceError::InvalidInput(_))));
    }

    #[test]
    fn analyze_rejects_non_positive_years() {
        let base = AnalysisInput::new(0, 1, 1, 0, 0, "UTC", 0.0, 0.0);
        assert!(matches!(
            analyze(NumerologyAnalysisInput::new(base.clone(), None, Some(2026))),
            Err(ServiceError::InvalidInput(_))
        ));

        let mut target_year_input = base;
        target_year_input.year = 2026;
        assert!(matches!(
            analyze(NumerologyAnalysisInput::new(
                target_year_input,
                None,
                Some(0)
            )),
            Err(ServiceError::InvalidInput(_))
        ));
    }

    #[test]
    fn analyze_rejects_lunar_input_instead_of_silently_mislabeling_it() {
        let mut base = AnalysisInput::new(2026, 2, 1, 0, 0, "UTC", 0.0, 0.0);
        base.is_lunar = true;
        let input = NumerologyAnalysisInput::new(base, None, Some(2026));

        assert!(matches!(analyze(input), Err(ServiceError::InvalidInput(_))));
    }
}
