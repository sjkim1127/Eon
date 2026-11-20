use anyhow::Result;
use clap::Args;
use eon_service::dto::SajuAnalysisInput;
use eon_service::facade;
use std::fmt::Write as _;

use super::{BirthArgs, EngineFormat, Rendered};

#[derive(Args, Debug)]
pub struct SajuCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Night rat hour option (Zi hour split)
    #[arg(long)]
    pub night_rat: bool,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

pub fn execute(cmd: &SajuCommand) -> Result<Rendered> {
    let input = cmd.birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male()?;
    let saju_in = SajuAnalysisInput::new(
        input.clone(),
        is_male,
        cmd.night_rat,
        Some(input.unknown_time),
    );

    let res = facade::analyze_saju(saju_in)?;

    let mut t = String::new();
    let p = &res.report.pillars;
    writeln!(t, "== Four Pillars of Destiny (Saju / BaZi) ==")?;
    let hour_str = match &p.hour {
        Some(h) => format!("{}({}-{})", h, h.stem.hangul(), h.branch.hangul()),
        None => "- (birth time unknown)".to_string(),
    };
    writeln!(
        t,
        "Year: {}({}-{}) | Month: {}({}-{}) | Day: {}({}-{}) | Hour: {}",
        p.year,
        p.year.stem.hangul(),
        p.year.branch.hangul(),
        p.month,
        p.month.stem.hangul(),
        p.month.branch.hangul(),
        p.day,
        p.day.stem.hangul(),
        p.day.branch.hangul(),
        hour_str
    )?;
    writeln!(
        t,
        "Day Master Strength: {:?} (Score: {:.1})",
        res.report.strength.strength_type, res.report.strength.strength_score
    )?;
    writeln!(t, "Structure: {:?}", res.report.structure.structure)?;
    writeln!(t, "Yongshin (Primary): {:?}", res.report.yongshin.primary)?;
    writeln!(
        t,
        "Five Elements Throughput: {:.1}%",
        res.qi_topology.throughput * 100.0
    )?;
    if let Some(gt) = &res.report.golden_time {
        writeln!(
            t,
            "Golden Period: Age {}~{} (Avg: {:.1} pts)",
            gt.start_age, gt.end_age, gt.average_score
        )?;
    }

    Rendered::new(&res, t)
}

pub fn run(cmd: SajuCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
