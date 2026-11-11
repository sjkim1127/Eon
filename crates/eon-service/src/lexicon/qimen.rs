use crate::dto::SymbolInterpretation;
use eon_qimen::core::{Door, Star};

pub fn interpret_door(door: Door) -> Option<SymbolInterpretation> {
    match door {
        Door::Sheng => Some(SymbolInterpretation {
            engine: "QiMen DunJia".to_string(),
            symbol: "Envoy Door: Sheng Men (Life Door)".to_string(),
            classical_grounding: "QiMen DunJia Secret Treasury: 'Sheng Men belongs to Earth and embodies the germination and flourishing of all creation; supreme for wealth generation, venture creation, and high-value acquisitions.'".to_string(),
            modern_translation: "Generating enterprise momentum, unlocking new revenue pipelines, capital allocation, and enterprise valuation scaling.".to_string(),
            shadow_and_caution: "Complacent optimism, overinvesting without liquidity safeguards, and neglect of unit economics.".to_string(),
            tactical_remedy: "Act without hesitation when growth windows open, while enforcing strict minimum operating cash reserves.".to_string(),
        }),
        Door::Kai => Some(SymbolInterpretation {
            engine: "QiMen DunJia".to_string(),
            symbol: "Envoy Door: Kai Men (Open Door)".to_string(),
            classical_grounding: "Yan Bo Diao Sou Ge: 'Kai Men belongs to Metal and signifies legitimate institutional authority and executive promotion; auspicious for enterprise launches and official governance.'".to_string(),
            modern_translation: "Institutional integration, brand launch, strategic joint ventures with public/enterprise titans, and legitimate leadership authority.".to_string(),
            shadow_and_caution: "Excessive bureaucratic formalism and slow decision cycles due to rigid compliance obsession.".to_string(),
            tactical_remedy: "Submit authoritative proposals through official institutional channels with complete transparency to secure official validation.".to_string(),
        }),
        Door::Xiu => Some(SymbolInterpretation {
            engine: "QiMen DunJia".to_string(),
            symbol: "Envoy Door: Xiu Men (Rest Door)".to_string(),
            classical_grounding: "QiMen DunJia Secret Treasury: 'Xiu Men belongs to Water and represents deep restoration, harmony, and alliance; best for convening trusted benefactors, reconciliation, and tactical rest.'".to_string(),
            modern_translation: "Strategic regrouping, trusted co-founder alliances, onboarding key advisors and patrons, and psychological replenishment.".to_string(),
            shadow_and_caution: "Loss of combat edge, excessive lethargy, and missing hard project deadlines.".to_string(),
            tactical_remedy: "Pause unilateral frontal breakthroughs; leverage warm networking and restorative retreat to recharge your strategic reservoir.".to_string(),
        }),
        Door::Shang => Some(SymbolInterpretation {
            engine: "QiMen DunJia".to_string(),
            symbol: "Envoy Door: Shang Men (Harm Door)".to_string(),
            classical_grounding: "QiMen DunJia Secret Treasury: 'Shang Men belongs to Wood and governs offensive hunting, debt recovery, and assertive enforcement, yet carries sharp risk of self-injury and legal strife.'".to_string(),
            modern_translation: "Aggressive debt collection, forensic auditing, radical corporate restructuring, and fierce territorial competition.".to_string(),
            shadow_and_caution: "Emotional escalation triggering costly legal warfare, and permanent rupture of key human networks.".to_string(),
            tactical_remedy: "Execute aggressive enforcement strictly within established legal boundaries; ruthlessly forbid personal vendettas.".to_string(),
        }),
        Door::Du => Some(SymbolInterpretation {
            engine: "QiMen DunJia".to_string(),
            symbol: "Envoy Door: Du Men (Delusion Door)".to_string(),
            classical_grounding: "Yan Bo Diao Sou Ge: 'Du Men belongs to Wood and closes off external channels; suited for concealment, stealth preparation, and confidential deep research.'".to_string(),
            modern_translation: "Stealth-mode R&D, trade secret security, core infrastructure hardening, and minimizing external public exposure.".to_string(),
            shadow_and_caution: "Siloed isolation from the market, lack of user feedback, and building without market validation.".to_string(),
            tactical_remedy: "Recognize that the current phase is for proprietary technical perfection, not public marketing; perfect the algorithm in silence.".to_string(),
        }),
        Door::Jing => Some(SymbolInterpretation {
            engine: "QiMen DunJia".to_string(),
            symbol: "Envoy Door: Jing Men (Scenery Door)".to_string(),
            classical_grounding: "QiMen DunJia Secret Treasury: 'Jing Men belongs to Fire and represents brilliant culture, documents, examination honors, and contractual celebrations.'".to_string(),
            modern_translation: "Major PR and media campaigns, keynote addresses, patent/copyright licensing agreements, and high-impact visual branding.".to_string(),
            shadow_and_caution: "Superficial flash without product substance, public controversy, and exposure to deceptive contractual clauses.".to_string(),
            tactical_remedy: "Ensure technical product excellence matches external marketing brilliance; scrutinize contractual fine print microscopically.".to_string(),
        }),
        Door::Si => Some(SymbolInterpretation {
            engine: "QiMen DunJia".to_string(),
            symbol: "Envoy Door: Si Men (Death Door)".to_string(),
            classical_grounding: "QiMen DunJia: 'Si Men belongs to Earth and marks the final harvest where seeds are buried; suitable for real estate acquisitions, audits, and terminating obsolete legacies.'".to_string(),
            modern_translation: "Shutting down zombie projects, ruthlessly deprecating legacy systems, and acquiring tangible distressed assets.".to_string(),
            shadow_and_caution: "Severe energy stagnation, depression, and attempting to launch premature new initiatives in unready soil.".to_string(),
            tactical_remedy: "Do not attempt to sow new crops in winter; provide an honorable funeral to defunct legacies and purify the operational soil.".to_string(),
        }),
        Door::Jing2 => Some(SymbolInterpretation {
            engine: "QiMen DunJia".to_string(),
            symbol: "Envoy Door: Jing Men (Fear Door)".to_string(),
            classical_grounding: "Yan Bo Diao Sou Ge: 'Jing Men belongs to Metal and governs shock, litigation, and courtroom arguments; valuable for detecting latent hazards and legal defense.'".to_string(),
            modern_translation: "Compliance auditing, proactive crisis management (Crisis PR), and assertive legal defense through counsel.".to_string(),
            shadow_and_caution: "Baseless panic and irrational terror triggered by malicious rumors, resulting in loss of equanimity.".to_string(),
            tactical_remedy: "Refuse to react emotionally to threats; assemble cold, indisputable factual documentation and deliver a definitive legal rebuttal.".to_string(),
        }),
    }
}

