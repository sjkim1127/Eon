use serde::{Deserialize, Serialize};
use crate::chart::VedicChart;
use crate::planets::VedicPlanet;

#[derive(Debug, Serialize, Deserialize)]
pub struct VedicAnalysisReport {
    pub primary_karakas: KarakaSummary,
    pub house_summary: Vec<HouseRating>,
    pub dasha_focus: String,
    pub overall_strength_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KarakaSummary {
    pub atmakaraka: VedicPlanet,
    pub amatyakaraka: VedicPlanet,
    pub darakaraka: VedicPlanet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HouseRating {
    pub house: u8,
    pub rating: String, // e.g. "Excellent", "Challenging"
    pub total_score: f64,
}

impl VedicAnalysisReport {
    pub fn generate(chart: &VedicChart) -> Self {
        let ak = chart.karakas.iter().find(|k| matches!(k.role, crate::analysis::jaimini::JaiminiKarakaRole::Atmakaraka)).map(|k| k.planet).unwrap_or(VedicPlanet::Sun);
        let amk = chart.karakas.iter().find(|k| matches!(k.role, crate::analysis::jaimini::JaiminiKarakaRole::Amatyakaraka)).map(|k| k.planet).unwrap_or(VedicPlanet::Sun);
        let dk = chart.karakas.iter().find(|k| matches!(k.role, crate::analysis::jaimini::JaiminiKarakaRole::Darakaraka)).map(|k| k.planet).unwrap_or(VedicPlanet::Sun);

        let mut house_summary = Vec::new();
        for h in &chart.bhava_strengths {
            let rating = if h.total_score > 400.0 { "Excellent" }
                        else if h.total_score > 300.0 { "Strong" }
                        else if h.total_score > 200.0 { "Average" }
                        else { "Weak" };
            
            house_summary.push(HouseRating {
                house: h.house,
                rating: rating.to_string(),
                total_score: h.total_score,
            });
        }

        Self {
            primary_karakas: KarakaSummary {
                atmakaraka: ak,
                amatyakaraka: amk,
                darakaraka: dk,
            },
            house_summary,
            dasha_focus: "Focus on current Mahadasha".to_string(), // Placeholder for logic
            overall_strength_score: chart.bhava_strengths.iter().map(|h| h.total_score).sum::<f64>() / 12.0,
        }
    }
}
