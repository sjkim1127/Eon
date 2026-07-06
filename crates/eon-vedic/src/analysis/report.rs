use crate::analysis::dasha::{DashaPeriod, YoginiDasha};
use crate::analysis::tajika::{Saham, TajikaBala, TajikaEngine};
use crate::analysis::varga_interpretation::{VargaInterpretation, VargaInterpretationEngine};
use crate::analysis::yogas::{YogaEngine, YogaResult};
use crate::chart::VedicChart;
use crate::planets::VedicPlanet;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

    // --- Jaimini Extension ---
    #[serde(default)]
    pub chara_dasha_timeline: Vec<crate::analysis::jaimini::SignDashaPeriod>,
    #[serde(default)]
    pub all_karakas: Vec<crate::analysis::jaimini::KarakaAssignment>,

    // Advanced Metrics
    #[serde(default)]
    pub arudha_lagna: u8,
    #[serde(default)]
    pub upapada_lagna: u8,
    #[serde(default)]
    pub special_lagnas_summary: Vec<(String, u8)>,

    // Varga Integration
    #[serde(default)]
    pub varga_interpretations: Vec<VargaInterpretation>,
    pub d9_marriage_analysis: String,
    pub d10_career_analysis: String,
    #[serde(default)]
    pub kalachakra_timeline: Vec<crate::prediction::kalachakra::KalaChakraPeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TajikaReport {
    pub year_lord: Option<VedicPlanet>,
    pub muntha_rasi: u8,
    pub sahams: Vec<Saham>,
    pub harsha_bala_summary: Vec<(VedicPlanet, u32)>,
    #[serde(default)]
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KarakaSummary {
    pub atmakaraka: VedicPlanet,
    pub amatyakaraka: VedicPlanet,
    pub bhratrukaraka: Option<VedicPlanet>,
    pub matrukaraka: Option<VedicPlanet>,
    pub pitrikaraka: Option<VedicPlanet>,
    pub putrakaraka: Option<VedicPlanet>,
    pub gnatikaraka: Option<VedicPlanet>,
    pub darakaraka: VedicPlanet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HouseRating {
    pub house: u8,
    pub rating: String, // e.g. "Excellent", "Strong", "Average", "Weak"
    pub total_score: f64,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub reasons: Vec<String>,
}

impl VedicAnalysisReport {
    pub fn generate(
        chart: &VedicChart,
        birth_time: chrono::DateTime<Utc>,
        _birth_lagna_rasi: u8,
    ) -> Self {
        let get_karaka = |role: crate::analysis::jaimini::JaiminiKarakaRole| {
            chart
                .karakas
                .iter()
                .find(|k| std::mem::discriminant(&k.role) == std::mem::discriminant(&role))
                .map(|k| k.planet)
        };

        let ak = get_karaka(crate::analysis::jaimini::JaiminiKarakaRole::Atmakaraka)
            .unwrap_or(VedicPlanet::Sun);
        let amk = get_karaka(crate::analysis::jaimini::JaiminiKarakaRole::Amatyakaraka)
            .unwrap_or(VedicPlanet::Sun);
        let bk = get_karaka(crate::analysis::jaimini::JaiminiKarakaRole::Bhratrukaraka);
        let mk = get_karaka(crate::analysis::jaimini::JaiminiKarakaRole::Matrukaraka);
        let pik = get_karaka(crate::analysis::jaimini::JaiminiKarakaRole::Pitrikaraka);
        let pk = get_karaka(crate::analysis::jaimini::JaiminiKarakaRole::Putrakaraka);
        let gk = get_karaka(crate::analysis::jaimini::JaiminiKarakaRole::Gnatikaraka);
        let dk = get_karaka(crate::analysis::jaimini::JaiminiKarakaRole::Darakaraka)
            .unwrap_or(VedicPlanet::Sun);

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

            let summary = match h.house {
                1 => "Vitality & Personality",
                2 => "Wealth & Speech",
                3 => "Effort & Siblings",
                4 => "Mother & Domestic Joy",
                5 => "Creativity & Progeny",
                6 => "Enemies, Debt & Health",
                7 => "Partnership & Public",
                8 => "Longevity & Transformation",
                9 => "Wisdom & Fortune",
                10 => "Career & Visibility",
                11 => "Gains & Fulfillment",
                12 => "Expenses & Spiritual Depth",
                _ => "Life Area",
            }
            .to_string();

            house_summary.push(HouseRating {
                house: h.house,
                rating: rating.to_string(),
                total_score: h.total_score,
                summary,
                description: format!(
                    "Capacity for specific life themes related to House {}.",
                    h.house
                ),
                reasons: h.reasons.clone(),
            });
        }

        // Calculate Dasha Focus (Vimshottari)
        let moon_pos = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon);
        let (dasha_focus, dasha_timeline) = if let Some(m) = moon_pos {
            let mut timeline = crate::analysis::dasha::VimshottariDasha::calculate_timeline(
                birth_time,
                m.sidereal_deg,
                3, // Maha + Antar + Pratyantar
            );
            crate::analysis::dasha::VimshottariDasha::attach_interpretations(&mut timeline, chart);
            let current_time = chrono::Utc::now();
            let focus = if let Some(current_dasha) = timeline
                .iter()
                .find(|d| d.start_time <= current_time && d.end_time > current_time)
            {
                format!("Current Major Period: {:?}", current_dasha.lord)
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
        let al = chart
            .arudha_padas
            .iter()
            .find(|a| a.house == 1)
            .map(|a| a.rasi)
            .unwrap_or(0);
        let ul = chart
            .arudha_padas
            .iter()
            .find(|a| a.house == 12)
            .map(|a| a.rasi)
            .unwrap_or(0);
        let special_lagnas_summary: Vec<_> = chart
            .special_lagnas
            .iter()
            .map(|s| (s.name.clone(), s.rasi))
            .collect();

        // Varga Interpretation Integration
        let planets_to_check = [
            VedicPlanet::Sun,
            VedicPlanet::Moon,
            VedicPlanet::Mars,
            VedicPlanet::Mercury,
            VedicPlanet::Jupiter,
            VedicPlanet::Venus,
            VedicPlanet::Saturn,
        ];
        let mut varga_interpretations = Vec::new();
        for p in planets_to_check {
            varga_interpretations.push(VargaInterpretationEngine::interpret_planet(chart, p));
        }
        let d9_marriage_analysis = VargaInterpretationEngine::analyze_marriage(chart);
        let d10_career_analysis = VargaInterpretationEngine::analyze_career(chart);

        // Chara Dasha (Jaimini)
        let chara_dasha_timeline =
            crate::analysis::jaimini::JaiminiEngine::calculate_chara_dasha(chart);

        // Kala Chakra Dasha
        let kalachakra_timeline = if let Some(m) = moon_pos {
            crate::prediction::kalachakra::KalaChakraDasha::calculate(m.sidereal_deg, birth_time)
        } else {
            Vec::new()
        };

        Self {
            primary_karakas: KarakaSummary {
                atmakaraka: ak,
                amatyakaraka: amk,
                bhratrukaraka: bk,
                matrukaraka: mk,
                pitrikaraka: pik,
                putrakaraka: pk,
                gnatikaraka: gk,
                darakaraka: dk,
            },
            house_summary,
            dasha_focus,
            dasha_timeline,
            yogini_timeline,
            chara_dasha_timeline,
            all_karakas: chart.karakas.clone(),
            nakshatra_info,
            overall_strength_score: chart
                .bhava_strengths
                .iter()
                .map(|h| h.total_score)
                .sum::<f64>()
                / 12.0,
            sade_sati: crate::analysis::gochara::SadeSatiPhase::None, // Expected to be populated by service/caller
            yogas,
            arudha_lagna: al,
            upapada_lagna: ul,
            special_lagnas_summary,
            varga_interpretations,
            d9_marriage_analysis,
            d10_career_analysis,
            kalachakra_timeline,
        }
    }
}

impl TajikaReport {
    pub fn generate(chart: &VedicChart, birth_lagna_rasi: u8, age_years: u32) -> Self {
        let sahams = TajikaEngine::calculate_sahams(chart);
        let mut harsha_bala_summary = Vec::new();
        let planets_to_check = [
            VedicPlanet::Sun,
            VedicPlanet::Moon,
            VedicPlanet::Mars,
            VedicPlanet::Mercury,
            VedicPlanet::Jupiter,
            VedicPlanet::Venus,
            VedicPlanet::Saturn,
        ];
        for p in planets_to_check {
            harsha_bala_summary.push((p, TajikaBala::calculate_harsha_bala(chart, p)));
        }

        let muntha_rasi = TajikaEngine::calculate_muntha(birth_lagna_rasi, age_years);
        let year_lord = TajikaEngine::select_year_lord(chart, birth_lagna_rasi, age_years);

        Self {
            year_lord: Some(year_lord),
            muntha_rasi,
            sahams,
            harsha_bala_summary,
            summary: format!(
                "Annual Chart Summary: Year Lord is {:?}, Muntha in Sign {}.",
                year_lord, muntha_rasi
            ),
        }
    }

    pub fn to_text_summary(&self) -> String {
        self.summary.clone()
    }
}

impl VedicAnalysisReport {
    pub fn to_text_summary(&self) -> String {
        let mut s = String::new();
        s.push_str("# Vedic Chart Analysis Summary\n\n");

        s.push_str("## 💠 Key Indicators (Jaimini Karakas)\n");
        s.push_str(&format!(
            "- **Atmakaraka (Self)**: {:?}\n",
            self.primary_karakas.atmakaraka
        ));
        s.push_str(&format!(
            "- **Amatyakaraka (Career)**: {:?}\n",
            self.primary_karakas.amatyakaraka
        ));
        if let Some(bk) = self.primary_karakas.bhratrukaraka {
            s.push_str(&format!(
                "- **Bhratrukaraka (Siblings/Effort)**: {:?}\n",
                bk
            ));
        }
        if let Some(mk) = self.primary_karakas.matrukaraka {
            s.push_str(&format!("- **Matrukaraka (Mother/Home)**: {:?}\n", mk));
        }
        if let Some(pk) = self.primary_karakas.putrakaraka {
            s.push_str(&format!(
                "- **Putrakaraka (Children/Intelligence)**: {:?}\n",
                pk
            ));
        }
        s.push_str(&format!(
            "- **Darakaraka (Partner)**: {:?}\n",
            self.primary_karakas.darakaraka
        ));
        s.push('\n');

        s.push_str("## 🧭 Jaimini Indicators\n");
        if self.arudha_lagna > 0 {
            s.push_str(&format!(
                "- **Arudha Lagna (AL)**: Sign {}\n",
                self.arudha_lagna
            ));
        }
        if self.upapada_lagna > 0 {
            s.push_str(&format!(
                "- **Upapada Lagna (UL)**: Sign {}\n",
                self.upapada_lagna
            ));
        }
        for (name, rasi) in &self.special_lagnas_summary {
            s.push_str(&format!("- **{}**: Sign {}\n", name, rasi));
        }
        s.push('\n');

        s.push_str(&format!("- **Nakshatra**: {}\n", self.nakshatra_info));
        s.push_str(&format!("- **Vimshottari Dasha**: {}\n", self.dasha_focus));
        if let Some(y) = self.yogini_timeline.first() {
            s.push_str(&format!(
                "- **Current Yogini**: {} (ruled by {:?})\n",
                y.name.as_ref().unwrap_or(&"Unknown".to_string()),
                y.lord
            ));
        }
        if let Some(c) = self.chara_dasha_timeline.first() {
            s.push_str(&format!(
                "- **Chara Dasha**: Sign {} until {}\n",
                c.rasi,
                c.end_time.format("%Y-%m-%d")
            ));
        }
        s.push('\n');

        s.push_str("## 🏠 House Capacities\n");
        let mut strong_houses: Vec<_> = self
            .house_summary
            .iter()
            .filter(|h| h.total_score > 300.0)
            .collect();
        strong_houses.sort_by(|a, b| {
            b.total_score
                .partial_cmp(&a.total_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

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
        s.push_str(&format!(
            "- **D9 Navamsa**: {}\n",
            self.d9_marriage_analysis
        ));
        s.push_str(&format!(
            "- **D10 Dasamsa**: {}\n\n",
            self.d10_career_analysis
        ));

        s.push_str("## 🚀 Special Status (Vargottama / Pushkar)\n");
        let special_planets: Vec<String> = self
            .varga_interpretations
            .iter()
            .filter(|vi| vi.is_vargottama || vi.is_pushkar_navamsa)
            .map(|vi| {
                let mut status = Vec::new();
                if vi.is_vargottama {
                    status.push("Vargottama");
                }
                if vi.is_pushkar_navamsa {
                    status.push("Pushkar Navamsa");
                }
                format!("{:?}({})", vi.planet, status.join("+"))
            })
            .collect();
        if special_planets.is_empty() {
            s.push_str("- No planets in special varga status.\n");
        } else {
            s.push_str(&format!("- **Planets**: {}\n", special_planets.join(", ")));
        }
        s.push('\n');

        s.push_str("## 🪐 Transit Alerts & Gochara\n");
        // Sade Sati
        match self.sade_sati {
            crate::analysis::gochara::SadeSatiPhase::Rising => s.push_str("- **Sade Sati (Rising)**: Saturn has entered the 12th from Moon. A period of internal shifts begins.\n"),
            crate::analysis::gochara::SadeSatiPhase::Peak => s.push_str("- **Sade Sati (Peak)**: Saturn is over the Moon. Focus on emotional resilience and discipline.\n"),
            crate::analysis::gochara::SadeSatiPhase::Setting => s.push_str("- **Sade Sati (Setting)**: The intensity is fading as Saturn reaches the 2nd from Moon.\n"),
            crate::analysis::gochara::SadeSatiPhase::None => s.push_str("- No Sade Sati active.\n"),
        }
        s.push('\n');
        s.push_str("Detailed planetary transits including Murti Nirnaya and Vedha obstructions are available in the interactive dashboard.\n\n");

        s.push_str(&format!(
            "## 📊 Overall Chart Strength: {:.1}/600\n",
            self.overall_strength_score
        ));

        s
    }
}
