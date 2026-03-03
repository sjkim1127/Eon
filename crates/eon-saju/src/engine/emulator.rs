//! LifePathEmulator: 100년 인생 경로를 시뮬레이션하고 분석합니다.
//!

use crate::analysis::dynamic_luck::DynamicLuckAnalysis;
use crate::core::element::Element;
use crate::core::ganzi::GanZi;
use crate::core::pillars::{FourPillars, SajuError};
use crate::engine::vm::{LifeFrame, SajuVM};
use eon_core::Gender;
use serde::{Deserialize, Serialize};

/// 1년 단위의 시계열 인생 점수 데이터
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearlyScore {
    pub year: i32,
    pub age: u32,

    // 1. Base Score (기존 종합 점수)
    pub total_score: f64,

    // 2. 다차원 십성 점수
    pub wealth_score: f64,
    pub career_score: f64,
    pub academic_score: f64,
    pub health_score: f64,

    // 3. 변동성 및 메타데이터
    pub volatility_index: f64,      // 0.0 ~ 100.0
    pub is_transition_period: bool, // 교운기 여부 플래그

    // 4. 이동평균선 데이터 보관용
    pub trend_ma_5yr: Option<f64>,
}

/// 100년 인생 시뮬레이션 결과 리포트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifePathReport {
    /// 연도별 시계열 점수 데이터
    pub timeline: Vec<YearlyScore>,
    /// 연도별 프레임 목록
    pub frames: Vec<LifeFrame>,
    /// 인생의 최정점 연도 (나이)
    pub peak_age: u32,
    /// 인생의 최저점 연도 (나이)
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

        // 원국 기준 필요 데이터
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

            // 현재 대운 및 주변 대운 정보 탐색
            let mut current_major_cycle = None;
            let mut start_age_of_major = 0;
            for c in &major_luck.cycles {
                if age >= c.start_age {
                    current_major_cycle = Some(c.clone());
                    start_age_of_major = c.start_age;
                }
            }
            let major_ganzi = current_major_cycle
                .as_ref()
                .map(|c| c.ganzi)
                .unwrap_or(self.vm.natal.month);

            // 볼라틸리티(변동성, Volatility) 인덱스 계산 - 교운기
            let mut dt = (age as i32 - start_age_of_major as i32).abs();
            if dt > 5 {
                // 교운기는 대운 시작 전후이므로 나중 대운과의 거리도 고려
                // 다음 대운까지 남은 시간
                if let Some(next_c) = major_luck.cycles.iter().find(|c| c.start_age > age) {
                    dt = (next_c.start_age as i32 - age as i32).abs();
                }
            }
            let is_transition_period = dt <= 2;

            // T(t) = max(0, 1.0 - (dt / 2.0))
            let t_val = (1.0 - (dt as f64 / 2.0)).max(0.0);

            let mut v_val = 0.0;
            // 이전 대운과 현재 대운(또는 새 대운) 사이의 벡터 거리
            if is_transition_period {
                // 이전 대운 또는 다음 대운 찾기
                let mut alt_ganzi = self.vm.natal.month;
                if age >= start_age_of_major && dt <= 2 && age < start_age_of_major + 3 {
                    // 방금 시작한 교운기 -> 이전 대운과의 차이
                    if let Some(prev_idx) = major_luck
                        .cycles
                        .iter()
                        .position(|c| c.start_age == start_age_of_major)
                        .unwrap_or(0)
                        .checked_sub(1)
                    {
                        alt_ganzi = major_luck.cycles[prev_idx].ganzi;
                    }
                } else if dt <= 2 {
                    // 곧 시작할 교운기 -> 다음 대운과의 차이
                    if let Some(next_c) = major_luck.cycles.iter().find(|c| c.start_age > age) {
                        alt_ganzi = next_c.ganzi;
                    }
                }

                // V 계산: 두 간지의 오행 차이 유클리디안 거리
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

            // LifeFrame 추출
            let frame = self
                .vm
                .step(age, major_ganzi, yearly_ganzi, None, None, None);

            // 다이나믹 분석 재생성 (M(X,t)와 C(t) 계산을 위함)
            let dynamic = DynamicLuckAnalysis::analyze(
                &self.vm.natal,
                Some(major_ganzi),
                Some(yearly_ganzi),
                None,
                None,
                None,
            );

            // 세운 충돌 계수 C(t)
            let mut c_val = 1.0;
            for (_clash, p1, p2) in &dynamic.combined_relations.branch_clashes {
                if p1.contains("일지")
                    || p2.contains("일지")
                    || p1.contains("월지")
                    || p2.contains("월지")
                {
                    c_val = 1.5;
                }
            }
            if !dynamic.combined_relations.stem_clashes.is_empty() && c_val > 1.0 {
                c_val = 2.0; // 천극지충
            }

            let volatility_index = (10.0 + (t_val * v_val * c_val * 20.0)).clamp(0.0, 100.0);

            // --- 다차원 십성 점수(Ten Gods Multi-dimensional Score) 계산 ---
            let mut e_scores: [f64; 5] = [0.0; 5];

            for el_idx in 0..5 {
                let el = Element::from_index(el_idx);

                // S(X): 원국 + 대운 + 세운의 기본 세력
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

                // F(X): 기본 우호도 (-1.0 ~ 1.5)
                let priority = self
                    .vm
                    .get_element_priority(el, primary, assistant, thermal);
                // 0.0 기준을 1.0(보통)으로 맞추고 용신일때 1.5, 기신일때 0.5 등으로 스케일링
                // get_element_priority는 1.0, 0.5, -0.8 등을 반환
                let mut f_val = 1.0 + (priority as f64 * 0.5);
                if f_val < 0.2 {
                    f_val = 0.2;
                } // 최소값 보장

                // M(X,t): 합충형해파 변동
                let mut m_val = 1.0;
                // 세운 지지가 충당하는 원국/대운 요소의 오행일 때 세력 감소
                for (_clash, _p1, _p2) in &dynamic.combined_relations.branch_clashes {
                    // 충 발생 시 해당되는 오행(지장간 본기 기준)에 영향
                    // 간략화를 위해 세운 지지의 오행과 충하는 오행이 el인지 확인
                    if yearly_ganzi.branch.element().controls() == el
                        || yearly_ganzi.branch.element().controlled_by() == el
                    {
                        m_val -= 0.3;
                    }
                }
                for (_six, _p1, _p2) in &dynamic.combined_relations.six_combinations {
                    if yearly_ganzi.branch.element() == el {
                        m_val += 0.3;
                    } // 합으로 세력 강화
                }
                if m_val < 0.4 {
                    m_val = 0.4;
                }
                if m_val > 1.8 {
                    m_val = 1.8;
                }

                e_scores[el_idx as usize] = s_val * f_val * m_val;
            }

            // 카테고리별 핵심 십성 매핑 (Base 50점 + E(X) 가중치 합산 * scale)
            let base = 30.0;
            let scale_factor = 5.0; // 점수 증폭

            // 재물(Wealth) = 재성 0.6 + 식상 0.4
            let wealth_el = dm_element.controls();
            let shik_el = dm_element.generates();
            let wealth_raw =
                (e_scores[wealth_el as usize] * 0.6) + (e_scores[shik_el as usize] * 0.4);
            let wealth_score = (base + (wealth_raw * scale_factor)).clamp(0.0, 100.0);

            // 직업/명예(Career) = 관성 0.7 + 인성 0.3
            let career_el = dm_element.controlled_by();
            let yin_el = dm_element.generated_by();
            let career_raw =
                (e_scores[career_el as usize] * 0.7) + (e_scores[yin_el as usize] * 0.3);
            let career_score = (base + (career_raw * scale_factor)).clamp(0.0, 100.0);

            // 학업/문서(Academic) = 인성 0.7 + 식상 0.3
            let academic_raw =
                (e_scores[yin_el as usize] * 0.7) + (e_scores[shik_el as usize] * 0.3);
            let academic_score = (base + (academic_raw * scale_factor)).clamp(0.0, 100.0);

            // 건강/독립(Health) = 비겁 0.5 + 식상 0.5
            let health_raw =
                (e_scores[dm_element as usize] * 0.5) + (e_scores[shik_el as usize] * 0.5);
            let health_score = (base + (health_raw * scale_factor)).clamp(0.0, 100.0);

            timeline.push(YearlyScore {
                year,
                age,
                total_score: frame.score as f64,
                wealth_score,
                career_score,
                academic_score,
                health_score,
                volatility_index,
                is_transition_period,
                trend_ma_5yr: None, // 뒤에서 일괄 계산
            });
            frames_buffer.push(frame);
        }

        if timeline.is_empty() {
            return Err(SajuError::CalculationError(
                "No emulation frames generated".to_string(),
            ));
        }

        // --- 4. 시계열 점수 보정 (Smoothing & Momentum) ---
        // 5년 이동평균(MA) 선 (과거 2년 + 현재 + 미래 2년)
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
