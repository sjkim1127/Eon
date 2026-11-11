//! Lifecycle Statistics and Golden Era Analytics Engine
//!
//! Extracts peak life periods and score distributions from simulation data.

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
    /// Extract golden era from simulation timeline using a sliding window
    pub fn find_golden_time(timeline: &[YearlyScore], window_size: usize) -> Option<GoldenTime> {
        if timeline.len() < window_size {
            return None;
        }

        let mut max_avg = -1.0;
        let mut best_start = 0;

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
                "Ages {} to {} represent the highest-density golden period.",
                timeline[best_start].age,
                timeline[best_start + window_size - 1].age
            ),
        })
    }

    /// Analyze score distribution across the entire timeline
    pub fn analyze_distribution(timeline: &[YearlyScore]) -> Vec<(u32, f32)> {
        timeline
            .iter()
            .map(|f| (f.age, f.total_score as f32))
            .collect()
    }
}
