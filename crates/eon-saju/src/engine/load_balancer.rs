use serde::{Deserialize, Serialize};
use crate::engine::vm::LifeFrame;

/// 운의 부하 상태
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrafficStatus {
    Idle,       // 평온·안정
    Normal,     // 일반 흐름
    Overloaded, // 급변·주의
    SystemDown, // 위험·극저점
}

/// 부하 분산 진단 결과
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

        // ─── Pass 1: 단일 프레임 / 인접 두 프레임 패턴 ───────────────────────
        for i in 1..frames.len() {
            let prev = &frames[i - 1];
            let curr = &frames[i];
            let score_diff = curr.score - prev.score;
            let abs_diff   = score_diff.abs();

            // 1-a. 운세 급등 구간
            if score_diff >= 15.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::Overloaded,
                    reason: format!("운세 급등 구간 (상승폭 {:.0}점)", abs_diff),
                    strategy: "상승 에너지가 큰 시기입니다. 기회를 놓치지 않되, 과잉 확장은 주의하세요.".to_string(),
                });
            // 1-b. 운세 급락 구간
            } else if score_diff <= -15.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::Overloaded,
                    reason: format!("운세 급락 구간 (하락폭 {:.0}점)", abs_diff),
                    strategy: "에너지가 급격히 줄어드는 시기입니다. 무리한 결정은 미루고 변화에 유연하게 대응하세요.".to_string(),
                });
            // 1-c. 점진적 하락 주의 (5~14점 하락)
            } else if score_diff <= -8.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::Normal,
                    reason: format!("운세 하락 흐름 (하락폭 {:.0}점)", abs_diff),
                    strategy: "서서히 에너지가 줄어드는 흐름입니다. 지출과 새로운 투자를 자제하고 내실을 다지세요.".to_string(),
                });
            }

            // 1-d. 고점 도달
            if curr.score > 85.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::Overloaded,
                    reason: "좋은 기운이 강하게 몰리는 최고점 시기입니다.".to_string(),
                    strategy: "성취욕이 높아지는 시기이지만 과욕을 경계하고 안정을 함께 챙기세요.".to_string(),
                });
            // 1-e. 저점 도달
            } else if curr.score < 25.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::SystemDown,
                    reason: "어려움이 집중되는 저점 시기입니다.".to_string(),
                    strategy: "새로운 도전보다는 현상 유지에 집중하고, 소중한 것을 지키는 데 에너지를 쏟으세요.".to_string(),
                });
            }

            // 1-f. 극심한 저점 (< 15)
            if curr.score < 15.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::SystemDown,
                    reason: "인생 최저 구간에 해당하는 매우 어려운 해입니다.".to_string(),
                    strategy: "최소한의 안전망을 확보하고 중요한 결정은 모두 미루세요. 주변의 도움을 적극 구하세요.".to_string(),
                });
            }

            // 1-g. 회복 신호 (저점 → 보통 이상으로 반등)
            if prev.score < 40.0 && curr.score >= 55.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::Normal,
                    reason: "운세 반등 구간 — 어두운 시기가 지나고 회복 흐름이 시작됩니다.".to_string(),
                    strategy: "새로운 시작을 위한 기회가 열리는 시기입니다. 망설임 없이 도전해 보세요.".to_string(),
                });
            }

            // 1-h. 안정 구간 (변동이 매우 작고 중간점수대 유지)
            if abs_diff < 3.0 && curr.score >= 45.0 && curr.score <= 65.0 {
                // 직전 프레임이 안정 구간이 아닌 경우(안정기 시작점)에만 기록
                let prev_stable = i >= 2
                    && (frames[i - 1].score - frames[i - 2].score).abs() < 3.0
                    && frames[i - 1].score >= 45.0
                    && frames[i - 1].score <= 65.0;
                if !prev_stable {
                    diagnostics.push(LoadBalanceDiagnostic {
                        age: curr.age,
                        status: TrafficStatus::Idle,
                        reason: format!("평온한 안정 구간 (~{}점대 유지)", curr.score.round() as i32),
                        strategy: "큰 변화 없이 안정된 시기입니다. 조용히 역량을 쌓고 다음 기회를 준비하기 좋습니다.".to_string(),
                    });
                }
            }
        }

        // ─── Pass 2: 스트릭(연속 패턴) 감지 ──────────────────────────────────
        {
            let mut low_streak: u32 = 0;
            let mut low_start_age: u32 = 0;
            let mut high_streak: u32 = 0;
            let mut high_start_age: u32 = 0;

            for frame in frames.iter() {
                // 장기 침체 스트릭 (연속 3년 이상 40점 미만)
                if frame.score < 40.0 {
                    if low_streak == 0 { low_start_age = frame.age; }
                    low_streak += 1;
                    if low_streak == 3 {
                        diagnostics.push(LoadBalanceDiagnostic {
                            age: frame.age,
                            status: TrafficStatus::SystemDown,
                            reason: format!("장기 침체 구간 ({}세부터 3년 이상 지속)", low_start_age),
                            strategy: "지금은 새로운 시작보다 내면을 가다듬고 힘을 비축하는 시기입니다. 가까운 사람에게 의지하세요.".to_string(),
                        });
                    }
                } else {
                    low_streak = 0;
                }

                // 황금기 스트릭 (연속 3년 이상 70점 초과)
                if frame.score > 70.0 {
                    if high_streak == 0 { high_start_age = frame.age; }
                    high_streak += 1;
                    if high_streak == 3 {
                        diagnostics.push(LoadBalanceDiagnostic {
                            age: frame.age,
                            status: TrafficStatus::Normal,
                            reason: format!("황금기 지속 ({}세부터 3년 이상 이어지는 호운)", high_start_age),
                            strategy: "좋은 흐름이 이어지고 있습니다. 이 시기에 중요한 목표를 과감하게 추진하세요.".to_string(),
                        });
                    }
                } else {
                    high_streak = 0;
                }
            }
        }

        // 나이 역순 정렬 (최근 → 과거 순)
        diagnostics.sort_by(|a, b| b.age.cmp(&a.age));
        // 중복 나이 제거 (같은 나이에 복수 진단이 있을 경우 우선순위 높은 상태 유지)
        diagnostics.dedup_by(|a, b| {
            if a.age == b.age {
                // SystemDown > Overloaded > Normal > Idle 우선순위
                let priority = |s: &TrafficStatus| match s {
                    TrafficStatus::SystemDown  => 3,
                    TrafficStatus::Overloaded  => 2,
                    TrafficStatus::Normal      => 1,
                    TrafficStatus::Idle        => 0,
                };
                priority(&a.status) <= priority(&b.status)
            } else {
                false
            }
        });
        diagnostics
    }
}
