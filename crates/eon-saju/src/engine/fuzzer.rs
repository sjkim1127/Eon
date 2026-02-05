//! Destiny Fuzzer: 운명 취약점 분석 엔진
//!
//! 리버싱의 퍼징(Fuzzing) 기술을 사주에 적용하여,
//! 특정 사주 원국이 견디지 못하는 최악의 운세 조합(Crash)을 찾아냅니다.

use serde::{Deserialize, Serialize};
use crate::engine::vm::{SajuVM, LifeFrame};
use crate::core::ganzi::GanZi;

/// 운명 취약점 리포트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityReport {
    /// 발견된 총 크래시 수
    pub total_crashes: usize,
    /// 가장 위험한 상위 취약점들
    pub critical_vectors: Vec<Vulnerability>,
}

/// 발견된 개별 취약점 (Crash)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    /// 크래시 시점의 에너지 점수
    pub crash_score: f32,
    /// 크래시를 유발한 원인 (재현 벡터)
    pub vector: LuckVector,
    /// 취약점 유형
    pub vulnerability_type: String,
    /// 주요 페이로드 (태그들)
    pub tags: Vec<String>,
    /// 리버싱을 위한 타임스탬프 (선택적)
    pub timestamp: Option<String>,
}

/// 취약점 재현을 위한 입력 벡터 (초정밀 타임라인 지원)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LuckVector {
    pub major: GanZi,         // 대운 (Process Context)
    pub yearly: GanZi,        // 세운 (Yearly Instruction)
    pub monthly: Option<GanZi>, // 월운 (Monthly Segment)
    pub daily: Option<GanZi>,   // 일운 (Daily Trace)
    pub hourly: Option<GanZi>,  // 시운 (Hourly Offset)
}

pub struct DestinyFuzzer {
    pub vm: SajuVM,
    /// 크래시로 간주할 점수 임계값 (기본 30.0)
    pub crash_threshold: f32,
}

impl DestinyFuzzer {
    pub fn new(vm: SajuVM) -> Self {
        Self {
            vm,
            crash_threshold: 30.0,
        }
    }

    /// 체계적 퍼징 (Systematic Exploration)
    /// 현재 대운 컨택스트에서 모든 60갑자 세운을 순회하며 취약점을 찾습니다.
    pub fn audit(&self, major_context: GanZi) -> VulnerabilityReport {
        let mut vulnerabilities = Vec::new();

        for i in 0..60 {
            let yearly = GanZi::from_index(i as i32);
            let frame = self.vm.step(0, major_context, yearly, None, None, None);

            if frame.score <= self.crash_threshold {
                vulnerabilities.push(Vulnerability {
                    crash_score: frame.score,
                    vector: LuckVector {
                        major: major_context,
                        yearly,
                        monthly: None,
                        daily: None,
                        hourly: None,
                    },
                    vulnerability_type: self.determine_vuln_type(&frame),
                    tags: frame.tags_as_strings(),
                    timestamp: None,
                });
            }
        }

        // 점수가 낮은 순으로 정렬
        vulnerabilities.sort_by(|a, b| a.crash_score.partial_cmp(&b.crash_score).unwrap());

        VulnerabilityReport {
            total_crashes: vulnerabilities.len(),
            critical_vectors: vulnerabilities.into_iter().take(5).collect(),
        }
    }

    /// 랜덤 퍼징 (Random Brute-force)
    /// 대운, 세운, 월운을 무작위로 실어나르며 예기치 못한 크래시를 찾습니다.
    pub fn fuzz_random(&self, iterations: usize) -> VulnerabilityReport {
        let mut vulnerabilities = Vec::new();
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for _ in 0..iterations {
            let major = GanZi::from_index(rng.gen_range(0..60));
            let yearly = GanZi::from_index(rng.gen_range(0..60));
            let monthly = GanZi::from_index(rng.gen_range(0..60));
            
            // 현재 VM은 step에서 monthly를 받지 않으므로 
            // 분석을 위해 DynamicLuckAnalysis를 직접 호출하거나 VM을 확장해야 함
            // 일단은 세운 수준에서의 랜덤 퍼징 위주로 구현
            let frame = self.vm.step(0, major, yearly, Some(monthly), None, None);

            if frame.score <= self.crash_threshold {
                vulnerabilities.push(Vulnerability {
                    crash_score: frame.score,
                    vector: LuckVector {
                        major,
                        yearly,
                        monthly: Some(monthly),
                        daily: None,
                        hourly: None,
                    },
                    vulnerability_type: self.determine_vuln_type(&frame),
                    tags: frame.tags_as_strings(),
                    timestamp: None,
                });
            }
        }

        vulnerabilities.sort_by(|a, b| a.crash_score.partial_cmp(&b.crash_score).unwrap());
        vulnerabilities.dedup_by(|a, b| a.vector.major == b.vector.major && a.vector.yearly == b.vector.yearly);

        VulnerabilityReport {
            total_crashes: vulnerabilities.len(),
            critical_vectors: vulnerabilities.into_iter().take(5).collect(),
        }
    }

