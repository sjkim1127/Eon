use crate::birth::prepare_birth_context;
use crate::dto::{
    AnalysisMeta, BirthTimePrecision, HumanDesignAnalysisInput, HumanDesignAnalysisOutput,
};
use crate::error::ServiceError;

pub fn analyze(input: HumanDesignAnalysisInput) -> Result<HumanDesignAnalysisOutput, ServiceError> {
    let birth_ctx = prepare_birth_context(&input.base, None, false)?;

    let dt = birth_ctx
        .birth_info
        .to_utc()
        .map_err(|e| ServiceError::BirthInfo(e.to_string()))?;

    let engine = eon_astro::AstroEngine::new();
    let result = eon_human_design::calculate_human_design(&engine, dt)
        .map_err(|e| ServiceError::HumanDesign(e.to_string()))?;

    Ok(HumanDesignAnalysisOutput {
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
