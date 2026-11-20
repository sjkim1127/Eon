use anyhow::Result;
use clap::Args;
use eon_service::dto::ZwdsAnalysisInput;
use eon_service::facade;
use std::fmt::Write as _;

use super::{BirthArgs, EngineFormat, Rendered};

#[derive(Args, Debug)]
pub struct ZwdsCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Year for the annual (流年) palace and four transformations (defaults to the current year)
    #[arg(long)]
    pub target_year: Option<i32>,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

pub fn execute(cmd: &ZwdsCommand) -> Result<Rendered> {
    let input = cmd.birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male()?;
    let zwds_in = ZwdsAnalysisInput::new(input, is_male, cmd.target_year);
    let res = facade::analyze_zwds(zwds_in)?;

    let mut t = String::new();
    let soul = &res.chart.palaces[res.chart.soul_idx];
    let body = &res.chart.palaces[res.chart.body_idx];
    writeln!(t, "== Zi Wei Dou Shu (Purple Star Astrology) ==")?;
    writeln!(
        t,
        "Life Palace: {} | Body Palace: {}",
        soul.name.english(),
        body.name.english()
    )?;
    writeln!(t, "Five Elements Bureau: {:?}", res.chart.five_elements)?;
    writeln!(t, "Stars in Life Palace:")?;
    for s in &soul.stars {
        writeln!(t, "- {:?} ({:?})", s.star, s.brightness)?;
    }
    if !res.chart.destiny_patterns.is_empty() {
        writeln!(
            t,
            "Key Destiny Formations (Total {}):",
            res.chart.destiny_patterns.len()
        )?;
        for p in &res.chart.destiny_patterns {
            writeln!(t, "- {}: {}", p.name_english, p.description_english)?;
        }
    }

    Rendered::new(&res, t)
}

pub fn run(cmd: ZwdsCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
