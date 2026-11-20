use anyhow::Result;
use clap::Args;
use eon_service::dto::QimenAnalysisInput;
use eon_service::facade;
use std::fmt::Write as _;

use super::{BirthArgs, EngineFormat, Rendered};

#[derive(Args, Debug)]
pub struct QimenCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

pub fn execute(cmd: &QimenCommand) -> Result<Rendered> {
    let input = cmd.birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male()?;
    let qimen_in = QimenAnalysisInput::new(input, is_male);
    let res = facade::analyze_qimen(qimen_in)?;

    let mut t = String::new();
    writeln!(t, "== QiMen DunJia ==")?;
    writeln!(
        t,
        "Ju (Structure): {} Ju {}",
        if res.report.pan.is_yin_ju {
            "Yin Dun (陰遁)"
        } else {
            "Yang Dun (陽遁)"
        },
        res.report.pan.ju_number
    )?;
    if let Some(chief) = &res.report.pan.value_chief_star {
        writeln!(t, "Chief Star (Zhi Fu): {:?}", chief)?;
    }
    if let Some(envoy) = &res.report.pan.value_envoy_door {
        writeln!(t, "Envoy Door (Zhi Shi): {:?}", envoy)?;
    }
    writeln!(
        t,
        "Palaces Deployed: {} palaces",
        res.report.pan.palaces.len()
    )?;

    Rendered::new(&res, t)
}

pub fn run(cmd: QimenCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
