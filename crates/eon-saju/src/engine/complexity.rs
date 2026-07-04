use crate::engine::vm::LifeFrame;
use serde::{Deserialize, Serialize};

/// 인생 복잡도 분석 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityAnalysis {
    /// 순환 복잡도 (Cyclomatic Complexity: M = P + 1)
    /// P: 결정 지점(Decision Points)의 수
    pub cyclomatic_complexity: u32,

    /// 코드 안정성 등급 (시스템 복잡도 기준)
    pub stability_grade: String,

    /// 유지보수 엔트로피 (인생의 파란만장함 정도)
    pub entropy: f32,

    /// 주요 분기점(Decision Nodes) 위치
    pub decision_nodes: Vec<u32>,
}

pub struct DestinyComplexity;

impl DestinyComplexity {
    pub fn analyze(frames: &[LifeFrame]) -> ComplexityAnalysis {
        let mut decision_points = 0;
        let mut decision_nodes = Vec::new();
        let mut total_conflict_weight = 0.0;

        for frame in frames {
            let mut is_decision_point = false;

            // 1. 충격 및 형살 (Exception Handling)
            // 충(Clash)이나 형(Punishment)은 실행 흐름의 예외 상황인 분기점으로 간주
            let conflict_count = frame
                .tags
                .iter()
                .filter(|t| {
                    t.contains_pattern("충") || t.contains_pattern("형") || t.contains_pattern("해")
                })
                .count();

            if conflict_count > 0 {
                is_decision_point = true;
                total_conflict_weight += conflict_count as f32;
            }

            // 2. 특수 시그니처 (Conditional Branch)
            // 천지덕합, 용신충극 등 강력한 시그니처는 시스템의 상태 변화를 유발하는 결정 지점
            if !frame.signatures.is_empty() {
                is_decision_point = true;
                total_conflict_weight += frame.signatures.len() as f32 * 1.5;
            }

            if is_decision_point {
                decision_points += 1;
                decision_nodes.push(frame.age);
            }
        }

        let m = decision_points + 1; // M = decision points + 1
        let entropy = (total_conflict_weight / frames.len() as f32) * 10.0;

        let stability_grade = match m {
            0..=10 => "A (Linear & Stable)".to_string(),
            11..=30 => "B (Structured)".to_string(),
            31..=60 => "C (Complex/Spaghetti)".to_string(),
            _ => "D (High Entropy/Unstable)".to_string(),
        };

        ComplexityAnalysis {
            cyclomatic_complexity: m,
            stability_grade,
            entropy,
            decision_nodes,
        }
    }
}

impl std::fmt::Display for ComplexityAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【Destiny Cyclomatic Complexity】")?;
        writeln!(
            f,
            "Complexity (M): {} (Decision Points + 1)",
            self.cyclomatic_complexity
        )?;
        writeln!(f, "Stability Grade: {}", self.stability_grade)?;
        writeln!(f, "Maintenance Entropy: {:.2}", self.entropy)?;

        let path_type = if self.cyclomatic_complexity > 50 {
            "스파게티 코드 (Spaghetti Path)"
        } else if self.cyclomatic_complexity > 20 {
            "구조적 분기 (Structured Path)"
        } else {
            "단순 수차적 흐름 (Linear Path)"
        };

        writeln!(f, "Estimated Path Type: {}", path_type)?;

        if !self.decision_nodes.is_empty() {
            let nodes_str = self
                .decision_nodes
                .iter()
                .take(10)
                .map(|age| format!("{}세", age))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(f, "Main Decision Nodes: {} ...", nodes_str)?;
        }

        Ok(())
    }
}
