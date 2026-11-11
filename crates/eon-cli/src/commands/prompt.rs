use anyhow::Result;
use clap::Args;
use eon_service::dto::*;
use eon_service::facade;

use super::BirthArgs;
use crate::format::agent::format_agent_context;

#[derive(Args, Debug)]
pub struct PromptCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Add system prompt instruction prefix for LLMs
    #[arg(short, long)]
    pub system: bool,
}

pub fn run(cmd: PromptCommand) -> Result<()> {
    let input = cmd.birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male();

    let saju_in = SajuAnalysisInput::new(input.clone(), is_male, false, Some(false));
    let vedic_in = VedicAnalysisInput::new(input.clone(), Some(false), None);
    let western_in = WesternAnalysisInput::new(input.clone(), "Placidus".to_string());
    let zwds_in = ZwdsAnalysisInput::new(input.clone(), is_male, None);
    let hd_in = HumanDesignAnalysisInput::new(input.clone());
    let qimen_in = QimenAnalysisInput::new(input.clone(), is_male);
    let num_in = NumerologyAnalysisInput::new(input.clone(), cmd.birth.name.clone(), None);

    let saju_res = facade::analyze_saju(saju_in)?;
    let vedic_res = facade::analyze_vedic(vedic_in)?;
    let western_res = facade::analyze_western(western_in).ok();
    let zwds_res = facade::analyze_zwds(zwds_in).ok();
    let hd_res = facade::analyze_human_design(hd_in).ok();
    let qimen_res = facade::analyze_qimen(qimen_in).ok();
    let num_res = facade::analyze_numerology(num_in).ok();

    let omni_input = OmniDestinyTierInput {
        saju: saju_res.clone(),
        vedic: vedic_res.clone(),
        western: western_res.clone(),
        zwds: zwds_res.clone(),
        human_design: hd_res.clone(),
        qimen: qimen_res.clone(),
        numerology: num_res.clone(),
        transit: None,
    };

    let tier_res = facade::analyze_destiny_tier_omni(omni_input)?;

    if cmd.system {
        println!("You are an expert Destiny Intelligence AI Counselor powered by the Eon 8-Engine Framework.");
        println!("Below is the verified multi-disciplinary astrological and metaphysical calculation context for the client.");
        println!("Synthesize these cross-engine insights to provide deep, actionable, compassionate, and precise counseling.\n");
    }

    let text = format_agent_context(
        &input,
        Some(&saju_res),
        Some(&vedic_res),
        western_res.as_ref(),
        zwds_res.as_ref(),
        hd_res.as_ref(),
        qimen_res.as_ref(),
        num_res.as_ref(),
        Some(&tier_res),
    );

    println!("{}", text);
    Ok(())
}
