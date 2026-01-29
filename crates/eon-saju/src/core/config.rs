//! 사주 분석용 각종 가중치 및 임계값 설정
//! 
//! ML 기반 최적화나 사용자 튜닝을 용이하게 하기 위해 상수로 분리합니다.

/// 위치별 가중치 (110점법)
pub mod weights {
    pub const TOTAL_WEIGHT: f32 = 11.0;
    pub const WEIGHT_MONTH_BRANCH: f32 = 3.5; // 35점
    pub const WEIGHT_DAY_BRANCH: f32 = 1.5;   // 15점
    pub const WEIGHT_OTHER_BRANCH: f32 = 1.0; // 년지, 시지 (각 10점)
    pub const WEIGHT_STEM: f32 = 1.0;         // 각 천간 10점 (일간 포함 4개)
}

/// 분석 임계값 (Thresholds)
pub mod thresholds {
    /// 종격(극단적 강약) 판단 비율 (%)
    pub const POLARIZED_RATIO_HIGH: f32 = 80.0;
    pub const POLARIZED_RATIO_LOW: f32 = 20.0;
    
    /// 조후 지수 임계값
    pub const THERMAL_EXTREME: i32 = 40;
    pub const THERMAL_MODERATE: i32 = 25;
}

/// VM 시뮬레이션 파라미터
pub mod vm_params {
    /// 기본 시작 점수
    pub const BASE_SCORE: f32 = 50.0;
    
    /// 충(Clash) 영향력
    pub const CLASH_PENALTY_BAD: f32 = -20.0;
    pub const CLASH_GAIN_GOOD: f32 = 10.0;
    pub const CLASH_DEFAULT: f32 = -5.0;
}
