use crate::dto::CrossEngineSynthesisGuideline;

pub fn generate_synthesis(
    day_master_name: Option<&str>,
    sun_sign_name: Option<&str>,
    hd_type: Option<&str>,
    numerology_lp: Option<u8>,
    numerology_py: Option<u8>,
    is_seven_killings_or_intense: bool,
    has_wealth_yoga_or_structure: bool,
) -> (
    String,
    String,
    Vec<CrossEngineSynthesisGuideline>,
    Vec<String>,
    Vec<String>,
) {
    let dm_clean = day_master_name.unwrap_or("Unknown");
    let sun_clean = sun_sign_name.unwrap_or("Unknown");
    let hd_clean = hd_type.unwrap_or("Unknown");
    let lp_val = numerology_lp.unwrap_or(1);
    let py_val = numerology_py.unwrap_or(1);

    // 1. Archetypal Persona Determination
    let persona_title = if dm_clean.contains("Jia")
        || dm_clean.contains("Geng")
        || lp_val == 1
        || lp_val == 8
        || hd_clean.to_lowercase().contains("manifestor")
        || hd_clean.to_lowercase().contains("manifesting")
    {
        "The Sovereign Architect & Civilizational Pioneer"
    } else if dm_clean.contains("Bing")
        || dm_clean.contains("Ding")
        || lp_val == 3
        || lp_val == 5
        || sun_clean.contains("Leo")
        || sun_clean.contains("Aries")
        || sun_clean.contains("Sagittarius")
    {
        "The Radiant Catalyst & Paradigm Innovator"
    } else if dm_clean.contains("Ren")
        || dm_clean.contains("Gui")
        || lp_val == 7
        || lp_val == 11
        || hd_clean.to_lowercase().contains("projector")
    {
        "The Intuitive Strategist & Abyss Navigator"
    } else if dm_clean.contains("Wu") || dm_clean.contains("Ji") || lp_val == 4 || lp_val == 22 {
        "The Master Builder & Bedrock Infrastructure Operator"
    } else {
        "The Harmonizing Alchemist & Universal Values Transmitter"
    };

    let archetypal_persona = format!(
        "{} [Day Master: {} | Sun: {} | Type: {} | LP: {}]",
        persona_title, dm_clean, sun_clean, hd_clean, lp_val
    );

    // 2. Core Essence Summary
    let core_essence_summary = format!(
        "This chart displays remarkable multi-dimensional convergence across classical BaZi structural mechanics, Vedic karmic yogas, Western planetary aspect geometry, Human Design energetic mechanics, and Pythagorean numerological epicycles. The core energy of Day Master '{}', Western Sun '{}', Human Design '{}' type, and Numerology Life Path '{}' harmonize to forge a sovereign leadership blueprint—one that releases immense real-world executive power when governing independent systems and executing without external subordination.",
        dm_clean, sun_clean, hd_clean, lp_val
    );

    // 3. Cross-Engine Synthesis Guidelines
    let career_guideline = if has_wealth_yoga_or_structure {
        CrossEngineSynthesisGuideline {
            domain: "Career & Wealth Architecture".to_string(),
            combined_insight: "The chart's commanding structural momentum in BaZi, auspicious Vedic Dhana/Raja Yogas, and high-frequency numerological drive amplify one another. This configuration explicitly promises major enterprise attainment by fusing proprietary intellectual assets with capital leverage beyond linear labor.".to_string(),
            strategic_action: "Look beyond temporary short-term ripples; build proprietary platforms and high-barrier intellectual property (IP) systems that generate compound enterprise value. Prioritize establishing clear governance frameworks that command elite talent and capital.".to_string(),
        }
    } else {
        CrossEngineSynthesisGuideline {
            domain: "Career & Wealth Architecture".to_string(),
            combined_insight: "Deep investigative research, proprietary technical engineering, and creative vision serve as the primary engine converting specialized capability into enterprise equity. Recognized credentials, patents, and technical defensibility constitute your most valuable balance sheet assets.".to_string(),
            strategic_action: "Establish streamlined commercial distribution pipelines that bridge technical and creative breakthroughs directly to market liquidity. Partner with pragmatic operational and commercial co-founders.".to_string(),
        }
    };

    let relationship_guideline = CrossEngineSynthesisGuideline {
        domain: "Interpersonal Chemistry & Leadership".to_string(),
        combined_insight: "Profound diagnostic insight and sovereign autonomy enable you to instantly detect structural inefficiencies, pretense, and hidden liabilities in external parties. While this forms the bedrock of visionary leadership, zero tolerance for compromise can provoke unnecessary friction.".to_string(),
        strategic_action: "Do not attempt to fix or recalibrate every person you encounter. Intervene only within spheres where you possess formal mandate, recognition, and invitation. Establishing clear mutual division of responsibilities (R&R) and psychological buffer zones preserves optimal collaborative synergy.".to_string(),
    };

    let timing_guideline = CrossEngineSynthesisGuideline {
        domain: "Operational Rhythm & Timing Strategy".to_string(),
        combined_insight: format!(
            "Currently aligned with Pythagorean Personal Year {} vibration, intersecting with QiMen DunJia spatial vectors and major life-cycle progressions. Discerning between macro energetic ebbs and flows unlocks maximum strategic harvest with minimal resistance.",
            py_val
        ),
        strategic_action: match py_val {
            1 | 8 => "A supreme window for bold sovereign initiatives, venture creation, or major capital deployment. Execute decisive strategic initiatives with uncompromising focus.".to_string(),
            2 | 6 => "A period dedicated to strategic alliances, team cohesion, and internal governance hardening. Avoid head-on collisions; build defensive, collaborative coalitions.".to_string(),
            3 | 5 => "Highly advantageous for outbound public communication, brand evangelism, global market expansion, and business model pivots. Radiate creative firepower into the world.".to_string(),
            4 | 7 => "Exercise strategic restraint against external expansion; prioritize R&D, core infrastructure optimization, patent filings, and psychological renewal. Defer speculative bets.".to_string(),
            _ => "A grand transition phase dedicated to cleanly concluding multi-year initiatives, deprecating legacy liabilities, and clearing operational runway for the next cycle.".to_string(),
        },
    };

    let synthesis_guidelines = vec![career_guideline, relationship_guideline, timing_guideline];

    // 4. Tactical DOs and DON'Ts
    let mut tactical_dos = vec![
        "Build and defend your sovereign niche and institutional governance system independently.".to_string(),
        "Base pivotal strategic decisions on somatic intuition (gut/sacral response) and formal strategy invitation rather than mental pressure.".to_string(),
        "Monetize high-barrier intangible assets early: proprietary intellectual property, patents, specialized architecture, and brand equity.".to_string(),
        "Enforce mandatory solitary thinking and recharge blocks (Solo Buffer Time) in your weekly calendar to prevent cognitive and nervous depletion.".to_string(),
    ];

    let mut tactical_donts = vec![
        "Never intervene in external disputes or offer unsolicited advice without explicit recognition, invitation, or commercial mandate.".to_string(),
        "Never compromise your foundational principles or long-term vision under the pressure of transient external approval or vanity.".to_string(),
        "Refuse the trap of operational micromanagement and solitary hero syndrome; delegate execution to empowered lieutenants.".to_string(),
        "Never execute irreversible capital bets or bind yourself to long-term contracts during moments of acute emotional volatility or haste.".to_string(),
    ];

    if is_seven_killings_or_intense {
        tactical_dos.insert(
            0,
            "During high-pressure crisis inflection points, enforce a strict 48-hour strategic reflection buffer before reacting."
                .to_string(),
        );
        tactical_donts.insert(
            0,
            "Never express unilateral wrath during crises; burning trust equity with core allies in an instant can cause permanent systemic damage.".to_string(),
        );
    }

    (
        archetypal_persona,
        core_essence_summary,
        synthesis_guidelines,
        tactical_dos,
        tactical_donts,
    )
}
