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

/// 신강신약 점수 산출 가중치
pub mod strength_scores {
    pub const CRITERIA_SCORE: f32 = 25.0;     // 득령, 득지, 득시 각각의 점수
    pub const DEUK_SE_THRESHOLD: f32 = 5.5;   // 득세 판정 임계값
    pub const DEUK_SE_WEIGHT: f32 = 0.25;     // 득세 점수 반영비율
}

/// 지장간 통근 가중치
pub mod root_weights {
    pub const MAIN_ROOT: f32 = 1.0;           // 정기
    pub const MIDDLE_ROOT: f32 = 0.6;         // 중기
    pub const REMAIN_ROOT: f32 = 0.3;         // 여기
    pub const SARYEONG_BONUS: f32 = 1.2;      // 사령 보너스 (20%)
    pub const MIN_DEUK_JI_SCORE: f32 = 3.0;   // 득지 판정 최소 점수
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
