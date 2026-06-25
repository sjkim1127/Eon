use crate::dto::*;
use crate::error::ServiceError;

pub fn analyze_saju(input: SajuAnalysisInput) -> Result<SajuAnalysisOutput, ServiceError> {
    crate::services::saju::analyze(input)
}

pub fn analyze_vedic(input: VedicAnalysisInput) -> Result<VedicAnalysisOutput, ServiceError> {
    crate::services::vedic::analyze(input)
}

pub fn analyze_vedic_compatibility(input: VedicCompatibilityInput) -> Result<VedicCompatibilityOutput, ServiceError> {
    crate::services::vedic::analyze_compatibility(input)
}

pub fn analyze_zwds(input: ZwdsAnalysisInput) -> Result<ZwdsAnalysisOutput, ServiceError> {
    crate::services::zwds::analyze(input)
}

pub fn analyze_transit(input: TransitAnalysisInput) -> Result<TransitAnalysisOutput, ServiceError> {
    crate::services::transit::analyze(input)
}

pub fn analyze_ai_audit(input: SajuAnalysisInput) -> Result<AiAuditOutput, ServiceError> {
    crate::services::ai_audit::analyze(input)
}



pub fn analyze_destiny_tier(
    saju: SajuAnalysisOutput,
    vedic: VedicAnalysisOutput,
    transit: Option<TransitAnalysisOutput>,
) -> Result<TierResult, ServiceError> {
    Ok(crate::services::tier::analyze(saju, vedic, transit))
}

pub fn analyze_iching(input: SajuAnalysisInput) -> Result<IChingAnalysisOutput, ServiceError> {
    let saju_res = analyze_saju(input.clone())?;
    let pillars = &saju_res.report.pillars;
    let birth_year = input.base.year;
    
    let res = eon_saju::analysis::heluo::calculate_heluo(
        birth_year,
        input.is_male,
        &pillars.year,
        &pillars.month,
        &pillars.day,
        &pillars.hour,
    );
    
    Ok(IChingAnalysisOutput {
        meta: saju_res.meta,
        result: res,
    })
}

pub fn analyze_western(input: WesternAnalysisInput) -> Result<WesternAnalysisOutput, ServiceError> {
    crate::services::western::analyze(input)
}
