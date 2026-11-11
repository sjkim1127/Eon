use anyhow::Result;
use clap::Args;
use eon_service::dto::WesternAnalysisInput;
use eon_service::facade;

use super::{BirthArgs, OutputFormat};

#[derive(Args, Debug)]
pub struct WesternCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// House system: Placidus (default), Koch, WholeSign, Equal
    #[arg(long, default_value = "Placidus")]
    pub house_system: String,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: OutputFormat,
}

pub fn run(cmd: WesternCommand) -> Result<()> {
    let input = cmd.birth.to_analysis_input()?;
    let west_in = WesternAnalysisInput::new(input, cmd.house_system);
    let res = facade::analyze_western(west_in)?;

    match cmd.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        _ => {
            println!("== Western Astrology ==");
            for p in &res.result.planets {
                println!(
                    "- {:<8}: Sign {} {:05.2}° (House {})",
                    p.name, p.sign_index, p.longitude, p.house_number
                );
            }
            println!(
                "Aspects: {} | Aspect Configurations: {}",
                res.result.aspects.len(),
                res.result.aspect_patterns.len()
            );
        }
    }

    Ok(())
}
