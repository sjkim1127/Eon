use crate::birth::prepare_birth_context;
use crate::dto::{
    AnalysisMeta, BirthTimePrecision, HumanDesignAnalysisInput, HumanDesignAnalysisOutput,
};
use crate::error::ServiceError;
use chrono::Datelike;

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

pub fn analyze_connection(
    input1: HumanDesignAnalysisInput,
    input2: HumanDesignAnalysisInput,
) -> Result<eon_human_design::connection::HumanDesignConnectionResult, ServiceError> {
    let res1 = analyze(input1)?;
    let res2 = analyze(input2)?;

    Ok(eon_human_design::connection::calculate_connection_chart(
        &res1.result,
        &res2.result,
    ))
}
pub fn analyze_transit(
    natal_input: HumanDesignAnalysisInput,
    transit_time: chrono::DateTime<chrono::Utc>,
) -> Result<eon_human_design::transit::HumanDesignTransitResult, ServiceError> {
    let natal_res = analyze(natal_input.clone())?;

    let engine = eon_astro::AstroEngine::new();
    let transit_chart = eon_human_design::calculate_human_design(&engine, transit_time)
        .map_err(|e| ServiceError::HumanDesign(e.to_string()))?;

    let composite_connection =
        eon_human_design::connection::calculate_connection_chart(&natal_res.result, &transit_chart);

    Ok(eon_human_design::transit::HumanDesignTransitResult {
        natal_chart: natal_res.result,
        transit_chart,
        composite_connection,
        target_date: transit_time.to_rfc3339(),
        is_return: None,
    })
}

pub fn calculate_return(
    natal_input: HumanDesignAnalysisInput,
    return_type: eon_human_design::transit::ReturnType,
    target_year: i32,
) -> Result<eon_human_design::transit::HumanDesignTransitResult, ServiceError> {
    let natal_res = analyze(natal_input.clone())?;
    let engine = eon_astro::AstroEngine::new();

    let birth_ctx = prepare_birth_context(&natal_input.base, None, false)?;
    let dt = birth_ctx
        .birth_info
        .to_utc()
        .map_err(|e| ServiceError::BirthInfo(e.to_string()))?;

    let (planet_id, target_long, approx_age) = match return_type {
        eon_human_design::transit::ReturnType::Solar => {
            let sun_data = engine
                .get_planet_full(dt, 0, 256)
                .map_err(|e| ServiceError::HumanDesign(e.to_string()))?;
            let age_diff = target_year - dt.year();
            (0, sun_data.0, age_diff as f64) // SE_SUN
        }
        eon_human_design::transit::ReturnType::Saturn => {
            let saturn_data = engine
                .get_planet_full(dt, 6, 256)
                .map_err(|e| ServiceError::HumanDesign(e.to_string()))?;
            (6, saturn_data.0, 29.4) // SE_SATURN
        }
        eon_human_design::transit::ReturnType::Chiron => {
            let chiron_data = engine
                .get_planet_full(dt, 15, 256)
                .map_err(|e| ServiceError::HumanDesign(e.to_string()))?;
            (15, chiron_data.0, 50.7) // SE_CHIRON
        }
    };

    let search_start = dt + chrono::Duration::days((approx_age * 365.2425) as i64);

    let exact_return_time = engine
        .find_time_for_planet_longitude(search_start, target_long, planet_id)
        .map_err(|e| ServiceError::HumanDesign(e.to_string()))?;

    let transit_chart = eon_human_design::calculate_human_design(&engine, exact_return_time)
        .map_err(|e| ServiceError::HumanDesign(e.to_string()))?;

    let composite_connection =
        eon_human_design::connection::calculate_connection_chart(&natal_res.result, &transit_chart);

    Ok(eon_human_design::transit::HumanDesignTransitResult {
        natal_chart: natal_res.result,
        transit_chart,
        composite_connection,
        target_date: exact_return_time.to_rfc3339(),
        is_return: Some(return_type),
    })
}

pub fn analyze_penta(
    inputs: Vec<HumanDesignAnalysisInput>,
) -> Result<eon_human_design::penta::PentaResult, ServiceError> {
    let mut charts = Vec::new();
    for input in inputs {
        let res = analyze(input)?;
        charts.push(res.result);
    }

    Ok(eon_human_design::penta::calculate_penta(&charts))
}
