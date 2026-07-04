use crate::birth::prepare_birth_context;
use crate::dto::{AnalysisMeta, BirthTimePrecision, WesternAnalysisInput, WesternAnalysisOutput};
use crate::error::ServiceError;

pub fn analyze(input: WesternAnalysisInput) -> Result<WesternAnalysisOutput, ServiceError> {
    let birth_ctx = prepare_birth_context(&input.base, None, false)?;

    let dt = birth_ctx
        .birth_info
        .to_utc()
        .map_err(|e| ServiceError::BirthInfo(e.to_string()))?;

    let house_char = input.house_system.chars().next().unwrap_or('P');

    let result = eon_western::calculate_western(dt, input.base.lat, input.base.lon, house_char)
        .map_err(|e| ServiceError::Western(e.to_string()))?;

    Ok(WesternAnalysisOutput {
        meta: AnalysisMeta {
            precision: BirthTimePrecision::Exact,
            input_time: birth_ctx.input_time_string,
            corrected_time: birth_ctx.corrected_time_string,
            is_dst: birth_ctx.is_dst,
            dst_offset_hours: birth_ctx.dst_offset_hours,
            analysis_timezone: input.base.timezone.clone(),
        },
        result,
    })
}
