//! # Eon Saju (四柱)
//!
//! 사주(四柱, Four Pillars of Destiny) 계산 라이브러리입니다.

pub mod core;
pub mod analysis;
pub mod engine;
pub mod report;

// Re-exports for convenience
pub use core::stem::HeavenlyStem;
pub use core::branch::EarthlyBranch;
pub use core::element::{Element, Polarity, ElementRelation};
pub use core::ganzi::GanZi;
pub use core::pillars::{FourPillars, SajuInput};
pub use core::ten_gods::{TenGod, TenGodAnalysis};
pub use core::twelve_stages::{TwelveStage, TwelveStageAnalysis};
pub use core::branch_days::{SaryeongAnalysis};
pub use core::nayin::NayinType;

pub use analysis::major_luck::{MajorLuck, MajorLuckAnalysis, LuckDirection};
pub use analysis::periodic_luck::{YearlyLuck, MonthlyLuck, LuckAnalysis};
pub use analysis::spirit_markers::{SpiritMarker, SpiritMarkerAnalysis};
pub use analysis::relationships::RelationshipAnalysis;
pub use analysis::strength::{StrengthType, StrengthAnalysis};
pub use analysis::transformations::{TransformationAnalysis, EffectiveElement};
pub use analysis::power::{IntegratedAnalysis, AnalysisOptions};
pub use analysis::structure::{StructureType, StructureAnalysis};
pub use analysis::yongshin::{YongshinType, YongshinAnalysis};
pub use analysis::void::{VoidAnalysis};
pub use analysis::dynamic_luck::DynamicLuckAnalysis;

pub use engine::vm::{SajuVM, LifeFrame};
pub use engine::emulator::{LifePathEmulator, LifePathReport};
pub use engine::fuzzer::{DestinyFuzzer, VulnerabilityReport, Vulnerability, LuckVector};
pub use engine::signatures::{LuckSignature, SignatureSeverity};
pub use engine::linter::{DestinyLinter, SajuLint, LintSeverity};
pub use engine::topology::{QiTopology, TopologyAnalysis, QiNode};
pub use engine::load_balancer::{KarmaLoadBalancer, LoadBalanceDiagnostic, TrafficStatus};
pub use engine::complexity::{DestinyComplexity, ComplexityAnalysis};
pub use engine::ttd::{DestinyDebugger, RootCause, LifeDiff};
pub use engine::entropy::{DestinyEntropy, EntropyAnalysis, ObfuscationLevel};
pub use engine::die::{DestinyItEasy, DieAnalysis};
pub use engine::interprocess::{CompatibilityAuditor, CompatibilityAudit};
