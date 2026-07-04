//! 자미두수 분석 서비스 모듈
//!
//! 입력된 데이터를 바탕으로 생년월일시 보정을 수행하고, 자미두수 엔진을 호출해 성반과 대한/유년을 연산합니다.

use crate::birth::prepare_birth_context;
use crate::dto::{AnalysisMeta, BirthTimePrecision, ZwdsAnalysisInput, ZwdsAnalysisOutput};
use crate::error::ServiceError;
use eon_core::Gender;
use eon_zwds::annual::calculate_liunian;
use eon_zwds::build_chart;

/// 자미두수 분석을 수행합니다.
pub fn analyze(input: ZwdsAnalysisInput) -> Result<ZwdsAnalysisOutput, ServiceError> {
    let gender = if input.is_male {
        Gender::Male
    } else {
        Gender::Female
    };

    // 자미두수는 통상적으로 경도 기반 진태양시 보정을 하지 않으므로 apply_tst = false
    let birth_ctx = prepare_birth_context(&input.base, Some(gender), false)?;

    // 1. 성반 빌드
    let mut chart = build_chart(&birth_ctx.birth_info)
        .map_err(|e| ServiceError::Zwds(format!("자미두수 성반 생성 실패: {}", e)))?;

    // 2. 대상 연도 지정 (기본값은 현재 연도 2026년)
    let target_year = input.target_year.unwrap_or(2026);

    // 3. 대상 연도에 따른 유년(流年) 계산
    let current_liu_nian = calculate_liunian(target_year);

    // 성반 내 유년 궁 표시 업데이트
    for palace in chart.palaces.iter_mut() {
        if palace.index == current_liu_nian.palace_idx {
            palace.is_current_liu_nian = true;
        }
    }

    // 4. 나이 계산 및 현재 대한(大限) 식별
    // 자미두수 대운(대한)은 태어난 해(1세)부터 시작해 만 나이/한국 나이가 아닌 오행국 나이(예: 2~11세) 기준입니다.
    // 여기서는 간단하게 경과 연도(target_year - birth_year + 1)를 나이로 보고 현재 대한을 매핑합니다.
    let age = (target_year - birth_ctx.birth_info.year).max(0) as u32 + 1; // 세는 나이 기준

    let current_daxian = chart
        .daxian
        .iter()
        .find(|d| age >= d.age_start && age <= d.age_end)
        .cloned()
        .unwrap_or_else(|| {
            // 매핑되지 않으면 첫 번째 대한 리턴
            chart.daxian.first().cloned().expect("대한 목록이 비어있음")
        });

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