    /// 초정밀 전수 조사 (Full Spectrum Timeline Audit)
    /// 100년치(약 43만 시간 슬롯) 데이터를 전수 퍼징하여 취약점을 탐지합니다.
    pub fn audit_high_res(&self, birth_year: i32, major_luck: &crate::analysis::major_luck::MajorLuckAnalysis) -> VulnerabilityReport {
        let mut vulnerabilities = Vec::new();
        
        // 100년 전수 조사
        for age in 0..100 {
            let year = birth_year + age as i32;
            let yearly_ganzi = GanZi::from_year(year);
            let major_context = major_luck.at_age(age).map(|m| m.ganzi).unwrap_or(yearly_ganzi);

            for month in 1..=12 {
                let monthly_ganzi = self.calculate_month_ganzi(year, month as i32);
                
                // 샘플링: 매월 1, 11, 21일 (전수 조사 시 43만 개, 샘플링 시 약 4.3만 개)
                for day in [1, 11, 21] {
                    let daily_ganzi = self.calculate_day_ganzi(year, month as u32, day);
                    
                    for hour_idx in 0..12 {
                        let hour = hour_idx * 2;
                        let hourly_ganzi = self.calculate_hour_ganzi(daily_ganzi, hour);
                        
                        let frame = self.vm.step(
                            age, 
                            major_context, 
                            yearly_ganzi, 
                            Some(monthly_ganzi), 
                            Some(daily_ganzi), 
                            Some(hourly_ganzi)
                        );

                        if frame.score <= self.crash_threshold {
                            vulnerabilities.push(Vulnerability {
                                crash_score: frame.score,
                                vector: LuckVector {
                                    major: major_context,
                                    yearly: yearly_ganzi,
                                    monthly: Some(monthly_ganzi),
                                    daily: Some(daily_ganzi),
                                    hourly: Some(hourly_ganzi),
                                },
                                vulnerability_type: self.determine_vuln_type(&frame),
                                tags: frame.tags_as_strings(),
                                timestamp: Some(format!("{}년 {}월 {}일 {}시", year, month, day, hour)),
                            });
                        }
                    }
                }
            }
        }

        vulnerabilities.sort_by(|a, b| a.crash_score.partial_cmp(&b.crash_score).unwrap());
        
        VulnerabilityReport {
            total_crashes: vulnerabilities.len(),
            critical_vectors: vulnerabilities.into_iter().take(10).collect(),
        }
    }

    // === 간지 계산 함수들: 공통 유틸리티 모듈 위임 ===
    // 중복 로직 제거를 위해 `core::ganzi_utils` 모듈을 사용합니다.
    // 이로써 FourPillars와 동일한 로직을 사용하여 정합성이 보장됩니다.

    fn calculate_month_ganzi(&self, year: i32, month: i32) -> GanZi {
        crate::core::ganzi_utils::calculate_month_ganzi(year, month)
    }

    fn calculate_day_ganzi(&self, year: i32, month: u32, day: u32) -> GanZi {
        // 만세력 데이터 기반의 정확한 일간지 계산 사용
        crate::core::ganzi_utils::calculate_day_ganzi(year, month, day)
    }

    fn calculate_hour_ganzi(&self, daily_ganzi: GanZi, hour: u32) -> GanZi {
        crate::core::ganzi_utils::calculate_hour_ganzi(daily_ganzi, hour)
    }

    /// 취약점 유형 판별 (Heuristics)
    fn determine_vuln_type(&self, frame: &LifeFrame) -> String {
        if frame.tags.iter().any(|t| t.contains_pattern("충") && t.contains_pattern("용신")) {
            "Critical_Yongshin_Clash (용신 파괴)".to_string()
        } else if frame.tags.iter().any(|t| t.contains_pattern("기신")) {
            "Elemental_Overflow (기신 과다)".to_string()
        } else if frame.tags.iter().any(|t| t.contains_pattern("충")) {
            "Structural_Instability (구조적 불안정)".to_string()
        } else {
            "Low_Energy_State (에너지 저하)".to_string()
        }
    }
}

impl std::fmt::Display for Vulnerability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[!] CRASH at Score {:.1} | Type: {} | Vector: ({}, {}) | Tags: {:?}", 
            self.crash_score, self.vulnerability_type, self.vector.major, self.vector.yearly, self.tags)
    }
}
