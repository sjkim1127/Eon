use anyhow::Result;
use clap::Args;
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
    let is_male = cmd.birth.is_male()?;
    let name = cmd.birth.name.clone();

    let omni = facade::analyze_all_synthesized(input, is_male, name)?;

    if cmd.system {
        println!("You are an expert Destiny Intelligence AI Counselor powered by the Eon 8-Engine Framework.");
        println!("Below is the verified multi-disciplinary astrological and metaphysical calculation context for the client.");
        println!("Synthesize these cross-engine insights to provide deep, actionable, compassionate, and precise counseling.\n");
    }

    let text = format_agent_context(
        &omni.input,
        Some(&omni.saju),
        omni.vedic.as_ref(),
        omni.western.as_ref(),
        omni.zwds.as_ref(),
        omni.human_design.as_ref(),
        omni.qimen.as_ref(),
        omni.numerology.as_ref(),
        Some(&omni.tier),
        Some(&omni.handbook),
    );

    println!("{}", text);
    Ok(())
}
