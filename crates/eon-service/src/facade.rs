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

pub fn analyze_destiny_tier(
    saju: SajuAnalysisOutput,
    vedic: VedicAnalysisOutput,
    transit: Option<TransitAnalysisOutput>,
) -> Result<TierResult, ServiceError> {
    Ok(crate::services::tier::analyze(saju, Some(vedic), transit))
}

pub fn analyze_destiny_tier_omni(
    input: crate::dto::OmniDestinyTierInput,
) -> Result<TierResult, ServiceError> {
    Ok(crate::services::tier::analyze_omni(input))
}

pub fn analyze_all_synthesized(
    input: AnalysisInput,
    is_male: bool,
    name: Option<String>,
) -> Result<SynthesizedOmniAnalysis, ServiceError> {
    let saju_in = SajuAnalysisInput::new(input.clone(), is_male, false, Some(input.unknown_time));
    let vedic_in = VedicAnalysisInput::new(input.clone(), Some(input.unknown_time), None);
    let western_in = WesternAnalysisInput::new(input.clone(), "Placidus".to_string());
    let zwds_in = ZwdsAnalysisInput::new(input.clone(), is_male, None);
    let hd_in = HumanDesignAnalysisInput::new(input.clone());
    let qimen_in = QimenAnalysisInput::new(input.clone(), is_male);
    let num_in = NumerologyAnalysisInput::new(input.clone(), name, None);

    let unknown_time = input.unknown_time;

    let (saju_res, vedic_res, western_res, zwds_res, hd_res, qimen_res, num_res) =
        std::thread::scope(|s| {
            let saju_handle = s.spawn(|| analyze_saju(saju_in));
            let vedic_handle = s.spawn(|| analyze_vedic(vedic_in));
            let western_handle = s.spawn(|| analyze_western(western_in).ok());
            let zwds_handle = s.spawn(|| analyze_zwds(zwds_in).ok());
            let hd_handle = s.spawn(|| analyze_human_design(hd_in).ok());
            let qimen_handle = s.spawn(|| analyze_qimen(qimen_in).ok());
            let num_handle = s.spawn(|| analyze_numerology(num_in).ok());

            let saju = saju_handle
                .join()
                .unwrap_or_else(|_| Err(ServiceError::Saju("thread panicked".to_string())));
            let vedic = vedic_handle
                .join()
                .unwrap_or_else(|_| Err(ServiceError::Vedic("thread panicked".to_string())));
            let western = western_handle.join().unwrap_or(None);
            let zwds = zwds_handle.join().unwrap_or(None);
            let hd = hd_handle.join().unwrap_or(None);
            let qimen = qimen_handle.join().unwrap_or(None);
            let num = num_handle.join().unwrap_or(None);

            (saju, vedic, western, zwds, hd, qimen, num)
        });

    let saju = saju_res?;
    // Vedic is mandatory for a known birth time, but intentionally skipped without one.
    let vedic = match vedic_res {
        Ok(v) => Some(v),
        Err(ServiceError::BirthTimeRequired(_)) if unknown_time => None,
        Err(e) => return Err(e),
    };

    let skipped_engines = if unknown_time {
        [
            ("Vedic astrology", vedic.is_none()),
            ("Western astrology", western_res.is_none()),
            ("Zi Wei Dou Shu", zwds_res.is_none()),
            ("Human Design", hd_res.is_none()),
            ("QiMen DunJia", qimen_res.is_none()),
        ]
        .into_iter()
        .filter(|(_, skipped)| *skipped)
        .map(|(engine, _)| SkippedEngine {
            engine: engine.to_string(),
            reason: "Birth time of day is unknown; this engine depends on the ascendant, houses or hour-based chart and is not evaluated.".to_string(),
        })
        .collect()
    } else {
        Vec::new()
    };

    let omni_input = OmniDestinyTierInput {
        saju: saju.clone(),
        vedic: vedic.clone(),
        western: western_res.clone(),
        zwds: zwds_res.clone(),
        human_design: hd_res.clone(),
        qimen: qimen_res.clone(),
        numerology: num_res.clone(),
        transit: None,
    };

    let tier = analyze_destiny_tier_omni(omni_input)?;

    let handbook = crate::lexicon::build_handbook(
        &saju,
        vedic.as_ref(),
        western_res.as_ref(),
        zwds_res.as_ref(),
        hd_res.as_ref(),
        qimen_res.as_ref(),
        num_res.as_ref(),
    );

    Ok(SynthesizedOmniAnalysis {
        input,
        tier,
        saju,
        vedic,
        western: western_res,
        zwds: zwds_res,
        human_design: hd_res,
        qimen: qimen_res,
        numerology: num_res,
        handbook,
        skipped_engines,
    })
}

pub fn analyze_iching(input: SajuAnalysisInput) -> Result<IChingAnalysisOutput, ServiceError> {
    let saju_res = analyze_saju(input.clone())?;
    let pillars = &saju_res.report.pillars;
    let birth_year = input.base.year;
    let hour_pillar = pillars
        .hour
        .ok_or_else(|| ServiceError::BirthTimeRequired("I Ching HeLuo".to_string()))?;

    let res = eon_saju::analysis::heluo::calculate_heluo(
        birth_year,
        input.is_male,
        &pillars.year,
        &pillars.month,
        &pillars.day,
        &hour_pillar,
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

pub fn analyze_numerology(
    input: NumerologyAnalysisInput,
) -> Result<NumerologyAnalysisOutput, ServiceError> {
    crate::services::numerology::analyze(input)
}
