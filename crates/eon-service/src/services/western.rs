use crate::birth::prepare_birth_context;
use crate::dto::{
    AnalysisMeta, BirthTimePrecision, WesternAnalysisInput, WesternAnalysisOutput,
    WesternProgressionOutput, WesternSynastryOutput,
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

/// Secondary progression (one day = one year) of the natal chart to `target_time`.
pub fn analyze_progression(
    input: WesternAnalysisInput,
    target_time: DateTime<Utc>,
) -> Result<WesternProgressionOutput, ServiceError> {
    crate::birth::require_known_birth_time(&input.base, "Western astrology")?;
    let house_char = parse_house_system(&input.house_system)?;
    let birth_ctx = prepare_birth_context(&input.base, None, false)?;
    let dt = birth_ctx
        .birth_info
        .to_utc()
        .map_err(|e| ServiceError::BirthInfo(e.to_string()))?;

    if target_time < dt {
        return Err(ServiceError::InvalidInput(
            "Progression target time must not be before the birth time".to_string(),
        ));
    }

    let result = eon_western::calculate_secondary_progression(
        dt,
        input.base.lat,
        input.base.lon,
        target_time,
        house_char,
    )
    .map_err(|e| ServiceError::Western(e.to_string()))?;

    Ok(WesternProgressionOutput {
        meta: AnalysisMeta {
            precision: BirthTimePrecision::Exact,
            input_time: birth_ctx.input_time_string,
            corrected_time: birth_ctx.corrected_time_string,
            is_dst: birth_ctx.is_dst,
            dst_offset_hours: birth_ctx.dst_offset_hours,
            analysis_timezone: input.base.timezone.clone(),
        },
        target_time,
        result,
    })
}
