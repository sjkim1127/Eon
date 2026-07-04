//! Comprehensive Saju Report Generator
//!
//! Aggregates various analysis results into a single structured report.

use crate::analysis::{
    analytics::GoldenTime,
    major_luck::MajorLuckAnalysis,
    power::{AnalysisOptions, IntegratedAnalysis},
    relationships::RelationshipAnalysis,
    spirit_markers::SpiritMarkerAnalysis,
    strength::StrengthAnalysis,
    structure::StructureAnalysis,
    supplementary_pillars::SupplementaryPillars,
    void::VoidAnalysis,
    yongshin::YongshinAnalysis,
};
use crate::core::pillars::FourPillars;
use crate::core::ten_gods::TenGodAnalysis;
use crate::engine::emulator::YearlyScore;
use crate::engine::vm::LifeFrame;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SajuReport {
    pub pillars: FourPillars,
    pub strength: StrengthAnalysis,
    pub yongshin: YongshinAnalysis,
    pub spirit_markers: SpiritMarkerAnalysis,
    pub structure: StructureAnalysis,
    pub major_luck: Option<MajorLuckAnalysis>,
    pub golden_time: Option<GoldenTime>,
    pub vm_summary: Option<String>,
    pub timeline: Vec<YearlyScore>,
    pub simulation_frames: Vec<LifeFrame>,
    pub ten_gods: TenGodAnalysis,
    pub power: IntegratedAnalysis,
    pub voids: VoidAnalysis,
    pub relationships: RelationshipAnalysis,
    pub supplementary_pillars: SupplementaryPillars,
}

impl SajuReport {
    pub fn new(pillars: FourPillars) -> Self {
        let strength = pillars.strength();
        let yongshin = pillars.yongshin();
        let spirit_markers = pillars.spirit_markers();
        let structure = pillars.structure();
        let ten_gods = pillars.ten_gods();
        let power = pillars.integrated_analysis(
            AnalysisOptions::default(),
            &crate::core::config::AnalysisConfig::default(),
        );
        let voids = pillars.void_analysis();
        let relationships = RelationshipAnalysis::from_pillars(&pillars);
        let mut supplementary_pillars = SupplementaryPillars::calculate(&pillars);

        // 하위 호환성 및 SSOT를 위해 신살 정보를 명시적으로 전달하여 해석 생성
        supplementary_pillars.analyze_interpretations(&spirit_markers.aux_shinsals);

        Self {
            pillars,
            strength,
            yongshin,
            spirit_markers,
            structure,
            major_luck: None,
            golden_time: None,
            vm_summary: None,
            timeline: Vec::new(),
            simulation_frames: Vec::new(),
            ten_gods,
            power,
            voids,
            relationships,
            supplementary_pillars,
        }
    }

    pub fn with_major_luck(mut self, major_luck: MajorLuckAnalysis) -> Self {
        self.major_luck = Some(major_luck);
        self
    }

    pub fn with_golden_time(mut self, golden_time: GoldenTime) -> Self {
        self.golden_time = Some(golden_time);
        self
    }

    pub fn with_vm_simulation(mut self, frames: Vec<LifeFrame>) -> Self {
        self.simulation_frames = frames;
        self
    }

    pub fn with_timeline(mut self, timeline: Vec<YearlyScore>) -> Self {
        self.timeline = timeline;
        self
    }

    pub fn with_vm_summary(mut self, summary: String) -> Self {
        self.vm_summary = Some(summary);
        self
    }

    /// Generate a markdown formatted report
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Eon Saju Comprehensive Report\n\n");

        md.push_str("## 1. Natal Chart (Four Pillars)\n");
        md.push_str("```\n");
        md.push_str(&self.pillars.to_string());
        md.push('\n');
        md.push_str(&self.pillars.hangul());
        md.push_str("\n```\n\n");

        md.push_str("### 1.1 Auxiliary Pillars\n");
        md.push_str(&format!(
            "- **Taewon (Pregnancy)**: {} ({})\n",
            self.supplementary_pillars.taewon.hanja(),
            self.supplementary_pillars.taewon.hangul()
        ));
        md.push_str(&format!(
            "- **Myeonggung (Destiny)**: {} ({})\n",
            self.supplementary_pillars.myeonggung.hanja(),
            self.supplementary_pillars.myeonggung.hangul()
        ));
        md.push_str(&format!(
            "- **Shingung (Body)**: {} ({})\n",
            self.supplementary_pillars.shingung.hanja(),
            self.supplementary_pillars.shingung.hangul()
        ));

