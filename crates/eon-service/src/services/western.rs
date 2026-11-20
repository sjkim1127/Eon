use crate::birth::prepare_birth_context;
use crate::dto::{
    AnalysisMeta, BirthTimePrecision, WesternAnalysisInput, WesternAnalysisOutput,
    WesternProfectionOutput, WesternProgressionOutput, WesternReturnOutput, WesternSolarArcOutput,
    WesternSynastryOutput, WesternTransitOutput,
};
use crate::error::ServiceError;
use chrono::{DateTime, Utc};

/// House systems accepted by the Western engine (full name or Swiss Ephemeris letter).
pub const HOUSE_SYSTEMS: &str =
    "Placidus, Koch, WholeSign, Equal, Porphyry, Regiomontanus, Campanus, Alcabitius, Morinus, Topocentric";

/// Maps a house system name (case-insensitive) or a single Swiss Ephemeris letter to its letter.
///
/// Unknown names are rejected instead of silently falling back to another system.
pub fn parse_house_system(value: &str) -> Result<char, ServiceError> {
    let v = value.trim();
    let normalized: String = v
        .chars()
        .filter(|c| !matches!(c, ' ' | '-' | '_'))
        .collect::<String>()
        .to_ascii_lowercase();

    let letter = match normalized.as_str() {
        "placidus" | "p" => 'P',
        "koch" | "k" => 'K',
        "wholesign" | "whole" | "w" => 'W',
        "equal" | "e" => 'E',
        "porphyry" | "porphyrius" | "o" => 'O',
        "regiomontanus" | "r" => 'R',
        "campanus" | "c" => 'C',
        "alcabitius" | "b" => 'B',
        "morinus" | "m" => 'M',
        "topocentric" | "polichpage" | "t" => 'T',
        _ => {
            return Err(ServiceError::InvalidInput(format!(
                "Unknown house system '{value}'. Expected one of: {HOUSE_SYSTEMS}"
            )))
        }
    };
    Ok(letter)
}

