use crate::dto::{AnalysisInput, AnalysisMeta, BirthTimePrecision, VedicAnalysisOutput};
use crate::error::ServiceError;
use crate::birth::prepare_birth_context;
use eon_vedic::analysis::report::VedicAnalysisReport;
use eon_vedic::core::chart::VedicChartCalculator;

pub fn analyze(input: AnalysisInput) -> Result<VedicAnalysisOutput, ServiceError> {
    let birth_ctx = prepare_birth_context(&input, None)?;
    
    let dt = birth_ctx.birth_info.to_utc()
        .ok_or_else(|| ServiceError::BirthInfo("Invalid date/time for Vedic".to_string()))?;

    let calculator = VedicChartCalculator::new();
    let chart = calculator.calculate(dt, input.lat, input.lon);

    let report = VedicAnalysisReport::generate(&chart, dt);

    // Gochara: natal moon rasi 기준으로 현재 트랜짓 분석
    let gochara = {
        let natal_moon_rasi = chart
            .planets
            .iter()
            .find(|p| p.planet == eon_vedic::planets::VedicPlanet::Moon)
            .map(|m| m.rasi)
            .unwrap_or(1);
        // v1에서는 Gochara 기준 시각을 일단 현재(chrono::Utc::now())로 유지하되 
        // 향후 CurrentContext 확장 가능하게 구조만 잡음
        let transit_chart = calculator.calculate(chrono::Utc::now(), input.lat, input.lon);
        eon_vedic::analysis::gochara::GocharaEngine::analyze(natal_moon_rasi, &transit_chart)
    };

    let varga_nakshatra_reports =
        eon_vedic::analysis::varga_nakshatra_report::build_varga_nakshatra_reports(&chart);

    Ok(VedicAnalysisOutput {
        meta: AnalysisMeta {
            // Vedic은 현재 input에 precision이 없으므로 기본값 Exact (지시서에 따라 향후 확장 가능)
            precision: BirthTimePrecision::Exact,
            corrected_time: birth_ctx.corrected_time_string,
            is_dst: birth_ctx.is_dst,
            dst_offset_hours: birth_ctx.dst_offset_hours,
            analysis_timezone: input.timezone,
        },
        report,
        chart,
        gochara,
        varga_nakshatra_reports,
    })
}
