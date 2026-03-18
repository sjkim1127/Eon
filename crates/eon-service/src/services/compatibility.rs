use crate::dto::{CompatibilityInput, CompatibilityOutput, VedicCompatibilityInput};
use crate::error::ServiceError;
use crate::birth::prepare_birth_context;
use eon_core::Gender;
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::engine::interprocess::CompatibilityAuditor;
use eon_saju::engine::vm::SajuVM;
use eon_vedic::analysis::compatibility::{CompatibilityEngine, CompatibilityResult};
use eon_vedic::core::chart::VedicChartCalculator;

pub fn analyze(input: CompatibilityInput) -> Result<CompatibilityOutput, ServiceError> {
    let make_pillars = |saju_in: &crate::dto::SajuAnalysisInput| -> Result<FourPillars, ServiceError> {
        let gender = if saju_in.is_male { Gender::Male } else { Gender::Female };
        let birth_ctx = prepare_birth_context(&saju_in.base, Some(gender))?;
        
        let saju_input = SajuInput::new_solar(
            birth_ctx.corrected_year,
            birth_ctx.corrected_month,
            birth_ctx.corrected_day,
            birth_ctx.corrected_hour,
            birth_ctx.corrected_minute,
        )
        .with_gender(gender)
        .with_night_rat_hour(saju_in.use_night_rat_hour);

        FourPillars::calculate(&saju_input)
            .map_err(|e| ServiceError::Saju(format!("사주 계산 실패: {}", e)))
    };

    let pillars1 = make_pillars(&input.person1)?;
    let pillars2 = make_pillars(&input.person2)?;

    let vm1 = SajuVM::new(pillars1);
    let vm2 = SajuVM::new(pillars2);
    let saju_audit = CompatibilityAuditor::audit(&vm1, &vm2);

    // Vedic 통합 계산 (지시서 v1 DTO 구조 대응)
    let calculator = VedicChartCalculator::new();
    let birth1_ctx = prepare_birth_context(&input.person1.base, None)?;
    let dt1 = birth1_ctx.birth_info.to_utc()
        .ok_or_else(|| ServiceError::BirthInfo("Invalid date 1".to_string()))?;
    let birth2_ctx = prepare_birth_context(&input.person2.base, None)?;
    let dt2 = birth2_ctx.birth_info.to_utc()
        .ok_or_else(|| ServiceError::BirthInfo("Invalid date 2".to_string()))?;
    
    let chart1 = calculator.calculate(dt1, input.person1.base.lat, input.person1.base.lon);
    let chart2 = calculator.calculate(dt2, input.person2.base.lat, input.person2.base.lon);
    let vedic_audit = CompatibilityEngine::analyze(&chart1, &chart2);
    
    Ok(CompatibilityOutput {
        saju: saju_audit,
        vedic: vedic_audit,
    })
}

pub fn analyze_vedic(input: VedicCompatibilityInput) -> Result<CompatibilityResult, ServiceError> {
    let calculator = VedicChartCalculator::new();
    
    let birth1_ctx = prepare_birth_context(&input.person1, None)?;
    let dt1 = birth1_ctx.birth_info.to_utc()
        .ok_or_else(|| ServiceError::BirthInfo("Invalid date 1".to_string()))?;
        
    let birth2_ctx = prepare_birth_context(&input.person2, None)?;
    let dt2 = birth2_ctx.birth_info.to_utc()
        .ok_or_else(|| ServiceError::BirthInfo("Invalid date 2".to_string()))?;

    let chart1 = calculator.calculate(dt1, input.person1.lat, input.person1.lon);
    let chart2 = calculator.calculate(dt2, input.person2.lat, input.person2.lon);
    
    Ok(CompatibilityEngine::analyze(&chart1, &chart2))
}
