use crate::dto::SymbolInterpretation;

pub fn interpret_type_strategy(chart_type: &str, strategy: &str) -> Option<SymbolInterpretation> {
    let type_lower = chart_type.to_lowercase();
    if type_lower.contains("manifesting generator") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: format!("Type: Manifesting Generator | Strategy: {}", strategy),
            classical_grounding: "The Definitive Book of Human Design: 'Equipped with direct motor-to-throat connectivity and the boundless stamina of the Sacral center; the multi-passionate master builder capable of hyper-speed execution.'".to_string(),
            modern_translation: "Hyper-speed multi-threaded executor, agile visionary founder, and relentless builder who skips non-essential steps and turns ideas into working systems with breathtaking speed.".to_string(),
            shadow_and_caution: "Impatience with slower collaborators, skipping vital architectural steps that cause bugs later, and acute frustration when blocked.".to_string(),
            tactical_remedy: "Wait for external clarity to respond (gut response) rather than initiating blindly from mental pressure; inform impacted stakeholders before executing high-speed pivots.".to_string(),
        })
    } else if type_lower.contains("projector") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: format!("Type: Projector | Strategy: {}", strategy),
            classical_grounding: "The Definitive Book of Human Design: 'A non-energy type and the modern guide of humanity; shining not by running motors directly, but by orchestrating and guiding the energy of others.'".to_string(),
            modern_translation: "Systems orchestrator, master consultant, and diagnostic guide who sees the latent potential in people and architects optimal operational leverage.".to_string(),
            shadow_and_caution: "Bitterness from unsolicited advice being rejected, chronic exhaustion from attempting to keep pace with sacral motor types.".to_string(),
            tactical_remedy: "Never initiate or force unsolicited counsel; hone your proprietary craft in quiet mastery and intervene only upon formal recognition and invitation.".to_string(),
        })
    } else if type_lower.contains("generator") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: format!("Type: Generator | Strategy: {}", strategy),
            classical_grounding: "Human Design: 'Possessing the tireless lifeforce energy of the Sacral center; the great builder and master craftsman of human civilization.'".to_string(),
            modern_translation: "Enduring master builder and deep craft artisan who creates monumental tangible achievements when immersed in work that resonates with their lifeforce.".to_string(),
            shadow_and_caution: "Deep frustration and burnout when trapped in obligations imposed by mental conditioning rather than genuine gut engagement.".to_string(),
            tactical_remedy: "Never make major life commitments from mental rationalization; move only in response to visceral sacral acoustic sounds and physical gut resonance.".to_string(),
        })
    } else if type_lower.contains("manifestor") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: format!("Type: Manifestor | Strategy: {}", strategy),
            classical_grounding: "Human Design: 'The sole pure initiator in human history; initiating new paradigms and launching brand-new historical trajectories without needing permission.'".to_string(),
            modern_translation: "Autonomous pioneer, first-mover founder, and sovereign innovator who ignites new frontiers independently.".to_string(),
            shadow_and_caution: "Surrounding resistance, alienation, and deep anger triggered by acting unilaterally without warning those in your blast radius.".to_string(),
            tactical_remedy: "Prior to initiating major actions, proactively inform everyone who will be impacted; informing transforms resistance into peaceful clearance.".to_string(),
        })
    } else if type_lower.contains("reflector") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: format!("Type: Reflector | Strategy: {}", strategy),
            classical_grounding: "Human Design: 'A lunar being with all nine centers undefined; the pristine mirror reflecting the holistic health and spiritual truth of the community.'".to_string(),
            modern_translation: "Chief diagnostician of systemic health, cultural barometer, and transcendent evaluator of community integrity.".to_string(),
            shadow_and_caution: "Devastating disappointment and physical contamination when trapped in toxic, unaligned environments.".to_string(),
            tactical_remedy: "Allow a full 28.5-day lunar transit cycle before executing major life decisions; be uncompromisingly selective about your physical and human environment.".to_string(),
        })
    } else {
        None
    }
}

pub fn interpret_profile(profile: &str) -> Option<SymbolInterpretation> {
    if profile.contains("6/2") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: "Profile: 6/2 (Role Model / Hermit)".to_string(),
            classical_grounding: "Human Design Profiles: 'The tripartite heroic journey: Phase 1 (until ~30) trial-and-error exploration, Phase 2 aloof observation on the roof, and Phase 3 descending as a living role model.'".to_string(),
            modern_translation: "A living standard-bearer whose youthful trials mature into unshakeable systemic wisdom, inspiring others from an elevated, objective vantage point.".to_string(),
            shadow_and_caution: "Aloof cynicism toward an imperfect world, and withdrawing into the hermit sanctuary so deeply that you refuse to share your gifts.".to_string(),
            tactical_remedy: "Honor the 2nd line hermit sanctuary to replenish energy, but step forward courageously when authentic recognition arrives to demonstrate exemplary leadership.".to_string(),
        })
    } else if profile.contains("6/3") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: "Profile: 6/3 (Role Model / Martyr)".to_string(),
            classical_grounding: "Human Design Profiles: 'The ultimate synthesis of visionary elevation and empirical experimentation; combining the high-altitude perspective of the 6 with the perpetual trial-and-error resilience of the 3.'".to_string(),
            modern_translation: "Resilient visionary architect and empirical pioneer who stress-tests real-world systems, discards what fails, and models authentic, battle-tested truth.".to_string(),
            shadow_and_caution: "Impatience with imperfect systems, self-blame during messy experimental iterations, and feeling restless in conventional static environments.".to_string(),
            tactical_remedy: "Reframe collisions and bumps as essential empirical telemetry; you are designed to discover what does not work so that civilization can build on what does.".to_string(),
        })
    } else if profile.contains("1/3") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: "Profile: 1/3 (Investigator / Martyr)".to_string(),
            classical_grounding: "Human Design Profiles: 'Rigorous investigative foundation-building paired with unyielding trial-and-error to filter out everything that does not work.'".to_string(),
            modern_translation: "Relentless fact-checker, empirical security auditor, and resilient researcher who discovers system vulnerabilities before anyone else.".to_string(),
            shadow_and_caution: "Paralysis from fear of incomplete foundations, research addiction as a shield against execution, and self-blame over iterative mistakes.".to_string(),
            tactical_remedy: "Celebrate failed tests as invaluable data points that narrow down the absolute truth; move decisively once foundational security is established.".to_string(),
        })
    } else if profile.contains("3/5") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: "Profile: 3/5 (Martyr / Heretic)".to_string(),
            classical_grounding: "Human Design Profiles: 'The seasoned crisis problem-solver whose experiential battle scars deliver unconventional, radical solutions to public crises.'".to_string(),
            modern_translation: "Frontline troubleshooter, pragmatic innovator, and magnetic leader who shatters conventional dogma to rescue organizations from systemic failure.".to_string(),
            shadow_and_caution: "Public backlash and reputational damage when failing to fulfill unrealistic external projections, leading to abrupt relationship severing.".to_string(),
            tactical_remedy: "Strictly bound expectations upfront to realistic deliverables; present clean, definitive results and exit before external projections sour.".to_string(),
        })
    } else {
        None
    }
}
