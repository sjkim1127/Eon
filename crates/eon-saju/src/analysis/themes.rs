use crate::analysis::Analyzable;
use crate::core::config::AnalysisConfig;
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ThemedAnalysis {
    pub career: CareerAnalysis,
    pub wealth: WealthAnalysis,
    pub romance: RomanceAnalysis,
    pub health: HealthAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CareerAnalysis {
    pub score: u32, // 0-100
    pub keywords: Vec<String>,
    pub summary: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WealthAnalysis {
    pub score: u32,
    pub keywords: Vec<String>,
    pub summary: String,
    pub flow: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RomanceAnalysis {
    pub score: u32,
    pub keywords: Vec<String>,
    pub summary: String,
    pub advice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HealthAnalysis {
    pub score: u32,
    pub keywords: Vec<String>,
    pub summary: String,
    pub vulnerable_organs: Vec<String>,
}

impl Analyzable for ThemedAnalysis {
    type Output = ThemedAnalysis;

    fn analyze(pillars: &FourPillars, config: &AnalysisConfig) -> Self::Output {
        let strength = pillars.strength_with_config(config);
        let deuk_se = &strength.deuk_se;

        // 1. Career
        let career_score = ((60
            + (deuk_se.guanxing_count as u32 * 8)
            + (deuk_se.shishang_count as u32 * 6)
            + (deuk_se.bijie_count as u32 * 4))
            .min(95))
        .max(50);

        let career_keywords = if deuk_se.guanxing_count >= 2 {
            vec![
                "Organizational Leadership".to_string(),
                "Systematic Execution".to_string(),
            ]
        } else if deuk_se.shishang_count >= 2 {
            vec![
                "Creative Expertise".to_string(),
                "Engineering & Product Vision".to_string(),
            ]
        } else if deuk_se.bijie_count >= 2 {
            vec![
                "Independent Operator".to_string(),
                "Autonomous Initiative".to_string(),
            ]
        } else {
            vec![
                "Collaborative Synergy".to_string(),
                "Strategic Planning".to_string(),
            ]
        };

        let career_summary = if deuk_se.guanxing_count >= 2 {
            "Thrives by cultivating trust and leadership within structured institutions and high-trust hierarchies.".to_string()
        } else if deuk_se.shishang_count >= 2 {
            "Excels when recognized for distinctive technical mastery, inventive design, or proprietary intellectual property.".to_string()
        } else {
            "Generates peak performance in environments granting high autonomy, self-direction, and independent ownership.".to_string()
        };

        let career_recommendation = if deuk_se.guanxing_count >= 2 {
            "Executive leadership, institutional management, enterprise architecture, or legal/regulatory governance.".to_string()
        } else if deuk_se.shishang_count >= 2 {
            "R&D engineering, architecture, content creation, specialized technical craft, or creative entrepreneurship.".to_string()
        } else {
            "Frontier tech innovation, founder-led startups, boutique advisory, or independent execution tracks."
                .to_string()
        };

        // 2. Wealth
        let wealth_score = ((50
            + (deuk_se.caisheng_count as u32 * 12)
            + (deuk_se.shishang_count as u32 * 6)
            + (deuk_se.yinxing_count as u32 * 4))
            .min(98))
        .max(45);

        let wealth_keywords = if deuk_se.caisheng_count >= 2 {
            vec![
                "Substantial Asset Base".to_string(),
                "Capital Allocation".to_string(),
            ]
        } else if deuk_se.shishang_count >= 1 {
            vec![
                "Output-to-Wealth Pipeline".to_string(),
                "Skill-Driven Monetization".to_string(),
            ]
        } else {
            vec![
                "Systematic Accumulation".to_string(),
                "Prudent Capital Protection".to_string(),
            ]
        };

        let wealth_summary = if deuk_se.caisheng_count >= 2 {
            "Possesses deep commercial affinity and strong acumen for compounding substantial real capital.".to_string()
        } else if deuk_se.shishang_count >= 1 {
            "Converts creative output and intellectual talent directly into high-leverage revenue streams.".to_string()
        } else {
            "Prioritizes disciplined savings, structural risk containment, and steady asset growth over high-risk speculation.".to_string()
        };

        let wealth_flow = if deuk_se.caisheng_count >= 2 {
            "Builds enduring net worth through diversified real estate, equities, and private equity investments.".to_string()
        } else {
            "Achieves optimal returns through patient expansion of high-conviction, scalable income pipelines."
                .to_string()
        };

        // 3. Romance
        let romance_score = ((55
            + (deuk_se.caisheng_count as u32 * 7)
            + (deuk_se.guanxing_count as u32 * 7)
            + (deuk_se.yinxing_count as u32 * 5))
            .min(95))
        .max(50);

        let romance_keywords = if deuk_se.caisheng_count >= 1 || deuk_se.guanxing_count >= 1 {
            vec![
                "Active Charm".to_string(),
                "Dynamic Partnership Resonance".to_string(),
            ]
        } else if deuk_se.yinxing_count >= 2 {
            vec![
                "Thoughtful Deliberation".to_string(),
                "Emotional Depth".to_string(),
            ]
        } else {
            vec![
                "Organic Affinity".to_string(),
                "Mutual Independence".to_string(),
            ]
        };

        let romance_summary = if deuk_se.caisheng_count >= 1 || deuk_se.guanxing_count >= 1 {
            "Attracts engaging relational opportunities with a strong blend of magnetic charisma and reliable support."
                .to_string()
        } else {
            "Takes time to evaluate partners deliberately, placing supreme value on intellectual resonance and emotional stability.".to_string()
        };

        let romance_advice =
            "Active empathic listening and frequent small disclosures of authentic feelings deepen relationship intimacy."
                .to_string();

        // 4. Health & Vulnerable Organs
        let mut elements = vec![
            pillars.year.stem.element(),
            pillars.year.branch.element(),
            pillars.month.stem.element(),
            pillars.month.branch.element(),
            pillars.day.stem.element(),
            pillars.day.branch.element(),
        ];
        if let Some(hour) = pillars.hour {
            elements.push(hour.stem.element());
            elements.push(hour.branch.element());
        }

        let mut wood_cnt = 0;
        let mut fire_cnt = 0;
        let mut earth_cnt = 0;
        let mut metal_cnt = 0;
        let mut water_cnt = 0;

        for e in elements {
            match e {
                Element::Wood => wood_cnt += 1,
                Element::Fire => fire_cnt += 1,
                Element::Earth => earth_cnt += 1,
                Element::Metal => metal_cnt += 1,
                Element::Water => water_cnt += 1,
            }
        }

        let mut vulnerable_organs = Vec::new();
        if wood_cnt == 0 || wood_cnt >= 4 {
            vulnerable_organs.push("Liver / Gallbladder (Wood)".to_string());
            vulnerable_organs.push("Nervous System".to_string());
        }
        if fire_cnt == 0 || fire_cnt >= 4 {
            vulnerable_organs.push("Heart / Small Intestine (Fire)".to_string());
            vulnerable_organs.push("Cardiovascular System".to_string());
        }
        if earth_cnt == 0 || earth_cnt >= 4 {
            vulnerable_organs.push("Spleen / Stomach (Earth)".to_string());
            vulnerable_organs.push("Digestive System".to_string());
        }
        if metal_cnt == 0 || metal_cnt >= 4 {
            vulnerable_organs.push("Lungs / Large Intestine (Metal)".to_string());
            vulnerable_organs.push("Respiratory System".to_string());
        }
        if water_cnt == 0 || water_cnt >= 4 {
            vulnerable_organs.push("Kidneys / Bladder (Water)".to_string());
            vulnerable_organs.push("Urogenital System".to_string());
        }

        if vulnerable_organs.is_empty() {
            vulnerable_organs.push("Stomach (Stress-induced)".to_string());
            vulnerable_organs.push("Autonomic Nervous System".to_string());
        }

        let health_score = 90 - (vulnerable_organs.len() as u32 * 5);

        let health_keywords = vec![
            "Elemental Balance".to_string(),
            "Stress Management".to_string(),
        ];
        let health_summary = format!(
            "Five-element balance indicates focus on {} maintenance and conscious lifestyle stress relief.",
            vulnerable_organs.first().cloned().unwrap_or_default()
        );

        ThemedAnalysis {
            career: CareerAnalysis {
                score: career_score,
                keywords: career_keywords,
                summary: career_summary,
                recommendation: career_recommendation,
            },
            wealth: WealthAnalysis {
                score: wealth_score,
                keywords: wealth_keywords,
                summary: wealth_summary,
                flow: wealth_flow,
            },
            romance: RomanceAnalysis {
                score: romance_score,
                keywords: romance_keywords,
                summary: romance_summary,
                advice: romance_advice,
            },
            health: HealthAnalysis {
                score: health_score,
                keywords: health_keywords,
                summary: health_summary,
                vulnerable_organs,
            },
        }
    }
}
