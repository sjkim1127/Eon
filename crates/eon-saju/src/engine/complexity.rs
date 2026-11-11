use crate::engine::vm::LifeFrame;
use serde::{Deserialize, Serialize};

/// Life complexity analysis outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityAnalysis {
    /// Cyclomatic complexity (M = P + 1)
    /// P: Number of decision points
    pub cyclomatic_complexity: u32,

    /// Code stability grade (based on system complexity)
    pub stability_grade: String,

    /// Maintenance entropy (degree of life volatility)
    pub entropy: f32,

    /// Major decision node positions (ages)
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

            // 1. Clashes and Punishments (Exception Handling)
            // Clashes and punishments represent exceptional branch points in execution flow
            let conflict_count = frame
                .tags
                .iter()
                .filter(|t| {
                    t.contains_pattern("Clash")
                        || t.contains_pattern("Punishment")
                        || t.contains_pattern("Harm")
                })
                .count();

            if conflict_count > 0 {
                is_decision_point = true;
                total_conflict_weight += conflict_count as f32;
            }

            // 2. Special signatures (Conditional Branch)
            // High-impact signatures trigger state transitions in the system
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
        writeln!(f, "[Destiny Cyclomatic Complexity]")?;
        writeln!(
            f,
            "Complexity (M): {} (Decision Points + 1)",
            self.cyclomatic_complexity
        )?;
        writeln!(f, "Stability Grade: {}", self.stability_grade)?;
        writeln!(f, "Maintenance Entropy: {:.2}", self.entropy)?;

        let path_type = if self.cyclomatic_complexity > 50 {
            "Spaghetti Path"
        } else if self.cyclomatic_complexity > 20 {
            "Structured Path"
        } else {
            "Linear Path"
        };

        writeln!(f, "Estimated Path Type: {}", path_type)?;

        if !self.decision_nodes.is_empty() {
            let nodes_str = self
                .decision_nodes
                .iter()
                .take(10)
                .map(|age| format!("{} yrs", age))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(f, "Main Decision Nodes: {} ...", nodes_str)?;
        }

        Ok(())
    }
}
