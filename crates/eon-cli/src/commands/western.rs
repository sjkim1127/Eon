use anyhow::{anyhow, Result};
use clap::{ArgGroup, Args};
use eon_service::dto::WesternAnalysisInput;
use eon_service::facade;
use eon_western::{WesternAspectData, SIGN_NAMES};
use std::fmt::Write as _;

use super::{BirthArgs, EngineFormat, Rendered};

#[derive(Args, Debug)]
#[command(group(
    ArgGroup::new("technique")
        .args(["progress_to", "transit_at", "solar_arc_to", "profection_at", "solar_return", "lunar_return_after"])
        .multiple(false)
))]
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

    /// Transits: the sky at this date over the natal chart
    #[arg(long)]
    pub transit_at: Option<String>,

    /// Solar arc directions to this date
    #[arg(long)]
    pub solar_arc_to: Option<String>,

    /// Annual profection (house and lord of the year) at this date
    #[arg(long)]
    pub profection_at: Option<String>,

    /// Solar return of this calendar year
    #[arg(long)]
    pub solar_return: Option<i32>,

    /// First lunar return on or after this date
    #[arg(long)]
    pub lunar_return_after: Option<String>,

    /// Cast the return chart for this city instead of the birthplace (relocated return)
    #[arg(long)]
    pub return_city: Option<String>,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

fn sign(index: usize) -> &'static str {
    SIGN_NAMES[index % 12]
}

fn aspect_lines(t: &mut String, aspects: &[WesternAspectData], limit: usize) -> Result<()> {
    for a in aspects.iter().take(limit) {
        writeln!(
            t,
            "- {} {:?} {} (orb {:.2}°, {:?})",
            a.body_a_name, a.aspect_type, a.body_b_name, a.orb, a.dynamics
        )?;
    }
    if aspects.len() > limit {
        writeln!(
            t,
            "  ... {} more (use --format json)",
            aspects.len() - limit
        )?;
    }
    Ok(())
}

fn relocation(city: &Option<String>) -> Result<Option<(f64, f64)>> {
    match city {
        None => Ok(None),
        Some(name) => eon_core::lookup_city(name)
            .map(|c| Some((c.latitude, c.longitude)))
            .ok_or_else(|| anyhow!("Unknown return city '{name}'")),
    }
}

