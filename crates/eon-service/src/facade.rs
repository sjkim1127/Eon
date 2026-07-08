use crate::dto::*;
use crate::error::ServiceError;

pub fn analyze_saju(input: SajuAnalysisInput) -> Result<SajuAnalysisOutput, ServiceError> {
    crate::services::saju::analyze(input)
}

pub fn analyze_vedic(input: VedicAnalysisInput) -> Result<VedicAnalysisOutput, ServiceError> {
    crate::services::vedic::analyze(input)
}

pub fn analyze_vedic_compatibility(
    input: VedicCompatibilityInput,
) -> Result<VedicCompatibilityOutput, ServiceError> {
    crate::services::vedic::analyze_compatibility(input)
}

pub fn analyze_zwds(input: ZwdsAnalysisInput) -> Result<ZwdsAnalysisOutput, ServiceError> {
    crate::services::zwds::analyze(input)
}

pub fn analyze_qimen(input: QimenAnalysisInput) -> Result<QimenAnalysisOutput, ServiceError> {
    crate::services::qimen::analyze_qimen(input)
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

pub fn analyze_human_design(
    input: HumanDesignAnalysisInput,
) -> Result<HumanDesignAnalysisOutput, ServiceError> {
    crate::services::human_design::analyze(input)
}

pub fn analyze_hd_connection(
    input1: HumanDesignAnalysisInput,
    input2: HumanDesignAnalysisInput,
) -> Result<eon_human_design::connection::HumanDesignConnectionResult, ServiceError> {
    crate::services::human_design::analyze_connection(input1, input2)
}

pub fn analyze_hd_transit(
    natal_input: HumanDesignAnalysisInput,
    transit_time: chrono::DateTime<chrono::Utc>,
) -> Result<eon_human_design::transit::HumanDesignTransitResult, ServiceError> {
    crate::services::human_design::analyze_transit(natal_input, transit_time)
}

pub fn analyze_hd_return(
    natal_input: HumanDesignAnalysisInput,
    return_type: eon_human_design::transit::ReturnType,
    target_year: i32,
) -> Result<eon_human_design::transit::HumanDesignTransitResult, ServiceError> {
    crate::services::human_design::calculate_return(natal_input, return_type, target_year)
}

pub fn analyze_hd_penta(
    inputs: Vec<HumanDesignAnalysisInput>,
) -> Result<eon_human_design::penta::PentaResult, ServiceError> {
    crate::services::human_design::analyze_penta(inputs)
}

pub fn generate_themed_report(
    input: ThemedReportInput,
) -> Result<ThemedReportOutput, ServiceError> {
    crate::services::report::generate(input)
}