        if !self.supplementary_pillars.interpretations.is_empty() {
            md.push_str("\n#### Interpretations\n");
            for interp in &self.supplementary_pillars.interpretations {
                let emoji = match interp.level {
                    crate::analysis::supplementary_pillars::InterpretationLevel::Auspicious => "✨",
                    crate::analysis::supplementary_pillars::InterpretationLevel::Caution => "⚠️",
                    crate::analysis::supplementary_pillars::InterpretationLevel::Danger => "🚨",
                    crate::analysis::supplementary_pillars::InterpretationLevel::Neutral => "•",
                };
                md.push_str(&format!(
                    "- **{} {}**: {}\n",
                    emoji, interp.pillar_name, interp.description
                ));
            }
        }
        md.push('\n');

        md.push_str("## 2. Basic Analysis\n");
        md.push_str(&format!(
            "- **Day Master**: {} ({})\n",
            self.pillars.day_master().hanja(),
            self.pillars.day_master().hangul()
        ));
        md.push_str(&format!(
            "- **Strength**: {} (Score: {:.1})\n",
            self.strength.strength_type.hangul(),
            self.strength.strength_score
        ));
        md.push_str(&format!(
            "- **Yongshin (Useful God)**: {:?}\n",
            self.strength.recommend_yongshin()
        ));

        md.push_str("\n### 4-Deuk Analysis\n");
        md.push_str(&format!(
            "- **Deuk-Ryeong**: {}\n",
            if self.strength.deuk_ryeong.acquired {
                "Yes"
            } else {
                "No"
            }
        ));
        md.push_str(&format!(
            "- **Deuk-Ji**: {}\n",
            if self.strength.deuk_ji.acquired {
                "Yes"
            } else {
                "No"
            }
        ));
        md.push_str(&format!(
            "- **Deuk-Si**: {}\n",
            if self.strength.deuk_si.acquired {
                "Yes"
            } else {
                "No"
            }
        ));
        md.push_str(&format!(
            "- **Deuk-Se**: {}\n",
            if self.strength.deuk_se.acquired {
                "Yes"
            } else {
                "No"
            }
        ));

        md.push_str("\n## 3. Structure Analysis (Gyeokguk)\n");
        md.push_str(&format!(
            "- **Structure**: {} ({})\n",
            self.structure.structure.hangul(),
            self.structure.structure.hanja()
        ));
        md.push_str(&format!("- **Summary**: {}\n", self.structure.summary));
        md.push_str(&format!(
            "- **Description**: {}\n",
            self.structure.description
        ));
        if !self.structure.reasons.is_empty() {
            md.push_str(&format!(
                "- **Reasons**: {}\n",
                self.structure.reasons.join(", ")
            ));
        }

        md.push_str("\n## 4. Spirit Markers (Shensha)\n");
        if self.spirit_markers.mapped_markers.is_empty() {
            md.push_str("- None detected.\n");
        } else {
            for m in &self.spirit_markers.mapped_markers {
                let emoji = match m.level {
                    crate::analysis::supplementary_pillars::InterpretationLevel::Auspicious => "✨",
                    crate::analysis::supplementary_pillars::InterpretationLevel::Caution => "⚠️",
                    crate::analysis::supplementary_pillars::InterpretationLevel::Danger => "🚨",
                    crate::analysis::supplementary_pillars::InterpretationLevel::Neutral => "•",
                };
                md.push_str(&format!(
                    "### {} {} ({})\n",
                    emoji,
                    m.marker.hangul(),
                    m.position.hangul()
                ));
                md.push_str(&format!("- **Summary**: {}\n", m.summary));
                md.push_str(&format!("- **Description**: {}\n", m.description));
                if !m.reasons.is_empty() {
                    md.push_str(&format!("- **Reasons**: {}\n", m.reasons.join(", ")));
                }
                md.push('\n');
            }
        }

