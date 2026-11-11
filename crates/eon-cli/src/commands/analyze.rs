use anyhow::Result;
use clap::Args;
use eon_service::dto::*;
use eon_service::facade;
use serde::Serialize;

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

#[derive(Serialize)]
pub struct FullOmniResult {
    pub input: AnalysisInput,
    pub tier: TierResult,
    pub saju: SajuAnalysisOutput,
    pub vedic: VedicAnalysisOutput,
    pub western: Option<WesternAnalysisOutput>,
    pub zwds: Option<ZwdsAnalysisOutput>,
    pub human_design: Option<HumanDesignAnalysisOutput>,
    pub qimen: Option<QimenAnalysisOutput>,
    pub numerology: Option<NumerologyAnalysisOutput>,
}

pub fn run(cmd: AnalyzeCommand) -> Result<()> {
    let input = cmd.birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male();

    // 1. Individual engine analyses
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

    // 2. Omni Destiny Tier 7.0 Synthesis
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

    // 3. Output by requested format
    match cmd.format {
        OutputFormat::Json => {
            let full = FullOmniResult {
                input,
                tier: tier_res,
                saju: saju_res,
                vedic: vedic_res,
                western: western_res,
                zwds: zwds_res,
                human_design: hd_res,
                qimen: qimen_res,
                numerology: num_res,
            };
            println!("{}", serde_json::to_string_pretty(&full)?);
        }
        OutputFormat::Agent | OutputFormat::Text => {
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
        }
        OutputFormat::Markdown => {
            let md = format_markdown_report(
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
            println!("{}", md);
        }
    }

    Ok(())
}
