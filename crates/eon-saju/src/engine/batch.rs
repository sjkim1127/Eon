//! Hardware-accelerated batch simulator
//!
//! Simulates hundreds of thousands of life frames in milliseconds via multi-core parallel processing.
//! Automatically falls back to sequential execution in WASM environments.

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::analysis::major_luck::MajorLuckAnalysis;
use crate::core::ganzi::GanZi;
use crate::core::pillars::FourPillars;
use crate::engine::vm::{LifeFrame, SajuVM};

pub struct BatchSimulator {
    vm: SajuVM,
}

impl BatchSimulator {
    pub fn new(pillars: FourPillars) -> Self {
        Self {
            vm: SajuVM::new(pillars),
        }
    }

    /// Simulate 1,000 years exhaustive life frames
    pub fn simulate_1000_years(
        &self,
        start_year: i32,
        major_luck: &MajorLuckAnalysis,
    ) -> Vec<LifeFrame> {
        let years: Vec<i32> = (start_year..start_year + 1000).collect();

        let mapper = |&year: &i32| {
            let age = (year - start_year).unsigned_abs();
            let yearly_ganzi = GanZi::from_year(year);
            let major_ganzi = major_luck
                .cycle_for_year(year)
                .map(|l| l.ganzi)
                .unwrap_or(yearly_ganzi);
            self.vm
                .step(age, major_ganzi, yearly_ganzi, None, None, None)
        };

        #[cfg(feature = "parallel")]
        {
            years.par_iter().map(mapper).collect()
        }

        #[cfg(not(feature = "parallel"))]
        {
            years.iter().map(mapper).collect()
        }
    }

    /// Simulate specified range of years
    pub fn simulate_range(
        &self,
        start_year: i32,
        end_year: i32,
        major_luck: &MajorLuckAnalysis,
    ) -> Vec<LifeFrame> {
        let years: Vec<i32> = (start_year..=end_year).collect();

        let mapper = |&year: &i32| {
            let age = (year - start_year).unsigned_abs();
            let yearly_ganzi = GanZi::from_year(year);
            let major_ganzi = major_luck
                .cycle_for_year(year)
                .map(|l| l.ganzi)
                .unwrap_or(yearly_ganzi);
            self.vm
                .step(age, major_ganzi, yearly_ganzi, None, None, None)
        };

        #[cfg(feature = "parallel")]
        {
            years.par_iter().map(mapper).collect()
        }

        #[cfg(not(feature = "parallel"))]
        {
            years.iter().map(mapper).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::branch::EarthlyBranch as B;
    use crate::core::pillars::SajuInput;
    use crate::core::stem::HeavenlyStem as S;
    use eon_core::Gender;

    #[test]
    fn batch_major_luck_follows_cycle_start_date_not_age() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        let major_luck = pillars
            .major_luck(Gender::Male, 2004, 11, 27, 22, 0)
            .unwrap();
        let sim = BatchSimulator::new(pillars);
        let frames = sim.simulate_range(2004, 2030, &major_luck);
        let major_at = |year: i32| frames[(year - 2004) as usize].major_ganzi;

        // 丁丑 runs 2017-12-27..2027-12-27: 2027 is still 丁丑, 2028 is 戊寅.
        assert_eq!(major_at(2018), GanZi::new(S::Ding, B::Chou));
        assert_eq!(major_at(2027), GanZi::new(S::Ding, B::Chou));
        assert_eq!(major_at(2028), GanZi::new(S::Wu, B::Yin));
    }
}
