use crate::analysis::yogas::{YogaEngine, YogaResult};
use crate::chart::VedicChart;
use crate::planets::VedicPlanet;
use crate::analysis::varga_interpretation::{VargaInterpretation, VargaInterpretationEngine};
use crate::analysis::tajika::{Saham, TajikaEngine, TajikaBala};
use crate::analysis::dasha::{DashaPeriod, YoginiDasha};
use chrono::{Utc, Datelike};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VedicAnalysisReport {
    pub primary_karakas: KarakaSummary,
    #[serde(default)]
    pub house_summary: Vec<HouseRating>,
    pub dasha_focus: String,
    #[serde(default)]
    pub dasha_timeline: Vec<DashaPeriod>,
    #[serde(default)]
    pub yogini_timeline: Vec<DashaPeriod>,
    pub nakshatra_info: String,
    pub overall_strength_score: f64,
    pub sade_sati: crate::analysis::gochara::SadeSatiPhase,
    #[serde(default)]
    pub yogas: Vec<YogaResult>,
    
    // Advanced Metrics
    #[serde(default)]
    pub arudha_lagna: u8,
    #[serde(default)]
    pub upapada_lagna: u8,
    #[serde(default)]
    pub special_lagnas_summary: Vec<(String, u8)>,
    
    // Tajika & Varga Integration
    #[serde(default)]
    pub sahams: Vec<Saham>,
    #[serde(default)]
    pub harsha_bala_summary: Vec<(VedicPlanet, u32)>,
    #[serde(default)]
    pub varga_interpretations: Vec<VargaInterpretation>,
    pub d9_marriage_analysis: String,
    pub d10_career_analysis: String,

    // Annual Chart
    pub year_lord: Option<VedicPlanet>,
    pub muntha_rasi: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KarakaSummary {
    pub atmakaraka: VedicPlanet,
    pub amatyakaraka: VedicPlanet,
    pub darakaraka: VedicPlanet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseRating {
    pub house: u8,
    pub rating: String, // e.g. "Excellent", "Strong", "Average", "Weak"
    pub total_score: f64,
}

impl VedicAnalysisReport {
    pub fn generate(chart: &VedicChart, birth_time: chrono::DateTime<Utc>, birth_lagna_rasi: u8) -> Self {
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
        let (dasha_focus, dasha_timeline) = if let Some(m) = moon_pos {
            let timeline = crate::analysis::dasha::VimshottariDasha::calculate_timeline(
                birth_time,
                m.sidereal_deg,
                2, // Mahadasha + Antardasha
            );
            let focus = if let Some(d) = timeline.first() {
                format!("Current Major Period: {:?}", d.lord)
            } else {
                "Dasha info unavailable".to_string()
            };
            (focus, timeline)
        } else {
            ("Moon position required for Dasha".to_string(), Vec::new())
        };

        // Yogini Dasha
        let yogini_timeline = if let Some(m) = moon_pos {
            YoginiDasha::calculate_timeline(birth_time, m.sidereal_deg)
        } else {
            Vec::new()
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

        // Jaimini Integration
        let al = chart.arudha_padas.iter().find(|a| a.house == 1).map(|a| a.rasi).unwrap_or(0);
        let ul = chart.arudha_padas.iter().find(|a| a.house == 12).map(|a| a.rasi).unwrap_or(0);
        let special_lagnas_summary = chart.special_lagnas.iter().map(|s| (s.name.clone(), s.rasi)).collect();

        // Tajika Integration
        let sahams = TajikaEngine::calculate_sahams(chart);
        let mut harsha_bala_summary = Vec::new();
        let planets_to_check = [
            VedicPlanet::Sun, VedicPlanet::Moon, VedicPlanet::Mars,
            VedicPlanet::Mercury, VedicPlanet::Jupiter, VedicPlanet::Venus, VedicPlanet::Saturn
        ];
        for p in planets_to_check {
            harsha_bala_summary.push((p, TajikaBala::calculate_harsha_bala(chart, p)));
        }

        // Varga Interpretation Integration
        let mut varga_interpretations = Vec::new();
        for p in planets_to_check {
            varga_interpretations.push(VargaInterpretationEngine::interpret_planet(chart, p));
        }
        let d9_marriage_analysis = VargaInterpretationEngine::analyze_marriage(chart);
        let d10_career_analysis = VargaInterpretationEngine::analyze_career(chart);

        // Annual Analysis
        let age_years = (chart.panchanga.current_time.year() - birth_time.year()).abs() as u32;
        let muntha_rasi = TajikaEngine::calculate_muntha(birth_lagna_rasi, age_years);
        let year_lord = TajikaEngine::select_year_lord(chart, birth_lagna_rasi, age_years);

        Self {
            primary_karakas: KarakaSummary {
                atmakaraka: ak,
                amatyakaraka: amk,
                darakaraka: dk,
            },
            house_summary,
            dasha_focus,
            dasha_timeline,
            yogini_timeline,
            nakshatra_info,
            overall_strength_score: chart
                .bhava_strengths
                .iter()
                .map(|h| h.total_score)
                .sum::<f64>()
                / 12.0,
            sade_sati,
            yogas,
            arudha_lagna: al,
            upapada_lagna: ul,
            special_lagnas_summary,
            sahams,
            harsha_bala_summary,
            varga_interpretations,
            d9_marriage_analysis,
            d10_career_analysis,
            year_lord: Some(year_lord),
            muntha_rasi,
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

        s.push_str("## 🧭 Jaimini Indicators\n");
        if self.arudha_lagna > 0 {
            s.push_str(&format!("- **Arudha Lagna (AL)**: Sign {}\n", self.arudha_lagna));
        }
        if self.upapada_lagna > 0 {
            s.push_str(&format!("- **Upapada Lagna (UL)**: Sign {}\n", self.upapada_lagna));
        }
        for (name, rasi) in &self.special_lagnas_summary {
            s.push_str(&format!("- **{}**: Sign {}\n", name, rasi));
        }
        s.push('\n');

        s.push_str("## 🌌 Cosmic Blueprints (Nakshatra & Dasha)\n");
        s.push_str(&format!("- **Nakshatra**: {}\n", self.nakshatra_info));
        s.push_str(&format!("- **Vimshottari Dasha**: {}\n", self.dasha_focus));
        if let Some(y) = self.yogini_timeline.first() {
            s.push_str(&format!("- **Current Yogini**: {} (ruled by {:?})\n", y.name.as_ref().unwrap_or(&"Unknown".to_string()), y.lord));
        }
        s.push('\n');

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
        s.push('\n');

        s.push_str("## 💍 Marriage & Career Insights (Divisional Charts)\n");
        s.push_str(&format!("- **D9 Navamsa**: {}\n", self.d9_marriage_analysis));
        s.push_str(&format!("- **D10 Dasamsa**: {}\n\n", self.d10_career_analysis));

        s.push_str("## 🪐 Tajika Annual Factors\n");
        s.push_str(&format!("- **Year Lord (Varsheshwara)**: {:?}\n", self.year_lord));
        s.push_str(&format!("- **Muntha Position**: Sign {}\n", self.muntha_rasi));
        if let Some(p) = self.sahams.iter().find(|s| s.name.contains("Punya")) {
            s.push_str(&format!("- **Punya Saham (Fortune)**: Sign {}\n", p.rasi));
        }
        let strong_bala = self.harsha_bala_summary.iter().filter(|(_, score)| *score >= 10).collect::<Vec<_>>();
        if !strong_bala.is_empty() {
             s.push_str("- **High Harsha Bala**: ");
             let lords: Vec<String> = strong_bala.iter().map(|(p, _)| format!("{:?}", p)).collect();
             s.push_str(&lords.join(", "));
             s.push('\n');
        }
        s.push('\n');

        s.push_str("## 🚀 Special Status (Vargottama / Pushkar)\n");
        let special_planets: Vec<String> = self.varga_interpretations.iter()
            .filter(|vi| vi.is_vargottama || vi.is_pushkar_navamsa)
            .map(|vi| {
                let mut status = Vec::new();
                if vi.is_vargottama { status.push("Vargottama"); }
                if vi.is_pushkar_navamsa { status.push("Pushkar Navamsa"); }
                format!("{:?}({})", vi.planet, status.join("+"))
            }).collect();
        if special_planets.is_empty() {
            s.push_str("- No planets in special varga status.\n");
        } else {
            s.push_str(&format!("- **Planets**: {}\n", special_planets.join(", ")));
        }
        s.push('\n');

        s.push_str("## 🪐 Transit Alerts\n");
        match self.sade_sati {
            crate::analysis::gochara::SadeSatiPhase::Rising => s.push_str("- **Sade Sati (Rising)**: Saturn has entered the 12th from Moon. A period of internal shifts begins.\n"),
            crate::analysis::gochara::SadeSatiPhase::Peak => s.push_str("- **Sade Sati (Peak)**: Saturn is over the Moon. Focus on emotional resilience and discipline.\n"),
            crate::analysis::gochara::SadeSatiPhase::Setting => s.push_str("- **Sade Sati (Setting)**: The intensity is fading as Saturn reaches the 2nd from Moon.\n"),
            crate::analysis::gochara::SadeSatiPhase::None => s.push_str("- No Sade Sati active. Normal transit rules apply.\n"),
        }
        s.push('\n');

        s.push_str(&format!(
            "## 📊 Overall Chart Strength: {:.1}/600\n",
            self.overall_strength_score
        ));

        s
    }
}
