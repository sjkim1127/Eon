use crate::dto::SymbolInterpretation;

pub fn interpret_yoga(yoga_name: &str) -> Option<SymbolInterpretation> {
    let name_lower = yoga_name.to_lowercase();
    if name_lower.contains("gaja") || name_lower.contains("kesari") {
        Some(SymbolInterpretation {
            engine: "Vedic Astrology (Jyotish)".to_string(),
            symbol: "Gaja Kesari Yoga (Elephant-Lion Royal Union)".to_string(),
            classical_grounding: "Brihat Parashara Hora Shastra (BPHS): 'When Jupiter occupies a Kendra from the Moon, the native overcomes all obstacles, enjoys virtuous renown, and commands enduring public respect and regal status.'".to_string(),
            modern_translation: "Dignified intellect, expansive public trust, and magnetic leadership that naturally draws authoritative mentors, sponsors, and benefactors during critical inflection points.".to_string(),
            shadow_and_caution: "Moral superiority, patronizing tendency to lecture others, and complacency lingering in advisory roles rather than decisive execution.".to_string(),
            tactical_remedy: "Do not remain purely an advisor; institutionalize your wisdom into educational platforms, publications, and commercial strategic advisory systems.".to_string(),
        })
    } else if name_lower.contains("raja")
        || name_lower.contains("l1")
        || name_lower.contains("l9")
        || name_lower.contains("l10")
    {
        Some(SymbolInterpretation {
            engine: "Vedic Astrology (Jyotish)".to_string(),
            symbol: format!("Raja Yoga (Royal Authority & Power) — {}", yoga_name),
            classical_grounding: "BPHS: 'When lords of Kendra houses (pillars of action) and Trikona houses (seats of divine grace) align, the native rises to supreme executive authority, prestige, and worldly command.'".to_string(),
            modern_translation: "C-suite executive, trusted operational director, and leader who wields delegated authority, elevated social status, and institutional command.".to_string(),
            shadow_and_caution: "Hubris, burnout from heavy administrative burdens, and generating envious political opposition within organizations.".to_string(),
            tactical_remedy: "Never hoard authority for self-aggrandizement; practice inclusive governance by distributing gains and empowered autonomy to core team members.".to_string(),
        })
    } else if name_lower.contains("dhana") {
        Some(SymbolInterpretation {
            engine: "Vedic Astrology (Jyotish)".to_string(),
            symbol: format!("Dhana Yoga (Wealth Magnetism & Accumulation) — {}", yoga_name),
            classical_grounding: "BPHS: 'When the lords of wealth houses (2nd and 11th) connect with the auspicious dharma houses (1st, 5th, and 9th), prosperity flows perpetually like an inexhaustible spring.'".to_string(),
            modern_translation: "High asset-scaling capability, where domain authority and public reputation directly translate into lucrative cash flow and compound investment returns.".to_string(),
            shadow_and_caution: "Obsession with material metrics, sudden capital drain from poor oversight, and evaluating human relationships purely on transactional terms.".to_string(),
            tactical_remedy: "Lock liquid windfalls into high-grade tangible assets or index foundations to build permanent, uncorrelated passive income infrastructure.".to_string(),
        })
    } else if name_lower.contains("budhaditya") {
        Some(SymbolInterpretation {
            engine: "Vedic Astrology (Jyotish)".to_string(),
            symbol: "Budhaditya Yoga (Mercury-Sun Intellectual Brilliance)".to_string(),
            classical_grounding: "Phaladeepika: 'When the Sun and Mercury conjoin in dignity, the native is exceptionally alert, sharp-witted, and widely celebrated in scholarship, eloquence, and statecraft.'".to_string(),
            modern_translation: "Incisive analytical acumen, persuasive data communication, and the rare ability to synthesize intricate technical systems into lucid, actionable mental models.".to_string(),
            shadow_and_caution: "Nervous exhaustion, analysis paralysis from over-rationalization, and friction caused by intellectual condescension.".to_string(),
            tactical_remedy: "Implement analog detox periods to cool down the nervous system; complement logic with intuitive emotional intelligence.".to_string(),
        })
    } else if name_lower.contains("sunapha")
        || name_lower.contains("anapha")
        || name_lower.contains("durdhara")
    {
        Some(SymbolInterpretation {
            engine: "Vedic Astrology (Jyotish)".to_string(),
            symbol: format!("Lunar Wealth & Self-Sufficiency Yoga — {}", yoga_name),
            classical_grounding: "BPHS: 'When benefics flank the Moon in adjacent houses, the native establishes independent prosperity through self-directed discipline and honorable character.'".to_string(),
            modern_translation: "Self-made wealth builder whose emotional equilibrium directly compounds tangible assets; fiercely independent financial resilience.".to_string(),
            shadow_and_caution: "Isolationism, reluctance to trust external collaborators, and carrying all operational weight single-handedly.".to_string(),
            tactical_remedy: "Incrementally form strategic, vetted partnerships to leverage collaborative division of labor.".to_string(),
        })
    } else {
        None
    }
}

pub fn interpret_nakshatra(nakshatra_info: &str) -> Option<SymbolInterpretation> {
    Some(SymbolInterpretation {
        engine: "Vedic Astrology (Jyotish)".to_string(),
        symbol: format!("Moon Nakshatra: {}", nakshatra_info),
        classical_grounding: "Taittiriya Brahmana: 'The Nakshatra is the celestial mansion of the soul, governing instinctive subconscious impulses, emotional sanctuary, and primordial memory.'".to_string(),
        modern_translation: "Subconscious behavioral drive, optimal psychological environment for flow state, and the intrinsic sensory frequency through which the soul interacts with the world.".to_string(),
        shadow_and_caution: "Defensive withdrawal and interpersonal rupture when the core emotional needs of the Nakshatra are starved.".to_string(),
        tactical_remedy: "Deliberately structure your daily routine around the innate environmental demands of your Nakshatra (autonomy, deep research, dynamic expression, or sanctuary).".to_string(),
    })
}
