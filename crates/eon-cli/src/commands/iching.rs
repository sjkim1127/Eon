use anyhow::Result;
use clap::Args;
use eon_service::dto::SajuAnalysisInput;
use eon_service::facade;
use std::fmt::Write as _;

use super::{BirthArgs, EngineFormat, Rendered};

#[derive(Args, Debug)]
pub struct IchingCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

pub fn execute(cmd: &IchingCommand) -> Result<Rendered> {
    let input = cmd.birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male()?;
    let saju_in = SajuAnalysisInput::new(input, is_male, false, None);
    let res = facade::analyze_iching(saju_in)?;

    let mut t = String::new();
    writeln!(t, "== I Ching / HeLuo Destiny Hexagrams (河洛理數) ==")?;
    writeln!(
        t,
        "Pre-natal Hexagram (先天卦): #{}",
        res.result.pre_natal_hexagram
    )?;
    writeln!(
        t,
        "Post-natal Hexagram (後天卦): #{}",
        res.result.post_natal_hexagram
    )?;
    writeln!(
        t,
        "Principal Governing Line (元堂爻): Line {}",
        res.result.yuan_dang_yao
    )?;
    writeln!(t, "Ti-Yong Relation (體用): {:?}", res.result.ti_yong)?;
    writeln!(
        t,
        "Lifetime Transformation Cycles: {} periods",
        res.result.lifetime_cycles.len()
    )?;
    for cycle in res.result.lifetime_cycles.iter().take(5) {
        writeln!(
            t,
            "- Ages {}~{}: Hexagram #{}",
            cycle.start_age, cycle.end_age, cycle.hexagram_index
        )?;
    }

    Rendered::new(&res, t)
}

pub fn run(cmd: IchingCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
