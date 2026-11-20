use anyhow::{anyhow, Result};
use clap::{Args, ValueEnum};
use eon_human_design::connection::HumanDesignConnectionResult;
use eon_human_design::transit::{HumanDesignTransitResult, ReturnType};
use eon_service::dto::HumanDesignAnalysisInput;
use eon_service::facade;
use std::fmt::Write as _;

use super::{BirthArgs, EngineFormat, Rendered};

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdReturn {
    Solar,
    Saturn,
    Chiron,
}

impl From<HdReturn> for ReturnType {
    fn from(value: HdReturn) -> Self {
        match value {
            HdReturn::Solar => ReturnType::Solar,
            HdReturn::Saturn => ReturnType::Saturn,
            HdReturn::Chiron => ReturnType::Chiron,
        }
    }
}

#[derive(Args, Debug)]
pub struct HdCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Overlay the sky of this date on the natal chart: 'YYYY-MM-DD[ HH:MM]' in the birth
    /// timezone, or RFC 3339
    #[arg(long, conflicts_with = "return_type")]
    pub transit_at: Option<String>,

    /// Planetary return chart overlaid on the natal chart (solar needs --year)
    #[arg(long = "return", value_enum, id = "return_type")]
    pub return_type: Option<HdReturn>,

    /// Year of the solar return
    #[arg(long)]
    pub year: Option<i32>,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

/// One-paragraph summary of how two charts connect (shared by compat, transit and return).
pub fn connection_text(conn: &HumanDesignConnectionResult) -> String {
    let channels = |v: &[(u8, u8)]| {
        if v.is_empty() {
            "-".to_string()
        } else {
            v.iter()
                .map(|(a, b)| format!("{a}-{b}"))
                .collect::<Vec<_>>()
                .join(", ")
        }
    };
    let mut centers: Vec<String> = conn
        .defined_centers
        .iter()
        .map(|c| format!("{c:?}"))
        .collect();
    centers.sort();

    format!(
        "Electromagnetic channels: {}\nCompromise channels: {}\nDominance channels: {}\nCompanionship channels: {}\nCombined defined centers ({}): {}\n",
        channels(&conn.electromagnetic_channels),
        channels(&conn.compromise_channels),
        channels(&conn.dominance_channels),
        channels(&conn.companionship_channels),
        centers.len(),
        centers.join(", ")
    )
}

fn overlay_text(title: &str, res: &HumanDesignTransitResult) -> Result<String> {
    let mut t = String::new();
    writeln!(t, "== {title} ==")?;
    writeln!(
        t,
        "Natal: {} | Profile {} | Authority {}",
        res.natal_chart.chart_type, res.natal_chart.profile, res.natal_chart.authority
    )?;
    writeln!(t, "Sky at {}", res.target_date)?;
    let mut gates = res.transit_chart.active_gates.clone();
    gates.sort_unstable();
    gates.dedup();
    writeln!(t, "Transit gates: {gates:?}")?;
    write!(t, "{}", connection_text(&res.composite_connection))?;
    Ok(t)
}

pub fn execute(cmd: &HdCommand) -> Result<Rendered> {
    let input = cmd.birth.to_analysis_input()?;
    let hd_in = HumanDesignAnalysisInput::new(input.clone());

    if let Some(at) = &cmd.transit_at {
        let when = eon_service::birth::parse_instant(at, &input.timezone)?;
        let res = facade::analyze_hd_transit(hd_in, when)?;
        let text = overlay_text("Human Design Transit", &res)?;
        return Rendered::new(&res, text);
    }

    if let Some(kind) = cmd.return_type {
        let year = match (kind, cmd.year) {
            (_, Some(y)) => y,
            (HdReturn::Solar, None) => {
                return Err(anyhow!("--return solar requires --year <YYYY>"));
            }
            // The year only matters for solar returns.
            (_, None) => input.year,
        };
        let res = facade::analyze_hd_return(hd_in, kind.into(), year)?;
        let text = overlay_text(&format!("Human Design {kind:?} Return"), &res)?;
        return Rendered::new(&res, text);
    }

    let res = facade::analyze_human_design(hd_in)?;

    let mut t = String::new();
    writeln!(t, "== Human Design ==")?;
    writeln!(t, "Type: {}", res.result.chart_type)?;
    writeln!(t, "Profile: {}", res.result.profile)?;
    writeln!(t, "Strategy: {}", res.result.strategy)?;
    writeln!(t, "Authority: {}", res.result.authority)?;
    writeln!(
        t,
        "Defined Centers (Total {}): {:?}",
        res.result.defined_centers.len(),
        res.result.defined_centers
    )?;
    writeln!(
        t,
        "Active Channels (Total {}): {:?}",
        res.result.active_channels.len(),
        res.result.active_channels
    )?;

    Rendered::new(&res, t)
}

pub fn run(cmd: HdCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
