use crate::birth::prepare_birth_context;
use crate::dto::{AnalysisMeta, BirthTimePrecision, QimenAnalysisInput, QimenAnalysisOutput};
use crate::error::ServiceError;
use eon_core::Gender;
use eon_qimen::analysis::report::QimenAnalysisReport;
use eon_qimen::builder::ju::calculate_ju;
use eon_qimen::builder::pan::build_qimen_pan;
use eon_saju::core::pillars::{FourPillars, SajuInput};

pub fn analyze_qimen(input: QimenAnalysisInput) -> Result<QimenAnalysisOutput, ServiceError> {
    crate::birth::require_known_birth_time(&input.base, "QiMen DunJia")?;
    let gender = if input.is_male {
        Gender::Male
    } else {
        Gender::Female
    };

    // 1. Prepare birth context (True Solar Time correction, etc.)
    let birth_ctx = prepare_birth_context(&input.base, Some(gender), true)?;
    let dt = birth_ctx
        .birth_info
        .to_utc()
        .map_err(|e| ServiceError::InvalidInput(e.to_string()))?;

    // 2. Calculate Four Pillars
    let saju_input = SajuInput::new_solar(
        birth_ctx.corrected_year,
        birth_ctx.corrected_month,
        birth_ctx.corrected_day,
        birth_ctx.corrected_hour,
        birth_ctx.corrected_minute,
    )
    .with_birth_utc(dt)
    .with_gender(gender)
    .with_night_rat_hour(input.use_night_rat_hour);

    let pillars = FourPillars::calculate(&saju_input)
        .map_err(|e| ServiceError::Saju(format!("Four Pillars calculation failed: {}", e)))?;

    // 3. Determine QiMen Ju number and Yin/Yang Dun mode
    let (is_yin_ju, ju_number) = calculate_ju(dt, pillars.day)
        .map_err(|e| ServiceError::InvalidInput(format!("QiMen Ju calculation failed: {}", e)))?;

    let hour_pillar = pillars
        .hour
        .ok_or_else(|| ServiceError::BirthTimeRequired("QiMen DunJia".to_string()))?;

    // 4. Execute QiMen Pan building algorithm
    let pan = build_qimen_pan(
        dt,
        pillars.year,
        pillars.month,
        pillars.day,
        hour_pillar,
        is_yin_ju,
        ju_number,
    );

    // 5. Generate analysis report
    let report = QimenAnalysisReport::generate(pan);

    let meta = AnalysisMeta {
        precision: BirthTimePrecision::Exact,
        input_time: birth_ctx.input_time_string,
        corrected_time: birth_ctx.corrected_time_string,
        is_dst: birth_ctx.is_dst,
        dst_offset_hours: birth_ctx.dst_offset_hours,
        analysis_timezone: input.base.timezone,
    };

    Ok(QimenAnalysisOutput { meta, report })
}
