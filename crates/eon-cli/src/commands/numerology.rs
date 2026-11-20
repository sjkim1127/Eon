use anyhow::Result;
use clap::Args;
use eon_service::dto::NumerologyAnalysisInput;
use eon_service::facade;
use std::fmt::Write as _;

use super::{BirthArgs, EngineFormat, Rendered};

#[derive(Args, Debug)]
pub struct NumerologyCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Target year for Personal Year calculation (defaults to current year)
    #[arg(long)]
    pub target_year: Option<u32>,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

pub fn execute(cmd: &NumerologyCommand) -> Result<Rendered> {
    let input = cmd.birth.to_analysis_input()?;
    let num_in = NumerologyAnalysisInput::new(input, cmd.birth.name.clone(), cmd.target_year);
    let res = facade::analyze_numerology(num_in)?;

    let needs_name = "N/A (pass --name with a Latin name)";
    let core = &res.result.core;
    let mut t = String::new();
    writeln!(t, "== Pythagorean Numerology ==")?;
    writeln!(
        t,
        "Life Path Number: {}{}",
        core.life_path,
        if res.result.is_master_life_path {
            " (Master Number)"
        } else {
            ""
        }
    )?;
    writeln!(
        t,
        "Expression Number: {}",
        core.expression
            .map(|v| v.to_string())
            .unwrap_or_else(|| needs_name.to_string())
    )?;
    writeln!(
        t,
        "Soul Urge Number: {}",
        core.soul_urge
            .map(|v| v.to_string())
            .unwrap_or_else(|| needs_name.to_string())
    )?;
    writeln!(
        t,
        "Personality Number: {}",
        core.personality
            .map(|v| v.to_string())
            .unwrap_or_else(|| needs_name.to_string())
    )?;
    writeln!(t, "Birthday Number: {}", core.birthday)?;
    writeln!(t, "Personal Year Vibration: {}", res.result.personal_year)?;
    writeln!(t, "Four Pinnacle Cycles:")?;
    for p in &res.result.pinnacles {
        writeln!(t, "- {}", p.description_en)?;
    }

    Rendered::new(&res, t)
}

pub fn run(cmd: NumerologyCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
