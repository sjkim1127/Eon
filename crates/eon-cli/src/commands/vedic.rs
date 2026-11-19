use anyhow::Result;
use clap::Args;
use eon_service::dto::VedicAnalysisInput;
use eon_service::facade;

use super::{BirthArgs, OutputFormat};

#[derive(Args, Debug)]
pub struct VedicCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: OutputFormat,
}

pub fn run(cmd: VedicCommand) -> Result<()> {
    let input = cmd.birth.to_analysis_input()?;
    let vedic_in = VedicAnalysisInput::new(input.clone(), Some(input.unknown_time), None);
    let res = facade::analyze_vedic(vedic_in)?;

    match cmd.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        _ => {
            println!("== Vedic Astrology (Jyotish) ==");
            println!("Ascendant (Lagna): {:?}", res.chart.ascendant);
            println!("Moon Nakshatra Info: {}", res.report.nakshatra_info);
            println!("Sade Sati: {:?}", res.report.sade_sati);
            println!(
                "Overall House Strength: {}",
                res.report.overall_strength_score
            );
            println!("Detected Yogas (Total {}):", res.report.yogas.len());
            for y in res.report.yogas.iter().take(5) {
                println!("- {} ({:?}): {}", y.name, y.quality, y.description);
            }
        }
    }

    Ok(())
}
