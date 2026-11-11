// crates/eon-service/src/services/report.rs
use crate::dto::{ReportTheme, SajuAnalysisOutput, ThemedReportInput, ThemedReportOutput};
use crate::error::ServiceError;
use crate::services::saju;
use chrono::Datelike;
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::element::Element;
use eon_saju::core::stem::HeavenlyStem;

pub fn generate(input: ThemedReportInput) -> Result<ThemedReportOutput, ServiceError> {
    // 1. Run Saju calculation
    let saju_out = saju::analyze(input.base.clone())?;

    // 2. Compose themed report title and content
    let (title, content) = match input.theme {
        ReportTheme::WealthAndCareer => generate_wealth_report(&saju_out, &input.user_name),
        ReportTheme::LoveAndMarriage => {
            generate_love_report(&saju_out, &input.user_name, input.base.is_male)
        }
        ReportTheme::HealthAndVitality => generate_health_report(&saju_out, &input.user_name),
    };

    Ok(ThemedReportOutput {
        meta: saju_out.meta,
        theme: input.theme,
        user_name: input.user_name,
        title,
        content,
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 1. Wealth & Career Themed Report Generator
// ─────────────────────────────────────────────────────────────────────────────
fn generate_wealth_report(saju: &SajuAnalysisOutput, name: &str) -> (String, String) {
    let title = format!("Lifetime Wealth & Career Analysis Report for {}", name);
    let mut s = String::new();

    let day_stem = saju.report.pillars.day.stem;
    let yong_sin_el = saju.report.yongshin.primary;

    // Section 1: Innate Wealth Temperament & Natural Gifts
    s.push_str("### 1. Innate Wealth Temperament & Natural Gifts\n\n");

    let day_stem_han = HeavenlyStem::HANJA[day_stem.index() as usize];

    s.push_str(&format!(
        "Your Day Master is **{:?} ({})**, and your balancing Yongshin (Useful God) element is **{:?}**.\n\n",
        day_stem, day_stem_han, yong_sin_el
    ));

    let temperament_desc = match yong_sin_el {
        Element::Wood => "You possess a wealth-generating nature driven by **Wood's expansive growth**. You excel at ideation, strategic planning, education, architecture, venture incubation, and transforming creative concepts into proprietary intellectual or commercial assets.",
        Element::Fire => "You channel **Fire's dynamic radiance** in your wealth endeavors. You thrive in high-visibility industries, marketing, IT, modern media, and artistic arenas, with natural flair for capitalizing on trends and rapid capital circulation.",
        Element::Earth => "You harness **Earth's stabilizing and accumulating energy**. Prioritizing credibility and trust, you build enduring wealth through tangible assets, real estate, brokerage, asset management, and conservative long-term compounding.",
        Element::Metal => "You embody **Metal's decisive precision and discernment**. Excelling in finance, securities, quantitative consulting, taxation, law, or high-tech manufacturing, you demonstrate disciplined risk control and return optimization.",
        Element::Water => "You wield **Water's fluid adaptability and systemic circulation**. Endowed with intellectual acuity and networking agility, you capture value through trade, logistics, digital networks, data, and intellectual property.",
    };
    s.push_str(temperament_desc);
    s.push_str("\n\n");

    // Section 2: Career Architecture & Bottlenecks
    s.push_str("### 2. Career Architecture & Bottlenecks (Qi Network Bottlenecks)\n\n");
    s.push_str("Based on Eon's Qi Topological Network analysis, your systemic career stability and structural bottlenecks are:\n\n");

    if let Some(comp) = &saju.complexity {
        s.push_str(&format!(
            "- **Cyclomatic Complexity**: {:.1} / **Stability Grade**: {:?}\n",
            comp.cyclomatic_complexity, comp.stability_grade
        ));
    }

    let bottleneck_str = if let Some(bn) = &saju.qi_topology.bottleneck {
        format!("{:?}", bn)
    } else {
        "None".to_string()
    };

    s.push_str(&format!(
        "- **Qi Network Bottleneck**: {}\n\n",
        bottleneck_str
    ));

    let bottleneck_advice = if let Some(bn) = &saju.qi_topology.bottleneck {
        format!(
            "Your career pathway encounters friction at the **{:?}** element node. To mitigate decision latency or professional friction, cultivate collaborative partnerships or credentials aligning with this element.",
            bn
        )
    } else {
        "Your internal Qi circulation demonstrates optimal equilibrium, allowing your core competencies to translate smoothly into professional recognition and sustained achievements.".to_string()
    };
    s.push_str(&bottleneck_advice);
    s.push_str("\n\n");

    // Section 3: Lifetime Wealth Luck Cycles & Turning Points
    s.push_str("### 3. Lifetime Wealth Luck Cycles & Turning Points\n\n");
    if let Some(gt) = &saju.report.golden_time {
        s.push_str(&format!(
            "Your peak energetic window (Golden Time) is identified as **ages {}-{}**.\n\n",
            gt.start_age, gt.end_age
        ));
    }

    s.push_str("Major luck cycle transitions favorable for establishing financial stability and strategic momentum:\n\n");
    s.push_str("| Decadal Cycle | Pillar | Ten Gods | Wealth & Career Guidance |\n");
    s.push_str("| --- | --- | --- | --- |\n");

    if let Some(ml) = &saju.report.major_luck {
        for cycle in ml.cycles.iter().take(5) {
            let stem_god = format!("{:?}", cycle.stem_god);
            let branch_god = format!("{:?}", cycle.branch_god);
            let stem_han = HeavenlyStem::HANJA[cycle.ganzi.stem.index() as usize];
            let branch_han = EarthlyBranch::HANJA[cycle.ganzi.branch.index() as usize];

            let is_good = stem_god.contains("Wealth")
                || stem_god.contains("Officer")
                || branch_god.contains("Wealth");
            let guide = if is_good {
                "Expansion phase. Recommended for proactive leadership, strategic investments, and role scaling."
            } else {
                "Consolidation & risk mitigation phase. Focus on internal reserves, cash liquidity, and defensive positioning."
            };
            s.push_str(&format!(
                "| Ages {}-{} | {}{} ({:?}{:?}) | {} / {} | {} |\n",
                cycle.start_age,
                cycle.end_age,
                stem_han,
                branch_han,
                cycle.ganzi.stem,
                cycle.ganzi.branch,
                stem_god,
                branch_god,
                guide
            ));
        }
    }
    s.push('\n');

    // Section 4: Strategic Action Guide
    s.push_str("### 4. Strategic Action Guide for Wealth Maximization\n\n");
    s.push_str("- **Elemental Balancing**: Harmonize your daily work environment (direction, colors, workspace setup) with your primary Yongshin element.\n");
    s.push_str("- **Benefactor Synergy**: Cultivate relationships with advisors or partners whose zodiac branches align with your Heavenly Noble (Tian Yi) stars.\n");
    s.push_str("- **Risk Management**: Rather than aggressive over-extension during transition cycles, balance structural risk through disciplined asset allocation.\n");

    (title, s)
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. Love & Marriage Themed Report Generator
// ─────────────────────────────────────────────────────────────────────────────
fn generate_love_report(saju: &SajuAnalysisOutput, name: &str, is_male: bool) -> (String, String) {
    let title = format!("Relationship & Marriage Dynamics Report for {}", name);
    let mut s = String::new();

    let day_stem = saju.report.pillars.day.stem;
    let day_branch = saju.report.pillars.day.branch;

    // Section 1: Innate Relationship Temperament & Ideal Partner Profile
    s.push_str("### 1. Innate Relationship Temperament & Ideal Partner Profile\n\n");

    let day_stem_han = HeavenlyStem::HANJA[day_stem.index() as usize];
    let day_branch_han = EarthlyBranch::HANJA[day_branch.index() as usize];

    s.push_str(&format!(
        "You were born with Day Master **{:?} ({})** and Spouse Palace **{:?} ({})**.\n\n",
        day_stem, day_stem_han, day_branch, day_branch_han
    ));

    let spouse_desc = if is_male {
        "In a male chart, the Day Branch reflects your partner's disposition and domestic anchor. The presence of integrity and steadfast energy indicates affinity with a responsible, loyal partner who shares long-term commitments and anchors family stability."
    } else {
        "In a female chart, the Day Branch reflects emotional resonance with your partner. The presence of perceptive and expressive energy indicates strong chemistry with a communicative, intellectually engaging partner who respects mutual independence."
    };
    s.push_str(spouse_desc);
    s.push_str("\n\n");

    // Section 2: Relationship Strengths and Cautionary Dynamics
    s.push_str("### 2. Relationship Strengths and Cautionary Dynamics (Void & Interactions)\n\n");
    s.push_str("Analysis of natal branch combinations, clashes, and void placements influencing your relationship terrain:\n\n");

    let has_conflict = saju.relationships.mapped_relationships.iter().any(|r| {
        r.relation_type.contains("Clash")
            || r.relation_type.contains("Punishment")
            || r.relation_type.contains("Harm")
            || r.relation_type.contains("Destruction")
    });
    let relationships_desc = if has_conflict {
        "Certain clash or friction patterns are observed around your Spouse Palace. While relationships may begin with intense passion, minor miscommunications can amplify over time. Transparent dialogue and scheduled reflection serve as vital anchors."
    } else {
        "Your Spouse Palace maintains smooth geometric harmony with neighboring branches. Disagreements are settled constructively, fostering deep, long-term trust and mutual respect."
    };
    s.push_str(relationships_desc);
    s.push_str("\n\n");

    // Section 3: Partnership Windows & Marriage Timing
    s.push_str("### 3. Partnership Windows & Marriage Timing (Twelve Stages)\n\n");
    s.push_str("Projected relationship vibrancy and milestone encounter windows across the upcoming 5 years:\n\n");
    s.push_str("| Year | Annual Pillar | Lifecycle Stage | Partnership Potential |\n");
    s.push_str("| --- | --- | --- | --- |\n");

    let current_year = chrono::Utc::now().year();
    let years = (current_year..current_year.saturating_add(5)).collect::<Vec<_>>();
    for (i, yr) in years.into_iter().enumerate() {
        let ganzi_value = eon_saju::core::ganzi::GanZi::from_year(yr);
        let ganzi = format!(
            "{}{} ({:?}{:?})",
            HeavenlyStem::HANJA[ganzi_value.stem.index() as usize],
            EarthlyBranch::HANJA[ganzi_value.branch.index() as usize],
            ganzi_value.stem,
            ganzi_value.branch
        );
        let stage = format!(
            "{:?}",
            eon_saju::core::twelve_stages::calculate_twelve_stage(day_stem, ganzi_value.branch)
        );
        let score = match i {
            0 => "[3/5] - Emergence of curious connections",
            1 => "[4/5] - Steady growth and deepened bonding",
            2 => "[5/5] - Optimal peak window for enduring commitment",
            3 => "[4/5] - Heightened charisma and romance",
            _ => "[3/5] - Grounded and responsible relationship building",
        };
        s.push_str(&format!(
            "| Year {} | {} | {} | {} |\n",
            yr, ganzi, stage, score
        ));
    }
    s.push('\n');

    // Section 4: Action Solutions
    s.push_str("### 4. Action Strategies for Harmonious Relationships\n\n");
    s.push_str("- **Mutual Empathy**: When tensions arise, actively practice empathetic listening before expressing counterarguments.\n");
    s.push_str("- **Complementary Synergy**: Partnering with someone who enriches your less-represented elements creates holistic life balance and enduring harmony.\n");

    (title, s)
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. Health & Vitality Themed Report Generator
// ─────────────────────────────────────────────────────────────────────────────
fn generate_health_report(saju: &SajuAnalysisOutput, name: &str) -> (String, String) {
    let title = format!("Health Constitution & Vitality Report for {}", name);
    let mut s = String::new();

    let day_stem = saju.report.pillars.day.stem;

    // Section 1: Health Constitution
    s.push_str("### 1. Innate Health Constitution from Five Elements Balance\n\n");

    let day_stem_han = HeavenlyStem::HANJA[day_stem.index() as usize];

    s.push_str(&format!(
        "Your constitution is anchored in Day Master **{:?} ({})**. Based on elemental distribution, your physiological balance shows:\n\n",
        day_stem, day_stem_han
    ));

    // Element distribution analysis
    let mut fire_count = 0;
    let mut water_count = 0;
    for pillar in &[
        &saju.report.pillars.year,
        &saju.report.pillars.month,
        &saju.report.pillars.day,
        &saju.report.pillars.hour,
    ] {
        if pillar.stem.element() == Element::Fire {
            fire_count += 1;
        }
        if pillar.branch.element() == Element::Fire {
            fire_count += 1;
        }
        if pillar.stem.element() == Element::Water {
            water_count += 1;
        }
        if pillar.branch.element() == Element::Water {
            water_count += 1;
        }
    }

    let health_desc = if fire_count > 3 {
        "Your chart exhibits an **abundance of Fire energy**. Prioritize cardiovascular wellness, hydration, eye care, and blood pressure regulation. Regular sleep schedules and cooling routines are highly beneficial."
    } else if water_count > 3 {
        "Your chart exhibits an **abundance of Water energy** or cooler thermal traits. Prioritize renal/urinary wellness, circulation, and lower-body strength. Warm beverages and consistent aerobic movement promote optimal equilibrium."
    } else {
        "Your five elements show balanced distribution with low risk of acute organ stress. Maintain vitality through seasonal transitions with wholesome nutrition, stretching, and steady sleep rhythms."
    };
    s.push_str(health_desc);
    s.push_str("\n\n");

    // Section 2: Vulnerabilities & Lints
    s.push_str("### 2. Physical Vulnerabilities & Warning Signals (Destiny Lints)\n\n");
    s.push_str("Eon linter diagnostics on potential constitutional vulnerabilities:\n\n");

    if saju.lints.is_empty() {
        s.push_str("- **Detected Warnings**: None (Systemic equilibrium is well-maintained)\n\n");
    } else {
        for lint in saju.lints.iter().take(2) {
            s.push_str(&format!("- **[{:?}]**: {}\n", lint.severity, lint.message));
        }
        s.push('\n');
    }

    // Section 3: Mind Healing & Wellness
    s.push_str("### 3. Mind Healing & Energy Management Guide\n\n");
    s.push_str(
        "Wellness and restorative guidelines for stress resilience and inner calibration:\n\n",
    );
    s.push_str("- **Recommended Movement**: Low-impact aerobic walking and mindful breathwork to elevate cellular oxygenation.\n");
    s.push_str("- **Color Therapy**: Incorporate restorative, grounding natural tones (soft greens and earthen neutrals) into your sleeping environment.\n");
    s.push_str("- **Stress Release**: Since rumination impacts digestive efficiency, dedicate 10 minutes daily to complete cognitive stillness and meditation.\n");

    (title, s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::{AnalysisInput, BirthTimePrecision, SajuAnalysisInput};

    #[test]
    fn test_themed_report_generation() {
        let base = AnalysisInput {
            year: 1990,
            month: 5,
            day: 15,
            hour: 10,
            minute: 0,
            is_lunar: false,
            is_leap_month: false,
            lat: 37.5665,
            lon: 126.9780,
            timezone: "Asia/Seoul".to_string(),
        };
        let saju_input = SajuAnalysisInput {
            base,
            is_male: true,
            use_night_rat_hour: false,
            precision: BirthTimePrecision::Exact,
        };

        // 1. Wealth & Career Report
        let input_wealth = ThemedReportInput {
            base: saju_input.clone(),
            theme: ReportTheme::WealthAndCareer,
            user_name: "Alex".to_string(),
        };
        let out_wealth = generate(input_wealth).unwrap();
        assert!(out_wealth.title.contains("Alex"));
        assert!(out_wealth.content.contains("Wealth"));

        // 2. Love & Marriage Report
        let input_love = ThemedReportInput {
            base: saju_input.clone(),
            theme: ReportTheme::LoveAndMarriage,
            user_name: "Alex".to_string(),
        };
        let out_love = generate(input_love).unwrap();
        assert!(out_love.title.contains("Alex"));
        assert!(out_love.content.contains("Relationship"));

        // 3. Health & Vitality Report
        let input_health = ThemedReportInput {
            base: saju_input,
            theme: ReportTheme::HealthAndVitality,
            user_name: "Alex".to_string(),
        };
        let out_health = generate(input_health).unwrap();
        assert!(out_health.title.contains("Alex"));
        assert!(out_health.content.contains("Health"));
    }
}
