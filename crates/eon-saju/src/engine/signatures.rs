use crate::analysis::dynamic_luck::DynamicLuckAnalysis;
use crate::engine::vm::LifeFrame;
use serde::{Deserialize, Serialize};

/// Luck pattern signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuckSignature {
    pub id: String,
    pub name: String,
    pub severity: SignatureSeverity,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SignatureSeverity {
    Info,
    Success,
    Warning,
    Critical,
}

pub struct SignatureScanner;

impl SignatureScanner {
    pub fn scan(frame: &LifeFrame, dynamic: &DynamicLuckAnalysis) -> Vec<LuckSignature> {
        let mut detected = Vec::new();

        // 1. Heavenly & Earthly Double Harmony
        if Self::check_double_harmony(dynamic) {
            detected.push(LuckSignature {
                id: "DOUBLE_HARMONY".to_string(),
                name: "Heavenly & Earthly Double Harmony".to_string(),
                severity: SignatureSeverity::Success,
                description:
                    "Both stems and branches form combinations, creating perfect alignment between environment and mindset."
                        .to_string(),
            });
        }

        // 2. Fatal Yongshin Clash
        if frame.score < 30.0 && frame.tags.iter().any(|t| t.contains_pattern("Clash")) {
            detected.push(LuckSignature {
                id: "FATAL_LUCK_CLASH".to_string(),
                name: "Fatal Yongshin Clash".to_string(),
                severity: SignatureSeverity::Critical,
                description:
                    "Core energy suffers severe collision, causing foundation instability requiring caution."
                        .to_string(),
            });
        }

        // 3. Wealth & Officer Noble Resonance
        let branch_hap_count = dynamic
            .combined_relations
            .six_combinations
            .iter()
            .filter(|(_, p1, p2)| p1.contains("luck") || p2.contains("luck"))
            .count()
            + dynamic
                .combined_relations
                .dominant_semi_combinations
                .iter()
                .filter(|(_, p1, p2)| p1.contains("luck") || p2.contains("luck"))
                .count()
            + dynamic
                .combined_relations
                .weak_semi_combinations
                .iter()
                .filter(|(_, p1, p2)| p1.contains("luck") || p2.contains("luck"))
                .count();

        if branch_hap_count >= 2 && frame.score > 65.0 {
            detected.push(LuckSignature {
                id: "GONG_GWI_RESONANCE".to_string(),
                name: "Wealth & Officer Noble Resonance".to_string(),
                severity: SignatureSeverity::Success,
                description: "Invisible qi resonates to attract honor and prosperity, marking an era of achievement."
                    .to_string(),
            });
        }

        // 4. The Dragon Gate (Ascension)
        if frame.score > 85.0 && frame.tags.iter().any(|t| t.contains_pattern("CoreLuck")) {
            detected.push(LuckSignature {
                id: "DRAGON_GATE".to_string(),
                name: "The Dragon Gate (Ascension)".to_string(),
                severity: SignatureSeverity::Success,
                description: "Crossing major threshold to achieve elevated status and leadership."
                    .to_string(),
            });
        }

        detected
    }

    fn check_double_harmony(dynamic: &DynamicLuckAnalysis) -> bool {
        // Filter combinations involving luck periods
        let has_dynamic_stem_hap = dynamic
            .combined_relations
            .stem_combinations
            .iter()
            .any(|(_, p1, p2)| p1.contains("luck") || p2.contains("luck"));

        let has_dynamic_branch_hap = dynamic
            .combined_relations
            .six_combinations
            .iter()
            .any(|(_, p1, p2)| p1.contains("luck") || p2.contains("luck"))
            || dynamic
                .combined_relations
                .dominant_semi_combinations
                .iter()
                .any(|(_, p1, p2)| p1.contains("luck") || p2.contains("luck"))
            || dynamic
                .combined_relations
                .weak_semi_combinations
                .iter()
                .any(|(_, p1, p2)| p1.contains("luck") || p2.contains("luck"))
            || !dynamic.combined_relations.triple_combinations.is_empty()
            || !dynamic.combined_relations.seasonal_combinations.is_empty();

        has_dynamic_stem_hap && has_dynamic_branch_hap
    }
}
