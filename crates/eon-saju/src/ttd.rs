use serde::{Deserialize, Serialize};
use crate::vm::LifeFrame;
use crate::emulator::LifePathReport;

/// TTD 분석 결과: 근본 원인 (Root Cause)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootCause {
    pub target_age: u32,
    pub root_cause_age: u32,
    pub reason: String,
}

/// TTD 분석 결과: 인생 경로 차이 (Diff)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeDiff {
    pub age: u32,
    pub score_delta: f32,
    pub added_tags: Vec<String>,
    pub removed_tags: Vec<String>,
}

pub struct DestinyDebugger;

impl DestinyDebugger {
    /// Destiny Backtrace: 특정 시점의 태그/상태의 근본 원인을 역방향 탐색
    pub fn backtrace(report: &LifePathReport, age: u32, target_tag: &str) -> Option<RootCause> {
        let frames = &report.frames;
        if age as usize >= frames.len() { return None; }

        // 현재 시점에 해당 태그가 없으면 분석 불가
        let current_frame = &frames[age as usize];
        let exists = current_frame.tags.iter().any(|t| t.contains(target_tag)) || 
                     current_frame.signatures.iter().any(|s| s.name.contains(target_tag) || s.id.contains(target_tag));
        
        if !exists { return None; }

        let mut root_cause_age = age;
        let mut current_age = age as i32;

        // 역방향으로 탐색하며 해당 태그가 처음 나타난 시점(Entry Point)을 찾음
        while current_age >= 0 {
            let frame = &frames[current_age as usize];
            let has_tag = frame.tags.iter().any(|t| t.contains(target_tag)) || 
                          frame.signatures.iter().any(|s| s.name.contains(target_tag) || s.id.contains(target_tag));
            
            if has_tag {
                root_cause_age = current_age as u32;
            } else {
                // 태그가 더 이상 발견되지 않으면 직전 시점이 진입점
                break;
            }
            current_age -= 1;
        }

        // 해당 시점의 대운(Major Luck)이 바뀐 시점인지 확인 (보통 대운이 근본 원인인 경우가 많음)
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
                "'{}' 현상이 포착된 지점({}세)으로부터 역추적한 결과, {}세 기점으로 로직이 변동되었습니다.", 
                target_tag, root_cause_age, major_change_age.min(root_cause_age)
            ),
        })
    }

    /// State Snapshoting & Diffing: 두 인생 경로의 차이 분석
    pub fn diff(report_a: &LifePathReport, report_b: &LifePathReport) -> Vec<LifeDiff> {
        let mut diffs = Vec::new();
        let max_len = report_a.frames.len().min(report_b.frames.len());

        for i in 0..max_len {
            let fa = &report_a.frames[i];
            let fb = &report_b.frames[i];

            let score_delta = fb.score - fa.score;
            
            let mut added_tags = Vec::new();
            for tag in &fb.tags {
                if !fa.tags.contains(tag) {
                    added_tags.push(tag.clone());
                }
            }

            let mut removed_tags = Vec::new();
            for tag in &fa.tags {
                if !fb.tags.contains(tag) {
                    removed_tags.push(tag.clone());
                }
            }

            // 유의미한 차이가 있는 경우에만 기록 (델타 분석)
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

    /// Time-Travel Breakpoints: 특정 조건을 만족하는 시점들 탐색
    pub fn find_breakpoints<F>(report: &LifePathReport, condition: F) -> Vec<u32>
    where F: Fn(&LifeFrame) -> bool {
        report.frames.iter()
            .filter(|f| condition(f))
            .map(|f| f.age)
            .collect()
    }
}
