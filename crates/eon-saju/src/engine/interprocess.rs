//! Inter-Process Compatibility (IPC) Audit Module
//!
//! Analyzes energy interference, resource contention (clash),
//! and synergies (combination) when two Saju systems (SajuVM) interact.

use crate::engine::vm::SajuVM;
use serde::{Deserialize, Serialize};

/// IPC analysis audit report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityAudit {
    /// Composite interaction score (0~100)
    pub sync_score: f32,
    /// Cross-system synergies
    pub synergies: Vec<String>,
    /// Cross-system conflicts (Race conditions / conflicts)
    pub conflicts: Vec<String>,
    /// Resource deadlocks
    pub deadlocks: Vec<String>,
    /// Combined ESIL trace summary upon joining both systems
    pub merged_esil_trace: String,
}

pub struct CompatibilityAuditor;

impl CompatibilityAuditor {
    /// Audit cross-system interaction between two SajuVM systems
    pub fn audit(system_a: &SajuVM, system_b: &SajuVM) -> CompatibilityAudit {
        let mut synergies = Vec::new();
        let mut conflicts = Vec::new();
        let mut deadlocks = Vec::new();
        let mut merged_trace = String::new();

        // 1. Cross-stem interface mapping
        let mut a_stems = vec![
            ("A_year_stem", system_a.natal.year.stem),
            ("A_month_stem", system_a.natal.month.stem),
            ("A_day_stem", system_a.natal.day.stem),
        ];
        if let Some(h) = system_a.natal.hour {
            a_stems.push(("A_hour_stem", h.stem));
        }
        let mut b_stems = vec![
            ("B_year_stem", system_b.natal.year.stem),
            ("B_month_stem", system_b.natal.month.stem),
            ("B_day_stem", system_b.natal.day.stem),
        ];
        if let Some(h) = system_b.natal.hour {
            b_stems.push(("B_hour_stem", h.stem));
        }

        // 2. Cross-branch resource contention analysis
        let mut a_branches = vec![
            ("A_year_branch", system_a.natal.year.branch),
            ("A_month_branch", system_a.natal.month.branch),
            ("A_day_branch", system_a.natal.day.branch),
        ];
        if let Some(h) = system_a.natal.hour {
            a_branches.push(("A_hour_branch", h.branch));
        }
        let mut b_branches = vec![
            ("B_year_branch", system_b.natal.year.branch),
            ("B_month_branch", system_b.natal.month.branch),
            ("B_day_branch", system_b.natal.day.branch),
        ];
        if let Some(h) = system_b.natal.hour {
            b_branches.push(("B_hour_branch", h.branch));
        }

        // 3. Scan cross-system combinations and clashes
        use crate::analysis::relationships::*;
        let mut sync_score: f32 = 50.0;

        for (p1, s1) in &a_stems {
            for (p2, s2) in &b_stems {
                if let Some(c) = StemCombination::check(*s1, *s2) {
                    synergies.push(format!(
                        "Stem Combination: {} <-> {} ({})",
                        p1,
                        p2,
                        c.transformed_element().hangul()
                    ));
                    sync_score += 5.0;
                    merged_trace.push_str(&format!("ipc_joint:{}-{}_hap; ", p1, p2));
                }
                if let Some(_c) = StemClash::check(*s1, *s2) {
                    conflicts.push(format!("Stem Clash: {} <-> {}", p1, p2));
                    sync_score -= 3.0;
                    merged_trace.push_str(&format!("ipc_collision:{}-{}_clash; ", p1, p2));
                }
            }
        }

        for (p1, b1) in &a_branches {
            for (p2, b2) in &b_branches {
                if let Some(_c) = BranchClash::check(*b1, *b2) {
                    deadlocks.push(format!("Branch Clash: {} <-> {}", p1, p2));
                    sync_score -= 5.0;
                    merged_trace.push_str(&format!("resource_lock:{}-{}_deadlock; ", p1, p2));
                }
                if let Some(_s) = SixCombination::check(*b1, *b2) {
                    synergies.push(format!("Six Combination: {} <-> {}", p1, p2));
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
