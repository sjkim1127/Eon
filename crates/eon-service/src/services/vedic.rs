use crate::birth::prepare_birth_context;
use crate::dto::{AnalysisMeta, VedicAnalysisInput, VedicAnalysisOutput};
use crate::error::ServiceError;
use chrono::Datelike;
use eon_vedic::analysis::report::VedicAnalysisReport;
use eon_vedic::core::chart::VedicChartCalculator;
use eon_vedic::core::planets::VedicPlanet;

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

    let transit_chart = calculator
        .calculate(input.current.now_utc, input.base.lat, input.base.lon)
        .map_err(|e| ServiceError::Vedic(e.to_string()))?;
    let mut gochara = eon_vedic::analysis::gochara::GocharaEngine::analyze(&chart, &transit_chart);
    apply_sign_entry_murti(
        &calculator,
        chart
            .planets
            .iter()
            .find(|position| position.planet == VedicPlanet::Moon)
            .map(|position| position.rasi)
            .unwrap_or(0),
        chart.ayanamsa,
        &mut gochara,
        input.current.now_utc,
        input.base.lat,
        input.base.lon,
    )?;

    let report = {
        let mut r = VedicAnalysisReport::generate(&chart, dt, chart.ascendant.rasi);
        // Unify Sade Sati for text summary
        r.sade_sati = gochara.sade_sati.clone();
        r
    };

    // Calculate Annual Chart & Tajika Report
    let target_year = input
        .target_year
        .unwrap_or_else(|| input.current.now_utc.year());
    let annual_chart = calculator
        .calculate_solar_return(dt, input.base.lat, input.base.lon, target_year)
        .map_err(|e| ServiceError::Vedic(e.to_string()))?;

    let age_years = (i64::from(target_year) - i64::from(dt.year())).unsigned_abs() as u32;
    let tajika_report = Some(eon_vedic::analysis::report::TajikaReport::generate(
        &annual_chart,
        chart.ascendant.rasi,
        age_years,
    ));

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

fn apply_sign_entry_murti(
    calculator: &VedicChartCalculator,
    natal_moon_rasi: u8,
    ayanamsa: f64,
    gochara: &mut eon_vedic::analysis::gochara::GocharaSummary,
    now: chrono::DateTime<chrono::Utc>,
    latitude: f64,
    longitude: f64,
) -> Result<(), ServiceError> {
    for transit in &mut gochara.transits {
        // Swiss Ephemeris exposes the node pair through Rahu's body id.
        // Ketu is exactly opposite Rahu, so both cross sign boundaries at
        // the same instant and can share the entry search.
        let entry_planet_id = if matches!(transit.planet, VedicPlanet::Rahu | VedicPlanet::Ketu) {
            VedicPlanet::Rahu.se_id()
        } else {
            transit.planet.se_id()
        };

        let entry = calculator
            .engine()
            .find_previous_planet_sidereal_sign_entry(now, entry_planet_id, ayanamsa)
            .map_err(|e| ServiceError::Vedic(e.to_string()))?;
        let entry_chart = calculator
            .calculate(entry, latitude, longitude)
            .map_err(|e| ServiceError::Vedic(e.to_string()))?;
        if let Some(entry_moon) = entry_chart
            .planets
            .iter()
            .find(|position| position.planet == VedicPlanet::Moon)
        {
            transit.murti = eon_vedic::analysis::gochara::GocharaEngine::calculate_murti(
                natal_moon_rasi,
                entry_moon.rasi,
            );
        }
    }
    Ok(())
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
