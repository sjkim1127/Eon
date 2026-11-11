use crate::dto::SymbolInterpretation;

pub fn interpret_structure(structure_name: &str) -> Option<SymbolInterpretation> {
    let name_lower = structure_name.to_lowercase();
    if name_lower.contains("pianguan")
        || name_lower.contains("sevenkilling")
        || name_lower.contains("chil-sal")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Seven Killings / PianGuan)", structure_name),
            classical_grounding: "ZiPing ZhenQuan: 'The Seven Killings is a fierce and formidable energy; only when subdued by Eating God or elevated through Resource does it transform into supreme authority, courage, and monumental executive leadership.'".to_string(),
            modern_translation: "Crisis manager, high-stakes innovator, and pioneering leader who thrives under pressure, expands territory, and executes decisive problem-solving.".to_string(),
            shadow_and_caution: "Extreme urge to control, harsh self-criticism, impatience with incompetence in others, and vulnerability to acute burnout.".to_string(),
            tactical_remedy: "Establish strategic buffer zones (intellectual retreat and rest). Avoid fighting on every single front alone; delegate operational authority and formalize strict systemic rules.".to_string(),
        })
    } else if name_lower.contains("zhengguan")
        || name_lower.contains("directofficer")
        || name_lower.contains("jeong-gwan")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Direct Officer / ZhengGuan)", structure_name),
            classical_grounding: "ZiPing ZhenQuan: 'The Direct Officer is the pinnacle of noble honor and governance; its purity must be safeguarded from clashes and Hurting Officer interference to stand as the cornerstone of institutions.'".to_string(),
            modern_translation: "Institutional elite, trusted organization builder, and corporate executive who maximizes governance, brand equity, and public credibility.".to_string(),
            shadow_and_caution: "Excessive bureaucratic rigidity, obsession with reputation and public perception, and vulnerability to inertia in rapidly shifting markets.".to_string(),
            tactical_remedy: "Maintain high structural integrity while intentionally creating psychological space to tolerate unorthodox approaches and resilient contingency planning.".to_string(),
        })
    } else if name_lower.contains("shishen")
        || name_lower.contains("eatinggod")
        || name_lower.contains("sik-sin")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Eating God / ShiShen)", structure_name),
            classical_grounding: "ZiPing ZhenQuan: 'The Eating God naturally refines and expresses personal talent to generate Wealth; it brings enduring abundance, longevity, and harmonious creative output.'".to_string(),
            modern_translation: "Specialized R&D researcher, innovative content creator, and intellectual producer who converts deep proprietary knowledge into sustainable economic value.".to_string(),
            shadow_and_caution: "Complacency, procrastination when external pressure is absent, and prioritizing self-satisfying exploration over commercialization.".to_string(),
            tactical_remedy: "Build systematic distribution pipelines that bridge research and creative breakthroughs directly into commercial business platforms.".to_string(),
        })
    } else if name_lower.contains("shangguan")
        || name_lower.contains("hurtingofficer")
        || name_lower.contains("sang-gwan")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Hurting Officer / ShangGuan)", structure_name),
            classical_grounding: "Di Tian Sui: 'The Hurting Officer possesses extraordinary brilliance and disruptive wit; when channeled into Wealth or disciplined by Resource, it produces world-shaping genius.'".to_string(),
            modern_translation: "Disruptive innovator, incisive legal/media strategist, and provocative trendsetter who dismantles legacy inefficiencies and redefines market conventions.".to_string(),
            shadow_and_caution: "Sharp tongue and pride that generate unnecessary adversaries, friction with established authority, and volatile project termination.".to_string(),
            tactical_remedy: "Secure recognized institutional credentials and proprietary IP to shield your talent; channel critical energy into commercially viable product development.".to_string(),
        })
    } else if name_lower.contains("piancai")
        || name_lower.contains("indirectwealth")
        || name_lower.contains("pyeon-jae")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Indirect Wealth / PianCai)", structure_name),
            classical_grounding: "San Ming Tong Hui: 'Indirect Wealth represents public capital and expansive trade; its scale is monumental, dominating market liquidity to amass generational enterprise.'".to_string(),
            modern_translation: "Large-scale capital allocator, venture catalyst, and market architect who connects spatial networks, strategic investments, and scalable business ventures.".to_string(),
            shadow_and_caution: "Overleveraged speculation, grandiose vanity that neglects operational unit economics, and liquidity strain from uncontrolled expansion.".to_string(),
            tactical_remedy: "Vigorously defend your operating cash flow safety margin; always partner with disciplined risk managers when executing aggressive asset investments.".to_string(),
        })
    } else if name_lower.contains("zhengcai")
        || name_lower.contains("directwealth")
        || name_lower.contains("jeong-jae")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Direct Wealth / ZhengCai)", structure_name),
            classical_grounding: "ZiPing ZhenQuan: 'Direct Wealth is the legitimate harvest of diligent labor and unwavering fidelity; it accumulates enduring wealth through precision and reliability.'".to_string(),
            modern_translation: "Data-driven financial strategist, steady cash-cow asset manager, and pragmatic COO who delivers predictable, compound growth.".to_string(),
            shadow_and_caution: "Excessive conservatism that forfeits high-convexity opportunities, extreme aversion to ambiguity, and hypersensitivity to minor temporary drawdowns.".to_string(),
            tactical_remedy: "Preserve the core stable asset base while systematically allocating 10-20% of capital into high-growth asymmetric upside opportunities.".to_string(),
        })
    } else if name_lower.contains("zhengyin")
        || name_lower.contains("directresource")
        || name_lower.contains("jeong-in")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Direct Resource / ZhengYin)", structure_name),
            classical_grounding: "ZiPing ZhenQuan: 'Resource is the benevolent maternal force that nourishes and protects; through erudition, ethics, and scholarship, it commands universal respect.'".to_string(),
            modern_translation: "Academic authority, chief strategist, and intellectual hub holding recognized credentials, foundational patents, and authoritative licenses.".to_string(),
            shadow_and_caution: "Detached idealism, paralysis by analysis, and low immunity to the transactional realities of commercial competition.".to_string(),
            tactical_remedy: "Do not leave knowledge purely theoretical; institutionalize it immediately through patents, copyrights, and commercial licensing systems.".to_string(),
        })
    } else if name_lower.contains("pianyin")
        || name_lower.contains("indirectresource")
        || name_lower.contains("pyeon-in")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Indirect Resource / PianYin)", structure_name),
            classical_grounding: "Di Tian Sui: 'Indirect Resource is the star of esoteric intuition and profound mysteries; in specialized arts and extraordinary sciences, it reaches peerless mastery.'".to_string(),
            modern_translation: "Deep-tech/AI architect, systems philosopher, and meta-cognitive strategist who surfaces irreplaceable insights in intricate, unconventional domains.".to_string(),
            shadow_and_caution: "Fickleness, withdrawal into isolation, and the habit of abandoning projects near completion due to sudden loss of interest.".to_string(),
            tactical_remedy: "Force an iterative feedback loop with real-world markets; partner with pragmatic execution and finance operators to ground your visionary insights.".to_string(),
        })
    } else if name_lower.contains("jianlu")
        || name_lower.contains("yangren")
        || name_lower.contains("bi-jian")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Self-Dominant / Lu-Ren)", structure_name),
            classical_grounding: "San Ming Tong Hui: 'Lu and Ren represent self-mastery soaring toward heaven; an indomitable pioneer who rises independently from scratch.'".to_string(),
            modern_translation: "Self-made independent founder, highly resilient entrepreneur, and determined builder who carves out an empire through sheer stamina and willpower.".to_string(),
            shadow_and_caution: "Stubborn refusal to compromise, equity disputes with co-founders, and dismissing constructive counsel from trusted advisors.".to_string(),
            tactical_remedy: "Establish transparent equity and profit-sharing frameworks upfront; retain rigorous legal and accounting auditors to guard against hubris.".to_string(),
        })
    } else {
        None
    }
}

