use anyhow::Result;
use clap::Args;
use eon_service::dto::HumanDesignAnalysisInput;
use eon_service::facade;

use super::{BirthArgs, OutputFormat};

#[derive(Args, Debug)]
pub struct HdCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: OutputFormat,
}

pub fn run(cmd: HdCommand) -> Result<()> {
    let input = cmd.birth.to_analysis_input()?;
    let hd_in = HumanDesignAnalysisInput::new(input);
    let res = facade::analyze_human_design(hd_in)?;

    match cmd.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        _ => {
            println!("== 휴먼 디자인 (Human Design) ==");
            println!("Type: {}", res.result.chart_type);
            println!("Profile: {}", res.result.profile);
            println!("Strategy: {}", res.result.strategy);
            println!("Authority: {}", res.result.authority);
            println!(
                "정의 센터: {}개 ({:?})",
                res.result.defined_centers.len(),
                res.result.defined_centers
            );
            println!(
                "활성 채널: {}개 ({:?})",
                res.result.active_channels.len(),
                res.result.active_channels
            );
        }
    }

    Ok(())
}
