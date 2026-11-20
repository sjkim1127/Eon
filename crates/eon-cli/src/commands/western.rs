use anyhow::Result;
use clap::Args;
use eon_service::dto::WesternAnalysisInput;
use eon_service::facade;
use std::fmt::Write as _;

use super::{BirthArgs, EngineFormat, Rendered};

#[derive(Args, Debug)]
pub struct WesternCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// House system: Placidus (default), Koch, WholeSign, Equal, Porphyry, Regiomontanus,
    /// Campanus, Alcabitius, Morinus, Topocentric
    #[arg(long, default_value = "Placidus")]
    pub house_system: String,

    /// Secondary progression (a day for a year) to this date: 'YYYY-MM-DD[ HH:MM]' in the
    /// birth timezone, or RFC 3339
    #[arg(long)]
    pub progress_to: Option<String>,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

pub fn execute(cmd: &WesternCommand) -> Result<Rendered> {
    let input = cmd.birth.to_analysis_input()?;
    let west_in = WesternAnalysisInput::new(input.clone(), cmd.house_system.clone());

    if let Some(target) = &cmd.progress_to {
        let target_time = eon_service::birth::parse_instant(target, &input.timezone)?;
        let res = facade::analyze_western_progression(west_in, target_time)?;

        let mut t = String::new();
        writeln!(
            t,
            "== Western Astrology: Secondary Progression to {} ==",
            target
        )?;
        writeln!(
            t,
            "Progressed age: {:.2} years",
            res.result.progressed_age_years
        )?;
        for p in &res.result.progressed_chart.planets {
            writeln!(
                t,
                "- {:<8}: Sign {} {:05.2}° (House {})",
                p.name, p.sign_index, p.degree_in_sign, p.house_number
            )?;
        }
        writeln!(
            t,
            "Progressed-to-natal aspects: {}",
            res.result.active_transit_aspects.len()
        )?;
        for a in res.result.active_transit_aspects.iter().take(10) {
            writeln!(
                t,
                "- progressed {} {:?} natal {} (orb {:.2}°)",
                a.body_a_name, a.aspect_type, a.body_b_name, a.orb
            )?;
        }
        return Rendered::new(&res, t);
    }

    let res = facade::analyze_western(west_in)?;

    let mut t = String::new();
    writeln!(t, "== Western Astrology ==")?;
    for p in &res.result.planets {
        writeln!(
            t,
            "- {:<8}: Sign {} {:05.2}° (House {})",
            p.name, p.sign_index, p.longitude, p.house_number
        )?;
    }
    writeln!(
        t,
        "Aspects: {} | Aspect Configurations: {}",
        res.result.aspects.len(),
        res.result.aspect_patterns.len()
    )?;

    Rendered::new(&res, t)
}

pub fn run(cmd: WesternCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
