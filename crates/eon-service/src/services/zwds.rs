//! Zi Wei Dou Shu analysis service module
//!
//! Performs birth datetime adjustments and calls the ZWDS engine to compute chart, Decadal Luck (Daxian), and Annual Luck (Liunian).

use crate::birth::prepare_birth_context;
use crate::dto::{AnalysisMeta, BirthTimePrecision, ZwdsAnalysisInput, ZwdsAnalysisOutput};
use crate::error::ServiceError;
use chrono::Datelike;
use eon_core::Gender;
use eon_zwds::annual::calculate_liunian;
use eon_zwds::build_chart;

/// Performs Zi Wei Dou Shu analysis.
pub fn analyze(input: ZwdsAnalysisInput) -> Result<ZwdsAnalysisOutput, ServiceError> {
    crate::birth::require_known_birth_time(&input.base, "Zi Wei Dou Shu")?;
    let gender = if input.is_male {
        Gender::Male
    } else {
        Gender::Female
    };

    // ZWDS typically does not apply longitude True Solar Time correction: apply_tst = false
    let birth_ctx = prepare_birth_context(&input.base, Some(gender), false)?;

    // ZWDS reads the standard (non-DST) clock: remove the DST hour, but no longitude shift.
    let mut zwds_birth = birth_ctx.birth_info.clone();
    let (sy, sm, sd, sh, smin) = birth_ctx.birth_info.standard_clock_datetime();
    zwds_birth.year = sy;
    zwds_birth.month = sm;
    zwds_birth.day = sd;
    zwds_birth.hour = sh;
    zwds_birth.minute = smin;

    // 1. Build chart
    let mut chart = build_chart(&zwds_birth)
        .map_err(|e| ServiceError::Zwds(format!("Failed to build ZWDS chart: {}", e)))?;

    // 2. Determine target year (defaults to current year)
    let target_year = input
        .target_year
        .unwrap_or_else(|| chrono::Utc::now().year());

    // 3. Compute Annual Luck (Liunian) for target year
    let current_liu_nian = calculate_liunian(target_year);

    // Update Liunian palace marker in chart
    for palace in chart.palaces.iter_mut() {
        if palace.index == current_liu_nian.palace_idx {
            palace.is_current_liu_nian = true;
        }
    }

    // 4. Calculate age and identify current Decadal Luck (Daxian)
    // ZWDS Daxian starts from Bureau start age rather than Western age.
    // Map current Daxian by elapsed years.
    let age =
        (target_year.saturating_sub(birth_ctx.birth_info.year).max(0) as u32).saturating_add(1);

    let current_daxian = chart
        .daxian
        .iter()
        .find(|d| age >= d.age_start && age <= d.age_end)
        .cloned()
        .or_else(|| chart.daxian.first().cloned())
        .ok_or_else(|| ServiceError::Zwds("Daxian list is empty".to_string()))?;

    Ok(ZwdsAnalysisOutput {
        meta: AnalysisMeta {
            precision: BirthTimePrecision::Exact,
            input_time: birth_ctx.input_time_string,
            corrected_time: birth_ctx.corrected_time_string,
            is_dst: birth_ctx.is_dst,
            dst_offset_hours: birth_ctx.dst_offset_hours,
            analysis_timezone: input.base.timezone,
        },
        chart,
        current_daxian,
        current_liu_nian,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::AnalysisInput;

    fn chart_key(input: AnalysisInput) -> (usize, usize, String) {
        let out = analyze(ZwdsAnalysisInput::new(input, true, Some(2024))).unwrap();
        (
            out.chart.soul_idx,
            out.chart.body_idx,
            format!("{:?}", out.chart.five_elements),
        )
    }

    #[test]
    fn dst_hour_is_removed_before_building_chart() {
        // Seoul 1988-07-01 01:10 is DST (UTC+10): the standard clock reads 00:10 (Zi hour).
        // Tokyo has no DST (UTC+9), so its 00:10 is the same standard-clock reading.
        let seoul_dst = AnalysisInput::new(1988, 7, 1, 1, 10, "Asia/Seoul", 37.5665, 126.978);
        let tokyo_std = AnalysisInput::new(1988, 7, 1, 0, 10, "Asia/Tokyo", 37.5665, 126.978);
        assert_eq!(chart_key(seoul_dst), chart_key(tokyo_std));
    }
}
