pub mod analytics;
pub mod dynamic_luck;
pub mod heluo;
pub mod major_luck;
pub mod periodic_luck;
pub mod power;
pub mod relationships;
pub mod shinsal;
pub mod spirit_markers;
pub mod strength;
pub mod structure;
pub mod supplementary_pillars;
pub mod transformations;
pub mod void;
pub mod yongshin;

pub use crate::core::config::AnalysisConfig;
pub use crate::core::pillars::FourPillars;

pub use dynamic_luck::DynamicLuckAnalysis;
pub use heluo::{calculate_heluo, Era, HeLuoCycle, HeLuoResult};
pub use supplementary_pillars::SupplementaryPillars;
pub use void::VoidAnalysis;

/// 사주 분석용 공통 인터페이스
pub trait Analyzable {
    type Output;
    fn analyze(pillars: &FourPillars, config: &AnalysisConfig) -> Self::Output;
}
