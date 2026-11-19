use anyhow::Result;
use clap::Args;
use eon_service::facade;

use super::{BirthArgs, OutputFormat};
use crate::format::agent::format_agent_context;
use crate::format::markdown::format_markdown_report;

#[derive(Args, Debug)]
pub struct AnalyzeCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Output format: agent (LLM Context), json (raw structured), markdown, text
    #[arg(short, long, value_enum, default_value = "agent")]
    pub format: OutputFormat,
}

pub fn run(cmd: AnalyzeCommand) -> Result<()> {
    let input = cmd.birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male()?;
    let name = cmd.birth.name.clone();

    let omni = facade::analyze_all_synthesized(input, is_male, name)?;

    match cmd.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&omni)?);
        }
        OutputFormat::Agent | OutputFormat::Text => {
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
        }
        OutputFormat::Markdown => {
            let md = format_markdown_report(
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
            println!("{}", md);
        }
    }

    Ok(())
}
