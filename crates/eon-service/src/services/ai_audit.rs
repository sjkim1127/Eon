use crate::dto::{AiAuditOutput, AnalysisMeta, SajuAnalysisInput};
use crate::error::ServiceError;
use crate::birth::prepare_birth_context;
use eon_ai::DestinyAIAuditor;
use eon_core::Gender;
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::engine::emulator::LifePathEmulator;

pub fn analyze(input: SajuAnalysisInput) -> Result<AiAuditOutput, ServiceError> {
    let gender = if input.is_male {
        Gender::Male
    } else {
        Gender::Female
    };

    let birth_ctx = prepare_birth_context(&input.base, Some(gender))?;

    let saju_input = SajuInput::new_solar(
        birth_ctx.corrected_year,
        birth_ctx.corrected_month,
        birth_ctx.corrected_day,
        birth_ctx.corrected_hour,
        birth_ctx.corrected_minute,
    )
    .with_gender(gender)
    .with_night_rat_hour(input.use_night_rat_hour);

    let pillars = FourPillars::calculate(&saju_input)
        .map_err(|e| ServiceError::Saju(format!("사주 계산 실패: {}", e)))?;

    let emulator = LifePathEmulator::new(pillars.clone(), gender, birth_ctx.corrected_year);
    let life_report = emulator
        .emulate()
        .map_err(|e| ServiceError::AiAudit(format!("에뮬레이션 실패: {}", e)))?;

    let context_dump = DestinyAIAuditor::build_agent_prompt(&pillars)
        + "\n\n=== CORE DUMP ===\n"
        + &DestinyAIAuditor::generate_context(&pillars, &life_report);

    Ok(AiAuditOutput {
        meta: AnalysisMeta {
            precision: input.precision,
            corrected_time: birth_ctx.corrected_time_string,
            is_dst: birth_ctx.is_dst,
            dst_offset_hours: birth_ctx.dst_offset_hours,
            analysis_timezone: input.base.timezone,
        },
        context_dump,
        peak_age: life_report.peak_age,
        valley_age: life_report.valley_age,
    })
}
