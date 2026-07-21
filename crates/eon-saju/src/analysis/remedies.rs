use crate::analysis::yongshin::YongshinAnalysis;
use crate::analysis::Analyzable;
use crate::core::config::AnalysisConfig;
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemediesAnalysis {
    pub lucky_colors: Vec<String>,
    pub lucky_numbers: Vec<u32>,
    pub lucky_directions: Vec<String>,
    pub lifestyle_advice: String,
    pub warning: String,
}

impl Analyzable for RemediesAnalysis {
    type Output = RemediesAnalysis;

    fn analyze(pillars: &FourPillars, config: &AnalysisConfig) -> Self::Output {
        let yongshin_analysis = YongshinAnalysis::analyze(pillars, config);
        let primary = yongshin_analysis.primary;
        let assistant = yongshin_analysis.assistant;

        let mut colors = Vec::new();
        let mut numbers = Vec::new();
        let mut directions = Vec::new();

        for &elem in &[primary, assistant] {
            match elem {
                Element::Wood => {
                    colors.push("초록(Green)".to_string());
                    colors.push("청색(Cyan)".to_string());
                    numbers.extend_from_slice(&[3, 8]);
                    directions.push("동쪽(East)".to_string());
                }
                Element::Fire => {
                    colors.push("빨강(Red)".to_string());
                    colors.push("주황(Orange)".to_string());
                    numbers.extend_from_slice(&[2, 7]);
                    directions.push("남쪽(South)".to_string());
                }
                Element::Earth => {
                    colors.push("노랑(Yellow)".to_string());
                    colors.push("브라운(Brown)".to_string());
                    numbers.extend_from_slice(&[5, 10]);
                    directions.push("중앙(Center)".to_string());
                }
                Element::Metal => {
                    colors.push("흰색(White)".to_string());
                    colors.push("실버(Silver)".to_string());
                    numbers.extend_from_slice(&[4, 9]);
                    directions.push("서쪽(West)".to_string());
                }
                Element::Water => {
                    colors.push("검정(Black)".to_string());
                    colors.push("남색(Navy)".to_string());
                    numbers.extend_from_slice(&[1, 6]);
                    directions.push("북쪽(North)".to_string());
                }
            }
        }

        colors.dedup();
        numbers.dedup();
        directions.dedup();

        let lifestyle_advice = match primary {
            Element::Wood => {
                "산책이나 등산 등 자연을 접하고 식물을 가꾸며, 시작하는 에너지를 키우는 것이 운을 높여줍니다."
            }
            Element::Fire => {
                "밝은 조명을 활용하고 유산소 운동이나 활발한 대인관계를 통해 열정과 표현력을 발현하세요."
            }
            Element::Earth => {
                "규칙적인 식습관과 신용 중심의 처세, 흙을 밟는 어싱(Earthing)이나 명상이 마음의 안정과 길운을 가져다줍니다."
            }
            Element::Metal => {
                "정돈된 환경 유지, 철저한 계획 수립 및 맺고 끊음이 명확한 생활 습관이 긍정적 기운을 보충해 줍니다."
            }
            Element::Water => {
                "충분한 수분 섭취, 수영이나 목욕, 정서적 유연성을 기르고 지식을 탐구하는 활동이 부족한 기운을 보강합니다."
            }
        }
        .to_string();

        let warning = match primary {
            Element::Wood => "충동적인 성급함이나 잦은 감정 기복으로 일을 그르치지 않도록 주의하세요.",
            Element::Fire => "과도한 욕심이나 조급증으로 인한 스트레스, 심혈관계 부담을 경계하세요.",
            Element::Earth => "타성에 젖어 고집을 부리거나 정체되지 않도록 유연한 사고를 유지하세요.",
            Element::Metal => "지나치게 냉정하거나 비판적인 태도로 타인과 대립하는 상황을 조심하세요.",
            Element::Water => "우울감이나 사색에 깊이 빠져 기회를 놓치지 않도록 밝고 긍정적인 행동력을 유지하세요.",
        }
        .to_string();

        RemediesAnalysis {
            lucky_colors: colors,
            lucky_numbers: numbers,
            lucky_directions: directions,
            lifestyle_advice,
            warning,
        }
    }
}
