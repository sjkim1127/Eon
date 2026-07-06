use crate::birth::prepare_birth_context;
use crate::dto::{AnalysisMeta, BirthTimePrecision, QimenAnalysisInput, QimenAnalysisOutput};
use crate::error::ServiceError;
use eon_core::Gender;
use eon_qimen::analysis::report::QimenAnalysisReport;
use eon_qimen::builder::ju::calculate_ju;
use eon_qimen::builder::pan::build_qimen_pan;
use eon_saju::core::pillars::{FourPillars, SajuInput};

pub fn analyze_qimen(input: QimenAnalysisInput) -> Result<QimenAnalysisOutput, ServiceError> {
    let gender = if input.is_male {
        Gender::Male
    } else {
        Gender::Female
    };

    // 1. 탄생 컨텍스트(진태양시 보정 등) 생성
    let birth_ctx = prepare_birth_context(&input.base, Some(gender), true)?;
    let dt = birth_ctx
        .birth_info
        .to_utc()
        .map_err(|e| ServiceError::InvalidInput(e.to_string()))?;

    // 2. 사주 명식(Four Pillars) 계산
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

    // 3. 기문둔갑 국수(Ju Number) 및 음양둔 산출
    let (is_yin_ju, ju_number) = calculate_ju(dt, pillars.day)
        .map_err(|e| ServiceError::InvalidInput(format!("기문둔갑 국수 계산 실패: {}", e)))?;

    // 4. 기문둔갑 포국(Pan Building) 알고리즘 실행
    let pan = build_qimen_pan(
        dt,
        pillars.year,
        pillars.month,
        pillars.day,
        pillars.hour,
        is_yin_ju,
        ju_number,
    );

    // 5. 분석 리포트 생성
    let report = QimenAnalysisReport::generate(pan);

    let meta = AnalysisMeta {
        precision: BirthTimePrecision::Exact,
        input_time: dt.to_rfc3339(),
        corrected_time: dt.to_rfc3339(),
        is_dst: false,
        dst_offset_hours: None,
        analysis_timezone: input.base.timezone,
    };

    Ok(QimenAnalysisOutput { meta, report })
}
