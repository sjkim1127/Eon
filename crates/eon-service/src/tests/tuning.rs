use crate::dto::*;
use crate::facade;
use chrono::{Utc, TimeZone};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Benchmark/Tuning only
    fn test_tier_v3_tuning() {
        use std::collections::BTreeMap;
        let mut tier_counts = BTreeMap::new();
        let mut component_totals = BTreeMap::new();
        let mut raw_scores = Vec::new();
        let mut tier_scores = Vec::new();

        let sample_count = 500;
        let now = Utc::now();

        for i in 0..sample_count {
            let year = 1950 + (i % 71);
            let month = 1 + (i % 12);
            let day = 1 + (i % 28);
            let hour = i % 24;
            let lon = 126.978 + (i as f64 * 0.001);
            let lat = 37.566 + (i as f64 * 0.001);

            let birth = AnalysisInput {
                year, month: month as u32, day: day as u32, hour: hour as u32, minute: 0,
                is_lunar: false, is_leap_month: false,
                lat, lon, timezone: "Asia/Seoul".to_string(),
            };

            let saju_input = SajuAnalysisInput {
                base: birth.clone(),
                is_male: i % 2 == 0,
                use_night_rat_hour: false,
                precision: BirthTimePrecision::Exact,
            };

            let vedic_input = VedicAnalysisInput::new(
                birth.year, birth.month, birth.day, birth.hour, birth.minute,
                birth.is_lunar, birth.is_leap_month,
                birth.lat, birth.lon, birth.timezone.clone(),
                Some(false), Some(now)
            );

            let saju_res = facade::analyze_saju(saju_input.clone()).expect("Saju failed");
            let vedic_res = facade::analyze_vedic(vedic_input).expect("Vedic failed");

            let transit_res = facade::analyze_transit(TransitAnalysisInput::new(
                birth.year, birth.month, birth.day, birth.hour, birth.minute,
                birth.is_lunar, birth.is_leap_month,
                saju_input.is_male, saju_input.use_night_rat_hour,
                birth.lon, birth.lat, birth.timezone.clone(),
                Some(false), Some(now)
            )).ok();

            let tier_res = facade::analyze_destiny_tier(saju_res, vedic_res, transit_res).unwrap();

            *tier_counts.entry(tier_res.destiny_tier.grade.clone()).or_insert(0) += 1;
            raw_scores.push(tier_res.destiny_raw_score);
            tier_scores.push(tier_res.destiny_score);

            for comp in tier_res.detailed_components {
                let entry = component_totals.entry(comp.label.clone()).or_insert((0.0, 0.0));
                entry.0 += comp.score;
                entry.1 += 1.0;
            }
        }

        println!("\n=== Destiny Tier v3 Tuning Report (Samples: {}) ===", sample_count);
        println!("Tier Distribution:");
        for grade in ["S+", "S", "A+", "A", "B+", "B", "C+", "C", "D+", "D", "E", "F"] {
            let count = tier_counts.get(grade).unwrap_or(&0);
            println!("  {:>3}: {:>4} ({:>5.1}%)", grade, count, (*count as f32 / sample_count as f32) * 100.0);
        }

        let avg_raw = raw_scores.iter().sum::<f32>() / sample_count as f32;
        let avg_tier = tier_scores.iter().sum::<f32>() / sample_count as f32;
        println!("\nScore Stats:");
        println!("  Avg Raw Score:  {:.2}", avg_raw);
        println!("  Avg Tier Score: {:.2}", avg_tier);

        println!("\nComponent Averages (0-100 scale):");
        for (label, (total, count)) in &component_totals {
            println!("  {:<15}: {:.2}", label, total / count);
        }
        println!("================================================\n");
    }
}