pub fn interpret_star(star: Star) -> Option<SymbolInterpretation> {
    match star {
        Star::Fu => Some(SymbolInterpretation {
            engine: "QiMen DunJia".to_string(),
            symbol: "Chief Star: Tian Fu (Assistant Star)".to_string(),
            classical_grounding: "QiMen DunJia Secret Treasury: 'Tian Fu carries the refined essence of academic distinction; the supreme star for scholarly research, education, and cultivating talent.'".to_string(),
            modern_translation: "Academic research, intellectual property (IP) architecture, executive coaching, and high-level strategic advisory.".to_string(),
            shadow_and_caution: "Detached theoretical idealism that neglects commercial monetization and real-world execution.".to_string(),
            tactical_remedy: "Package deep academic insights into structured digital content and commercial enterprise frameworks.".to_string(),
        }),
        Star::Qin => Some(SymbolInterpretation {
            engine: "QiMen DunJia".to_string(),
            symbol: "Chief Star: Tian Qin (Central Sovereign Star)".to_string(),
            classical_grounding: "QiMen DunJia: 'Tian Qin occupies the central 5th Palace, commanding all eight directions with absolute neutrality and sovereign authority.'".to_string(),
            modern_translation: "Mission control command, central platform hub, and supreme governance architect who mediates multi-stakeholder conflicts.".to_string(),
            shadow_and_caution: "Centralized operational bottlenecking and single-point-of-failure hero syndrome.".to_string(),
            tactical_remedy: "Establish clear constitutional governance principles and immediately distribute execution authority to edge nodes.".to_string(),
        }),
        Star::Xin => Some(SymbolInterpretation {
            engine: "QiMen DunJia".to_string(),
            symbol: "Chief Star: Tian Xin (Strategist Star)".to_string(),
            classical_grounding: "QiMen DunJia: 'Tian Xin governs deep wisdom, medicine, and military strategy; highly auspicious for resolving structural illnesses and grand campaigns.'".to_string(),
            modern_translation: "Strategic management consulting, healthcare and biotech architecture, and diagnosing deep bottlenecks in complex enterprise systems.".to_string(),
            shadow_and_caution: "Clinical over-dissection of personal faults that erodes human trust and camaraderie.".to_string(),
            tactical_remedy: "Pair diagnostic critique with actionable, turnkey healing frameworks that leave partners empowered.".to_string(),
        }),
        Star::Chong => Some(SymbolInterpretation {
            engine: "QiMen DunJia".to_string(),
            symbol: "Chief Star: Tian Chong (The Impetuous Pioneer Star)".to_string(),
            classical_grounding: "QiMen DunJia: 'Tian Chong belongs to Wood and symbolizes thunderous assault and fearless momentum; supreme for launching bold attacks, expeditions, and breaking through enemy lines.'".to_string(),
            modern_translation: "Pioneering execution speed, bold market entry, blitzscaling, and shattering competitive inertia with overwhelming tactical force.".to_string(),
            shadow_and_caution: "Reckless impulsiveness, failing to anticipate structural traps, and burning out ammunition before the siege is won.".to_string(),
            tactical_remedy: "Pair lightning-fast blitzes with reliable rear-guard logistics; look before you leap into irreversible commitments.".to_string(),
        }),
        _ => None,
    }
}
