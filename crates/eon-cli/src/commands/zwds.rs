use anyhow::Result;
use clap::Args;
use eon_service::dto::ZwdsAnalysisInput;
use eon_service::facade;

use super::{BirthArgs, OutputFormat};

#[derive(Args, Debug)]
pub struct ZwdsCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: OutputFormat,
}

pub fn run(cmd: ZwdsCommand) -> Result<()> {
    let input = cmd.birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male();
    let zwds_in = ZwdsAnalysisInput::new(input, is_male, None);
    let res = facade::analyze_zwds(zwds_in)?;

    match cmd.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        _ => {
            let soul = &res.chart.palaces[res.chart.soul_idx];
            println!("== 자미두수 (Zi Wei Dou Shu) ==");
            println!(
                "명궁: {} | 신궁: {}",
                soul.name.korean(),
                res.chart.palaces[res.chart.body_idx].name.korean()
            );
            println!("오행국: {:?}", res.chart.five_elements);
            println!("명궁 주성:");
            for s in &soul.stars {
                println!("- {} ({:?})", s.star.korean(), s.brightness);
            }
            if !res.chart.destiny_patterns.is_empty() {
                println!("성반 주요 격국: {}개", res.chart.destiny_patterns.len());
                for p in &res.chart.destiny_patterns {
                    println!("- {}: {}", p.name_korean, p.description_korean);
                }
            }
        }
    }

    Ok(())
}
