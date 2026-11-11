use crate::dto::SymbolInterpretation;

pub fn interpret_major_star(star_name: &str) -> Option<SymbolInterpretation> {
    let name_lower = star_name.to_lowercase();
    if name_lower.contains("pojun") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: PoJun (The Pioneer & Breaker)".to_string(),
            classical_grounding: "Zi Wei Dou Shu Complete Treatise: 'PoJun is the 7th star of the Northern Dipper, the god of consumption and exploration; it dismantles the obsolete to construct revolutionary orders.'".to_string(),
            modern_translation: "Disruptive innovator, paradigm-shifting game changer, and relentless pioneer who tears down stagnant legacy structures to build 0-to-1 empires.".to_string(),
            shadow_and_caution: "Impatience with routine, destructive rebellion that prematurely dismantles working systems, and volatile resource consumption.".to_string(),
            tactical_remedy: "Channel disruptive energy outwards against real market bottlenecks and engineering challenges, never against your own foundation or personal relationships.".to_string(),
        })
    } else if name_lower.contains("qisha") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: QiSha (The General of Sovereign Command)".to_string(),
            classical_grounding: "Zi Wei Dou Shu Complete Treatise: 'QiSha is the 6th star of the Southern Dipper, embodying supreme military command; it makes independent decisions and spearheads unyielding advances.'".to_string(),
            modern_translation: "Authoritative executive commander, decisive CEO, and battlefield strategist who shoulders solitary accountability and breaks through high-friction frontlines.".to_string(),
            shadow_and_caution: "Autocratic isolation, zero tolerance for perceived failure, and burning out team morale through unyielding intensity.".to_string(),
            tactical_remedy: "Cultivate proactive emotional resonance; institutionalize open channels for candid feedback and delegate operational autonomy to trusted lieutenants.".to_string(),
        })
    } else if name_lower.contains("ziwei") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: ZiWei (The Emperor & Sovereign Star)".to_string(),
            classical_grounding: "Zi Wei Dou Shu Complete Treatise: 'ZiWei is the supreme ruler of the Northern Dipper, commanding the myriad stars and embodying noble majesty, statesmanship, and overarching governance.'".to_string(),
            modern_translation: "Pinnacle authority, dignified brand leader, and holistic statesman who orchestrates complex ecosystems and casts an unshakeable institutional vision.".to_string(),
            shadow_and_caution: "Vulnerability to flatterers, sacrificing concrete commercial substance for aristocratic pride, and neglecting operational detail.".to_string(),
            tactical_remedy: "Surround yourself with pragmatic operational counselors (such as ZuoFu and YouBi) and prioritize operational grounding above prestige.".to_string(),
        })
    } else if name_lower.contains("wuqu") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: WuQu (The Financial Sovereign & Diamond Precision)".to_string(),
            classical_grounding: "Zi Wei Dou Shu Complete Treatise: 'WuQu is the master of wealth and enterprise; resolute, substantive, and possessing the unyielding precision of cold metal.'".to_string(),
            modern_translation: "Financial architect, data-driven quantitative operator, and pragmatic asset allocator who demands measurable unit economics and relentless capital efficiency.".to_string(),
            shadow_and_caution: "Transactional callousness, interpersonal isolation caused by excessive calculation, and emotional austerity.".to_string(),
            tactical_remedy: "Intentionally cultivate warm human connection and emotional empathy alongside quantitative rigor.".to_string(),
        })
    } else if name_lower.contains("taiyang") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: TaiYang (The Celestial Sun & Benevolent Radiance)".to_string(),
            classical_grounding: "Zi Wei Dou Shu Complete Treatise: 'TaiYang rules the zenith of the heavens; it governs impartial justice, public renown, and bestows illumination upon all living beings.'".to_string(),
            modern_translation: "Global visionary, public speaker, movement catalyst, and international statesman who champions civic advancement and open ecosystems.".to_string(),
            shadow_and_caution: "Burnout from exhausting yourself on behalf of ungrateful parties, performative vanity, and overextending into external disputes.".to_string(),
            tactical_remedy: "Fill your own reservoir and defend unit economics before bestowing philanthropic charity; establish airtight contracts before public evangelism.".to_string(),
        })
    } else if name_lower.contains("taiyin") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: TaiYin (The Lunar Empress & Receptive Abundance)".to_string(),
            classical_grounding: "Zi Wei Dou Shu Complete Treatise: 'TaiYin is the gentle moonlight; it symbolizes private wealth, aesthetic grace, real estate, and accumulated capital reserves.'".to_string(),
            modern_translation: "Meticulous strategist, real asset custodian, and soft-power architect gifted with refined aesthetic intuition and psychological insight.".to_string(),
            shadow_and_caution: "Melancholic mood swings, passive-aggressive withdrawal when hurt, and avoiding direct confrontation at the cost of personal boundaries.".to_string(),
            tactical_remedy: "Channel emotional sensitivity into stable tangible investments and creative architecture; spend active hours in direct sunlight.".to_string(),
        })
    } else if name_lower.contains("tianfu") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: TianFu (The Celestial Treasury & Sovereign Vault)".to_string(),
            classical_grounding: "Zi Wei Dou Shu Complete Treatise: 'TianFu is the master of the Southern Dipper and the heavenly vault; conservative, stable, and guarding generational wealth.'".to_string(),
            modern_translation: "Chief Financial Officer (CFO), treasury custodian, and enterprise risk architect who ensures unassailable balance sheet defense during systemic crises.".to_string(),
            shadow_and_caution: "Risk aversion bordering on stagnation, reluctance to venture outside comfort zones, and missing generational transitions.".to_string(),
            tactical_remedy: "Carve out an intentional venture sandbox within your reserves to allocate a disciplined portion of capital into asymmetric innovation.".to_string(),
        })
    } else if name_lower.contains("tanlang") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: TanLang (The Alchemical Charisma & Opportunity Seeker)".to_string(),
            classical_grounding: "Zi Wei Dou Shu Complete Treatise: 'TanLang is the star of desire, social charm, and spiritual alchemy; versatile, magnetic, and gifted in seizing fleeting opportunity.'".to_string(),
            modern_translation: "Magnetic networker, trend architect, and lifestyle/entertainment powerhouse who leverages social capital and psychological nuance.".to_string(),
            shadow_and_caution: "Fragmented focus from chasing too many shiny opportunities, financial leakage through hedonistic indulgence, and superficial alliances.".to_string(),
            tactical_remedy: "Channel your magnetism into a single, high-leverage commercial thesis; anchor desires through meditative focus and spiritual discipline.".to_string(),
        })
    } else if name_lower.contains("tianliang") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: TianLiang (The Sage Guardian & Strategic Problem-Solver)".to_string(),
            classical_grounding: "Zi Wei Dou Shu Complete Treatise: 'TianLiang is the elder statesman and guardian of longevity; it turns adversity into triumph, resolves impossible disputes, and governs auditing, ethics, and judicial clarity.'".to_string(),
            modern_translation: "Chief advisory counsel, institutional auditor, and crisis mediator who diagnoses systemic liabilities, restores governance, and transforms organizational crises into enduring competitive moats.".to_string(),
            shadow_and_caution: "Pedantic lecturing, unsolicited moralizing, and inviting adversity simply to experience the gratification of resolving it.".to_string(),
            tactical_remedy: "Intervene only when formally invited and compensated; focus your profound diagnostic intellect on systemic engineering rather than personal disputes.".to_string(),
        })
    } else {
        None
    }
}
