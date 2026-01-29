pub mod analytics;
pub mod relationships;
pub mod transformations;
pub mod strength;
pub mod structure;
pub mod yongshin;
pub mod power;
pub mod spirit_markers;
pub mod void;
pub mod major_luck;
pub mod periodic_luck;
pub mod dynamic_luck;
pub mod shinsal;

pub use crate::core::config::AnalysisConfig;
pub use crate::core::pillars::FourPillars;

/// 사주 분석용 공통 인터페이스
pub trait Analyzable {
    type Output;
    fn analyze(pillars: &FourPillars, config: &AnalysisConfig) -> Self::Output;
}

