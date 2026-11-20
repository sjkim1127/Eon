use anyhow::Result;
use clap::Args;
use eon_service::dto::{SajuAnalysisInput, TransitAnalysisInput};
use eon_service::facade;
use std::fmt::Write as _;

use super::{BirthArgs, EngineFormat, Rendered};

#[derive(Args, Debug)]
pub struct TransitCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Moment to evaluate: 'YYYY-MM-DD[ HH:MM]' in the birth timezone, or RFC 3339 (default: now)
    #[arg(long)]
    pub at: Option<String>,

    /// Night rat hour option (Zi hour split)
    #[arg(long)]
    pub night_rat: bool,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

pub fn execute(cmd: &TransitCommand) -> Result<Rendered> {
    let input = cmd.birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male()?;
    let now = match &cmd.at {
        Some(text) => Some(eon_service::birth::parse_instant(text, &input.timezone)?),
        None => None,
    };
    let saju_in = SajuAnalysisInput::new(
        input.clone(),
        is_male,
        cmd.night_rat,
        Some(input.unknown_time),
    );
    let res = facade::analyze_transit(TransitAnalysisInput::new(saju_in, now))?;

    let mut t = String::new();
    writeln!(t, "== Saju Luck Cycles (運勢) ==")?;
    writeln!(t, "Age: {}", res.current_age)?;
    let y = &res.yearly_luck;
    writeln!(
        t,
        "Year  {}: {} (stem {:?} / branch {:?})",
        y.year, y.ganzi, y.stem_god, y.branch_god
    )?;
    let m = &res.monthly_luck;
    writeln!(
        t,
        "Month {}-{:02}: {} (stem {:?} / branch {:?})",
        m.year, m.month, m.ganzi, m.stem_god, m.branch_god
    )?;
    let d = &res.daily_luck;
    writeln!(
        t,
        "Day   {}-{:02}-{:02}: {} (stem {:?} / branch {:?})",
        d.year, d.month, d.day, d.ganzi, d.stem_god, d.branch_god
    )?;
    let h = &res.hourly_luck;
    writeln!(
        t,
        "Hour  {:02}:00: {} (stem {:?} / branch {:?})",
        h.hour, h.ganzi, h.stem_god, h.branch_god
    )?;
    for (label, events) in [
        ("Year", &y.special_events),
        ("Month", &m.special_events),
        ("Day", &d.special_events),
    ] {
        for event in events {
            writeln!(t, "  [{label}] {event}")?;
        }
    }
    if let Some(frame) = &res.current_frame {
        writeln!(
            t,
            "Current frame score: {:.1} (major luck {})",
            frame.score, frame.major_ganzi
        )?;
    }
    if !res.nearby_diagnostics.is_empty() {
        writeln!(t, "Nearby load diagnostics:")?;
        for d in res.nearby_diagnostics.iter().take(6) {
            writeln!(
                t,
                "- age {} {:?}: {} -> {}",
                d.age, d.status, d.reason, d.strategy
            )?;
        }
    }

    Rendered::new(&res, t)
}

pub fn run(cmd: TransitCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
