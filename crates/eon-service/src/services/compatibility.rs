use crate::dto::{AnalysisMeta, CompatibilityAuditDto, CompatibilityInput, CompatibilityOutput, SajuAnalysisInput, VedicCompatibilityResultDto};
use crate::error::ServiceError;
use crate::birth::prepare_birth_context;
use eon_core::Gender;
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::engine::interprocess::CompatibilityAuditor;
use eon_saju::engine::vm::SajuVM;
use eon_vedic::analysis::compatibility::CompatibilityEngine;
use eon_vedic::core::chart::VedicChartCalculator;

pub fn analyze(input: CompatibilityInput) -> Result<CompatibilityOutput, ServiceError> {
    let p1_gender = if input.person1.is_male { Gender::Male } else { Gender::Female };
    let p1_ctx = prepare_birth_context(&input.person1.base, Some(p1_gender), true)?;
    let person1_meta = AnalysisMeta {
        precision: input.person1.precision,
        input_time: p1_ctx.input_time_string,
        corrected_time: p1_ctx.corrected_time_string,
        is_dst: p1_ctx.is_dst,
        dst_offset_hours: p1_ctx.dst_offset_hours,
        analysis_timezone: input.person1.base.timezone.clone(),
    };

    let p2_gender = if input.person2.is_male { Gender::Male } else { Gender::Female };
    let p2_ctx = prepare_birth_context(&input.person2.base, Some(p2_gender), true)?;
    let person2_meta = AnalysisMeta {
        precision: input.person2.precision,
        input_time: p2_ctx.input_time_string,
        corrected_time: p2_ctx.corrected_time_string,
        is_dst: p2_ctx.is_dst,
        dst_offset_hours: p2_ctx.dst_offset_hours,
        analysis_timezone: input.person2.base.timezone.clone(),
    };

    let saju_audit = analyze_saju_only(&input.person1, &input.person2)?;
    let saju = CompatibilityAuditDto {
        sync_score: saju_audit.sync_score,
        synergies: saju_audit.synergies,
        conflicts: saju_audit.conflicts,
        deadlocks: saju_audit.deadlocks,
        merged_esil_trace: saju_audit.merged_esil_trace,
    };
    
    let vedic_input = crate::dto::VedicCompatibilityInput {
        person1: input.person1.base.clone(),
        person2: input.person2.base.clone(),
    };
    let vedic_res = analyze_vedic_only(&vedic_input)?;
    let mut vedic = VedicCompatibilityResultDto {
        total_score: vedic_res.total_score,
        varna: vedic_res.varna,
        vashya: vedic_res.vashya,
        tara: vedic_res.tara,
        yoni: vedic_res.yoni,
        maitri: vedic_res.maitri,
        gana: vedic_res.gana,
        bhakoot: vedic_res.bhakoot,
        nadi: vedic_res.nadi,
        message: vedic_res.message,
    };
    
    // Add warning for unknown time
    if person1_meta.precision == crate::dto::BirthTimePrecision::UnknownTimeNoonProxy 
        || person2_meta.precision == crate::dto::BirthTimePrecision::UnknownTimeNoonProxy 
    {
        let warning = "태어난 시간을 모를 경우 달의 위치가 달라져 아쉬타쿠타 점수가 부정확할 수 있습니다.";
        if !vedic.message.is_empty() {
            vedic.message.push_str(" ");
        }
        vedic.message.push_str(warning);
    }
    
    Ok(CompatibilityOutput {
        person1_meta,
        person2_meta,
        saju,
        vedic,
    })
}

fn analyze_saju_only(
    p1: &SajuAnalysisInput,
    p2: &SajuAnalysisInput,
) -> Result<eon_saju::engine::interprocess::CompatibilityAudit, ServiceError> {
    let make_pillars = |saju_in: &SajuAnalysisInput| -> Result<FourPillars, ServiceError> {
        let gender = if saju_in.is_male { Gender::Male } else { Gender::Female };
        let birth_ctx = prepare_birth_context(&saju_in.base, Some(gender), true)?;
        
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

    let pillars1 = make_pillars(p1)?;
    let pillars2 = make_pillars(p2)?;

    let vm1 = SajuVM::new(pillars1);
    let vm2 = SajuVM::new(pillars2);
    Ok(CompatibilityAuditor::audit(&vm1, &vm2))
}

fn analyze_vedic_only(
    input: &crate::dto::VedicCompatibilityInput,
) -> Result<eon_vedic::analysis::compatibility::CompatibilityResult, ServiceError> {
    let calculator = VedicChartCalculator::new();
    
    let birth1_ctx = prepare_birth_context(&input.person1, None, false)?;
    let dt1 = birth1_ctx.birth_info.to_utc()
        .ok_or_else(|| ServiceError::BirthInfo("Invalid date 1".to_string()))?;
        
    let birth2_ctx = prepare_birth_context(&input.person2, None, false)?;
    let dt2 = birth2_ctx.birth_info.to_utc()
        .ok_or_else(|| ServiceError::BirthInfo("Invalid date 2".to_string()))?;

    let chart1 = calculator.calculate(dt1, input.person1.lat, input.person1.lon);
    let chart2 = calculator.calculate(dt2, input.person2.lat, input.person2.lon);
    
    Ok(CompatibilityEngine::analyze(&chart1, &chart2))
}
