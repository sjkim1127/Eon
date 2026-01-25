//! LifePathEmulator: 100년 인생 경로를 시뮬레이션하고 분석합니다.
//! 

use serde::{Deserialize, Serialize};
use crate::engine::vm::{SajuVM, LifeFrame};
use crate::core::pillars::FourPillars;
use crate::core::pillars::SajuError;
use eon_core::Gender;
use crate::core::ganzi::GanZi;

/// 100년 인생 시뮬레이션 결과 리포트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifePathReport {
    /// 연도별 프레임 목록
    pub frames: Vec<LifeFrame>,
    /// 인생의 최정점 연도 (나이, 점수)
    pub peak_age: u32,
    /// 인생의 최저점 연도 (나이, 점수)
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

    /// 100년 인생 경로를 에뮬레이션합니다.
    pub fn emulate(&self) -> Result<LifePathReport, SajuError> {
        let mut frames = Vec::new();
        
        // 대운 흐름 계산
        // 정확한 시작 시점을 위해 원국 생성시의 데이터를 가져오는 것이 좋으나, 
        // 여기서는 에뮬레이션의 목적상 간단히 분석용 대운 객체 생성
        let major_luck = self.vm.natal.major_luck(self.gender, self.birth_year, 1, 1, 0, 0)?;
        
        // 0세부터 100세까지 매년 시뮬레이션
        for age in 0..=100 {
            let year = self.birth_year + age as i32;
            let yearly_ganzi = GanZi::from_year(year);
            
            // 현재 나이에 해당하는 대운 찾기
            let major_ganzi = self.get_major_at_age(&major_luck, age);
            
            let frame = self.vm.step(age, yearly_ganzi, major_ganzi, None, None, None);
            frames.push(frame);
        }

        let peak_age = frames.iter().max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
            .map(|f| f.age).unwrap();
        let valley_age = frames.iter().min_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
            .map(|f| f.age).unwrap();

        Ok(LifePathReport {
            frames,
            peak_age,
            valley_age,
        })
    }

    /// 특정 나이에 해당하는 대운 간지를 반환합니다.
    fn get_major_at_age(&self, major: &crate::analysis::major_luck::MajorLuckAnalysis, age: u32) -> GanZi {
        // 대운 시작 전(보통 1~10세 사이)은 월주를 대운으로 보거나 
        // 대운이 아직 시작되지 않은 상태로 보지만, 여기서는 첫 번째 대운 이전은 월주를 사용
        
        // 실제로는 더 정규한 로직이 필요하지만, 단순화를 위해 현재 구간에 맞는 것을 찾음
        let current_cycle = major.cycles.iter()
            .filter(|c| age >= c.start_age)
            .last()
            .map(|c| c.ganzi)
            .unwrap_or(self.vm.natal.month); // 대운 시작 전은 월주 기준

        current_cycle
    }
}
