use anyhow::Result;
use clap::Args;
use eon_service::dto::VedicAnalysisInput;
use eon_service::facade;
use std::fmt::Write as _;

use super::{BirthArgs, EngineFormat, Rendered};

#[derive(Args, Debug)]
pub struct VedicCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Ayanamsa: Lahiri (default), FaganBradley, Raman, Krishnamurti
    #[arg(long)]
    pub ayanamsa: Option<String>,

    /// Year for the solar return (Varshaphala) / Tajika annual chart (defaults to the current year)
    #[arg(long)]
    pub target_year: Option<i32>,

    /// Vedic year type: Gregorian (default), Savana, Sidereal
    #[arg(long)]
    pub year_type: Option<String>,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

pub fn execute(cmd: &VedicCommand) -> Result<Rendered> {
    let input = cmd.birth.to_analysis_input()?;
    let mut vedic_in = VedicAnalysisInput::new(input.clone(), Some(input.unknown_time), None);
    vedic_in.ayanamsa = cmd.ayanamsa.clone();
    vedic_in.target_year = cmd.target_year;
    vedic_in.year_type = cmd.year_type.clone();
    let res = facade::analyze_vedic(vedic_in)?;

    let mut t = String::new();
    writeln!(t, "== Vedic Astrology (Jyotish) ==")?;
    writeln!(t, "Ascendant (Lagna): {}", res.chart.ascendant.describe())?;
    writeln!(t, "Moon Nakshatra Info: {}", res.report.nakshatra_info)?;
    writeln!(t, "Sade Sati: {:?}", res.report.sade_sati)?;
    writeln!(
        t,
        "Overall House Strength: {}",
        res.report.overall_strength_score
    )?;
    if !res.chart.bhava_strengths.is_empty() {
        let line: Vec<String> = res
            .chart
            .bhava_strengths
            .iter()
            .map(|b| format!("H{}={:.1}", b.house, b.total_score))
            .collect();
        writeln!(t, "Bhava Bala: {}", line.join(" "))?;
    }
    let sav = res.chart.sav.points;
    if sav.iter().any(|p| *p > 0) {
        let line: Vec<String> = sav
            .iter()
            .enumerate()
            .map(|(i, p)| format!("H{}={}", i + 1, p))
            .collect();
        writeln!(
            t,
            "Sarvashtakavarga (total {}): {}",
            sav.iter().map(|p| u32::from(*p)).sum::<u32>(),
            line.join(" ")
        )?;
    }
    writeln!(t, "Detected Yogas (Total {}):", res.report.yogas.len())?;
    for y in res.report.yogas.iter().take(5) {
        writeln!(t, "- {} ({:?}): {}", y.name, y.quality, y.description)?;
    }

    Rendered::new(&res, t)
}

pub fn run(cmd: VedicCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
