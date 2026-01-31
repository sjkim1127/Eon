//! 사주 분석용 각종 가중치 및 임계값 설정
//!
//! ML 기반 최적화나 사용자 튜닝을 용이하게 하기 위해 Struct로 관리합니다.

use serde::{Deserialize, Serialize};

/// 전역 분석 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub weights: WeightsConfig,
    pub strength: StrengthConfig,
    pub root: RootConfig,
    pub thermal: ThermalConfig,
    pub vm: VmConfig,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            weights: WeightsConfig::default(),
            strength: StrengthConfig::default(),
            root: RootConfig::default(),
            thermal: ThermalConfig::default(),
            vm: VmConfig::default(),
        }
    }
}

/// 위치별 가중치 (110점법)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightsConfig {
    pub total_weight: f32,
    pub month_branch: f32,
    pub day_branch: f32,
    pub other_branch: f32,
    pub stem: f32,
}

impl Default for WeightsConfig {
    fn default() -> Self {
        Self {
            total_weight: 11.0,
            month_branch: 3.5,
            day_branch: 1.5,
            other_branch: 1.0,
            stem: 1.0,
        }
    }
}

/// 신강신약 점수 산출 가중치
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrengthConfig {
    pub criteria_score: f32,
    pub deuk_se_threshold: f32,
    pub deuk_se_weight: f32,
    pub polarized_high: f32,
    pub polarized_low: f32,
}

impl Default for StrengthConfig {
    fn default() -> Self {
        Self {
            criteria_score: 25.0,
            deuk_se_threshold: 5.5,
            deuk_se_weight: 0.25,
            polarized_high: 80.0,
            polarized_low: 20.0,
        }
    }
}

/// 지장간 통근 가중치
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootConfig {
    pub main_root: f32,
    pub middle_root: f32,
    pub remain_root: f32,
    pub saryeong_bonus: f32,
    pub min_deuk_ji_score: f32,
}

impl Default for RootConfig {
    fn default() -> Self {
        Self {
            main_root: 1.0,
            middle_root: 0.6,
            remain_root: 0.3,
            saryeong_bonus: 1.2,
            min_deuk_ji_score: 3.0,
        }
    }
}

/// 조후 지수 임계값
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalConfig {
    pub extreme: i32,
    pub moderate: i32,
}

impl Default for ThermalConfig {
    fn default() -> Self {
        Self {
            extreme: 40,
            moderate: 25,
        }
    }
}

/// VM 시뮬레이션 파라미터
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmConfig {
    pub base_score: f32,
    pub clash_bad: f32,
    pub clash_good: f32,
    pub clash_default: f32,

    // 파이프라인 가중치
    pub pipeline_major: f32,
    pub pipeline_yearly: f32,
    pub pipeline_monthly: f32,
    pub pipeline_daily: f32,
    pub pipeline_hourly: f32,

    // 인터럽트 페널티
    pub irq_critical: f32,
    pub irq_overflow: f32,
    pub irq_stall: f32,
    pub irq_service: f32,

    // 파이프라인 제어
    pub stall_penalty: f32,
    pub forwarding_bonus: f32,
    pub memory_dump_weight: f32,
    pub decay_factor: f32,
}

impl Default for VmConfig {
    fn default() -> Self {
        Self {
            base_score: 50.0,
            clash_bad: -20.0,
            clash_good: 10.0,
            clash_default: -5.0,

            pipeline_major: 10.0,
            pipeline_yearly: 15.0,
            pipeline_monthly: 5.0,
            pipeline_daily: 2.0,
            pipeline_hourly: 1.0,

            irq_critical: 20.0,
            irq_overflow: 15.0,
            irq_stall: 10.0,
            irq_service: 7.0,

            stall_penalty: 0.5,
            forwarding_bonus: 1.2,
            memory_dump_weight: 0.3,
            decay_factor: 0.95,
        }
    }
}
