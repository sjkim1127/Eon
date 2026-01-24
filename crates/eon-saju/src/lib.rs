//! # Eon Saju (四柱)
//!
//! 사주(四柱, Four Pillars of Destiny) 계산 라이브러리입니다.
//!
//! ## 주요 기능
//!
//! - 천간(天干) / 지지(地支) 타입
//! - 오행(五行) 상생상극 로직
//! - 60갑자 사이클
//! - 사주 팔자 계산
//!
//! ## 사용 예시
//!
//! ```rust
//! use eon_saju::{FourPillars, SajuInput};
//! use chrono::{TimeZone, Local};
//!
//! let input = SajuInput {
//!     year: 1990,
//!     month: 5,
//!     day: 15,
//!     hour: 14,
//!     minute: 30,
//!     is_lunar: false,
//!     is_leap_month: false,
//! };
//!
//! let pillars = FourPillars::calculate(&input).unwrap();
//! println!("{}", pillars);
//! ```

pub mod stem;
pub mod branch;
pub mod element;
pub mod ganzi;
pub mod pillars;
pub mod calendar;
pub mod ten_gods;
pub mod major_luck;
pub mod periodic_luck;
pub mod spirit_markers;
pub mod relationships;
pub mod twelve_stages;
pub mod strength;
pub mod transformations;
pub mod power;
pub mod structure;
pub mod yongshin;
pub mod void;
pub mod branch_days;
pub mod dynamic_luck;
pub mod vm;
pub mod emulator;
pub mod fuzzer;
pub mod signatures;
pub mod linter;
pub mod topology;
pub mod load_balancer;
pub mod complexity;
pub mod ttd;
pub mod entropy;
pub mod die;

pub use stem::HeavenlyStem;
pub use branch::EarthlyBranch;
pub use element::{Element, Polarity, ElementRelation};
pub use ganzi::GanZi;
pub use pillars::{FourPillars, SajuInput};
pub use ten_gods::{TenGod, TenGodAnalysis};
pub use major_luck::{MajorLuck, MajorLuckAnalysis, LuckDirection};
pub use periodic_luck::{YearlyLuck, MonthlyLuck, LuckAnalysis};
pub use spirit_markers::{SpiritMarker, SpiritMarkerAnalysis};
pub use relationships::RelationshipAnalysis;
pub use twelve_stages::{TwelveStage, TwelveStageAnalysis};
pub use strength::{StrengthType, StrengthAnalysis};
pub use transformations::{TransformationAnalysis, EffectiveElement};
pub use power::{IntegratedAnalysis, AnalysisOptions};
pub use structure::{StructureType, StructureAnalysis};
pub use yongshin::{YongshinType, YongshinAnalysis};
pub use void::{VoidAnalysis};
pub use branch_days::{SaryeongAnalysis};
pub use dynamic_luck::DynamicLuckAnalysis;
pub use vm::{SajuVM, LifeFrame};
pub use emulator::{LifePathEmulator, LifePathReport};
pub use fuzzer::{DestinyFuzzer, VulnerabilityReport, Vulnerability, LuckVector};
pub use signatures::{LuckSignature, SignatureSeverity};
pub use linter::{DestinyLinter, SajuLint, LintSeverity};
pub use topology::{QiTopology, TopologyAnalysis, QiNode};
pub use load_balancer::{KarmaLoadBalancer, LoadBalanceDiagnostic, TrafficStatus};
pub use complexity::{DestinyComplexity, ComplexityAnalysis};
pub use ttd::{DestinyDebugger, RootCause, LifeDiff};
pub use entropy::{DestinyEntropy, EntropyAnalysis, ObfuscationLevel};
pub use die::{DestinyItEasy, DieAnalysis};
