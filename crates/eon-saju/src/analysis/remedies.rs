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
                    colors.push("Green".to_string());
                    colors.push("Cyan".to_string());
                    numbers.extend_from_slice(&[3, 8]);
                    directions.push("East".to_string());
                }
                Element::Fire => {
                    colors.push("Red".to_string());
                    colors.push("Orange".to_string());
                    numbers.extend_from_slice(&[2, 7]);
                    directions.push("South".to_string());
                }
                Element::Earth => {
                    colors.push("Yellow".to_string());
                    colors.push("Brown".to_string());
                    numbers.extend_from_slice(&[5, 10]);
                    directions.push("Center".to_string());
                }
                Element::Metal => {
                    colors.push("White".to_string());
                    colors.push("Silver".to_string());
                    numbers.extend_from_slice(&[4, 9]);
                    directions.push("West".to_string());
                }
                Element::Water => {
                    colors.push("Black".to_string());
                    colors.push("Navy".to_string());
                    numbers.extend_from_slice(&[1, 6]);
                    directions.push("North".to_string());
                }
            }
        }

        colors.dedup();
        numbers.dedup();
        directions.dedup();

        let lifestyle_advice = match primary {
            Element::Wood => {
                "Cultivate starting energy by immersing in nature, gardening, and walking in green spaces."
            }
            Element::Fire => {
                "Express warmth and vitality through bright lighting, aerobic exercise, and active interpersonal communication."
            }
            Element::Earth => {
                "Maintain steady routines, cultivate trustworthiness, and practice earthing or meditation to stabilize your center."
            }
            Element::Metal => {
                "Organize your physical environment, structure systematic routines, and maintain clear boundaries."
            }
            Element::Water => {
                "Engage in knowledge acquisition, stay well-hydrated, and practice reflective writing to deepen adaptability."
            }
        }
        .to_string();

        let warning = match primary {
            Element::Wood => "Beware of impulsive impatience and abrupt emotional fluctuations that disrupt steady progress.",
            Element::Fire => "Guard against burnout, high cardiovascular stress, and over-committing out of sudden passion.",
            Element::Earth => "Avoid complacency, stubborn rigidity, and stagnation by consciously inviting flexible perspectives.",
            Element::Metal => "Watch out for overly harsh criticism or emotional detachment that alienates key collaborators.",
            Element::Water => "Avoid slipping into melancholy or passive overthinking; pair reflection with proactive execution.",
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