pub fn analyze(input: WesternAnalysisInput) -> Result<WesternAnalysisOutput, ServiceError> {
    crate::birth::require_known_birth_time(&input.base, "Western astrology")?;
    let house_char = parse_house_system(&input.house_system)?;
    let birth_ctx = prepare_birth_context(&input.base, None, false)?;

    let dt = birth_ctx
        .birth_info
        .to_utc()
        .map_err(|e| ServiceError::BirthInfo(e.to_string()))?;

    let result = eon_western::calculate_western(dt, input.base.lat, input.base.lon, house_char)
        .map_err(|e| ServiceError::Western(e.to_string()))?;

    Ok(WesternAnalysisOutput {
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

/// Synastry (inter-chart aspects) and composite chart of two people.
pub fn analyze_synastry(
    a: WesternAnalysisInput,
    b: WesternAnalysisInput,
    name_a: &str,
    name_b: &str,
) -> Result<WesternSynastryOutput, ServiceError> {
    let chart_a = analyze(a)?;
    let chart_b = analyze(b)?;

    let synastry =
        eon_western::calculate_synastry(&chart_a.result, &chart_b.result, name_a, name_b);
    let composite = eon_western::calculate_composite(&chart_a.result, &chart_b.result)
        .map_err(|e| ServiceError::Western(e.to_string()))?;

    Ok(WesternSynastryOutput {
        meta_a: chart_a.meta,
        meta_b: chart_b.meta,
        synastry,
        composite,
    })
}

/// Birth moment, meta and house system shared by the timing techniques.
struct Natal {
    meta: AnalysisMeta,
    birth_utc: DateTime<Utc>,
    house_char: char,
}

fn prepare_natal(input: &WesternAnalysisInput) -> Result<Natal, ServiceError> {
    crate::birth::require_known_birth_time(&input.base, "Western astrology")?;
    let house_char = parse_house_system(&input.house_system)?;
    let birth_ctx = prepare_birth_context(&input.base, None, false)?;
    let birth_utc = birth_ctx
        .birth_info
        .to_utc()
        .map_err(|e| ServiceError::BirthInfo(e.to_string()))?;

    Ok(Natal {
        meta: AnalysisMeta {
            precision: BirthTimePrecision::Exact,
            input_time: birth_ctx.input_time_string,
            corrected_time: birth_ctx.corrected_time_string,
            is_dst: birth_ctx.is_dst,
            dst_offset_hours: birth_ctx.dst_offset_hours,
            analysis_timezone: input.base.timezone.clone(),
        },
        birth_utc,
        house_char,
    })
}

fn western_err(e: eon_western::WesternError) -> ServiceError {
    match e {
        eon_western::WesternError::InvalidInput(msg) => ServiceError::InvalidInput(msg),
        other => ServiceError::Western(other.to_string()),
    }
}

fn require_after_birth(
    natal: &Natal,
    target_time: DateTime<Utc>,
    what: &str,
) -> Result<(), ServiceError> {
    if target_time < natal.birth_utc {
        return Err(ServiceError::InvalidInput(format!(
            "{what} target time must not be before the birth time"
        )));
    }
    Ok(())
}

/// Secondary progression (one day = one year) of the natal chart to `target_time`.
pub fn analyze_progression(
    input: WesternAnalysisInput,
    target_time: DateTime<Utc>,
) -> Result<WesternProgressionOutput, ServiceError> {
    let natal = prepare_natal(&input)?;
    require_after_birth(&natal, target_time, "Progression")?;

    let result = eon_western::calculate_secondary_progression(
        natal.birth_utc,
        input.base.lat,
        input.base.lon,
        target_time,
        natal.house_char,
    )
    .map_err(western_err)?;

    Ok(WesternProgressionOutput {
        meta: natal.meta,
        target_time,
        result,
    })
}

/// The sky at `target_time` laid over the natal chart.
pub fn analyze_transit(
    input: WesternAnalysisInput,
    target_time: DateTime<Utc>,
) -> Result<WesternTransitOutput, ServiceError> {
    let natal = prepare_natal(&input)?;
    let result = eon_western::calculate_transits(
        natal.birth_utc,
        input.base.lat,
        input.base.lon,
        target_time,
        natal.house_char,
    )
    .map_err(western_err)?;

    Ok(WesternTransitOutput {
        meta: natal.meta,
        result,
    })
}

/// Solar return of calendar `year`; `relocation` casts the chart for another place.
pub fn analyze_solar_return(
    input: WesternAnalysisInput,
    year: i32,
    relocation: Option<(f64, f64)>,
) -> Result<WesternReturnOutput, ServiceError> {
    let natal = prepare_natal(&input)?;
    let (lat, lon) = relocation.unwrap_or((input.base.lat, input.base.lon));
    let result = eon_western::calculate_solar_return(
        natal.birth_utc,
        input.base.lat,
        input.base.lon,
        year,
        lat,
        lon,
        natal.house_char,
    )
    .map_err(western_err)?;

    Ok(WesternReturnOutput {
        meta: natal.meta,
        result,
    })
}

/// First lunar return on or after `after`.
pub fn analyze_lunar_return(
    input: WesternAnalysisInput,
    after: DateTime<Utc>,
    relocation: Option<(f64, f64)>,
) -> Result<WesternReturnOutput, ServiceError> {
    let natal = prepare_natal(&input)?;
    require_after_birth(&natal, after, "Lunar return")?;
    let (lat, lon) = relocation.unwrap_or((input.base.lat, input.base.lon));
    let result = eon_western::calculate_lunar_return(
        natal.birth_utc,
        input.base.lat,
        input.base.lon,
        after,
        lat,
        lon,
        natal.house_char,
    )
    .map_err(western_err)?;

    Ok(WesternReturnOutput {
        meta: natal.meta,
        result,
    })
}

/// Solar arc directions to `target_time`.
pub fn analyze_solar_arc(
    input: WesternAnalysisInput,
    target_time: DateTime<Utc>,
) -> Result<WesternSolarArcOutput, ServiceError> {
    let natal = prepare_natal(&input)?;
    require_after_birth(&natal, target_time, "Solar arc")?;
    let result = eon_western::calculate_solar_arc(
        natal.birth_utc,
        input.base.lat,
        input.base.lon,
        target_time,
        natal.house_char,
    )
    .map_err(western_err)?;

    Ok(WesternSolarArcOutput {
        meta: natal.meta,
        result,
    })
}

/// Annual profection at `target_time`.
pub fn analyze_profection(
    input: WesternAnalysisInput,
    target_time: DateTime<Utc>,
) -> Result<WesternProfectionOutput, ServiceError> {
    let natal = prepare_natal(&input)?;
    let result = eon_western::calculate_profection(
        natal.birth_utc,
        input.base.lat,
        input.base.lon,
        target_time,
        natal.house_char,
    )
    .map_err(western_err)?;

    Ok(WesternProfectionOutput {
        meta: natal.meta,
        result,
    })
}