        md.push_str("## 5. Relationships (Relationships Analysis)\n");
        if self.relationships.mapped_relationships.is_empty() {
            md.push_str("- No significant relationships detected.\n");
        } else {
            for rel in &self.relationships.mapped_relationships {
                let emoji = match rel.level {
                    crate::analysis::supplementary_pillars::InterpretationLevel::Auspicious => "🤝",
                    crate::analysis::supplementary_pillars::InterpretationLevel::Caution => "⚡",
                    crate::analysis::supplementary_pillars::InterpretationLevel::Danger => "🚨",
                    crate::analysis::supplementary_pillars::InterpretationLevel::Neutral => "•",
                };
                md.push_str(&format!(
                    "### {} {} ({})\n",
                    emoji,
                    rel.name,
                    rel.positions.join(", ")
                ));
                md.push_str(&format!("- **Summary**: {}\n", rel.summary));
                md.push_str(&format!("- **Description**: {}\n", rel.description));
                if !rel.reasons.is_empty() {
                    md.push_str(&format!("- **Reasons**: {}\n", rel.reasons.join(", ")));
                }
                if let Some(elem) = rel.transformed_element {
                    md.push_str(&format!("- **Change**: Transformed to {}\n", elem.hangul()));
                }
                md.push('\n');
            }
        }
        md.push('\n');

        if let Some(major) = &self.major_luck {
            md.push_str("## 6. Major Luck Cycles (Daeyun)\n");
            md.push_str(&format!("- **Direction**: {}\n", major.direction));
            md.push_str(&format!("- **Start Age**: {}\n", major.start_age));
            md.push_str("\n| Order | Age | GanZi | Start Date |\n");
            md.push_str("|---|---|---|---|\n");
            for (i, cycle) in major.cycles.iter().enumerate() {
                md.push_str(&format!(
                    "| {} | {} | {} | {} |\n",
                    i + 1,
                    cycle.start_age,
                    cycle.ganzi.hangul(),
                    cycle.start_date.format("%Y-%m-%d")
                ));
            }
            md.push('\n');
        }

        if let Some(golden) = &self.golden_time {
            md.push_str("## 7. Golden Time Analysis (AI/VM)\n");
            md.push_str(&format!(
                "- **Period**: Age {} - {}\n",
                golden.start_age, golden.end_age
            ));
            md.push_str(&format!("- **Avg Score**: {:.2}\n", golden.average_score));
            md.push_str(&format!("- **Description**: {}\n", golden.description));
            md.push('\n');
        }

        if !self.simulation_frames.is_empty() {
            md.push_str("## 8. Life Simulation Details\n");
            md.push_str("### 8.1 Energy Balance (Qi Registers)\n");
            md.push_str("| Age | Year | Score | Wood | Fire | Earth | Metal | Water |\n");
            md.push_str("|---|---|---|---|---|---|---|---|\n");

            // 10년 단위로 요약 출력
            for frame in self.simulation_frames.iter().step_by(10) {
                let r = &frame.register_state;
                md.push_str(&format!(
                    "| {} | {} | {:.1} | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% |\n",
                    frame.age,
                    frame.ganzi.hangul(),
                    frame.score,
                    r.r0_wood,
                    r.r1_fire,
                    r.r2_earth,
                    r.r3_metal,
                    r.r4_water
                ));
            }

            md.push_str("\n### 8.2 Key Life Events (ESIL Trace Summary)\n");
            // 큰 변화가 있거나 상위 5개 프레임 추출
            let mut key_frames: Vec<_> = self.simulation_frames.iter().collect();
            key_frames.sort_by(|a, b| {
                b.score
                    .partial_cmp(&a.score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            for frame in key_frames.iter().take(5) {
                if !frame.tags.is_empty() {
                    md.push_str(&format!(
                        "- **Age {} ({})**: Score {:.1} | {}\n",
                        frame.age,
                        frame.ganzi.hangul(),
                        frame.score,
                        frame.tags_as_strings().join(", ")
                    ));
                    if !frame.esil_trace.is_empty() {
                        md.push_str(&format!("  - `ESIL`: {}\n", frame.esil_trace));
                    }
                }
            }
            md.push('\n');
        }

        if let Some(summary) = &self.vm_summary {
            md.push_str("## 9. Simulation Summary\n");
            md.push_str(summary);
            md.push('\n');
        }

        md
    }
}
