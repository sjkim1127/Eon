use serde::{Deserialize, Serialize};
use crate::engine::vm::LifeFrame;
use crate::analysis::dynamic_luck::DynamicLuckAnalysis;

/// 운세 패턴 시그니처
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

        // 1. 천지덕합 (Heavenly & Earthly Double Harmony)
        if Self::check_double_harmony(dynamic) {
            detected.push(LuckSignature {
                id: "DOUBLE_HARMONY".to_string(),
                name: "천지덕합 (天地德合)".to_string(),
                severity: SignatureSeverity::Success,
                description: "천간과 지지가 모두 합이 되어 환경과 정신이 완벽한 조화를 이루는 시기입니다.".to_string(),
            });
        }

        // 2. 용신 충극 (Fatal Yongshin Clash)
        if frame.score < 30.0 && frame.tags.iter().any(|t| t.contains("충")) {
            detected.push(LuckSignature {
                id: "FATAL_LUCK_CLASH".to_string(),
                name: "용신충극 (用神沖剋)".to_string(),
                severity: SignatureSeverity::Critical,
                description: "핵심 에너지가 강력한 충돌로 인해 기반이 크게 흔들리는 위기 상황입니다.".to_string(),
            });
        }

        // 3. 재공귀인 (Gong-Gwi - 가상 패턴: 복합적인 합의 중첩)
        let branch_hap_count = dynamic.combined_relations.six_combinations.iter().filter(|(_, p1, p2)| p1.contains("운") || p2.contains("운")).count() + 
                               dynamic.combined_relations.dominant_semi_combinations.iter().filter(|(_, p1, p2)| p1.contains("운") || p2.contains("운")).count() +
                               dynamic.combined_relations.weak_semi_combinations.iter().filter(|(_, p1, p2)| p1.contains("운") || p2.contains("운")).count();

        if branch_hap_count >= 2 && frame.score > 65.0 {
            detected.push(LuckSignature {
                id: "GONG_GWI_RESONANCE".to_string(),
                name: "재공귀인 (財官貴人) 공명".to_string(),
                severity: SignatureSeverity::Success,
                description: "보이지 않는 기운이 명예와 재물을 끌어오는 강력한 성취의 시기입니다.".to_string(),
            });
        }

        // 4. 등용문 (The Dragon Gate)
        if frame.score > 85.0 && frame.tags.iter().any(|t| t.contains("핵심운")) {
           detected.push(LuckSignature {
                id: "DRAGON_GATE".to_string(),
                name: "등용문 (登龍門)".to_string(),
                severity: SignatureSeverity::Success,
                description: "인생의 큰 관문을 넘어 신분이 상승하거나 큰 조직의 수장이 되는 운세입니다.".to_string(),
            });
        }

        detected
    }

    fn check_double_harmony(dynamic: &DynamicLuckAnalysis) -> bool {
        // 대운이나 세운이 개입된 합만 필터링
        let has_dynamic_stem_hap = dynamic.combined_relations.stem_combinations.iter()
            .any(|(_, p1, p2)| p1.contains("운") || p2.contains("운"));
        
        let has_dynamic_branch_hap = dynamic.combined_relations.six_combinations.iter().any(|(_, p1, p2)| p1.contains("운") || p2.contains("운")) ||
                                    dynamic.combined_relations.dominant_semi_combinations.iter().any(|(_, p1, p2)| p1.contains("운") || p2.contains("운")) ||
                                    dynamic.combined_relations.weak_semi_combinations.iter().any(|(_, p1, p2)| p1.contains("운") || p2.contains("운")) ||
                                    !dynamic.combined_relations.triple_combinations.is_empty() ||
                                    !dynamic.combined_relations.seasonal_combinations.is_empty();
        
        has_dynamic_stem_hap && has_dynamic_branch_hap
    }
}
