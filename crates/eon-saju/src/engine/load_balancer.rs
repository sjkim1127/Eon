use serde::{Deserialize, Serialize};
use crate::engine::vm::LifeFrame;

/// 운의 부하 상태
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrafficStatus {
    Idle,           // 평온한 상태
    Normal,         // 일반적인 트래픽
    Overloaded,     // 급격한 변화 또는 과부하 (DDoS 공격 수준)
    SystemDown,     // 치명적 충돌 또는 최저 건강 상태
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

        for i in 1..frames.len() {
            let prev = &frames[i-1];
            let curr = &frames[i];

            // 1. 운세 급변 구간 감지 (급격한 상승/하락)
            let score_diff = (curr.score - prev.score).abs();
            if score_diff >= 15.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::Overloaded,
                    reason: format!("운세 급변 구간 (변동 강도: {:.0}점)", score_diff),
                    strategy: "에너지 변화가 클 때입니다. 무리한 결정은 미루고 변화에 유연하게 대응하세요.".to_string(),
                });
            }

            // 2. 운세 집중 구간 (지속적인 고득점 또는 저득점)
            if curr.score > 85.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::Overloaded,
                    reason: "좋은 기운이 강하게 몰리는 시기입니다.".to_string(),
                    strategy: "성취욕이 높아지는 시기이지만 과욕을 경계하고 안정을 함께 챙기세요.".to_string(),
                });
            } else if curr.score < 25.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::SystemDown,
                    reason: "어려움이 집중되는 시기입니다.".to_string(),
                    strategy: "새로운 도전보다는 현상 유지에 집중하고, 소중한 것을 지키는 데 에너지를 쏟으세요.".to_string(),
                });
            }
        }

        // 결과 요약 (너무 많으면 주요 포인트만)
        diagnostics.sort_by(|a, b| b.age.cmp(&a.age)); // 가시성을 위해 나이순
        diagnostics
    }
}
