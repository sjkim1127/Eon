//! Zi Wei Dou Shu analysis service module
//!
//! Performs birth datetime adjustments and calls the ZWDS engine to compute chart, Decadal Luck (Daxian), and Annual Luck (Liunian).

use crate::birth::prepare_birth_context;
use crate::dto::{
    AnalysisMeta, BirthTimePrecision, ZwdsAnalysisInput, ZwdsAnalysisOutput,
    ZwdsCompatibilityOutput,
};
use crate::error::ServiceError;
use chrono::Datelike;
use eon_core::Gender;
use eon_zwds::annual::calculate_liunian;
use eon_zwds::{build_chart, ZwdsChart};

/// Natal chart plus the meta block shared by every ZWDS entry point.
fn build_natal(input: &ZwdsAnalysisInput) -> Result<(ZwdsChart, AnalysisMeta), ServiceError> {
    crate::birth::require_known_birth_time(&input.base, "Zi Wei Dou Shu")?;
    let gender = if input.is_male {
        Gender::Male
    } else {
        Gender::Female
    };

    // ZWDS typically does not apply longitude True Solar Time correction: apply_tst = false
    let birth_ctx = prepare_birth_context(&input.base, Some(gender), false)?;

    // ZWDS reads the standard (non-DST) clock: remove the DST hour, but no longitude shift.
    let mut zwds_birth = birth_ctx.birth_info.clone();
    let (sy, sm, sd, sh, smin) = birth_ctx.birth_info.standard_clock_datetime();
    zwds_birth.year = sy;
    zwds_birth.month = sm;
    zwds_birth.day = sd;
    zwds_birth.hour = sh;
    zwds_birth.minute = smin;

    let chart = build_chart(&zwds_birth)
        .map_err(|e| ServiceError::Zwds(format!("Failed to build ZWDS chart: {}", e)))?;

    let meta = AnalysisMeta {
        precision: BirthTimePrecision::Exact,
        input_time: birth_ctx.input_time_string,
        corrected_time: birth_ctx.corrected_time_string,
        is_dst: birth_ctx.is_dst,
        dst_offset_hours: birth_ctx.dst_offset_hours,
        analysis_timezone: input.base.timezone.clone(),
    };
    Ok((chart, meta))
}

/// Performs Zi Wei Dou Shu analysis.
pub fn analyze(input: ZwdsAnalysisInput) -> Result<ZwdsAnalysisOutput, ServiceError> {
    // 1. Build chart
    let (mut chart, meta) = build_natal(&input)?;

    // 2. Determine target year (defaults to current year)
    let target_year = input
        .target_year
        .unwrap_or_else(|| chrono::Utc::now().year());

    // 3. Compute Annual Luck (Liunian) for target year
    let current_liu_nian = calculate_liunian(target_year);

    // Update Liunian palace marker in chart
    for palace in chart.palaces.iter_mut() {
        if palace.index == current_liu_nian.palace_idx {
            palace.is_current_liu_nian = true;
        }
    }

    // 4. Identify current Decadal Luck (Daxian) from the nominal age (虛歲, counted from the
    // lunar birth year). ZWDS Daxian starts from the Bureau start age rather than Western age.
    let age = chart.nominal_age(target_year);

    let current_daxian = chart
        .daxian_for_age(age)
        .cloned()
        .or_else(|| chart.daxian.first().cloned())
        .ok_or_else(|| ServiceError::Zwds("Daxian list is empty".to_string()))?;

    // 5. Minor limit (小限), 斗君, current-decade 四化 and the 命宮 三方四正
    let minor_limit = chart.xiaoxian(target_year);
    let dou_jun = chart.dou_jun(target_year);
    let daxian_si_hua = chart.daxian_si_hua(&current_daxian);
    let soul_san_fang_si_zheng = chart.san_fang_si_zheng_at(chart.soul_idx);

    Ok(ZwdsAnalysisOutput {
        meta,
        chart,
        current_daxian,
        current_liu_nian,
        minor_limit: Some(minor_limit),
        dou_jun: Some(dou_jun),
        daxian_si_hua: Some(daxian_si_hua),
        soul_san_fang_si_zheng: Some(soul_san_fang_si_zheng),
    })
}

