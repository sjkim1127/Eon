//! 하드웨어 가속 시뮬레이터
//!
//! 멀티코어 병렬 처리를 통해 수십만 개의 인생 프레임을 수 밀리초 내에 시뮬레이션합니다.
//! WASM 환경에서는 순차 처리로 자동 폴백됩니다.

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

    /// 1000년 전수 조사 시뮬레이션
    pub fn simulate_1000_years(
        &self,
        start_year: i32,
        major_luck: &MajorLuckAnalysis,
    ) -> Vec<LifeFrame> {
        let years: Vec<i32> = (start_year..start_year + 1000).collect();

        let mapper = |&year: &i32| {
            let age = (year - start_year).abs() as u32;
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

    /// 특정 범위의 연도 시뮬레이션
    pub fn simulate_range(
        &self,
        start_year: i32,
        end_year: i32,
        major_luck: &MajorLuckAnalysis,
    ) -> Vec<LifeFrame> {
        let years: Vec<i32> = (start_year..=end_year).collect();

        let mapper = |&year: &i32| {
            let age = (year - start_year).abs() as u32;
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
