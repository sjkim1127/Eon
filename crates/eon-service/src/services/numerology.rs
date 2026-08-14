use crate::dto::{
    AnalysisMeta, BirthTimePrecision, NumerologyAnalysisInput, NumerologyAnalysisOutput,
};
use crate::error::ServiceError;
use chrono::Datelike;

pub fn analyze(input: NumerologyAnalysisInput) -> Result<NumerologyAnalysisOutput, ServiceError> {
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
