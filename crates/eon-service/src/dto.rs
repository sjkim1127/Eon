use crate::error::ServiceError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct SajuAnalysisInput {
    #[serde(flatten)]
    pub base: AnalysisInput,
    pub is_male: bool,
    pub use_night_rat_hour: bool,
    pub precision: BirthTimePrecision,
}

impl SajuAnalysisInput {
    pub fn new(
        base: AnalysisInput,
        is_male: bool,
        use_night_rat_hour: bool,
        unknown_time: Option<bool>,
    ) -> Self {
        let precision = if unknown_time.unwrap_or(false) {
            BirthTimePrecision::UnknownTimeNoonProxy
        } else {
            BirthTimePrecision::Exact
        };

        Self {
            base,
            is_male,
            use_night_rat_hour,
            precision,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitAnalysisInput {
    #[serde(flatten)]
    pub base: SajuAnalysisInput,
    pub current: CurrentContext,
}

impl TransitAnalysisInput {
    pub fn new(base: SajuAnalysisInput, now_utc: Option<DateTime<Utc>>) -> Self {
        let analysis_timezone = base.base.timezone.clone();

        Self {
            base,
            current: CurrentContext {
                now_utc: now_utc.unwrap_or_else(Utc::now),
                analysis_timezone,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VedicAnalysisInput {
    #[serde(flatten)]
    pub base: AnalysisInput,
    pub precision: BirthTimePrecision,
    pub current: CurrentContext,
    pub target_year: Option<i32>,
}

impl VedicAnalysisInput {
    pub fn new(
        base: AnalysisInput,
        unknown_time: Option<bool>,
        now_utc: Option<DateTime<Utc>>,
    ) -> Self {
        let precision = if unknown_time.unwrap_or(false) {
            BirthTimePrecision::UnknownTimeNoonProxy
        } else {
            BirthTimePrecision::Exact
        };

        let now = now_utc.unwrap_or_else(Utc::now);
        let analysis_timezone = base.timezone.clone();

        Self {
            base,
            precision,
            current: CurrentContext {
                now_utc: now,
                analysis_timezone,
            },
            target_year: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZwdsAnalysisInput {
    #[serde(flatten)]
    pub base: AnalysisInput,
    pub is_male: bool,
    pub target_year: Option<i32>,
}

impl ZwdsAnalysisInput {
    pub fn new(base: AnalysisInput, is_male: bool, target_year: Option<i32>) -> Self {
        Self {
            base,
            is_male,
            target_year,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentContext {
    pub now_utc: DateTime<Utc>,
    pub analysis_timezone: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BirthTimePrecision {
    Exact,
    UnknownTimeNoonProxy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisMeta {
    pub precision: BirthTimePrecision,
    pub input_time: String,
    pub corrected_time: String,
    pub is_dst: bool,
    pub dst_offset_hours: Option<i32>,
    pub analysis_timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct VedicAnalysisOutput {
    pub meta: AnalysisMeta,
    pub report: eon_vedic::analysis::report::VedicAnalysisReport,
    pub tajika_report: Option<eon_vedic::analysis::report::TajikaReport>,
    pub chart: eon_vedic::core::chart::VedicChart,
    pub annual_chart: Option<eon_vedic::core::chart::VedicChart>,
    pub gochara: eon_vedic::analysis::gochara::GocharaSummary,
    pub varga_nakshatra_reports: eon_vedic::analysis::varga_nakshatra_report::VargaNakshatraReports,
    pub kp_analysis: Option<eon_vedic::analysis::kp::KpAnalysis>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZwdsAnalysisOutput {
    pub meta: AnalysisMeta,
    pub chart: eon_zwds::ZwdsChart,
    pub current_daxian: eon_zwds::types::DaXian,
    pub current_liu_nian: eon_zwds::types::LiuNian,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IChingAnalysisOutput {
    pub meta: AnalysisMeta,
    pub result: eon_saju::analysis::heluo::HeLuoResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WesternAnalysisInput {
    #[serde(flatten)]
    pub base: AnalysisInput,
    pub house_system: String, // "P", "K", "W", "E" 등
}

impl WesternAnalysisInput {
    pub fn new(base: AnalysisInput, house_system: String) -> Self {
        Self { base, house_system }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WesternAnalysisOutput {
    pub meta: AnalysisMeta,
    pub result: eon_western::WesternResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HumanDesignAnalysisInput {
    #[serde(flatten)]
    pub base: AnalysisInput,
}

impl HumanDesignAnalysisInput {
    pub fn new(base: AnalysisInput) -> Self {
        Self { base }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HumanDesignAnalysisOutput {
    pub meta: AnalysisMeta,
    pub result: eon_human_design::HumanDesignResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VedicCompatibilityInput {
    pub male: AnalysisInput,
    pub female: AnalysisInput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VedicCompatibilityOutput {
    pub meta: AnalysisMeta,
    pub report: eon_vedic::analysis::matching::CompatibilityReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct TransitAnalysisOutput {
    pub meta: AnalysisMeta,
    pub yearly_luck: eon_saju::analysis::periodic_luck::YearlyLuck,
    pub monthly_luck: eon_saju::analysis::periodic_luck::MonthlyLuck,
    pub monthly_lucks: Vec<eon_saju::analysis::periodic_luck::MonthlyLuck>,
    pub daily_luck: eon_saju::analysis::periodic_luck::DailyLuck,
    pub hourly_luck: eon_saju::analysis::periodic_luck::HourlyLuck,
    pub current_age: u32,
    pub current_frame: Option<LifeFrameDto>,
    pub nearby_diagnostics: Vec<eon_saju::engine::load_balancer::LoadBalanceDiagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiAuditOutput {
    pub meta: AnalysisMeta,
    pub context_dump: String,
    pub peak_age: u32,
    pub valley_age: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TierGrade {
    pub grade: String,
    pub label: String,
    pub desc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DomainTier {
    pub house: u8,
    pub domain: String,
    pub tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ScoreResult {
    pub score: f32,
    pub highlights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DestinyComponent {
    pub key: String,
    pub label: String,
    pub score: f32,
    pub weight: f32,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
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
    // --- 확장된 티어 모델 필드 ---
    pub destiny_raw_score: f32,
    pub destiny_tier_score: f32,
    pub detailed_components: Vec<DestinyComponent>,
    pub tier_model_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct AnalysisRequest {
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
    pub unknown_time: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SajuAnalysisRequest {
    #[serde(flatten)]
    #[ts(flatten)]
    pub base: AnalysisRequest,
    pub is_male: bool,
    pub use_night_rat_hour: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct VedicAnalysisRequest {
    #[serde(flatten)]
    #[ts(flatten)]
    pub base: AnalysisRequest,
    pub now_utc: Option<String>,
    pub target_year: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TransitAnalysisRequest {
    #[serde(flatten)]
    #[ts(flatten)]
    pub base: SajuAnalysisRequest,
    pub now_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DestinyTierRequest {
    #[ts(type = "any")]
    pub saju: SajuAnalysisOutput,
    #[ts(type = "any")]
    pub vedic: VedicAnalysisOutput,
    #[ts(type = "any")]
    pub transit: Option<TransitAnalysisOutput>,
}

impl TryFrom<SajuAnalysisRequest> for SajuAnalysisInput {
    type Error = ServiceError;

    fn try_from(req: SajuAnalysisRequest) -> Result<Self, Self::Error> {
        let precision = if req.base.unknown_time.unwrap_or(false) {
            BirthTimePrecision::UnknownTimeNoonProxy
        } else {
            BirthTimePrecision::Exact
        };

        Ok(Self {
            base: AnalysisInput {
                year: req.base.year,
                month: req.base.month,
                day: req.base.day,
                hour: req.base.hour,
                minute: req.base.minute,
                is_lunar: req.base.is_lunar,
                is_leap_month: req.base.is_leap_month,
                lat: req.base.lat,
                lon: req.base.lon,
                timezone: req.base.timezone,
            },
            is_male: req.is_male,
            use_night_rat_hour: req.use_night_rat_hour.unwrap_or(false),
            precision,
        })
    }
}

impl TryFrom<TransitAnalysisRequest> for TransitAnalysisInput {
    type Error = ServiceError;

    fn try_from(req: TransitAnalysisRequest) -> Result<Self, Self::Error> {
        let base = SajuAnalysisInput::try_from(req.base)?;
        let now_utc = if let Some(dt_str) = req.now_utc {
            DateTime::from_str(&dt_str)
                .map_err(|e| ServiceError::InvalidInput(format!("Invalid now_utc: {e}")))?
        } else {
            Utc::now()
        };

        let analysis_timezone = base.base.timezone.clone();
        Ok(Self {
            base,
            current: CurrentContext {
                now_utc,
                analysis_timezone,
            },
        })
    }
}

impl TryFrom<VedicAnalysisRequest> for VedicAnalysisInput {
    type Error = ServiceError;

    fn try_from(req: VedicAnalysisRequest) -> Result<Self, Self::Error> {
        let precision = if req.base.unknown_time.unwrap_or(false) {
            BirthTimePrecision::UnknownTimeNoonProxy
        } else {
            BirthTimePrecision::Exact
        };

        let now_utc = if let Some(dt_str) = req.now_utc {
            DateTime::from_str(&dt_str)
                .map_err(|e| ServiceError::InvalidInput(format!("Invalid now_utc: {e}")))?
        } else {
            Utc::now()
        };

        let analysis_timezone = req.base.timezone.clone();

        Ok(Self {
            base: AnalysisInput {
                year: req.base.year,
                month: req.base.month,
                day: req.base.day,
                hour: req.base.hour,
                minute: req.base.minute,
                is_lunar: req.base.is_lunar,
                is_leap_month: req.base.is_leap_month,
                lat: req.base.lat,
                lon: req.base.lon,
                timezone: req.base.timezone,
            },
            precision,
            current: CurrentContext {
                now_utc,
                analysis_timezone,
            },
            target_year: req.target_year,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReportTheme {
    WealthAndCareer,   // 재물 및 커리어 테마
    LoveAndMarriage,   // 연애 및 결혼 테마
    HealthAndVitality, // 건강 및 마음 치유 테마
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemedReportInput {
    pub base: SajuAnalysisInput,
    pub theme: ReportTheme,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemedReportOutput {
    pub meta: AnalysisMeta,
    pub theme: ReportTheme,
    pub user_name: String,
    pub title: String,
    pub content: String,
}
