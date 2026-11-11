use anyhow::Result;
use clap::Args;
use eon_service::dto::NumerologyAnalysisInput;
use eon_service::facade;

use super::{BirthArgs, OutputFormat};

#[derive(Args, Debug)]
pub struct NumerologyCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Target year for Personal Year calculation (defaults to current year)
    #[arg(long)]
    pub target_year: Option<u32>,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: OutputFormat,
}

pub fn run(cmd: NumerologyCommand) -> Result<()> {
    let input = cmd.birth.to_analysis_input()?;
    let num_in = NumerologyAnalysisInput::new(input, cmd.birth.name.clone(), cmd.target_year);
    let res = facade::analyze_numerology(num_in)?;

    match cmd.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        _ => {
            let core = &res.result.core;
            println!("== 피타고라스 수비학 (Pythagorean Numerology) ==");
            println!(
                "생애여정수 (Life Path): {}{}",
                core.life_path,
                if res.result.is_master_life_path {
                    " (🌟 Master Number)"
                } else {
                    ""
                }
            );
            println!("표현수 (Expression): {}", core.expression);
            println!("영혼수 (Soul Urge): {}", core.soul_urge);
            println!("인격수 (Personality): {}", core.personality);
            println!("생일수 (Birthday): {}", core.birthday);
            println!("개인 연운수 (Personal Year): {}", res.result.personal_year);
            println!("4대 정점 주기:");
            for p in &res.result.pinnacles {
                println!("- {}", p.description_ko);
            }
        }
    }

    Ok(())
}