/// Two-person Zi Wei Dou Shu 合盤 (heuristic; see `eon_zwds::compat`).
pub fn analyze_compatibility(
    a: ZwdsAnalysisInput,
    b: ZwdsAnalysisInput,
) -> Result<ZwdsCompatibilityOutput, ServiceError> {
    let (chart_a, meta_a) = build_natal(&a)?;
    let (chart_b, meta_b) = build_natal(&b)?;
    Ok(ZwdsCompatibilityOutput {
        meta_a,
        meta_b,
        result: eon_zwds::compat::compare_charts(&chart_a, &chart_b),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::AnalysisInput;

    fn chart_key(input: AnalysisInput) -> (usize, usize, String) {
        let out = analyze(ZwdsAnalysisInput::new(input, true, Some(2024))).unwrap();
        (
            out.chart.soul_idx,
            out.chart.body_idx,
            format!("{:?}", out.chart.five_elements),
        )
    }

    #[test]
    fn dst_hour_is_removed_before_building_chart() {
        // Seoul 1988-07-01 01:10 is DST (UTC+10): the standard clock reads 00:10 (Zi hour).
        // Tokyo has no DST (UTC+9), so its 00:10 is the same standard-clock reading.
        let seoul_dst = AnalysisInput::new(1988, 7, 1, 1, 10, "Asia/Seoul", 37.5665, 126.978);
        let tokyo_std = AnalysisInput::new(1988, 7, 1, 0, 10, "Asia/Tokyo", 37.5665, 126.978);
        assert_eq!(chart_key(seoul_dst), chart_key(tokyo_std));
    }

    #[test]
    fn output_carries_minor_limit_dou_jun_and_decade_sihua() {
        // 2004-11-27 22:00 (甲申 year, lunar 10/16, 亥 hour), male, target 2026 (丙午).
        let input = AnalysisInput::new(2004, 11, 27, 22, 0, "Asia/Seoul", 37.5665, 126.978);
        let out = analyze(ZwdsAnalysisInput::new(input, true, Some(2026))).unwrap();
        // Nominal age 2026 − 2004 + 1 = 23, counted from the lunar birth year.
        let x = out.minor_limit.expect("小限");
        assert_eq!((x.year, x.age), (2026, 23));
        // 申 year → 小限 age 1 in 戌 (8); male runs forward 22 steps → (8 + 22) mod 12 = 6 (申).
        assert_eq!(x.palace_idx, (8 + 22) % 12);
        let d = out.dou_jun.expect("斗君");
        // 午 (4) − (10 − 1) + 亥 (11) = 6 → 申.
        assert_eq!(d.palace_idx, 6);
        let sh = out.daxian_si_hua.expect("大限四化");
        assert_eq!(sh.from_palace_idx, out.current_daxian.palace_idx);
        assert_eq!(sh.landings.len(), 4);
        assert!(out.soul_san_fang_si_zheng.is_some());
        assert!(out.chart.palaces.iter().all(|p| p.twelve_gods.is_some()));
    }

    #[test]
    fn compatibility_is_deterministic_and_bounded() {
        let a = AnalysisInput::new(1990, 5, 15, 10, 30, "Asia/Seoul", 37.5665, 126.978);
        let b = AnalysisInput::new(1992, 9, 3, 21, 10, "Asia/Seoul", 37.5665, 126.978);
        let run = || {
            analyze_compatibility(
                ZwdsAnalysisInput::new(a.clone(), true, None),
                ZwdsAnalysisInput::new(b.clone(), false, None),
            )
            .unwrap()
        };
        let r = run();
        assert_eq!(r, run());
        assert!((0.0..=100.0).contains(&r.result.harmony_score));
        assert_eq!(r.result.a_to_b.len(), 8);
    }
}
