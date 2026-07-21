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

        // 1. Career (직업/커리어)
        let career_score = ((60
            + (deuk_se.guanxing_count as u32 * 8)
            + (deuk_se.shishang_count as u32 * 6)
            + (deuk_se.bijie_count as u32 * 4))
            .min(95))
        .max(50);

        let career_keywords = if deuk_se.guanxing_count >= 2 {
            vec!["조직 리더십".to_string(), "체계적 역량".to_string()]
        } else if deuk_se.shishang_count >= 2 {
            vec!["창의적 전문성".to_string(), "기술/기획력".to_string()]
        } else if deuk_se.bijie_count >= 2 {
            vec![
                "독립 자영/프리랜서".to_string(),
                "주도적 실행력".to_string(),
            ]
        } else {
            vec!["수용적 협업".to_string(), "안정적 기획".to_string()]
        };

        let career_summary = if deuk_se.guanxing_count >= 2 {
            "조직 및 가치 체계 내에서 신뢰와 리더십을 바탕으로 성장하는 스타일입니다.".to_string()
        } else if deuk_se.shishang_count >= 2 {
            "독창적인 기술이나 아이디어를 바탕으로 전문성을 인정받을 때 가장 빛납니다.".to_string()
        } else {
            "자율성과 독자적인 판단력을 존중받는 업무 환경에서 높은 성과를 냅니다.".to_string()
        };

        let career_recommendation = if deuk_se.guanxing_count >= 2 {
            "공공기관, 대기업, 체계화된 조직 내 핵심 보직이나 전문 관리직이 적합합니다.".to_string()
        } else if deuk_se.shishang_count >= 2 {
            "연구개발, 디자이너, 콘텐츠 크리에이터, 전문 자격 기술직이 유망합니다.".to_string()
        } else {
            "주도적으로 프로젝트를 이끌 수 있는 전문 프론티어 분야나 스타트업이 적합합니다."
                .to_string()
        };

        // 2. Wealth (재물운)
        let wealth_score = ((50
            + (deuk_se.caisheng_count as u32 * 12)
            + (deuk_se.shishang_count as u32 * 6)
            + (deuk_se.yinxing_count as u32 * 4))
            .min(98))
        .max(45);

        let wealth_keywords = if deuk_se.caisheng_count >= 2 {
            vec!["풍부한 재물 기반".to_string(), "자산 운용".to_string()]
        } else if deuk_se.shishang_count >= 1 {
            vec!["식상생재".to_string(), "기술/성과 수익".to_string()]
        } else {
            vec!["안정적 저축".to_string(), "지출 관리 필요".to_string()]
        };

        let wealth_summary = if deuk_se.caisheng_count >= 2 {
            "재물 인연이 깊으며 실속 있게 자산을 증식할 수 있는 사주 구조입니다.".to_string()
        } else if deuk_se.shishang_count >= 1 {
            "자신의 재능과 활동이 곧 직간접적인 수익으로 이어지는 재물 흐름을 가집니다.".to_string()
        } else {
            "무리한 투기보다 꾸준한 저축과 계획적인 리스크 관리가 재물운을 지켜줍니다.".to_string()
        };

        let wealth_flow = if deuk_se.caisheng_count >= 2 {
            "다각화된 투자나 부동산/금융 자산을 통해 안정적 부를 구축하는 흐름입니다.".to_string()
        } else {
            "한 번에 큰 이익을 노리기보다 꾸준한 파이프라인 확장을 통한 장기 성장이 유리합니다."
                .to_string()
        };

        // 3. Romance (연애/인연)
        let romance_score = ((55
            + (deuk_se.caisheng_count as u32 * 7)
            + (deuk_se.guanxing_count as u32 * 7)
            + (deuk_se.yinxing_count as u32 * 5))
            .min(95))
        .max(50);

        let romance_keywords = if deuk_se.caisheng_count >= 1 || deuk_se.guanxing_count >= 1 {
            vec!["적극적 매력".to_string(), "활발한 이성운".to_string()]
        } else if deuk_se.yinxing_count >= 2 {
            vec!["신중함".to_string(), "정서적 깊이".to_string()]
        } else {
            vec!["자연스러운 인연".to_string(), "상호 존중".to_string()]
        };

        let romance_summary = if deuk_se.caisheng_count >= 1 || deuk_se.guanxing_count >= 1 {
            "이성과의 인연이 활발하며 배려와 이끌림을 동시에 줄 수 있는 인연운을 지닙니다."
                .to_string()
        } else {
            "신중하게 상대를 탐색하고 서로의 깊은 정서적 안정감을 중요하게 생각합니다.".to_string()
        };

        let romance_advice =
            "상대방의 입장을 먼저 공감해 주고 사소한 대화를 자주 나누는 것이 깊은 관계 발전에 도움이 됩니다."
                .to_string();

        // 4. Health (건강운 & 장기 취약도)
        let elements = [
            pillars.year.stem.element(),
            pillars.year.branch.element(),
            pillars.month.stem.element(),
            pillars.month.branch.element(),
            pillars.day.stem.element(),
            pillars.day.branch.element(),
            pillars.hour.stem.element(),
            pillars.hour.branch.element(),
        ];

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
            vulnerable_organs.push("간/담(Liver/Gallbladder)".to_string());
            vulnerable_organs.push("신경계".to_string());
        }
        if fire_cnt == 0 || fire_cnt >= 4 {
            vulnerable_organs.push("심장/소장".to_string());
            vulnerable_organs.push("혈관계".to_string());
        }
        if earth_cnt == 0 || earth_cnt >= 4 {
            vulnerable_organs.push("위장/비장(Stomach)".to_string());
            vulnerable_organs.push("소화기계".to_string());
        }
        if metal_cnt == 0 || metal_cnt >= 4 {
            vulnerable_organs.push("폐/대장(Lungs)".to_string());
            vulnerable_organs.push("호흡기계".to_string());
        }
        if water_cnt == 0 || water_cnt >= 4 {
            vulnerable_organs.push("신장/방광(Kidney)".to_string());
            vulnerable_organs.push("비뇨생식계".to_string());
        }

        if vulnerable_organs.is_empty() {
            vulnerable_organs.push("위장계(스트레스 주의)".to_string());
            vulnerable_organs.push("자율신경계".to_string());
        }

        let health_score = 90 - (vulnerable_organs.len() as u32 * 5);

        let health_keywords = vec!["체질 밸런스".to_string(), "스트레스 케어".to_string()];
        let health_summary = format!(
            "오행 균형 분석에 따라 {} 관련 장기 관리 및 일상 스트레스 완화에 유의하는 것이 좋습니다.",
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
