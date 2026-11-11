use crate::dto::SymbolInterpretation;

pub fn interpret_aspect_pattern(pattern_name: &str) -> Option<SymbolInterpretation> {
    let name_lower = pattern_name.to_lowercase();
    if name_lower.contains("tsquare") || name_lower.contains("t-square") {
        Some(SymbolInterpretation {
            engine: "Western Astrology".to_string(),
            symbol: "T-Square Aspect Configuration".to_string(),
            classical_grounding: "Psychological Astrology: 'Two planets in an intense 180-degree opposition channeled through a 90-degree square into a single Focal Planet, generating immense catalytic friction.'".to_string(),
            modern_translation: "A formidable engine of achievement fueled by acute inner tension. Restless dissatisfaction and relentless pressure that propel extraordinary breakthroughs and monumental resilience.".to_string(),
            shadow_and_caution: "Chronic internal agitation, punitive perfectionism, and recurring friction manifesting in the life sphere governed by the focal planet.".to_string(),
            tactical_remedy: "Consciously activate the empty leg directly opposite the focal planet to vent built-up pressure and cultivate restorative equilibrium.".to_string(),
        })
    } else if name_lower.contains("grandtrine") || name_lower.contains("grand-trine") {
        Some(SymbolInterpretation {
            engine: "Western Astrology".to_string(),
            symbol: "Grand Trine Aspect Configuration".to_string(),
            classical_grounding: "Hellenistic Astrology: 'Three planets in identical elemental resonance harmoniously cycling at 120 degrees, bestowing effortless organic talent and divine flow.'".to_string(),
            modern_translation: "Innate genius requiring minimal friction to operate, intuitive mastery over elemental currents, and providential luck that opens unexpected escape routes during crises.".to_string(),
            shadow_and_caution: "Complacent inertia, reliance on effortless talent without grit, and low tolerance for grinding resistance or sustained discipline.".to_string(),
            tactical_remedy: "Impose external deadlines, commercial metrics, and rigorous challenges onto your natural gifts to forge tangible, monetizable outputs.".to_string(),
        })
    } else if name_lower.contains("grandcross") || name_lower.contains("grand-cross") {
        Some(SymbolInterpretation {
            engine: "Western Astrology".to_string(),
            symbol: "Grand Cross Aspect Configuration".to_string(),
            classical_grounding: "Classical Astrology: 'Four planets locked in a continuous 90-degree cross, imposing unyielding multi-directional trials across all foundational pillars of life.'".to_string(),
            modern_translation: "The forge of titans. The crucible where steel is tempered through extreme adversity, producing individuals who leave indelible historical marks after conquering insurmountable odds.".to_string(),
            shadow_and_caution: "Sensory and physical breakdown under unrelenting multi-front pressure, acute paralysis, and destructive volatility.".to_string(),
            tactical_remedy: "Never attempt to fight on all four axes at once; ruthlessly prioritize one critical front at a time and maintain strategic detachment.".to_string(),
        })
    } else if name_lower.contains("stellium") {
        Some(SymbolInterpretation {
            engine: "Western Astrology".to_string(),
            symbol: "Stellium (High-Density Planetary Cluster)".to_string(),
            classical_grounding: "Classical Astrology: 'A concentrated fusion of three or more planets occupying a single house or zodiac sign, flooding that life arena with concentrated destiny.'".to_string(),
            modern_translation: "Hyper-specialist capability, laser-like focus, and unassailable domain dominance within a specific theater of human endeavor.".to_string(),
            shadow_and_caution: "Cognitive tunnel vision, where disruptions in that single arena threaten to destabilize the native's entire worldview.".to_string(),
            tactical_remedy: "Periodically inspect and nurture the polar opposite house to prevent catastrophic imbalance in personal well-being.".to_string(),
        })
    } else if name_lower.contains("yod") {
        Some(SymbolInterpretation {
            engine: "Western Astrology".to_string(),
            symbol: "Yod (Finger of Fate / Divine Adjustment)".to_string(),
            classical_grounding: "Esoteric Astrology: 'A sextile base of 60 degrees channeling via dual 150-degree quincunxes into a single apex planet, demanding persistent spiritual adjustment.'".to_string(),
            modern_translation: "Providential course correction. A career trajectory that undergoes subtle yet relentless recalibrations to fulfill an unmistakable spiritual or societal mission.".to_string(),
            shadow_and_caution: "Subtle perpetual irritation and fatigue from navigating unexpected detours when attempting to pursue conventional linear paths.".to_string(),
            tactical_remedy: "Relinquish the compulsion for rigid micro-control; welcome nonlinear redirections as precision tuning toward your authentic life calling.".to_string(),
        })
    } else {
        None
    }
}
