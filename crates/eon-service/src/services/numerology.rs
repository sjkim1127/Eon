use crate::dto::{
    AnalysisMeta, BirthTimePrecision, NumerologyAnalysisInput, NumerologyAnalysisOutput,
    NumerologyCompatibilityOutput,
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
    // Target date: an explicit month/day gives Personal Month/Day; a bare year gives the
    // Personal Year only; with neither, today's date is used.
    let target = match (input.target_month, input.target_day) {
        (Some(m), Some(d)) => {
            let y = input
                .target_year
                .unwrap_or_else(|| chrono::Utc::now().year() as u32);
            NaiveDate::from_ymd_opt(y as i32, m, d).ok_or_else(|| {
                ServiceError::InvalidInput(format!("Invalid target date: {y:04}-{m:02}-{d:02}"))
            })?;
            eon_numerology::TargetDate::full(y, m, d)
        }
        (None, None) => match input.target_year {
            Some(y) => eon_numerology::TargetDate::year_only(y),
            None => {
                let today = chrono::Utc::now().date_naive();
                eon_numerology::TargetDate::full(today.year() as u32, today.month(), today.day())
            }
        },
        _ => {
            return Err(ServiceError::InvalidInput(
                "Target month and day must be given together".to_string(),
            ))
        }
    };

    let res = eon_numerology::calculate_numerology_ext(
        year,
        month,
        day,
        input.name_latin.as_deref(),
        target,
        input.system,
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

/// Numerology compatibility of two people. Both must use the same letter system.
pub fn analyze_compatibility(
    a: NumerologyAnalysisInput,
    b: NumerologyAnalysisInput,
) -> Result<NumerologyCompatibilityOutput, ServiceError> {
    if a.system != b.system {
        return Err(ServiceError::InvalidInput(format!(
            "Both people must use the same numerology system (got {} and {})",
            a.system, b.system
        )));
    }
    let person_a = analyze(a)?;
    let person_b = analyze(b)?;
    let compatibility = eon_numerology::compare_numerology(&person_a.result, &person_b.result);
    Ok(NumerologyCompatibilityOutput {
        person_a,
        person_b,
        compatibility,
    })
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
        assert!(output.result.core.expression.is_some());
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

    #[test]
    fn analyze_target_date_gives_personal_month_and_day() {
        let base = AnalysisInput::new(1990, 5, 28, 0, 0, "UTC", 0.0, 0.0);
        let input = NumerologyAnalysisInput::new(base, None, None).with_target_date(2026, 11, 20);
        let out = analyze(input).unwrap();
        assert_eq!(out.result.personal_year, 7);
        assert_eq!(out.result.personal_month, Some(9));
        assert_eq!(out.result.personal_day, Some(2));
    }

    #[test]
    fn analyze_target_year_only_has_no_month_or_day() {
        let base = AnalysisInput::new(1990, 5, 28, 0, 0, "UTC", 0.0, 0.0);
        let out = analyze(NumerologyAnalysisInput::new(base, None, Some(2026))).unwrap();
        assert_eq!(out.result.personal_month, None);
        assert_eq!(out.result.personal_day, None);
    }

    #[test]
    fn analyze_rejects_invalid_or_partial_target_date() {
        let base = AnalysisInput::new(1990, 5, 28, 0, 0, "UTC", 0.0, 0.0);
        let bad =
            NumerologyAnalysisInput::new(base.clone(), None, None).with_target_date(2026, 2, 30);
        assert!(matches!(analyze(bad), Err(ServiceError::InvalidInput(_))));

        let mut partial = NumerologyAnalysisInput::new(base, None, Some(2026));
        partial.target_month = Some(3);
        assert!(matches!(
            analyze(partial),
            Err(ServiceError::InvalidInput(_))
        ));
    }

    #[test]
    fn analyze_uses_requested_letter_system() {
        let base = AnalysisInput::new(1990, 5, 15, 0, 0, "UTC", 0.0, 0.0);
        let input = NumerologyAnalysisInput::new(base, Some("Dave".to_string()), Some(2026))
            .with_system(eon_numerology::NumerologySystem::Chaldean);
        let out = analyze(input).unwrap();
        assert_eq!(
            out.result.system,
            eon_numerology::NumerologySystem::Chaldean
        );
        assert_eq!(out.result.core.expression, Some(7));
    }

    #[test]
    fn compatibility_combines_both_people() {
        let mk = |name: &str| {
            NumerologyAnalysisInput::new(
                AnalysisInput::new(1990, 5, 15, 0, 0, "UTC", 0.0, 0.0),
                Some(name.to_string()),
                Some(2026),
            )
        };
        let out = analyze_compatibility(mk("Dave"), mk("John Doe")).unwrap();
        assert_eq!(out.compatibility.total_score, 69.5);
        assert_eq!(out.person_a.result.core.expression, Some(5));
        assert_eq!(out.person_b.result.core.expression, Some(8));

        let chaldean = mk("Dave").with_system(eon_numerology::NumerologySystem::Chaldean);
        assert!(matches!(
            analyze_compatibility(chaldean, mk("John Doe")),
            Err(ServiceError::InvalidInput(_))
        ));
    }
}
