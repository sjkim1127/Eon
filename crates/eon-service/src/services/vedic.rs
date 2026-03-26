use crate::dto::{AnalysisMeta, VedicAnalysisInput, VedicAnalysisOutput};
use crate::error::ServiceError;
use crate::birth::prepare_birth_context;
use eon_vedic::analysis::report::VedicAnalysisReport;
use eon_vedic::core::chart::VedicChartCalculator;
use chrono::Datelike;

pub fn analyze(input: VedicAnalysisInput) -> Result<VedicAnalysisOutput, ServiceError> {
    let birth_ctx = prepare_birth_context(&input.base, None, false)?;
    
    let dt = birth_ctx.birth_info.to_utc()
        .ok_or_else(|| ServiceError::BirthInfo("Invalid date/time for Vedic".to_string()))?;

    let calculator = VedicChartCalculator::new();
    let chart = calculator.calculate(dt, input.base.lat, input.base.lon);

    let report = VedicAnalysisReport::generate(&chart, dt, chart.ascendant.rasi);

    // Calculate Annual Chart
    let target_year = input.target_year.unwrap_or_else(|| input.current.now_utc.year());
    let annual_chart = calculator.calculate_solar_return(dt, input.base.lat, input.base.lon, target_year);

    // Gochara: natal moon rasi 기준으로 트랜짓 분석
        let gochara = {
            let natal_moon_rasi = chart
                .planets
                .iter()
                .find(|p| p.planet == eon_vedic::planets::VedicPlanet::Moon)
                .map(|m| m.rasi)
                .unwrap_or(1);
                
            let now_utc = input.current.now_utc;
            
            let transit_chart = calculator.calculate(now_utc, input.base.lat, input.base.lon);
            eon_vedic::analysis::gochara::GocharaEngine::analyze(natal_moon_rasi, &transit_chart)
        };

    let varga_nakshatra_reports =
        eon_vedic::analysis::varga_nakshatra_report::build_varga_nakshatra_reports(&chart);

    Ok(VedicAnalysisOutput {
        meta: AnalysisMeta {
            precision: input.precision,
            input_time: birth_ctx.input_time_string,
            corrected_time: birth_ctx.corrected_time_string,
            is_dst: birth_ctx.is_dst,
            dst_offset_hours: birth_ctx.dst_offset_hours,
            analysis_timezone: input.current.analysis_timezone,
        },
        report,
        chart,
        annual_chart: Some(annual_chart),
        gochara,
        varga_nakshatra_reports,
    })
}
