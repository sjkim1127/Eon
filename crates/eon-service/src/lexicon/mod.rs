pub mod human_design;
pub mod numerology;
pub mod qimen;
pub mod saju;
pub mod synthesis;
pub mod vedic;
pub mod western;
pub mod zwds;

use crate::dto::{
    HumanDesignAnalysisOutput, InterpretationHandbook, NumerologyAnalysisOutput,
    QimenAnalysisOutput, SajuAnalysisOutput, SymbolInterpretation, VedicAnalysisOutput,
    WesternAnalysisOutput, ZwdsAnalysisOutput,
};

pub fn build_handbook(
    saju: &SajuAnalysisOutput,
    vedic: &VedicAnalysisOutput,
    western: Option<&WesternAnalysisOutput>,
    zwds: Option<&ZwdsAnalysisOutput>,
    hd: Option<&HumanDesignAnalysisOutput>,
    qimen: Option<&QimenAnalysisOutput>,
    numerology: Option<&NumerologyAnalysisOutput>,
) -> InterpretationHandbook {
    let mut symbols: Vec<SymbolInterpretation> = Vec::new();

    // 1. Saju symbols
    let struct_str = format!("{:?}", saju.report.structure.structure);
    if let Some(interp) = saju::interpret_structure(&struct_str) {
        symbols.push(interp);
    }
    let yongshin_elem = format!("{:?}", saju.report.yongshin.primary);
    if let Some(interp) = saju::interpret_yongshin(&yongshin_elem) {
        symbols.push(interp);
    }

    // 2. Vedic symbols
    for yoga in &vedic.report.yogas {
        if let Some(interp) = vedic::interpret_yoga(&yoga.name) {
            symbols.push(interp);
        }
    }
    if let Some(interp) = vedic::interpret_nakshatra(&vedic.report.nakshatra_info) {
        symbols.push(interp);
    }

    // 3. Western symbols
    let mut sun_sign_name = None;
    if let Some(w) = western {
        for pattern in &w.result.aspect_patterns {
            if let Some(interp) = western::interpret_aspect_pattern(&pattern.name) {
                symbols.push(interp);
            }
        }
        if let Some(sun) = w.result.planets.iter().find(|p| p.name == "Sun") {
            sun_sign_name = Some(match sun.sign_index {
                0 => "Aries",
                1 => "Taurus",
                2 => "Gemini",
                3 => "Cancer",
                4 => "Leo",
                5 => "Virgo",
                6 => "Libra",
                7 => "Scorpio",
                8 => "Sagittarius",
                9 => "Capricorn",
                10 => "Aquarius",
                _ => "Pisces",
            });
        }
    }

    // 4. ZWDS symbols
    if let Some(z) = zwds {
        // Soul master
        let master_name = format!("{:?}", z.chart.soul_master);
        if let Some(interp) = zwds::interpret_major_star(&master_name) {
            symbols.push(interp);
        }
        // Major stars in Ming palace
        if z.chart.soul_idx < z.chart.palaces.len() {
            for star_info in &z.chart.palaces[z.chart.soul_idx].stars {
                let star_name = format!("{:?}", star_info.star);
                if let Some(interp) = zwds::interpret_major_star(&star_name) {
                    if !symbols.iter().any(|s| s.symbol == interp.symbol) {
                        symbols.push(interp);
                    }
                }
            }
        }
    }

    // 5. Human Design symbols
    let mut hd_type_str = None;
    if let Some(h) = hd {
        hd_type_str = Some(h.result.chart_type.as_str());
        if let Some(interp) =
            human_design::interpret_type_strategy(&h.result.chart_type, &h.result.strategy)
        {
            symbols.push(interp);
        }
        if let Some(interp) = human_design::interpret_profile(&h.result.profile) {
            symbols.push(interp);
        }
    }

    // 6. QiMen DunJia symbols
    if let Some(q) = qimen {
        if let Some(door) = q.report.pan.value_envoy_door {
            if let Some(interp) = qimen::interpret_door(door) {
                symbols.push(interp);
            }
        }
        if let Some(star) = q.report.pan.value_chief_star {
            if let Some(interp) = qimen::interpret_star(star) {
                symbols.push(interp);
            }
        }
    }

    // 7. Numerology symbols
    let mut lp_num = None;
    let mut py_num = None;
    if let Some(n) = numerology {
        lp_num = Some(n.result.core.life_path);
        py_num = Some(n.result.personal_year);
        if let Some(interp) = numerology::interpret_life_path(n.result.core.life_path) {
            symbols.push(interp);
        }
        if let Some(interp) = numerology::interpret_personal_year(n.result.personal_year) {
            symbols.push(interp);
        }
    }

    // Synthesize
    let day_master_name = format!("{:?}", saju.report.pillars.day.stem);
    let is_seven_killings_or_intense = struct_str.to_lowercase().contains("pianguan")
        || symbols
            .iter()
            .any(|s| s.symbol.contains("T-Square") || s.symbol.contains("Seven Killing"));
    let has_wealth_yoga_or_structure = struct_str.to_lowercase().contains("cai")
        || symbols
            .iter()
            .any(|s| s.symbol.contains("Dhana") || s.symbol.contains("Raja"));

    let (
        archetypal_persona,
        core_essence_summary,
        synthesis_guidelines,
        tactical_dos,
        tactical_donts,
    ) = synthesis::generate_synthesis(
        Some(&day_master_name),
        sun_sign_name,
        hd_type_str,
        lp_num,
        py_num,
        is_seven_killings_or_intense,
        has_wealth_yoga_or_structure,
    );

    InterpretationHandbook {
        archetypal_persona,
        core_essence_summary,
        symbols,
        synthesis_guidelines,
        tactical_dos,
        tactical_donts,
    }
}
