use eon_service::dto::*;

#[allow(clippy::too_many_arguments)]
pub fn format_markdown_report(
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
    let mut md = String::new();

    md.push_str("# Eon: 8-Engine Destiny Intelligence Report\n\n");
    let loc_str = if let Some(ref c) = input.city {
        format!(
            "**Location**: {c} (Lat: {:.4}, Lon: {:.4})",
            input.lat, input.lon
        )
    } else {
        format!("**Coordinates**: ({:.4}, {:.4})", input.lat, input.lon)
    };

    let std_meridian = eon_core::standard_meridian_from_tz(&input.timezone);
    let offset_mins = ((input.lon - std_meridian) * 4.0).round() as i32;
    let sign = if offset_mins >= 0 { "+" } else { "" };

    let solar_str = if let Some(s) = saju {
        let dst_str = if s.meta.is_dst { ", DST: Active" } else { "" };
        format!(
            "**True Solar Time**: {} ({}{}m{})",
            s.meta.corrected_time, sign, offset_mins, dst_str
        )
    } else {
        format!("**Solar Time Offset**: {}{}m", sign, offset_mins)
    };

    md.push_str(&format!(
        "**Birth**: {:04}-{:02}-{:02} {:02}:{:02} | **Timezone**: {} | {} | {}\n\n",
        input.year,
        input.month,
        input.day,
        input.hour,
        input.minute,
        input.timezone,
        loc_str,
        solar_str
    ));

    if let Some(t) = tier {
        md.push_str("## Omni Destiny Tier 7.0 Summary\n");
        md.push_str("| Metric | Score / Grade |\n|---|---|\n");
        md.push_str(&format!(
            "| **Overall Destiny Tier** | **{}** ({:.1} pts) |\n",
            t.destiny_tier.grade, t.destiny_tier_score
        ));
        md.push_str(&format!(
            "| **Birth Chart Potential** | **{}** ({:.1} pts) |\n",
            t.potential_tier.grade, t.potential_score
        ));
        md.push_str(&format!(
            "| **Current Transit Score** | {:.1} pts |\n",
            t.current_score
        ));
        md.push_str(&format!("| **Growth Gap** | +{:.1} pts |\n", t.growth_gap));
        md.push_str(&format!(
            "| **Risk / Stability Level** | {} |\n\n",
            t.risk_level
        ));

        if !t.detailed_components.is_empty() {
            md.push_str("### 15 Detailed Component Weights\n");
            md.push_str(
                "| Component | Weight | Score | Primary Analysis Factor |\n|---|---|---|---|\n",
            );
            for c in &t.detailed_components {
                let reason = c.reasons.first().cloned().unwrap_or_default();
                md.push_str(&format!(
                    "| {} | {:.0}% | {:.1} | {} |\n",
                    c.label,
                    c.weight * 100.0,
                    c.score,
                    reason
                ));
            }
            md.push('\n');
        }

        if !t.quantum_synergies.is_empty() {
            md.push_str("### 8D Quantum Cross Synergies\n");
            for s in &t.quantum_synergies {
                let badge = if s.score_delta >= 0.0 { "[+]" } else { "[-]" };
                md.push_str(&format!(
                    "- {} **{}** ({:+.0}pt): {}\n",
                    badge, s.title, s.score_delta, s.description
                ));
            }
            md.push('\n');
        }
    }

    if let Some(s) = saju {
        md.push_str("## 1. Four Pillars of Destiny (Saju / BaZi)\n");
        let p = &s.report.pillars;
        md.push_str(&format!(
            "- **Chart**: Year: {} ({:?}-{:?}) / Month: {} ({:?}-{:?}) / Day: {} ({:?}-{:?}) / Hour: {} ({:?}-{:?})\n",
            p.year,
            p.year.stem,
            p.year.branch,
            p.month,
            p.month.stem,
            p.month.branch,
            p.day,
            p.day.stem,
            p.day.branch,
            p.hour,
            p.hour.stem,
            p.hour.branch
        ));
        md.push_str(&format!(
            "- **Strength**: {:?} (Score: {:.1})\n",
            s.report.strength.strength_type, s.report.strength.strength_score
        ));
        md.push_str(&format!(
            "- **Structure**: {:?} | **Primary Yongshin**: {:?}\n\n",
            s.report.structure.structure, s.report.yongshin.primary
        ));
    }

    if let Some(v) = vedic {
        md.push_str("## 2. Vedic Astrology (Jyotish)\n");
        md.push_str(&format!(
            "- **Ascendant (Lagna)**: {:?}\n",
            v.chart.ascendant
        ));
        md.push_str(&format!(
            "- **Moon Nakshatra**: {}\n",
            v.report.nakshatra_info
        ));
        md.push_str(&format!(
            "- **Detected Yogas**: {}\n\n",
            v.report.yogas.len()
        ));
    }

    if let Some(w) = western {
        md.push_str("## 3. Western Astrology\n");
        md.push_str(&format!(
            "- **Planets**: {} | **Aspects**: {}\n",
            w.result.planets.len(),
            w.result.aspects.len()
        ));
        md.push_str(&format!(
            "- **Aspect Configurations**: {}\n\n",
            w.result.aspect_patterns.len()
        ));
    }

    if let Some(z) = zwds {
        md.push_str("## 4. Zi Wei Dou Shu\n");
        let soul = &z.chart.palaces[z.chart.soul_idx];
        md.push_str(&format!("- **Soul Palace**: {}\n", soul.name.english()));
        md.push_str(&format!(
            "- **Stars in Soul Palace**: {}\n\n",
            soul.stars.len()
        ));
    }

    if let Some(h) = hd {
        md.push_str("## 5. Human Design\n");
        md.push_str(&format!(
            "- **Chart Type**: {} | **Profile**: {}\n",
            h.result.chart_type, h.result.profile
        ));
        md.push_str(&format!(
            "- **Defined Centers**: {}\n\n",
            h.result.defined_centers.len()
        ));
    }

    if let Some(q) = qimen {
        md.push_str("## 6. QiMen DunJia\n");
        md.push_str(&format!(
            "- **Chart**: {} Ju {} (9 Palaces Deployed)\n\n",
            if q.report.pan.is_yin_ju {
                "Yin Dun"
            } else {
                "Yang Dun"
            },
            q.report.pan.ju_number
        ));
    }

    if let Some(n) = num {
        md.push_str("## 7. Pythagorean Numerology\n");
        md.push_str(&format!(
            "- **Life Path Number**: **{}**{}\n",
            n.result.core.life_path,
            if n.result.is_master_life_path {
                " (Master Number)"
            } else {
                ""
            }
        ));
        md.push_str(&format!(
            "- **Expression**: {} | **Soul Urge**: {} | **Personal Year**: {}\n\n",
            n.result.core.expression, n.result.core.soul_urge, n.result.personal_year
        ));
    }

    if let Some(hb) = handbook {
        md.push_str("## 8. Domain Interpretation Handbook & Archetypal Decoder\n");
        md.push_str(&format!("> **Persona**: {}\n\n", hb.archetypal_persona));
        md.push_str(&format!("{}\n\n", hb.core_essence_summary));

        if !hb.symbols.is_empty() {
            md.push_str("### Pinpoint Classical Symbol Decoders\n\n");
            for s in &hb.symbols {
                md.push_str(&format!("#### {} ({})\n", s.symbol, s.engine));
                md.push_str(&format!(
                    "- **Classical Grounding**: {}\n",
                    s.classical_grounding
                ));
                md.push_str(&format!(
                    "- **Modern Translation**: {}\n",
                    s.modern_translation
                ));
                md.push_str(&format!("- **Shadow & Risk**: {}\n", s.shadow_and_caution));
                md.push_str(&format!("- **Tactical Remedy**: {}\n\n", s.tactical_remedy));
            }
        }

        if !hb.tactical_dos.is_empty() || !hb.tactical_donts.is_empty() {
            md.push_str("### Tactical Directives\n\n");
            md.push_str("**DO**:\n");
            for d in &hb.tactical_dos {
                md.push_str(&format!("- [x] {}\n", d));
            }
            md.push_str("\n**DON'T**:\n");
            for d in &hb.tactical_donts {
                md.push_str(&format!("- [ ] {}\n", d));
            }
            md.push('\n');
        }
    }

    md
}
