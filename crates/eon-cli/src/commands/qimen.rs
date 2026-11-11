use anyhow::Result;
use clap::Args;
use eon_service::dto::QimenAnalysisInput;
use eon_service::facade;

use super::{BirthArgs, OutputFormat};

#[derive(Args, Debug)]
pub struct QimenCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: OutputFormat,
}

pub fn run(cmd: QimenCommand) -> Result<()> {
    let input = cmd.birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male();
    let qimen_in = QimenAnalysisInput::new(input, is_male);
    let res = facade::analyze_qimen(qimen_in)?;

    match cmd.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        _ => {
            println!("== QiMen DunJia ==");
            println!(
                "Ju (Structure): {} Ju {}",
                if res.report.pan.is_yin_ju {
                    "Yin Dun (陰遁)"
                } else {
                    "Yang Dun (陽遁)"
                },
                res.report.pan.ju_number
            );
            if let Some(chief) = &res.report.pan.value_chief_star {
                println!("Chief Star (Zhi Fu): {:?}", chief);
            }
            if let Some(envoy) = &res.report.pan.value_envoy_door {
                println!("Envoy Door (Zhi Shi): {:?}", envoy);
            }
            println!("Palaces Deployed: {} palaces", res.report.pan.palaces.len());
        }
    }

    Ok(())
}
