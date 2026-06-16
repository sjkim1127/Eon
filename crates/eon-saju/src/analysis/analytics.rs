//! 인생 통계 및 골든 타임 분석 엔진
//!
//! 시뮬레이션 데이터를 바탕으로 인생의 전성기(Peak)를 추출합니다.

use crate::engine::emulator::YearlyScore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoldenTime {
    pub start_age: u32,
    pub end_age: u32,
    pub average_score: f32,
    pub description: String,
}

pub struct Analyzer;

impl Analyzer {
    /// 시뮬레이션 데이터에서 골든 타임 추출 (Sliding Window)
    pub fn find_golden_time(timeline: &[YearlyScore], window_size: usize) -> Option<GoldenTime> {
        if timeline.len() < window_size {
            return None;
        }

        let mut max_avg = -1.0;
        let mut best_start = 0;

        // Sliding Window Average 계산
        for i in 0..=(timeline.len() - window_size) {
            let window = &timeline[i..i + window_size];
            let sum: f64 = window.iter().map(|f| f.total_score).sum();
            let avg = sum as f32 / window_size as f32;

            if avg > max_avg {
                max_avg = avg;
                best_start = i;
            }
        }

        Some(GoldenTime {
            start_age: timeline[best_start].age,
            end_age: timeline[best_start + window_size - 1].age,
            average_score: max_avg,
            description: format!(
                "{}세부터 {}세까지 가장 운의 밀도가 높은 골든 타임입니다.",
                timeline[best_start].age,
                timeline[best_start + window_size - 1].age
            ),
        })
    }

    /// 전체 시뮬레이션 기간 중 득점 분포 분석
    pub fn analyze_distribution(timeline: &[YearlyScore]) -> Vec<(u32, f32)> {
        timeline
            .iter()
            .map(|f| (f.age, f.total_score as f32))
            .collect()
    }
}
