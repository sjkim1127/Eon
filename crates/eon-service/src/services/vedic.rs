use crate::birth::prepare_birth_context;
use crate::dto::{AnalysisMeta, VedicAnalysisInput, VedicAnalysisOutput};
use crate::error::ServiceError;
use chrono::Datelike;
use eon_vedic::analysis::report::VedicAnalysisReport;
use eon_vedic::core::chart::VedicChartCalculator;

pub fn analyze(input: VedicAnalysisInput) -> Result<VedicAnalysisOutput, ServiceError> {
    let birth_ctx = prepare_birth_context(&input.base, None, false)?;

    let dt = birth_ctx
        .birth_info
        .to_utc()
        .map_err(|e| ServiceError::BirthInfo(e.to_string()))?;

    let calculator = VedicChartCalculator::new();
    let chart = calculator
        .calculate(dt, input.base.lat, input.base.lon)
        .map_err(|e| ServiceError::Vedic(e.to_string()))?;

    let report = {
        let mut r = VedicAnalysisReport::generate(&chart, dt, chart.ascendant.rasi);
        // Gochara: natal moon rasi 기준으로 트랜짓 분석
        let gochara_inner = {
            let now_utc = input.current.now_utc;
            let transit_chart = calculator
                .calculate(now_utc, input.base.lat, input.base.lon)
                .map_err(|e| ServiceError::Vedic(e.to_string()))?;
            eon_vedic::analysis::gochara::GocharaEngine::analyze(&chart, &transit_chart)
        };
        // Unify Sade Sati for text summary
        r.sade_sati = gochara_inner.sade_sati;
        r
    };

    // Calculate Annual Chart & Tajika Report
    let target_year = input
        .target_year
        .unwrap_or_else(|| input.current.now_utc.year());
    let annual_chart = calculator
        .calculate_solar_return(dt, input.base.lat, input.base.lon, target_year)
        .map_err(|e| ServiceError::Vedic(e.to_string()))?;

    let age_years = (target_year - dt.year()).unsigned_abs();
    let tajika_report = Some(eon_vedic::analysis::report::TajikaReport::generate(
        &annual_chart,
        chart.ascendant.rasi,
        age_years,
    ));

    // Final gochara for output (same as used in report unification)
    let gochara = {
        let now_utc = input.current.now_utc;
        let transit_chart = calculator
            .calculate(now_utc, input.base.lat, input.base.lon)
            .map_err(|e| ServiceError::Vedic(e.to_string()))?;
        eon_vedic::analysis::gochara::GocharaEngine::analyze(&chart, &transit_chart)
    };

    let varga_nakshatra_reports =
        eon_vedic::analysis::varga_nakshatra_report::build_varga_nakshatra_reports(&chart);

    let kp_analysis = Some(
        eon_vedic::analysis::kp::KpAnalysis::calculate(
            dt,
            input.base.lat,
            input.base.lon,
            chart.ayanamsa,
            &chart.planets,
            calculator.engine(),
        )
        .map_err(ServiceError::Vedic)?,
    );

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
        tajika_report,
        chart,
        annual_chart: Some(annual_chart),
        gochara,
        varga_nakshatra_reports,
        kp_analysis,
    })
}

pub fn analyze_compatibility(
    input: crate::dto::VedicCompatibilityInput,
) -> Result<crate::dto::VedicCompatibilityOutput, ServiceError> {
    let male_ctx = prepare_birth_context(&input.male, None, false)?;
    let male_dt = male_ctx
        .birth_info
        .to_utc()
        .map_err(|e| ServiceError::BirthInfo(e.to_string()))?;

    let female_ctx = prepare_birth_context(&input.female, None, false)?;
    let female_dt = female_ctx
        .birth_info
        .to_utc()
        .map_err(|e| ServiceError::BirthInfo(e.to_string()))?;

    let calculator = VedicChartCalculator::new();
    let male_chart = calculator
        .calculate(male_dt, input.male.lat, input.male.lon)
        .map_err(|e| ServiceError::Vedic(e.to_string()))?;
    let female_chart = calculator
        .calculate(female_dt, input.female.lat, input.female.lon)
        .map_err(|e| ServiceError::Vedic(e.to_string()))?;

    let report = eon_vedic::analysis::matching::MatchingEngine::calculate_compatibility(
        &male_chart,
        &female_chart,
    );

    Ok(crate::dto::VedicCompatibilityOutput {
        meta: AnalysisMeta {
            precision: crate::dto::BirthTimePrecision::Exact,
            input_time: male_ctx.input_time_string,
            corrected_time: male_ctx.corrected_time_string,
            is_dst: male_ctx.is_dst,
            dst_offset_hours: male_ctx.dst_offset_hours,
            analysis_timezone: input.male.timezone,
        },
        report,
    })
}
