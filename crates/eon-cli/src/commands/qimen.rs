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
            println!("== 기문둔갑 (QiMen DunJia) ==");
            println!(
                "국수: {} {}국",
                if res.report.pan.is_yin_ju {
                    "음둔"
                } else {
                    "양둔"
                },
                res.report.pan.ju_number
            );
            if let Some(chief) = &res.report.pan.value_chief_star {
                println!("직부성: {:?}", chief);
            }
            if let Some(envoy) = &res.report.pan.value_envoy_door {
                println!("직사문: {:?}", envoy);
            }
            println!("궁 배치 수: {}개 궁", res.report.pan.palaces.len());
        }
    }

    Ok(())
}
