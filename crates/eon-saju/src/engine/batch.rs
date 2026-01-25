//! 하드웨어 가속 시뮬레이터 (Rayon 기반)
//!
//! 멀티코어 병렬 처리를 통해 수십만 개의 인생 프레임을 수 밀리초 내에 시뮬레이션합니다.

use rayon::prelude::*;
use crate::engine::vm::{SajuVM, LifeFrame};
use crate::core::ganzi::GanZi;
use crate::core::pillars::FourPillars;
use crate::analysis::major_luck::MajorLuckAnalysis;

pub struct BatchSimulator {
    vm: SajuVM,
}

impl BatchSimulator {
    pub fn new(pillars: FourPillars) -> Self {
        Self {
            vm: SajuVM::new(pillars),
        }
    }

    /// 1000년 전수 조사 시뮬레이션 (병렬 처리)
    ///
    /// 특정 연도부터 1000년 동안의 모든 세운을 시뮬레이션합니다.
    pub fn simulate_1000_years(&self, start_year: i32, major_luck: &MajorLuckAnalysis) -> Vec<LifeFrame> {
        let years: Vec<i32> = (start_year..start_year + 1000).collect();
        
        // Rayon을 이용한 병렬 매핑
        years.into_par_iter()
            .map(|year| {
                let age = (year - start_year).abs() as u32; // 간단한 나이 계산
                let yearly_ganzi = GanZi::from_year(year);
                
                // 해당 나이의 대운 간지 추출 (없으면 세운과 동일하게 처리하거나 기본값)
                let major_ganzi = major_luck.at_age(age)
                    .map(|l| l.ganzi)
                    .unwrap_or(yearly_ganzi);

                self.vm.step(age, major_ganzi, yearly_ganzi, None, None, None)
            })
            .collect()
    }

    /// 특정 범위의 연도 시뮬레이션
    pub fn simulate_range(&self, start_year: i32, end_year: i32, major_luck: &MajorLuckAnalysis) -> Vec<LifeFrame> {
        let years: Vec<i32> = (start_year..=end_year).collect();

        years.into_par_iter()
            .map(|year| {
                let age = (year - start_year).abs() as u32;
                let yearly_ganzi = GanZi::from_year(year);
                let major_ganzi = major_luck.at_age(age)
                    .map(|l| l.ganzi)
                    .unwrap_or(yearly_ganzi);

                self.vm.step(age, major_ganzi, yearly_ganzi, None, None, None)
            })
            .collect()
    }
}
