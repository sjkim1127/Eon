use crate::engine::vm::LifeFrame;
use serde::{Deserialize, Serialize};

/// Luck load traffic status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrafficStatus {
    Idle,       // Tranquil / Stable
    Normal,     // Standard flow
    Overloaded, // Volatile surge / Caution
    SystemDown, // Critical trough / Danger
}

/// Load balance diagnostic outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalanceDiagnostic {
    pub age: u32,
    pub status: TrafficStatus,
    pub reason: String,
    pub strategy: String,
}

pub struct KarmaLoadBalancer;

impl KarmaLoadBalancer {
    pub fn diagnose(frames: &[LifeFrame]) -> Vec<LoadBalanceDiagnostic> {
        let mut diagnostics = Vec::new();

        // --- Pass 1: Single frame and adjacent frame patterns ---
        for i in 1..frames.len() {
            let prev = &frames[i - 1];
            let curr = &frames[i];
            let score_diff = curr.score - prev.score;
            let abs_diff = score_diff.abs();

            // 1-a. Surge interval
            if score_diff >= 15.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::Overloaded,
                    reason: format!("Surge interval (+{:.0} pts)", abs_diff),
                    strategy:
                        "Rising momentum is prominent. Seize opportunities without overextending."
                            .to_string(),
                });
            // 1-b. Plunge interval
            } else if score_diff <= -15.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::Overloaded,
                    reason: format!("Plunge interval (-{:.0} pts)", abs_diff),
                    strategy: "Energy drops sharply. Defer major speculative decisions and adapt flexibly.".to_string(),
                });
            // 1-c. Gradual decline warning
            } else if score_diff <= -8.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::Normal,
                    reason: format!("Declining interval (-{:.0} pts)", abs_diff),
                    strategy: "Energy gradually tapers. Exercise caution in expenditures and consolidate foundations.".to_string(),
                });
            }

            // 1-d. Peak reached
            if curr.score > 85.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::Overloaded,
                    reason: "Peak interval with strongly clustered favorable energy.".to_string(),
                    strategy:
                        "Ambitions run high; maintain grounded prudence alongside achievements."
                            .to_string(),
                });
            // 1-e. Trough reached
            } else if curr.score < 25.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::SystemDown,
                    reason: "Trough interval with concentrated friction.".to_string(),
                    strategy:
                        "Focus on maintaining baseline status quo and protecting core assets."
                            .to_string(),
                });
            }

            // 1-f. Severe trough (< 15)
            if curr.score < 15.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::SystemDown,
                    reason: "Critical minimum interval representing severe life turbulence.".to_string(),
                    strategy: "Secure a safety net, defer crucial bets, and seek active support from mentors.".to_string(),
                });
            }

            // 1-g. Recovery signal
            if prev.score < 40.0 && curr.score >= 55.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::Normal,
                    reason: "Rebound interval — darkness recedes as recovery momentum ignites."
                        .to_string(),
                    strategy:
                        "New doorways and opportunities are opening; step forward with conviction."
                            .to_string(),
                });
            }

            // 1-h. Stable equilibrium interval
            if abs_diff < 3.0 && curr.score >= 45.0 && curr.score <= 65.0 {
                // Record only at onset of stable interval
                let prev_stable = i >= 2
                    && (frames[i - 1].score - frames[i - 2].score).abs() < 3.0
                    && frames[i - 1].score >= 45.0
                    && frames[i - 1].score <= 65.0;
                if !prev_stable {
                    diagnostics.push(LoadBalanceDiagnostic {
                        age: curr.age,
                        status: TrafficStatus::Idle,
                        reason: format!(
                            "Tranquil equilibrium interval (~{:.0} pts)",
                            curr.score.round()
                        ),
                        strategy:
                            "Stable period free from volatile disruptions; build inner competence."
                                .to_string(),
                    });
                }
            }
        }

        // --- Pass 2: Continuous streak detection ---
        {
            let mut low_streak: u32 = 0;
            let mut low_start_age: u32 = 0;
            let mut high_streak: u32 = 0;
            let mut high_start_age: u32 = 0;

            for frame in frames.iter() {
                // Long-term slump streak (3+ years below 40)
                if frame.score < 40.0 {
                    if low_streak == 0 {
                        low_start_age = frame.age;
                    }
                    low_streak += 1;
                    if low_streak == 3 {
                        diagnostics.push(LoadBalanceDiagnostic {
                            age: frame.age,
                            status: TrafficStatus::SystemDown,
                            reason: format!("Extended slump (lasting 3+ years since age {})", low_start_age),
                            strategy: "Conserve inner energy and cultivate resilience rather than starting new ventures.".to_string(),
                        });
                    }
                } else {
                    low_streak = 0;
                }

                // Golden era streak (3+ years above 70)
                if frame.score > 70.0 {
                    if high_streak == 0 {
                        high_start_age = frame.age;
                    }
                    high_streak += 1;
                    if high_streak == 3 {
                        diagnostics.push(LoadBalanceDiagnostic {
                            age: frame.age,
                            status: TrafficStatus::Normal,
                            reason: format!("Golden era persistence (3+ continuous years of peak fortune since age {})", high_start_age),
                            strategy: "Fortunate momentum persists; proactively execute core long-term ambitions.".to_string(),
                        });
                    }
                } else {
                    high_streak = 0;
                }
            }
        }

        // Sort by age descending
        diagnostics.sort_by(|a, b| b.age.cmp(&a.age));
        // Deduplicate by age preserving highest priority status
        diagnostics.dedup_by(|a, b| {
            if a.age == b.age {
                // Priority order: SystemDown > Overloaded > Normal > Idle
                let priority = |s: &TrafficStatus| match s {
                    TrafficStatus::SystemDown => 3,
                    TrafficStatus::Overloaded => 2,
                    TrafficStatus::Normal => 1,
                    TrafficStatus::Idle => 0,
                };
                priority(&a.status) <= priority(&b.status)
            } else {
                false
            }
        });
        diagnostics
    }
}
