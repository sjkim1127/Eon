use crate::analysis::yogas::{YogaEngine, YogaResult};
use crate::chart::VedicChart;
use crate::planets::VedicPlanet;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VedicAnalysisReport {
    pub primary_karakas: KarakaSummary,
    pub house_summary: Vec<HouseRating>,
    pub dasha_focus: String,
    pub nakshatra_info: String,
    pub overall_strength_score: f64,
    pub sade_sati: crate::analysis::gochara::SadeSatiPhase,
    pub yogas: Vec<YogaResult>,
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
    pub rating: String, // e.g. "Excellent", "Strong", "Average", "Weak"
    pub total_score: f64,
}

impl VedicAnalysisReport {
    pub fn generate(chart: &VedicChart) -> Self {
        let ak = chart
            .karakas
            .iter()
            .find(|k| {
                matches!(
                    k.role,
                    crate::analysis::jaimini::JaiminiKarakaRole::Atmakaraka
                )
            })
            .map(|k| k.planet)
            .unwrap_or(VedicPlanet::Sun);
        let amk = chart
            .karakas
            .iter()
            .find(|k| {
                matches!(
                    k.role,
                    crate::analysis::jaimini::JaiminiKarakaRole::Amatyakaraka
                )
            })
            .map(|k| k.planet)
            .unwrap_or(VedicPlanet::Sun);
        let dk = chart
            .karakas
            .iter()
            .find(|k| {
                matches!(
                    k.role,
                    crate::analysis::jaimini::JaiminiKarakaRole::Darakaraka
                )
            })
            .map(|k| k.planet)
            .unwrap_or(VedicPlanet::Sun);

        // Calculate Sade Sati if Moon and Saturn are present
        let moon_rasi = chart
            .planets
            .iter()
            .find(|p| p.planet == VedicPlanet::Moon)
            .map(|p| p.rasi)
            .unwrap_or(1);
        let saturn_rasi = chart
            .planets
            .iter()
            .find(|p| p.planet == VedicPlanet::Saturn)
            .map(|p| p.rasi)
            .unwrap_or(1);
        let sade_sati =
            crate::analysis::gochara::GocharaEngine::calculate_sade_sati(moon_rasi, saturn_rasi);

        let mut house_summary = Vec::new();
        for h in &chart.bhava_strengths {
            let rating = if h.total_score > 400.0 {
                "Excellent"
            } else if h.total_score > 300.0 {
                "Strong"
            } else if h.total_score > 200.0 {
                "Average"
            } else {
                "Weak"
            };

            house_summary.push(HouseRating {
                house: h.house,
                rating: rating.to_string(),
                total_score: h.total_score,
            });
        }

        // Calculate Dasha Focus (Vimshottari)
        let moon_pos = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon);
        let dasha_focus = if let Some(m) = moon_pos {
            let timeline = crate::analysis::dasha::VimshottariDasha::calculate_timeline(
                Utc::now(),
                m.sidereal_deg,
                1,
            );
            if let Some(d) = timeline.first() {
                format!("Current Major Period: {:?}", d.lord)
            } else {
                "Dasha info unavailable".to_string()
            }
        } else {
            "Moon position required for Dasha".to_string()
        };

        // Nakshatra Info
        let nakshatra_info = if let Some(m) = moon_pos {
            let name = crate::analysis::nakshatra::NakshatraEngine::get_name(m.nakshatra);
            let pada_desc =
                crate::analysis::nakshatra::NakshatraEngine::get_pada_description(m.pada);
            format!("Birth Star: {} (Pada {}) - {}", name, m.pada, pada_desc)
        } else {
            "Nakshatra info unavailable".to_string()
        };

        // Yoga 계산
        let yogas = YogaEngine::check_yogas(chart);

        Self {
            primary_karakas: KarakaSummary {
                atmakaraka: ak,
                amatyakaraka: amk,
                darakaraka: dk,
            },
            house_summary,
            dasha_focus,
            nakshatra_info,
            overall_strength_score: chart
                .bhava_strengths
                .iter()
                .map(|h| h.total_score)
                .sum::<f64>()
                / 12.0,
            sade_sati,
            yogas,
        }
    }

    pub fn to_text_summary(&self) -> String {
        let mut s = String::new();
        s.push_str("# Vedic Chart Analysis Summary\n\n");

        s.push_str("## 💠 Key Indicators (Atman/Career/Relationship)\n");
        s.push_str(&format!(
            "- **Atmakaraka (Self)**: {:?}\n",
            self.primary_karakas.atmakaraka
        ));
        s.push_str(&format!(
            "- **Amatyakaraka (Career)**: {:?}\n",
            self.primary_karakas.amatyakaraka
        ));
        s.push_str(&format!(
            "- **Darakaraka (Partner)**: {:?}\n\n",
            self.primary_karakas.darakaraka
        ));

        s.push_str("## 🌌 Cosmic Blueprints (Nakshatra & Dasha)\n");
        s.push_str(&format!("- **Nakshatra**: {}\n", self.nakshatra_info));
        s.push_str(&format!("- **Dasha**: {}\n\n", self.dasha_focus));

        s.push_str("## 🏠 House Capacities\n");
        let mut strong_houses: Vec<_> = self
            .house_summary
            .iter()
            .filter(|h| h.total_score > 300.0)
            .collect();
        strong_houses.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());

        if strong_houses.is_empty() {
            s.push_str("- No exceptionally strong houses detected.\n");
        } else {
            for h in strong_houses.iter().take(3) {
                s.push_str(&format!(
                    "- **House {}**: {} ({:.1})\n",
                    h.house, h.rating, h.total_score
                ));
            }
        }
        s.push_str("\n");

        s.push_str("## 🪐 Transit Alerts\n");
        match self.sade_sati {
            crate::analysis::gochara::SadeSatiPhase::Rising => s.push_str("- **Sade Sati (Rising)**: Saturn has entered the 12th from Moon. A period of internal shifts begins.\n"),
            crate::analysis::gochara::SadeSatiPhase::Peak => s.push_str("- **Sade Sati (Peak)**: Saturn is over the Moon. Focus on emotional resilience and discipline.\n"),
            crate::analysis::gochara::SadeSatiPhase::Setting => s.push_str("- **Sade Sati (Setting)**: The intensity is fading as Saturn reaches the 2nd from Moon.\n"),
            crate::analysis::gochara::SadeSatiPhase::None => s.push_str("- No Sade Sati active. Normal transit rules apply.\n"),
        }
        s.push_str("\n");

        s.push_str(&format!(
            "## 📊 Overall Chart Strength: {:.1}/600\n",
            self.overall_strength_score
        ));

        s
    }
}
