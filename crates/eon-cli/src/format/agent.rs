use eon_service::dto::*;

fn zodiac_sign_name(idx: usize) -> &'static str {
    match idx % 12 {
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
        11 => "Pisces",
        _ => "Unknown",
    }
}

#[allow(clippy::too_many_arguments)]
pub fn format_agent_context(
    input: &AnalysisInput,
    saju: Option<&SajuAnalysisOutput>,
    vedic: Option<&VedicAnalysisOutput>,
    western: Option<&WesternAnalysisOutput>,
    zwds: Option<&ZwdsAnalysisOutput>,
    hd: Option<&HumanDesignAnalysisOutput>,
    qimen: Option<&QimenAnalysisOutput>,
    num: Option<&NumerologyAnalysisOutput>,
    tier: Option<&TierResult>,
    handbook: Option<&InterpretationHandbook>,
) -> String {
    let mut out = String::new();

    out.push_str("# [DESTINY INTELLIGENCE PROFILE FOR AI AGENT]\n\n");
    let loc_str = if let Some(ref c) = input.city {
        format!(
            "Location: {c} (Lat: {:.4}, Lon: {:.4})",
            input.lat, input.lon
        )
    } else {
        format!("Coordinates: ({:.4}, {:.4})", input.lat, input.lon)
    };

    let std_meridian = eon_core::standard_meridian_from_tz(&input.timezone);
    let offset_mins = ((input.lon - std_meridian) * 4.0).round() as i32;
    let sign = if offset_mins >= 0 { "+" } else { "" };

    let solar_correction_str = if let Some(s) = saju {
        let dst_str = if s.meta.is_dst { " | DST: Active" } else { "" };
        format!(
            "True Solar Time: {} ({}{}m{})",
            s.meta.corrected_time, sign, offset_mins, dst_str
        )
    } else {
        format!("Solar Time Offset: {}{}m", sign, offset_mins)
    };

    out.push_str(&format!(
        "> **Birth**: {:04}-{:02}-{:02} {:02}:{:02} ({}) | {} | {}\n\n",
        input.year,
        input.month,
        input.day,
        input.hour,
        input.minute,
        input.timezone,
        loc_str,
        solar_correction_str,
    ));

    // 1. Omni Destiny Tier 7.0
    if let Some(t) = tier {
        out.push_str("## 1. Omni Destiny Tier 7.0\n");
        out.push_str(&format!(
            "- **Overall Destiny Tier**: **{}** (Birth Chart Raw: {:.1}pt / Total Score: {:.1}pt / Potential: **{}** {:.1}pt)\n",
            t.destiny_tier.grade, t.destiny_raw_score, t.destiny_tier_score, t.potential_tier.grade, t.potential_score
        ));
        out.push_str(&format!(
            "- **Profile Type**: {} | **Growth Gap**: +{:.1}pt\n",
            t.profile, t.growth_gap
        ));

        if !t.quantum_synergies.is_empty() {
            out.push_str("- **8D Quantum Cross Synergies (Cross-Engine Resonance)**:\n");
            for s in &t.quantum_synergies {
                let sign = if s.score_delta >= 0.0 { "+" } else { "" };
                out.push_str(&format!(
                    "  * [{}] {}{:.0}pt — {}\n",
                    s.title, sign, s.score_delta, s.description
                ));
            }
        }

        if !t.strengths.is_empty() {
            out.push_str(&format!(
                "- **Key Strengths**: {}\n",
                t.strengths.join(", ")
            ));
        }
        if !t.weaknesses.is_empty() {
            out.push_str(&format!(
                "- **Key Vulnerabilities**: {}\n",
                t.weaknesses.join(", ")
            ));
        }
        out.push('\n');
    }

    // 2. Four Pillars of Destiny (Saju / BaZi)
    if let Some(s) = saju {
        out.push_str("## 2. Four Pillars of Destiny (Saju / BaZi)\n");
        let pillars = &s.report.pillars;
        out.push_str(&format!(
            "- **Four Pillars**: Year: {} ({:?}-{:?}) | Month: {} ({:?}-{:?}) | Day: {} ({:?}-{:?}) | Hour: {} ({:?}-{:?})\n",
            pillars.year,
            pillars.year.stem,
            pillars.year.branch,
            pillars.month,
            pillars.month.stem,
            pillars.month.branch,
            pillars.day,
            pillars.day.stem,
            pillars.day.branch,
            pillars.hour,
            pillars.hour.stem,
            pillars.hour.branch
        ));
        out.push_str(&format!(
            "- **Day Master Strength**: {:?} (Score: {:.1})\n",
            s.report.strength.strength_type, s.report.strength.strength_score
        ));
        out.push_str(&format!(
            "- **Structure & Yongshin**: Structure: {:?}, Primary Yongshin: {:?}\n",
            s.report.structure.structure, s.report.yongshin.primary
        ));
        out.push_str(&format!(
            "- **Five Elements Throughput**: {:.1}%\n",
            s.qi_topology.throughput * 100.0
        ));
        if let Some(gt) = &s.report.golden_time {
            out.push_str(&format!(
                "- **Golden Timing Period**: Age {} ~ {} (Avg Score: {:.1}pt)\n",
                gt.start_age, gt.end_age, gt.average_score
            ));
        }
        out.push('\n');
    }

    // 3. Vedic Astrology (Jyotish)
    if let Some(v) = vedic {
        out.push_str("## 3. Vedic Astrology (Jyotish)\n");
        out.push_str(&format!(
            "- **Ascendant (Lagna)**: {:?}\n",
            v.chart.ascendant
        ));
        out.push_str(&format!(
            "- **Moon Nakshatra**: {}\n",
            v.report.nakshatra_info
        ));
        out.push_str(&format!("- **Sade Sati**: {:?}\n", v.report.sade_sati));

        let top_yogas: Vec<_> = v.report.yogas.iter().take(4).collect();
        if !top_yogas.is_empty() {
            out.push_str("- **Key Yogas**:\n");
            for y in top_yogas {
                out.push_str(&format!(
                    "  * **{}** ({:?}): {}\n",
                    y.name, y.quality, y.description
                ));
            }
        }
        out.push('\n');
    }

    // 4. Western Astrology
    if let Some(w) = western {
        out.push_str("## 4. Western Astrology\n");
        let sun = w.result.planets.iter().find(|p| p.name == "Sun");
        let moon = w.result.planets.iter().find(|p| p.name == "Moon");
        let asc = w.result.houses.first();

        if let (Some(s), Some(m), Some(a)) = (sun, moon, asc) {
            out.push_str(&format!(
                "- **Big 3**: Sun ({}, H{}), Moon ({}, H{}), Ascendant ({})\n",
                zodiac_sign_name(s.sign_index),
                s.house_number,
                zodiac_sign_name(m.sign_index),
                m.house_number,
                zodiac_sign_name(a.sign_index)
            ));
        }
        if !w.result.aspect_patterns.is_empty() {
            let patterns: Vec<String> = w
                .result
                .aspect_patterns
                .iter()
                .map(|p| format!("{:?}", p.pattern_type))
                .collect();
            out.push_str(&format!(
                "- **Aspect Configurations**: {}\n",
                patterns.join(", ")
            ));
        }
        out.push('\n');
    }

    // 5. Zi Wei Dou Shu (Purple Star Astrology)
    if let Some(z) = zwds {
        out.push_str("## 5. Zi Wei Dou Shu (Purple Star Astrology)\n");
        let soul_palace = &z.chart.palaces[z.chart.soul_idx];
        let stars: Vec<String> = soul_palace
            .stars
            .iter()
            .map(|s| format!("{:?}", s.star))
            .collect();
        out.push_str(&format!(
            "- **Life Palace**: {} (Major Stars: {})\n",
            soul_palace.name.english(),
            if stars.is_empty() {
                "Empty Palace (No Major Stars)".to_string()
            } else {
                stars.join(", ")
            }
        ));
        if !z.chart.destiny_patterns.is_empty() {
            let pats: Vec<String> = z
                .chart
                .destiny_patterns
                .iter()
                .map(|p| p.name_english.clone())
                .collect();
            out.push_str(&format!(
                "- **Key Formations (Destiny Patterns)**: {}\n",
                pats.join(", ")
            ));
        }
        out.push('\n');
    }

    // 6. Human Design
    if let Some(h) = hd {
        out.push_str("## 6. Human Design\n");
        out.push_str(&format!(
            "- **Type**: {} | **Profile**: {} | **Strategy**: {}\n",
            h.result.chart_type, h.result.profile, h.result.strategy
        ));
        out.push_str(&format!(
            "- **Defined Centers**: {} ({:?})\n",
            h.result.defined_centers.len(),
            h.result.defined_centers
        ));
        out.push_str(&format!(
            "- **Active Channels**: {}\n",
            h.result.active_channels.len()
        ));
        out.push('\n');
    }

    // 7. QiMen DunJia
    if let Some(q) = qimen {
        out.push_str("## 7. QiMen DunJia\n");
        out.push_str(&format!(
            "- **Structure (Ju)**: {} Ju {}\n",
            if q.report.pan.is_yin_ju {
                "Yin Dun (陰遁)"
            } else {
                "Yang Dun (陽遁)"
            },
            q.report.pan.ju_number
        ));
        if let Some(chief) = &q.report.pan.value_chief_star {
            out.push_str(&format!("- **Chief Star (Zhi Fu)**: {:?}\n", chief));
        }
        if let Some(envoy) = &q.report.pan.value_envoy_door {
            out.push_str(&format!("- **Envoy Door (Zhi Shi)**: {:?}\n", envoy));
        }
        out.push('\n');
    }

    // 8. Pythagorean Numerology
    if let Some(n) = num {
        out.push_str("## 8. Pythagorean Numerology\n");
        let core = &n.result.core;
        out.push_str(&format!(
            "- **Life Path Number**: **{}**{}\n",
            core.life_path,
            if n.result.is_master_life_path {
                " (Master Number)"
            } else {
                ""
            }
        ));
        out.push_str(&format!(
            "- **Core Numbers**: Expression: {}, Soul Urge: {}, Personality: {}, Birthday: {}\n",
            core.expression, core.soul_urge, core.personality, core.birthday
        ));
        out.push_str(&format!(
            "- **Personal Year**: Cycle Vibration {}\n",
            n.result.personal_year
        ));
        out.push('\n');
    }

    // 9. Domain Interpretation Handbook & Archetypal Decoder
    if let Some(hb) = handbook {
        out.push_str("## 9. Domain Interpretation Handbook & Archetypal Decoder (고전 해설집 및 원형 디코더)\n\n");
        out.push_str(&format!(
            "> **Archetypal Persona**: {}\n",
            hb.archetypal_persona
        ));
        out.push_str(&format!(
            "> **Core Essence**: {}\n\n",
            hb.core_essence_summary
        ));

        if !hb.symbols.is_empty() {
            out.push_str("### [A. Pinpoint Classical Symbol Decoders]\n");
            out.push_str("*Verified classical grounding (자평진전, 궁통보감, BPHS, 자미두수전서, 기문비급 등) decoded into modern executive psychology:*\n\n");
            for s in &hb.symbols {
                out.push_str(&format!("#### {} [{}]\n", s.symbol, s.engine));
                out.push_str(&format!(
                    "- **Classical Grounding**: {}\n",
                    s.classical_grounding
                ));
                out.push_str(&format!(
                    "- **Modern Translation**: {}\n",
                    s.modern_translation
                ));
                out.push_str(&format!("- **Shadow & Risk**: {}\n", s.shadow_and_caution));
                out.push_str(&format!("- **Tactical Remedy**: {}\n\n", s.tactical_remedy));
            }
        }

        if !hb.synthesis_guidelines.is_empty() {
            out.push_str("### [B. Multi-Dimensional Synthesis Guidelines]\n");
            for g in &hb.synthesis_guidelines {
                out.push_str(&format!("#### {}\n", g.domain));
                out.push_str(&format!(
                    "- **Cross-Engine Insight**: {}\n",
                    g.combined_insight
                ));
                out.push_str(&format!(
                    "- **Strategic Action**: {}\n\n",
                    g.strategic_action
                ));
            }
        }

        out.push_str("### [C. Tactical Directives]\n");
        out.push_str("#### DO (강력 권장):\n");
        for d in &hb.tactical_dos {
            out.push_str(&format!("- [x] {}\n", d));
        }
        out.push('\n');

        out.push_str("#### DON'T (절대 주의):\n");
        for d in &hb.tactical_donts {
            out.push_str(&format!("- [ ] {}\n", d));
        }
        out.push('\n');
    }

    out.push_str("---\n");
    out.push_str("*Instructions for AI Agent: Ground your analysis strictly on the verified multi-disciplinary calculation data and the classical-to-modern decoders in Section 9. Interpret the detected symbols directly without hallucination, resolve engine nuances using the Cross-Engine Synthesis Guidelines, and deliver actionable counseling following the Tactical DOs & DON'Ts.*\n");

    out
}
