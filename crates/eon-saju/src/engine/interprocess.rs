//! Inter-Process Compatibility (IPC) Audit Module
//! 
//! 두 개의 사주 시스템(SajuVM)이 상호작용할 때 발생하는
//! 에너지 간섭, 자원 경합(상극), 시너지(합)를 분석합니다.

use serde::{Deserialize, Serialize};
use crate::engine::vm::SajuVM;

/// IPC 분석 결과 리포트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityAudit {
    /// 종합 상호작용 점수 (0~100)
    pub sync_score: f32,
    /// 시스템 간 합(Synergy)
    pub synergies: Vec<String>,
    /// 시스템 간 충돌(Race Conditions/Conflicts)
    pub conflicts: Vec<String>,
    /// 자원 잠금 현상 (Deadlocks)
    pub deadlocks: Vec<String>,
    /// 두 시스템 결합 시의 ESIL 트레이스 요약
    pub merged_esil_trace: String,
}

pub struct CompatibilityAuditor;

impl CompatibilityAuditor {
    /// 두 사주 시스템 간의 상호작용을 감사(Audit)합니다.
    pub fn audit(system_a: &SajuVM, system_b: &SajuVM) -> CompatibilityAudit {
        let mut synergies = Vec::new();
        let mut conflicts = Vec::new();
        let mut deadlocks = Vec::new();
        let mut merged_trace = String::new();
        
        // 1. 천간 교차 결합 분석 (Interface Mapping)
        let a_stems = [
            ("A_년간", system_a.natal.year.stem),
            ("A_월간", system_a.natal.month.stem),
            ("A_일간", system_a.natal.day.stem),
            ("A_시간", system_a.natal.hour.stem),
        ];
        let b_stems = [
            ("B_년간", system_b.natal.year.stem),
            ("B_월간", system_b.natal.month.stem),
            ("B_일간", system_b.natal.day.stem),
            ("B_시간", system_b.natal.hour.stem),
        ];

        // 2. 지지 교차 결합 분석 (Resource Contention)
        let a_branches = [
            ("A_년지", system_a.natal.year.branch),
            ("A_월지", system_a.natal.month.branch),
            ("A_일지", system_a.natal.day.branch),
            ("A_시지", system_a.natal.hour.branch),
        ];
        let b_branches = [
            ("B_년지", system_b.natal.year.branch),
            ("B_월지", system_b.natal.month.branch),
            ("B_일지", system_b.natal.day.branch),
            ("B_시지", system_b.natal.hour.branch),
        ];

        // 3. 교차 합충 스캔
        use crate::analysis::relationships::*;
        let mut sync_score: f32 = 50.0;

        for (p1, s1) in &a_stems {
            for (p2, s2) in &b_stems {
                if let Some(c) = StemCombination::check(*s1, *s2) {
                    synergies.push(format!("천간합: {} <-> {} ({})", p1, p2, c.transformed_element().hangul()));
                    sync_score += 5.0;
                    merged_trace.push_str(&format!("ipc_joint:{}-{}_hap; ", p1, p2));
                }
                if let Some(_c) = StemClash::check(*s1, *s2) {
                    conflicts.push(format!("천간충: {} <-> {}", p1, p2));
                    sync_score -= 3.0;
                    merged_trace.push_str(&format!("ipc_collision:{}-{}_clash; ", p1, p2));
                }
            }
        }

        for (p1, b1) in &a_branches {
            for (p2, b2) in &b_branches {
                if let Some(_c) = BranchClash::check(*b1, *b2) {
                    deadlocks.push(format!("지지충: {} <-> {}", p1, p2));
                    sync_score -= 5.0;
                    merged_trace.push_str(&format!("resource_lock:{}-{}_deadlock; ", p1, p2));
                }
                if let Some(_s) = SixCombination::check(*b1, *b2) {
                    synergies.push(format!("육합: {} <-> {}", p1, p2));
                    sync_score += 4.0;
                    merged_trace.push_str(&format!("memory_bridge:{}-{}_shared; ", p1, p2));
                }
            }
        }

        CompatibilityAudit {
            sync_score: sync_score.clamp(0.0, 100.0),
            synergies,
            conflicts,
            deadlocks,
            merged_esil_trace: merged_trace,
        }
    }
}
