use crate::birth::prepare_birth_context;
use crate::context::{calculate_current_age, resolve_analysis_local_datetime};
use crate::dto::{AnalysisMeta, LifeFrameDto, TransitAnalysisInput, TransitAnalysisOutput};
use crate::error::ServiceError;
use eon_core::Gender;
use eon_saju::analysis::periodic_luck::{MonthlyLuck, YearlyLuck};
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::engine::load_balancer::KarmaLoadBalancer;
use eon_saju::engine::vm::SajuVM;

pub fn analyze(input: TransitAnalysisInput) -> Result<TransitAnalysisOutput, ServiceError> {
    let gender = if input.base.is_male {
        Gender::Male
    } else {
        Gender::Female
    };

    let birth_ctx = prepare_birth_context(&input.base.base, Some(gender), true)?;

    let saju_input = SajuInput::new_solar(
        birth_ctx.corrected_year,
        birth_ctx.corrected_month,
        birth_ctx.corrected_day,
        birth_ctx.corrected_hour,
        birth_ctx.corrected_minute,
    )
    .with_gender(gender)
    .with_night_rat_hour(input.base.use_night_rat_hour);

    let pillars = FourPillars::calculate(&saju_input)
        .map_err(|e| ServiceError::Saju(format!("사주 계산 실패: {}", e)))?;

    // 분석 로컬 시각 결정
    let (cy, cm, cd, ch) = resolve_analysis_local_datetime(&input.current)?;

    // 세운/월운 계산
    let yearly_luck = YearlyLuck::calculate(cy, &pillars);
    let monthly_luck = MonthlyLuck::calculate(cy, cm, &pillars);
    let monthly_lucks: Vec<MonthlyLuck> = (1..=12)
        .map(|m| MonthlyLuck::calculate(cy, m, &pillars))
        .collect();

    // 일운/시운 계산
    let daily_luck = eon_saju::analysis::periodic_luck::DailyLuck::calculate(cy, cm, cd, &pillars);
    let hourly_luck =
        eon_saju::analysis::periodic_luck::HourlyLuck::calculate(cy, cm, cd, ch, &pillars);

    // 나이 계산
    let current_age = calculate_current_age(
        birth_ctx.corrected_year,
        birth_ctx.corrected_month,
        birth_ctx.corrected_day,
        &input.current,
    )?;

    // 시뮬레이션 및 프레임 추출
    let vm = SajuVM::new(pillars.clone());
    let frames = vm.simulate_life(0, 105);
    let current_frame = frames.iter().find(|f| f.age == current_age).cloned();

    let all_diagnostics = KarmaLoadBalancer::diagnose(&frames);
    let nearby_diagnostics = all_diagnostics
        .into_iter()
        .filter(|d| d.age >= current_age.saturating_sub(3) && d.age <= current_age + 5)
        .collect();

    let current_frame_dto = current_frame.map(|f| LifeFrameDto {
        age: f.age,
        ganzi: f.ganzi,
        major_ganzi: f.major_ganzi,
        score: f.score,
        tags: f.tags_as_strings(),
        esil_trace: f.esil_trace,
        register_state: f.register_state,
    });

    Ok(TransitAnalysisOutput {
        meta: AnalysisMeta {
            precision: input.base.precision,
            input_time: birth_ctx.input_time_string,
            corrected_time: birth_ctx.corrected_time_string,
            is_dst: birth_ctx.is_dst,
            dst_offset_hours: birth_ctx.dst_offset_hours,
            analysis_timezone: input.base.base.timezone,
        },
        yearly_luck,
        monthly_luck,
        monthly_lucks,
        daily_luck,
        hourly_luck,
        current_age,
        current_frame: current_frame_dto,
        nearby_diagnostics,
    })
}
