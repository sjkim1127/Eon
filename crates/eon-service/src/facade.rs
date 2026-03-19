use crate::dto::*;
use crate::error::ServiceError;

pub fn analyze_saju(input: SajuAnalysisInput) -> Result<SajuAnalysisOutput, ServiceError> {
    crate::services::saju::analyze(input)
}

pub fn analyze_vedic(input: VedicAnalysisInput) -> Result<VedicAnalysisOutput, ServiceError> {
    crate::services::vedic::analyze(input)
}

pub fn analyze_transit(input: TransitAnalysisInput) -> Result<TransitAnalysisOutput, ServiceError> {
    crate::services::transit::analyze(input)
}

pub fn analyze_ai_audit(input: SajuAnalysisInput) -> Result<AiAuditOutput, ServiceError> {
    crate::services::ai_audit::analyze(input)
}

pub fn analyze_compatibility(
    input: CompatibilityInput,
) -> Result<CompatibilityOutput, ServiceError> {
    crate::services::compatibility::analyze(input)
}

pub fn analyze_destiny_tier(
    saju: SajuAnalysisOutput,
    vedic: VedicAnalysisOutput,
    transit: Option<TransitAnalysisOutput>,
) -> Result<TierResult, ServiceError> {
    Ok(crate::services::tier::analyze(saju, vedic, transit))
}
