//! 사주 분석용 각종 가중치 및 임계값 설정
//!
//! ML 기반 최적화나 사용자 튜닝을 용이하게 하기 위해 Struct로 관리합니다.

use serde::{Deserialize, Serialize};

/// 전역 분석 설정
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalysisConfig {
    pub weights: WeightsConfig,
    pub strength: StrengthConfig,
    pub root: RootConfig,
    pub thermal: ThermalConfig,
    pub vm: VmConfig,
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
///
/// SajuVM이 인생 시뮬레이션을 실행할 때 사용하는 가중치와 페널티입니다.
/// ML 기반 최적화나 사용자 커스터마이징을 위해 외부에서 조정 가능합니다.
///
/// ## 튜닝 가이드
///
/// ### 기본 점수 (`base_score`)
/// - 기본값: 50.0
/// - 시뮬레이션 시작 점수. 중립적인 운세 상태를 나타냅니다.
///
/// ### 충돌 점수 (`clash_*`)
/// - `clash_bad`: 희신/용신 충돌 시 감점 (기본: -20.0)
/// - `clash_good`: 기신/구신 충돌 시 가점 (기본: +10.0, 개고 효과)
/// - `clash_default`: 일반적인 충돌 (기본: -5.0)
///
/// ### 파이프라인 가중치 (`pipeline_*`)
/// - 각 운(대운, 세운, 월운, 일운, 시운)이 점수에 미치는 기본 영향력
/// - 대운이 가장 크고, 세운, 월운, 일운, 시운 순으로 감소
///
/// ### 인터럽트 페널티 (`irq_*`)
/// - 흉살(백호살, 괴강살 등)에 의한 시스템 인터럽트 발생 시 감점
/// - `irq_critical`: 가장 심각한 예외 (백호살 등, 기본: -20.0)
/// - `irq_overflow`: 리소스 오버플로우 (괴강살 등, 기본: -15.0)
/// - `irq_stall`: 시스템 정지 (고신/과숙 등, 기본: -10.0)
/// - `irq_service`: 서비스 중단 (망신/겁살 등, 기본: -7.0)
///
/// ### 파이프라인 제어 (`stall_penalty`, `forwarding_bonus`, etc.)
/// - `stall_penalty`: 앞 단계와 상극 시 효율 감소 배율 (기본: 0.5, 절반)
/// - `forwarding_bonus`: 합화된 기운이 용신일 때 효율 증가 배율 (기본: 1.2)
/// - `memory_dump_weight`: 충에 의해 해방된 지장간 기운의 영향력 (기본: 0.3)
/// - `decay_factor`: 시간 흐름에 따른 기운 감쇠율 (기본: 0.95, 5% 감쇠)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmConfig {
    /// 시뮬레이션 시작 점수 (0-100 범위의 중앙값)
    pub base_score: f32,

    /// 희신/용신 충돌 시 감점 (음수)
    pub clash_bad: f32,
    /// 기신/구신 충돌 시 가점 (양수, 개고 효과)
    pub clash_good: f32,
    /// 일반적인 충돌 시 감점
    pub clash_default: f32,

    // 파이프라인 가중치 (각 운의 기본 영향력)
    /// 대운(大運) 가중치 - 10년 단위 대 흐름
    pub pipeline_major: f32,
    /// 세운(歲運) 가중치 - 1년 단위 운세
    pub pipeline_yearly: f32,
    /// 월운(月運) 가중치 - 월별 변동
    pub pipeline_monthly: f32,
    /// 일운(日運) 가중치 - 일별 변동
    pub pipeline_daily: f32,
    /// 시운(時運) 가중치 - 시간별 변동
    pub pipeline_hourly: f32,

    // 인터럽트 페널티 (흉살에 의한 시스템 인터럽트)
    /// 0x01: 고우선순위 시스템 충돌 (백호살 등)
    pub irq_critical: f32,
    /// 0x02: 리소스 오버플로우 (괴강살 등)
    pub irq_overflow: f32,
    /// 0x03: 시스템 정지/고독 (고신/과숙 등)
    pub irq_stall: f32,
    /// 0x04: 임시 서비스 중단 (망신/겁살 등)
    pub irq_service: f32,

    // 파이프라인 제어 파라미터
    /// 파이프라인 스톨 발생 시 효율 감소 배율 (0.0-1.0)
    pub stall_penalty: f32,
    /// 데이터 포워딩(합화) 성공 시 효율 증가 배율 (1.0+)
    pub forwarding_bonus: f32,
    /// 충에 의한 지장간 메모리 덤프 영향력 (0.0-1.0)
    pub memory_dump_weight: f32,
    /// 시간 흐름에 따른 기운 감쇠율 (0.0-1.0)
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
