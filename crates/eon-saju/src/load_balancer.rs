use serde::{Deserialize, Serialize};
use crate::vm::LifeFrame;

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

            // 1. Traffic Spike Detection (급격한 상승/하락)
            let score_diff = (curr.score - prev.score).abs();
            if score_diff >= 15.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::Overloaded,
                    reason: format!("Traffic Spike 감지 (변동폭: {:.1})", score_diff),
                    strategy: "Auto-Scaling 활성화: 멘탈 리소스를 확장하고 과도한 요청을 순차 처리(Queuing)하십시오.".to_string(),
                });
            }

            // 2. High Pressure (지속적인 고득점 또는 저득점 부하)
            if curr.score > 85.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::Overloaded,
                    reason: "Success_Overload: 과도한 성취로 인한 시스템 과열 가능성".to_string(),
                    strategy: "Throttling 필요: 성과를 분산시키고 내부 시스템 안정화를 위한 점검(Maintenance) 기간을 가지십시오.".to_string(),
                });
            } else if curr.score < 25.0 {
                diagnostics.push(LoadBalanceDiagnostic {
                    age: curr.age,
                    status: TrafficStatus::SystemDown,
                    reason: "Critical_Failure: 외부 충돌로 인한 서비스 다운 위기".to_string(),
                    strategy: "Backup Restore 준비: 기존의 관계 및 자산을 철저히 백업하고 최소 운영 모드(Safe Mode)로 진입하십시오.".to_string(),
                });
            }
        }

        // 결과 요약 (너무 많으면 주요 포인트만)
        diagnostics.sort_by(|a, b| b.age.cmp(&a.age)); // 가시성을 위해 나이순
        diagnostics
    }
}
