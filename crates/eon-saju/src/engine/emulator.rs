//! LifePathEmulator: Simulates and analyzes 100-year life path trajectory.
//!

use crate::analysis::dynamic_luck::DynamicLuckAnalysis;
use crate::core::element::Element;
use crate::core::ganzi::GanZi;
use crate::core::pillars::{FourPillars, SajuError};
use crate::engine::vm::{LifeFrame, SajuVM};
use eon_core::Gender;
use serde::{Deserialize, Serialize};

/// Time-series life score data for a single year
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YearlyScore {
    pub year: i32,
    pub age: u32,

    // 1. Base Score (simulation score)
    pub total_score: f64,

    // 2. Multi-dimensional Ten Gods scores
    pub wealth_score: f64,
    pub career_score: f64,
    pub academic_score: f64,
    pub health_score: f64,

    // 3. Volatility and metadata
    pub volatility_index: f64,      // 0.0 ~ 100.0
    pub is_transition_period: bool, // Transition period flag (Gyo-un-gi)

    // 4. Moving average data storage
    pub trend_ma_5yr: Option<f64>,
}

/// 100-year life path simulation report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifePathReport {
    /// Time-series score data by year
    pub timeline: Vec<YearlyScore>,
    /// List of frames by year
    pub frames: Vec<LifeFrame>,
    /// Peak life year (age)
    pub peak_age: u32,
    /// Trough life year (age)
    pub valley_age: u32,
}

pub struct LifePathEmulator {
    pub vm: SajuVM,
    pub gender: Gender,
    pub birth_year: i32,
}

impl LifePathEmulator {
    pub fn new(natal: FourPillars, gender: Gender, birth_year: i32) -> Self {
        Self {
            vm: SajuVM::new(natal),
            gender,
            birth_year,
        }
    }

