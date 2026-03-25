use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisInput {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub is_lunar: bool,
    pub is_leap_month: bool,
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SajuAnalysisInput {
    #[serde(flatten)]
    pub base: AnalysisInput,
    pub is_male: bool,
    pub use_night_rat_hour: bool,
    pub precision: BirthTimePrecision,
}

impl SajuAnalysisInput {
    pub fn new(
        year: i32, month: u32, day: u32, hour: u32, minute: u32,
        is_lunar: bool, is_leap_month: bool,
        is_male: bool, use_night_rat_hour: bool,
        lon: f64, lat: f64, timezone: String,
        unknown_time: Option<bool>,
    ) -> Self {
        let precision = if unknown_time.unwrap_or(false) {
            BirthTimePrecision::UnknownTimeNoonProxy
        } else {
            BirthTimePrecision::Exact
        };

        Self {
            base: AnalysisInput {
                year, month, day, hour, minute,
                is_lunar, is_leap_month, lat, lon, timezone,
            },
            is_male,
            use_night_rat_hour,
            precision,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitAnalysisInput {
    #[serde(flatten)]
    pub base: SajuAnalysisInput,
    pub current: CurrentContext,
}

impl TransitAnalysisInput {
    pub fn new(
        year: i32, month: u32, day: u32, hour: u32, minute: u32,
        is_lunar: bool, is_leap_month: bool,
        is_male: bool, use_night_rat_hour: bool,
        lon: f64, lat: f64, timezone: String,
        unknown_time: Option<bool>,
        now_utc: Option<DateTime<Utc>>,
    ) -> Self {
        let base = SajuAnalysisInput::new(
            year, month, day, hour, minute,
            is_lunar, is_leap_month, is_male, use_night_rat_hour,
            lon, lat, timezone.clone(), unknown_time,
        );

        Self {
            base,
            current: CurrentContext {
                now_utc: now_utc.unwrap_or_else(Utc::now),
                analysis_timezone: timezone,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VedicAnalysisInput {
    #[serde(flatten)]
    pub base: AnalysisInput,
    pub precision: BirthTimePrecision,
    pub current: CurrentContext,
}

impl VedicAnalysisInput {
    pub fn new(
        year: i32, month: u32, day: u32, hour: u32, minute: u32,
        is_lunar: bool, is_leap_month: bool,
        lat: f64, lon: f64, timezone: String,
        unknown_time: Option<bool>,
        now_utc: Option<DateTime<Utc>>,
    ) -> Self {
        let precision = if unknown_time.unwrap_or(false) {
            BirthTimePrecision::UnknownTimeNoonProxy
        } else {
            BirthTimePrecision::Exact
        };

        Self {
            base: AnalysisInput {
                year, month, day, hour, minute,
                is_lunar, is_leap_month, lat, lon, timezone: timezone.clone(),
            },
            precision,
            current: CurrentContext {
                now_utc: now_utc.unwrap_or_else(Utc::now),
                analysis_timezone: timezone,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityInput {
    pub person1: SajuAnalysisInput,
    pub person2: SajuAnalysisInput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VedicCompatibilityInput {
    pub person1: AnalysisInput,
    pub person2: AnalysisInput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentContext {
    pub now_utc: DateTime<Utc>,
    pub analysis_timezone: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BirthTimePrecision {
    Exact,
    UnknownTimeNoonProxy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMeta {
    pub precision: BirthTimePrecision,
    pub input_time: String,
    pub corrected_time: String,
    pub is_dst: bool,
    pub dst_offset_hours: Option<i32>,
    pub analysis_timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SajuAnalysisOutput {
    pub meta: AnalysisMeta,
    pub report: eon_saju::report::SajuReport,
    pub lints: Vec<eon_saju::engine::linter::SajuLint>,
    pub entropy: eon_saju::engine::entropy::EntropyAnalysis,
    pub qi_topology: eon_saju::engine::topology::TopologyAnalysis,
    pub load_diagnostics: Vec<eon_saju::engine::load_balancer::LoadBalanceDiagnostic>,
    pub crash_count: u32,
    pub vulnerability_report: Option<eon_saju::engine::fuzzer::VulnerabilityReport>,
    pub relationships: eon_saju::analysis::relationships::RelationshipAnalysis,
    pub void_analysis: eon_saju::analysis::void::VoidAnalysis,
    pub complexity: Option<eon_saju::engine::complexity::ComplexityAnalysis>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VedicAnalysisOutput {
    pub meta: AnalysisMeta,
    pub report: eon_vedic::analysis::report::VedicAnalysisReport,
    pub chart: eon_vedic::core::chart::VedicChart,
    pub gochara: eon_vedic::analysis::gochara::GocharaSummary,
    pub varga_nakshatra_reports:
        eon_vedic::analysis::varga_nakshatra_report::VargaNakshatraReports,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLuckDto {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub ganzi: eon_saju::core::ganzi::GanZi,
    pub stem_god: eon_saju::core::ten_gods::TenGod,
    pub branch_god: eon_saju::core::ten_gods::TenGod,
    pub influence: Option<eon_saju::analysis::dynamic_luck::LuckInfluence>,
    pub twelve_stage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeFrameDto {
    pub age: u32,
    pub ganzi: eon_saju::core::ganzi::GanZi,
    pub major_ganzi: eon_saju::core::ganzi::GanZi,
    pub score: f32,
    pub tags: Vec<String>,
    pub esil_trace: String,
    pub register_state: eon_saju::engine::vm::QiRegisters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitAnalysisOutput {
    pub meta: AnalysisMeta,
    pub yearly_luck: eon_saju::analysis::periodic_luck::YearlyLuck,
    pub monthly_luck: eon_saju::analysis::periodic_luck::MonthlyLuck,
    pub monthly_lucks: Vec<eon_saju::analysis::periodic_luck::MonthlyLuck>,
    pub daily_luck: DailyLuckDto,
    pub current_age: u32,
    pub current_frame: Option<LifeFrameDto>,
    pub nearby_diagnostics: Vec<eon_saju::engine::load_balancer::LoadBalanceDiagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAuditOutput {
    pub meta: AnalysisMeta,
    pub context_dump: String,
    pub peak_age: u32,
    pub valley_age: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompatibilityAuditDto {
    pub sync_score: f32,
    pub synergies: Vec<String>,
    pub conflicts: Vec<String>,
    pub deadlocks: Vec<String>,
    pub merged_esil_trace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VedicCompatibilityResultDto {
    pub total_score: f64,
    pub varna: f64,
    pub vashya: f64,
    pub tara: f64,
    pub yoni: f64,
    pub maitri: f64,
    pub gana: f64,
    pub bhakoot: f64,
    pub nadi: f64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompatibilityOutput {
    pub person1_meta: AnalysisMeta,
    pub person2_meta: AnalysisMeta,
    pub saju: CompatibilityAuditDto,
    pub vedic: VedicCompatibilityResultDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierGrade {
    pub grade: String,
    pub label: String,
    pub desc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreResult {
    pub score: f32,
    pub highlights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainTier {
    pub house: u8,
    pub domain: String,
    pub tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierResult {
    pub natal_score: f32,
    pub current_score: f32,
    pub destiny_score: f32,
    pub destiny_tier: TierGrade,
    pub potential_score: f32,
    pub potential_tier: TierGrade,
    pub domain_tiers: Vec<DomainTier>,
    pub saju_result: ScoreResult,
    pub vedic_result: ScoreResult,
    pub transit_result: ScoreResult,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub growth_gap: f32,
    pub risk_level: String,
    pub profile: String,
    pub version: String,
}
