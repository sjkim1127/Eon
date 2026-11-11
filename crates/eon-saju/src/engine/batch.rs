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
                .at_age(age)
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
                .at_age(age)
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