    /// Emulate 100-year life path trajectory
    pub fn emulate(&self) -> Result<LifePathReport, SajuError> {
        let mut timeline = Vec::with_capacity(101);
        let input = &self.vm.natal.raw_input;
        let major_luck = self.vm.natal.major_luck(
            self.gender,
            input.year,
            input.month,
            input.day,
            input.hour,
            input.minute,
        )?;

        // Natal reference data
        let dm_element = self.vm.natal.day_master_element();
        let base_dist = self.vm.natal.element_distribution();
        let primary = self.vm.yongshin.primary;
        let assistant = self.vm.yongshin.assistant;
        let thermal =
            crate::analysis::yongshin::calculate_thermal_index(&self.vm.natal, &self.vm.config);

        let mut frames_buffer = Vec::new();

        for age in 0..=100 {
            let year = self.birth_year + age as i32;
            let yearly_ganzi = GanZi::from_year(year);

            // Find current and adjacent major luck info
            // Precondition: major_luck.cycles is non-empty and sorted ascending by start_age
            // (guaranteed by MajorLuckAnalysis::calculate_astro). Using partition_point gives
            // O(log n) lookup and lets us borrow cycles instead of cloning them.
            let cycle_pos = major_luck.cycles.partition_point(|c| c.start_age <= age);
            let current_major_cycle = if cycle_pos > 0 {
                Some(&major_luck.cycles[cycle_pos - 1])
            } else {
                None
            };
            let start_age_of_major = current_major_cycle.map_or(0, |c| c.start_age);
            let major_ganzi = current_major_cycle
                .map(|c| c.ganzi)
                .unwrap_or(self.vm.natal.month);

            // Calculate volatility index - transition period (Gyo-un-gi)
            let mut dt = (age as i32 - start_age_of_major as i32).abs();
            if dt > 5 {
                // Transition period considers distance to adjacent major luck cycle
                // Time until next major luck cycle
                if let Some(next_c) = major_luck.cycles.get(cycle_pos) {
                    dt = (next_c.start_age as i32 - age as i32).abs();
                }
            }
            let is_transition_period = dt <= 2;

            // T(t) = max(0, 1.0 - (dt / 2.0))
            let t_val = (1.0 - (dt as f64 / 2.0)).max(0.0);

            let mut v_val = 0.0;
            // Vector distance between adjacent major luck cycles
            if is_transition_period {
                // Find previous or next major luck cycle
                let mut alt_ganzi = self.vm.natal.month;
                if age >= start_age_of_major && dt <= 2 && age < start_age_of_major + 3 {
                    // Just started cycle -> difference from previous cycle
                    // current cycle is at cycle_pos - 1; previous is at cycle_pos - 2
                    if cycle_pos >= 2 {
                        alt_ganzi = major_luck.cycles[cycle_pos - 2].ganzi;
                    }
                } else if dt <= 2 {
                    // About to start cycle -> difference from next cycle
                    if let Some(next_c) = major_luck.cycles.get(cycle_pos) {
                        alt_ganzi = next_c.ganzi;
                    }
                }

                // Compute V: Euclidean distance in element weights between two GanZi
                let mut vec_current = [0.0; 5];
                let mut vec_alt = [0.0; 5];
                vec_current[major_ganzi.stem.element() as usize] += 1.0;
                vec_current[major_ganzi.branch.element() as usize] += 1.0;
                vec_alt[alt_ganzi.stem.element() as usize] += 1.0;
                vec_alt[alt_ganzi.branch.element() as usize] += 1.0;

                let mut sq_sum: f64 = 0.0;
                for i in 0..5 {
                    let diff = vec_current[i] - vec_alt[i];
                    sq_sum += diff * diff;
                }
                v_val = sq_sum.sqrt();
            }

            // Extract LifeFrame
            let frame = self
                .vm
                .step(age, major_ganzi, yearly_ganzi, None, None, None);

            // Recompute dynamic analysis for M(X,t) and C(t)
            let dynamic = DynamicLuckAnalysis::analyze(
                &self.vm.natal,
                Some(major_ganzi),
                Some(yearly_ganzi),
                None,
                None,
                None,
            );

            // Yearly luck clash coefficient C(t)
            let mut c_val = 1.0;
            for (_clash, p1, p2) in &dynamic.combined_relations.branch_clashes {
                if p1.contains("day_branch")
                    || p2.contains("day_branch")
                    || p1.contains("month_branch")
                    || p2.contains("month_branch")
                {
                    c_val = 1.5;
                }
            }
            if !dynamic.combined_relations.stem_clashes.is_empty() && c_val > 1.0 {
                c_val = 2.0; // Heavenly & Earthly Double Clash
            }

            let volatility_index = (10.0 + (t_val * v_val * c_val * 20.0)).clamp(0.0, 100.0);

            // --- Multi-dimensional Ten Gods Score calculation ---
            let mut e_scores: [f64; 5] = [0.0; 5];

            for el_idx in 0..5 {
                let el = Element::from_index(el_idx);

                // S(X): Combined strength across natal, major, and yearly luck
                let mut s_val = 0.0;
                s_val += base_dist[el_idx as usize].1 as f64;
                if major_ganzi.stem.element() == el {
                    s_val += 1.0;
                }
                if major_ganzi.branch.element() == el {
                    s_val += 1.0;
                }
                if yearly_ganzi.stem.element() == el {
                    s_val += 1.0;
                }
                if yearly_ganzi.branch.element() == el {
                    s_val += 1.0;
                }

                // F(X): Baseline favorability (-1.0 ~ 1.5)
                let priority = self
                    .vm
                    .get_element_priority(el, primary, assistant, thermal);
                // Normalize 0.0 baseline to 1.0; amplify for yongshin, dampen for gishin
                let mut f_val = 1.0 + (priority as f64 * 1.5);
                if f_val < 0.2 {
                    f_val = 0.2;
                }

                // M(X,t): interaction modifier
                let mut m_val = 1.0;
                // Decrease power when yearly branch clashes with element
                for (_clash, _p1, _p2) in &dynamic.combined_relations.branch_clashes {
                    if yearly_ganzi.branch.element().controls() == el
                        || yearly_ganzi.branch.element().controlled_by() == el
                    {
                        m_val -= 0.3;
                    }
                }
                for (_six, _p1, _p2) in &dynamic.combined_relations.six_combinations {
                    if yearly_ganzi.branch.element() == el {
                        m_val += 0.3;
                    }
                }
                if m_val < 0.4 {
                    m_val = 0.4;
                }
                if m_val > 1.8 {
                    m_val = 1.8;
                }

                e_scores[el_idx as usize] = s_val * f_val * m_val;
            }

            // Category mapping for Ten Gods domains
            let base = 0.0;
            let scale_factor = 25.0; // Score amplification

            // Wealth = Wealth 0.6 + Output 0.4
            let wealth_el = dm_element.controls();
            let shik_el = dm_element.generates();
            let wealth_raw =
                (e_scores[wealth_el as usize] * 0.6) + (e_scores[shik_el as usize] * 0.4);
            let wealth_score = (base + (wealth_raw * scale_factor)).clamp(0.0, 100.0);

            // Career = Officer 0.7 + Resource 0.3
            let career_el = dm_element.controlled_by();
            let yin_el = dm_element.generated_by();
            let career_raw =
                (e_scores[career_el as usize] * 0.7) + (e_scores[yin_el as usize] * 0.3);
            let career_score = (base + (career_raw * scale_factor)).clamp(0.0, 100.0);

            // Academic = Resource 0.7 + Output 0.3
            let academic_raw =
                (e_scores[yin_el as usize] * 0.7) + (e_scores[shik_el as usize] * 0.3);
            let academic_score = (base + (academic_raw * scale_factor)).clamp(0.0, 100.0);

            // Health = Companion 0.5 + Output 0.5
            let health_raw =
                (e_scores[dm_element as usize] * 0.5) + (e_scores[shik_el as usize] * 0.5);
            let health_score = (base + (health_raw * scale_factor)).clamp(0.0, 100.0);

            // Unified total score: VM simulation score + average of 4 domains
            let vm_score = frame.score as f64;
            let integrated_total =
                ((vm_score + wealth_score + career_score + academic_score + health_score) / 5.0)
                    .clamp(0.0, 100.0);

            timeline.push(YearlyScore {
                year,
                age,
                total_score: integrated_total,
                wealth_score,
                career_score,
                academic_score,
                health_score,
                volatility_index,
                is_transition_period,
                trend_ma_5yr: None, // Computed in batch afterwards
            });
            frames_buffer.push(frame);
        }

        if timeline.is_empty() {
            return Err(SajuError::CalculationError(
                "No emulation frames generated".to_string(),
            ));
        }

        // --- 4. Time series score smoothing: 5-year Moving Average ---
        for i in 0..timeline.len() {
            let start = i.saturating_sub(2);
            let end = (i + 2).min(timeline.len() - 1);
            let mut sum = 0.0;
            let mut count = 0.0;

            for j in start..=end {
                sum += timeline[j].total_score;
                count += 1.0;
            }

            timeline[i].trend_ma_5yr = Some(sum / count);
        }

        let peak_age = timeline
            .iter()
            .max_by(|a, b| {
                a.total_score
                    .partial_cmp(&b.total_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|t| t.age)
            .unwrap_or(0);

        let valley_age = timeline
            .iter()
            .min_by(|a, b| {
                a.total_score
                    .partial_cmp(&b.total_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|t| t.age)
            .unwrap_or(0);

        Ok(LifePathReport {
            timeline,
            frames: frames_buffer,
            peak_age,
            valley_age,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pillars::SajuInput;

    #[test]
    fn test_multi_dimensional_scoring() {
        let input = SajuInput::new_solar(1990, 5, 10, 12, 0);
        let natal = FourPillars::calculate(&input).unwrap();
        let emulator = LifePathEmulator::new(natal, Gender::Male, 1990);

        let report = emulator.emulate().unwrap();
        assert_eq!(report.timeline.len(), 101);
        assert_eq!(report.frames.len(), 101);

        let yr = report.timeline.iter().find(|t| t.age == 30).unwrap();

        println!("YearlyScore at age 30: {:#?}", yr);
        assert!(yr.wealth_score >= 0.0);
        assert!(yr.career_score >= 0.0);
        assert!(yr.academic_score >= 0.0);
        assert!(yr.health_score >= 0.0);
        assert!(yr.volatility_index >= 0.0);
    }
}
