use anyhow::{anyhow, Result};
use clap::Args;
use eon_service::dto::HumanDesignAnalysisInput;
use eon_service::facade;
use std::fmt::Write as _;

use super::{BirthArgs, EngineFormat, Rendered};

#[derive(Args, Debug)]
pub struct PentaCommand {
    /// Group member as 'YYYY-MM-DD HH:MM[@city]' (repeat 3 to 5 times)
    #[arg(long = "member", required = true)]
    pub members: Vec<String>,

    /// Default city for members without '@city' (resolves coordinates and timezone)
    #[arg(short, long)]
    pub city: Option<String>,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

/// Parses `'1990-05-15 10:30@Busan'` into birth args (gender is irrelevant for Human Design).
pub fn parse_member(spec: &str, default_city: Option<&str>) -> BirthArgs {
    let (birth, city) = match spec.split_once('@') {
        Some((b, c)) => (b.trim(), Some(c.trim().to_string())),
        None => (spec.trim(), default_city.map(str::to_string)),
    };
    BirthArgs {
        birth: birth.to_string(),
        gender: "m".to_string(),
        city,
        lat: None,
        lon: None,
        tz: None,
        lunar: false,
        leap: false,
        name: None,
    }
}

pub fn execute(cmd: &PentaCommand) -> Result<Rendered> {
    if !(3..=5).contains(&cmd.members.len()) {
        return Err(anyhow!(
            "A Penta needs 3 to 5 members, got {}",
            cmd.members.len()
        ));
    }

    let mut inputs = Vec::new();
    for spec in &cmd.members {
        let args = parse_member(spec, cmd.city.as_deref());
        inputs.push(HumanDesignAnalysisInput::new(args.to_analysis_input()?));
    }
    let res = facade::analyze_hd_penta(inputs)?;

    let mut t = String::new();
    writeln!(t, "== Human Design Penta ({} members) ==", res.participants)?;
    writeln!(
        t,
        "Fully defined channels: {} / {}",
        res.fully_defined_channels,
        res.channels.len()
    )?;
    for c in &res.channels {
        let status = if c.is_active {
            "active".to_string()
        } else {
            format!("missing gates {:?}", c.missing_gates)
        };
        writeln!(
            t,
            "- {} ({}-{}): {}",
            c.name, c.channel.0, c.channel.1, status
        )?;
    }
    if !res.gaps.is_empty() {
        writeln!(
            t,
            "Gates needed to complete partial channels: {:?}",
            res.gaps
        )?;
    }

    Rendered::new(&res, t)
}

pub fn run(cmd: PentaCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
