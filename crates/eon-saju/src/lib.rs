#![allow(clippy::too_many_arguments, clippy::needless_range_loop, clippy::doc_lazy_continuation, clippy::manual_clamp, clippy::unnecessary_sort_by, clippy::vec_init_then_push)]
//! # Eon Saju (四柱)
//!
//! 사주(四柱, Four Pillars of Destiny) 계산 라이브러리입니다.

pub mod analysis;
pub mod core;
pub mod engine;
pub mod report;

// Re-exports for convenience
pub use core::branch::EarthlyBranch;
pub use core::branch_days::SaryeongAnalysis;
pub use core::config::AnalysisConfig;
pub use core::element::{Element, ElementRelation, Polarity};
pub use core::ganzi::GanZi;
pub use core::nayin::NayinType;
pub use core::pillars::{FourPillars, SajuInput};
pub use core::stem::HeavenlyStem;
pub use core::ten_gods::{TenGod, TenGodAnalysis};
pub use core::twelve_stages::{TwelveStage, TwelveStageAnalysis};

pub use analysis::dynamic_luck::DynamicLuckAnalysis;
pub use analysis::heluo::{calculate_heluo, Era, HeLuoCycle, HeLuoResult};
pub use analysis::major_luck::{LuckDirection, MajorLuck, MajorLuckAnalysis};
pub use analysis::periodic_luck::{LuckAnalysis, MonthlyLuck, YearlyLuck};
pub use analysis::power::{AnalysisOptions, IntegratedAnalysis};
pub use analysis::relationships::RelationshipAnalysis;
pub use analysis::spirit_markers::{SpiritMarker, SpiritMarkerAnalysis};
pub use analysis::strength::{StrengthAnalysis, StrengthType};
pub use analysis::structure::{StructureAnalysis, StructureType};
pub use analysis::transformations::{EffectiveElement, TransformationAnalysis};
pub use analysis::void::VoidAnalysis;
pub use analysis::yongshin::{YongshinAnalysis, YongshinType};

pub use engine::complexity::{ComplexityAnalysis, DestinyComplexity};
pub use engine::die::{DestinyItEasy, DieAnalysis};
pub use engine::emulator::{LifePathEmulator, LifePathReport};
pub use engine::entropy::{DestinyEntropy, EntropyAnalysis, ObfuscationLevel};
pub use engine::fuzzer::{DestinyFuzzer, LuckVector, Vulnerability, VulnerabilityReport};
pub use engine::interprocess::{CompatibilityAudit, CompatibilityAuditor};
pub use engine::linter::{DestinyLinter, LintSeverity, SajuLint};
pub use engine::load_balancer::{KarmaLoadBalancer, LoadBalanceDiagnostic, TrafficStatus};
pub use engine::signatures::{LuckSignature, SignatureSeverity};
pub use engine::topology::{QiNode, QiTopology, TopologyAnalysis};
pub use engine::ttd::{DestinyDebugger, LifeDiff, RootCause};
pub use engine::vm::{LifeFrame, SajuVM};
