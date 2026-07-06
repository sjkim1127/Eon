use crate::birth::prepare_birth_context;
use crate::dto::{AnalysisMeta, BirthTimePrecision, QimenAnalysisInput, QimenAnalysisOutput};
use crate::error::ServiceError;
use eon_core::Gender;
use eon_qimen::analysis::report::QimenAnalysisReport;
use eon_qimen::core::QimenPan;
use eon_saju::core::ganzi::GanZi;

use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::stem::HeavenlyStem;

pub fn analyze_qimen(input: QimenAnalysisInput) -> Result<QimenAnalysisOutput, ServiceError> {
    let gender = if input.is_male {
        Gender::Male
    } else {
        Gender::Female
    };
    let birth_ctx = prepare_birth_context(&input.base, Some(gender), true)?;
    let dt = birth_ctx
        .birth_info
        .to_utc()
        .map_err(|e| ServiceError::InvalidInput(e.to_string()))?;

    // 스캐폴딩: 임시로 년주, 월주, 일주, 시주를 Dummy 값으로 사용하거나 saju-core를 불러와야 하지만,
    // 현재는 뼈대만 구축하는 단계이므로 QimenPan을 더미 데이터로 채웁니다.
    // 추후 eon-saju::core::calendar::LunarCalendar 등과 연동하여 제대로 된 시주(Hour Pillar)를 가져옵니다.

    let pan = QimenPan {
        time: dt,
        hour_pillar: GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi), // 갑자(JiaZi) 더미
        is_yin_ju: true,                                               // 둔국 더미
        ju_number: 1,                                                  // 국수 더미
    };

    let report = QimenAnalysisReport::generate(pan);

    let meta = AnalysisMeta {
        precision: BirthTimePrecision::Exact, // 기본값
        input_time: dt.to_rfc3339(),
        corrected_time: dt.to_rfc3339(), // 진태양시 보정 추후 적용
        is_dst: false,
        dst_offset_hours: None,
        analysis_timezone: input.base.timezone,
    };

    Ok(QimenAnalysisOutput { meta, report })
}
