use anyhow::{anyhow, Result};
use clap::{Args, ValueEnum};
use eon_numerology::NumerologySystem;
use eon_service::dto::NumerologyAnalysisInput;
use eon_service::facade;
use std::fmt::Write as _;

use super::{BirthArgs, EngineFormat, Rendered};

/// Letter-to-number table for the name numbers.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NumerologySystemArg {
    #[default]
    Pythagorean,
    Chaldean,
}

impl From<NumerologySystemArg> for NumerologySystem {
    fn from(arg: NumerologySystemArg) -> Self {
        match arg {
            NumerologySystemArg::Pythagorean => NumerologySystem::Pythagorean,
            NumerologySystemArg::Chaldean => NumerologySystem::Chaldean,
        }
    }
}

#[derive(Args, Debug)]
pub struct NumerologyCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Target year for Personal Year calculation (defaults to the current year)
    #[arg(long, conflicts_with = "target_date")]
    pub target_year: Option<u32>,

    /// Target date 'YYYY-MM-DD' for Personal Year, Month and Day (defaults to today)
    #[arg(long)]
    pub target_date: Option<String>,

    /// Letter system for the name numbers: pythagorean (default) or chaldean
    #[arg(long, value_enum, default_value = "pythagorean")]
    pub system: NumerologySystemArg,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

/// Parses 'YYYY-MM-DD' (the calendar validity is checked by the service).
pub fn parse_target_date(raw: &str) -> Result<(u32, u32, u32)> {
    let invalid = || anyhow!("Invalid target date '{raw}'. Expected 'YYYY-MM-DD'");
    let mut parts = raw.trim().splitn(3, '-');
    let mut next = || -> Result<u32> {
        parts
            .next()
            .ok_or_else(invalid)?
            .parse()
            .map_err(|_| invalid())
    };
    Ok((next()?, next()?, next()?))
}

pub fn execute(cmd: &NumerologyCommand) -> Result<Rendered> {
    let input = cmd.birth.to_analysis_input()?;
    let mut num_in = NumerologyAnalysisInput::new(input, cmd.birth.name.clone(), cmd.target_year)
        .with_system(cmd.system.into());
    if let Some(raw) = &cmd.target_date {
        let (y, m, d) = parse_target_date(raw)?;
        num_in = num_in.with_target_date(y, m, d);
    }
    let res = facade::analyze_numerology(num_in)?;

    let needs_name = "N/A (pass --name with a Latin name)";
    let core = &res.result.core;
    let opt = |v: Option<u8>| {
        v.map(|v| v.to_string())
            .unwrap_or_else(|| needs_name.to_string())
    };
    let mut t = String::new();
    writeln!(t, "== {} Numerology ==", res.result.system.label())?;
    writeln!(
        t,
        "Life Path Number: {}{}",
        core.life_path,
        if res.result.is_master_life_path {
            " (Master Number)"
        } else {
            ""
        }
    )?;
    writeln!(t, "Expression Number: {}", opt(core.expression))?;
    writeln!(t, "Soul Urge Number: {}", opt(core.soul_urge))?;
    writeln!(t, "Personality Number: {}", opt(core.personality))?;
    writeln!(t, "Birthday Number: {}", core.birthday)?;
    writeln!(t, "Maturity Number: {}", opt(core.maturity))?;
    writeln!(t, "Personal Year Vibration: {}", res.result.personal_year)?;
    if let Some(m) = res.result.personal_month {
        writeln!(t, "Personal Month Vibration: {m}")?;
    }
    if let Some(d) = res.result.personal_day {
        writeln!(t, "Personal Day Vibration: {d}")?;
    }
    if res.result.karmic_debts.is_empty() {
        writeln!(t, "Karmic Debt: none")?;
    } else {
        writeln!(t, "Karmic Debt:")?;
        for k in &res.result.karmic_debts {
            writeln!(
                t,
                "- {}/{} in {}: {}",
                k.debt_number, k.reduces_to, k.source, k.meaning_en
            )?;
        }
    }
    if let Some(ni) = &res.result.name_insights {
        let list = |v: &[u8]| {
            if v.is_empty() {
                "none".to_string()
            } else {
                v.iter().map(u8::to_string).collect::<Vec<_>>().join(", ")
            }
        };
        writeln!(t, "Hidden Passion: {}", list(&ni.hidden_passions))?;
        writeln!(t, "Balance Number: {}", ni.balance)?;
        writeln!(
            t,
            "Cornerstone: {} ({}), Capstone: {} ({})",
            ni.cornerstone.letter, ni.cornerstone.value, ni.capstone.letter, ni.capstone.value
        )?;
        writeln!(
            t,
            "Missing Numbers (Karmic Lessons): {}",
            list(&ni.missing_numbers)
        )?;
    }
    writeln!(t, "Four Pinnacle Cycles:")?;
    for p in &res.result.pinnacles {
        writeln!(t, "- {}", p.description_en)?;
    }

    Rendered::new(&res, t)
}

pub fn run(cmd: NumerologyCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