pub fn interpret_yongshin(element: &str) -> Option<SymbolInterpretation> {
    let elem_upper = element.to_uppercase();
    if elem_upper.contains("WATER") {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: "Primary Yongshin: Water (Fluid Intelligence & Cooling Balance)".to_string(),
            classical_grounding: "Qiong Tong Bao Jian: 'Water is the primal source of wisdom; it circulates through all things and provides the indispensable cooling brake against raging fire.'".to_string(),
            modern_translation: "Strategic flexibility, data-driven synthesis, deliberate calm, global distribution networks, and patient tactical timing.".to_string(),
            shadow_and_caution: "Overheated impatience, impulsive financial deployment, and exhaustion caused by unbraked momentum.".to_string(),
            tactical_remedy: "Enforce a mandatory 24-hour cooling reflection period before major commitments; verify claims with hard metrics and maintain restorative routines.".to_string(),
        })
    } else if elem_upper.contains("WOOD") {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: "Primary Yongshin: Wood (Growth Momentum & Pioneering Vision)".to_string(),
            classical_grounding: "Di Tian Sui: 'Wood is the onset of vitality and ascending expansion; it initiates brand-new frontiers and proclaims forward momentum.'".to_string(),
            modern_translation: "Launching new business initiatives, conceptualizing creative proposals, talent mentorship, and forging foundational alliances.".to_string(),
            shadow_and_caution: "Directionless overexpansion, emotional isolation, and lethargy stemming from lack of a compelling long-range vision.".to_string(),
            tactical_remedy: "Leverage morning hours for high-focus work; initiate structured learning sprints and maintain regular aerobic physical grounding.".to_string(),
        })
    } else if elem_upper.contains("FIRE") {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: "Primary Yongshin: Fire (Illumination, Visibility & Passionate Drive)".to_string(),
            classical_grounding: "Qiong Tong Bao Jian: 'Fire represents radiance and warmth; it illuminates obscurity to reveal hidden value to the world and ignites collective enthusiasm.'".to_string(),
            modern_translation: "Visual branding, public relations, keynote communication, market evangelism, and generating contagious commercial momentum.".to_string(),
            shadow_and_caution: "Suppressed melancholy, excessive introversion, and keeping core value hidden behind closed doors.".to_string(),
            tactical_remedy: "Expose your work and ideas proactively to external audiences; engage in visible thought leadership and maintain vibrant physical environments.".to_string(),
        })
    } else if elem_upper.contains("EARTH") {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: "Primary Yongshin: Earth (Stability, Infrastructure & Trust)".to_string(),
            classical_grounding: "Di Tian Sui: 'Earth is the foundational mother and mediator of all elements; it absorbs abrupt seasonal volatility to bestow enduring stability.'".to_string(),
            modern_translation: "Real asset acquisition, platform and infrastructure engineering, trusted mediation, and robust contractual structuring.".to_string(),
            shadow_and_caution: "Emotional volatility, erratic pivots, and ungrounded anxiety without structural bedrock.".to_string(),
            tactical_remedy: "Institutionalize steady daily operating rhythms; maintain a balanced tangible asset portfolio and reinforce reliable commitments.".to_string(),
        })
    } else if elem_upper.contains("METAL") {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: "Primary Yongshin: Metal (Decisive Precision & Structural Pruning)".to_string(),
            classical_grounding: "Qiong Tong Bao Jian: 'Metal embodies harvest and unsparing decision; it prunes unproductive branches like autumn frost to harvest pure grain.'".to_string(),
            modern_translation: "Restructuring, core competency concentration, decisive contract termination, financial precision, and elite systems engineering.".to_string(),
            shadow_and_caution: "Indecisive lingering over past commitments, failure to prune toxic relationships, and wasteful resource diffusion.".to_string(),
            tactical_remedy: "Relentlessly cut out low-yield obligations (radical minimalism); enforce rigorous quantitative accountability and celebrate decisive closure.".to_string(),
        })
    } else {
        None
    }
}
