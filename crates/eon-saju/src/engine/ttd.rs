use crate::engine::emulator::LifePathReport;
use crate::engine::vm::LifeFrame;
use serde::{Deserialize, Serialize};

/// TTD analysis outcome: Root Cause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootCause {
    pub target_age: u32,
    pub root_cause_age: u32,
    pub reason: String,
}

/// TTD analysis outcome: Life path difference (Diff)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeDiff {
    pub age: u32,
    pub score_delta: f32,
    pub added_tags: Vec<String>,
    pub removed_tags: Vec<String>,
}

pub struct DestinyDebugger;

impl DestinyDebugger {
    /// Destiny Backtrace: Reverse trace root cause of a specific tag or state
    pub fn backtrace(report: &LifePathReport, age: u32, target_tag: &str) -> Option<RootCause> {
        let frames = &report.frames;
        if age as usize >= frames.len() {
            return None;
        }

        // Cannot analyze if tag does not exist at current target point
        let current_frame = &frames[age as usize];
        let exists = current_frame
            .tags
            .iter()
            .any(|t| t.contains_pattern(target_tag))
            || current_frame
                .signatures
                .iter()
                .any(|s| s.name.contains(target_tag) || s.id.contains(target_tag));

        if !exists {
            return None;
        }

        let mut root_cause_age = age;
        let mut current_age = age as i32;

        // Traverse backwards to locate entry point where tag first emerged
        while current_age >= 0 {
            let frame = &frames[current_age as usize];
            let has_tag = frame.tags.iter().any(|t| t.contains_pattern(target_tag))
                || frame
                    .signatures
                    .iter()
                    .any(|s| s.name.contains(target_tag) || s.id.contains(target_tag));

            if has_tag {
                root_cause_age = current_age as u32;
            } else {
                break;
            }
            current_age -= 1;
        }

        // Check if root cause corresponds to a major luck shift
        let mut major_change_age = root_cause_age;
        let root_major = frames[root_cause_age as usize].major_ganzi;
        let mut check_age = root_cause_age as i32;
        while check_age >= 0 {
            if frames[check_age as usize].major_ganzi != root_major {
                major_change_age = (check_age + 1) as u32;
                break;
            }
            check_age -= 1;
        }

        Some(RootCause {
            target_age: age,
            root_cause_age: major_change_age.min(root_cause_age),
            reason: format!(
                "Tracing back from where '{}' was observed (age {}), state transitioned starting at age {}.",
                target_tag, root_cause_age, major_change_age.min(root_cause_age)
            ),
        })
    }

    /// State Snapshotting & Diffing: Analyze delta between two life paths
    pub fn diff(report_a: &LifePathReport, report_b: &LifePathReport) -> Vec<LifeDiff> {
        let mut diffs = Vec::new();
        let max_len = report_a.frames.len().min(report_b.frames.len());

        for i in 0..max_len {
            let fa = &report_a.frames[i];
            let fb = &report_b.frames[i];

            let score_delta = fb.score - fa.score;

            let mut added_tags = Vec::new();
            for tag in &fb.tags {
                if !fa.tags.iter().any(|t| t == tag) {
                    added_tags.push(tag.to_string());
                }
            }

            let mut removed_tags = Vec::new();
            for tag in &fa.tags {
                if !fb.tags.iter().any(|t| t == tag) {
                    removed_tags.push(tag.to_string());
                }
            }

            // Record only when significant delta exists
            if score_delta.abs() > 3.0 || !added_tags.is_empty() || !removed_tags.is_empty() {
                diffs.push(LifeDiff {
                    age: fa.age,
                    score_delta,
                    added_tags,
                    removed_tags,
                });
            }
        }

        diffs
    }

    /// Time-Travel Breakpoints: Locate frames matching custom criteria
    pub fn find_breakpoints<F>(report: &LifePathReport, condition: F) -> Vec<u32>
    where
        F: Fn(&LifeFrame) -> bool,
    {
        report
            .frames
            .iter()
            .filter(|f| condition(f))
            .map(|f| f.age)
            .collect()
    }
}
