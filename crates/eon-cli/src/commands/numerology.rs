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
            println!("== Pythagorean Numerology ==");
            println!(
                "Life Path Number: {}{}",
                core.life_path,
                if res.result.is_master_life_path {
                    " (🌟 Master Number)"
                } else {
                    ""
                }
            );
            println!("Expression Number: {}", core.expression);
            println!("Soul Urge Number: {}", core.soul_urge);
            println!("Personality Number: {}", core.personality);
            println!("Birthday Number: {}", core.birthday);
            println!("Personal Year Vibration: {}", res.result.personal_year);
            println!("Four Pinnacle Cycles:");
            for p in &res.result.pinnacles {
                println!("- {}", p.description_ko);
            }
        }
    }

    Ok(())
}