pub fn execute(cmd: &WesternCommand) -> Result<Rendered> {
    let input = cmd.birth.to_analysis_input()?;
    let west_in = WesternAnalysisInput::new(input.clone(), cmd.house_system.clone());
    let when = |text: &str| eon_service::birth::parse_instant(text, &input.timezone);

    if let Some(target) = &cmd.progress_to {
        let res = facade::analyze_western_progression(west_in, when(target)?)?;

        let mut t = String::new();
        writeln!(t, "== Western: Secondary Progression to {target} ==")?;
        writeln!(
            t,
            "Progressed age: {:.2} years",
            res.result.progressed_age_years
        )?;
        for p in &res.result.progressed_chart.planets {
            writeln!(
                t,
                "- {:<10}: {} {:05.2}° (House {})",
                p.name,
                sign(p.sign_index),
                p.degree_in_sign,
                p.house_number
            )?;
        }
        writeln!(
            t,
            "Progressed-to-natal aspects: {}",
            res.result.active_transit_aspects.len()
        )?;
        aspect_lines(&mut t, &res.result.active_transit_aspects, 10)?;
        return Rendered::new(&res, t);
    }

    if let Some(target) = &cmd.transit_at {
        let res = facade::analyze_western_transit(west_in, when(target)?)?;

        let mut t = String::new();
        writeln!(t, "== Western: Transits at {target} ==")?;
        for p in &res.result.planets {
            writeln!(
                t,
                "- {:<10}: {} {:05.2}°{} -> natal house {}",
                p.name,
                sign(p.sign_index),
                p.degree_in_sign,
                if p.is_retrograde { " R" } else { "" },
                p.natal_house
            )?;
        }
        writeln!(t, "Transit-to-natal aspects: {}", res.result.aspects.len())?;
        aspect_lines(&mut t, &res.result.aspects, 15)?;
        return Rendered::new(&res, t);
    }

    if let Some(target) = &cmd.solar_arc_to {
        let res = facade::analyze_western_solar_arc(west_in, when(target)?)?;

        let mut t = String::new();
        writeln!(t, "== Western: Solar Arc Directions to {target} ==")?;
        writeln!(
            t,
            "Age {:.2} | solar arc {:.2}°",
            res.result.age_years, res.result.arc_degrees
        )?;
        for d in &res.result.directed {
            writeln!(
                t,
                "- {:<10}: {} {:05.2}°",
                d.name,
                sign(d.sign_index),
                d.degree_in_sign
            )?;
        }
        writeln!(t, "Directed-to-natal aspects: {}", res.result.aspects.len())?;
        aspect_lines(&mut t, &res.result.aspects, 12)?;
        return Rendered::new(&res, t);
    }

    if let Some(target) = &cmd.profection_at {
        let res = facade::analyze_western_profection(west_in, when(target)?)?;
        let r = &res.result;

        let mut t = String::new();
        writeln!(t, "== Western: Annual Profection at {target} ==")?;
        writeln!(
            t,
            "Age {} -> profected house {} ({})",
            r.age,
            r.profected_house,
            sign(r.profected_sign_index)
        )?;
        writeln!(
            t,
            "Lord of the year: {} (natal {} in house {})",
            r.year_lord,
            sign(r.year_lord_natal_sign_index),
            r.year_lord_natal_house
        )?;
        if r.planets_in_profected_house.is_empty() {
            writeln!(t, "Natal planets in that house: none")?;
        } else {
            writeln!(
                t,
                "Natal planets in that house: {}",
                r.planets_in_profected_house.join(", ")
            )?;
        }
        return Rendered::new(&res, t);
    }

    let returns = if let Some(year) = cmd.solar_return {
        Some((
            format!("Solar Return {year}"),
            facade::analyze_western_solar_return(
                west_in.clone(),
                year,
                relocation(&cmd.return_city)?,
            )?,
        ))
    } else if let Some(after) = &cmd.lunar_return_after {
        Some((
            format!("Lunar Return after {after}"),
            facade::analyze_western_lunar_return(
                west_in.clone(),
                when(after)?,
                relocation(&cmd.return_city)?,
            )?,
        ))
    } else {
        None
    };
    if let Some((title, res)) = returns {
        let chart = &res.result.return_chart;
        let mut t = String::new();
        writeln!(t, "== Western: {title} ==")?;
        writeln!(
            t,
            "Exact time (UTC): {}",
            res.result.exact_time.to_rfc3339()
        )?;
        writeln!(
            t,
            "Ascendant: {} {:05.2}° | Midheaven: {} {:05.2}°",
            sign((chart.ascendant / 30.0) as usize),
            chart.ascendant % 30.0,
            sign((chart.midheaven / 30.0) as usize),
            chart.midheaven % 30.0
        )?;
        for p in &chart.planets {
            writeln!(
                t,
                "- {:<10}: {} {:05.2}° (House {})",
                p.name,
                sign(p.sign_index),
                p.degree_in_sign,
                p.house_number
            )?;
        }
        writeln!(
            t,
            "Return-to-natal aspects: {}",
            res.result.aspects_to_natal.len()
        )?;
        aspect_lines(&mut t, &res.result.aspects_to_natal, 12)?;
        return Rendered::new(&res, t);
    }

    let res = facade::analyze_western(west_in)?;
    let r = &res.result;

    let mut t = String::new();
    writeln!(t, "== Western Astrology ==")?;
    for p in &r.planets {
        writeln!(
            t,
            "- {:<10}: {} {:05.2}° (House {})",
            p.name,
            sign(p.sign_index),
            p.degree_in_sign,
            p.house_number
        )?;
    }
    writeln!(
        t,
        "Aspects: {} | Aspect Configurations: {}",
        r.aspects.len(),
        r.aspect_patterns.len()
    )?;
    if !r.mutual_receptions.is_empty() {
        let pairs: Vec<String> = r
            .mutual_receptions
            .iter()
            .map(|m| format!("{} <-> {}", m.planet_a, m.planet_b))
            .collect();
        writeln!(t, "Mutual receptions: {}", pairs.join(", "))?;
    }
    if !r.antiscia.is_empty() {
        writeln!(t, "Antiscia contacts: {}", r.antiscia.len())?;
        for c in r.antiscia.iter().take(5) {
            writeln!(
                t,
                "- {} {:?} {} (orb {:.2}°)",
                c.body_a, c.kind, c.body_b, c.orb
            )?;
        }
    }
    if !r.midpoints.is_empty() {
        writeln!(t, "Midpoint contacts: {}", r.midpoints.len())?;
        for m in r.midpoints.iter().take(5) {
            writeln!(
                t,
                "- {} = {}/{} (orb {:.2}°)",
                m.contact_body, m.body_a, m.body_b, m.orb
            )?;
        }
    }

    Rendered::new(&res, t)
}

pub fn run(cmd: WesternCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
