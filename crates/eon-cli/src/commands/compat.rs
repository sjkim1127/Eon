use anyhow::{anyhow, Result};
use clap::{Args, ValueEnum};
use eon_service::dto::{
    HumanDesignAnalysisInput, NumerologyAnalysisInput, SajuAnalysisInput, VedicCompatibilityInput,
    WesternAnalysisInput,
};
use eon_service::facade;
use serde_json::{json, Map, Value};
use std::fmt::Write as _;

use super::hd::connection_text;
use super::{BirthArgs, EngineFormat, PartnerArgs, Rendered};

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompatSystem {
    Saju,
    Vedic,
    Western,
    Hd,
    /// Life Path / Expression / Soul Urge pairs (heuristic; not part of `all`)
    Numerology,
    All,
}

#[derive(Args, Debug)]
pub struct CompatCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    #[command(flatten)]
    pub partner: PartnerArgs,

    /// Which system to compare with: saju (stems/branches), vedic (Ashtakoota 36 points),
    /// western (synastry + composite), hd (connection chart), numerology (heuristic number
    /// pairs; use --name and --partner-name for Expression/Soul Urge; not part of `all`), or all
    #[arg(long, value_enum, default_value = "all")]
    pub system: CompatSystem,

    /// House system for the Western charts
    #[arg(long, default_value = "Placidus")]
    pub house_system: String,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

struct Part {
    json: Value,
    text: String,
}

fn saju_part(a: &BirthArgs, b: &BirthArgs) -> Result<Part> {
    let (ia, ib) = (a.to_analysis_input()?, b.to_analysis_input()?);
    let res = facade::analyze_saju_compatibility(
        SajuAnalysisInput::new(ia.clone(), a.is_male()?, false, Some(ia.unknown_time)),
        SajuAnalysisInput::new(ib.clone(), b.is_male()?, false, Some(ib.unknown_time)),
    )?;

    let mut t = String::new();
    writeln!(t, "-- Saju (Four Pillars) --")?;
    writeln!(t, "Sync score: {:.1} / 100", res.audit.sync_score)?;
    for (label, items) in [
        ("Synergies", &res.audit.synergies),
        ("Conflicts", &res.audit.conflicts),
        ("Deadlocks", &res.audit.deadlocks),
    ] {
        if !items.is_empty() {
            writeln!(t, "{label}:")?;
            for item in items.iter().take(8) {
                writeln!(t, "- {item}")?;
            }
        }
    }
    Ok(Part {
        json: serde_json::to_value(&res)?,
        text: t,
    })
}

fn vedic_part(a: &BirthArgs, b: &BirthArgs) -> Result<Part> {
    // Ashtakoota is defined as groom (male) vs bride (female). Use the stated genders when they
    // differ; otherwise keep the given order.
    let (male, female, swapped_note) = match (a.is_male()?, b.is_male()?) {
        (false, true) => (b, a, None),
        (true, false) => (a, b, None),
        _ => (
            a,
            b,
            Some("Both people have the same gender: the first is scored as the groom and the second as the bride."),
        ),
    };
    let res = facade::analyze_vedic_compatibility(VedicCompatibilityInput {
        male: male.to_analysis_input()?,
        female: female.to_analysis_input()?,
    })?;

    let mut t = String::new();
    writeln!(t, "-- Vedic (Ashtakoota) --")?;
    if let Some(note) = swapped_note {
        writeln!(t, "Note: {note}")?;
    }
    writeln!(
        t,
        "Total: {:.1} / 36 ({})",
        res.report.total_score,
        if res.report.is_compatible {
            "compatible"
        } else {
            "not recommended"
        }
    )?;
    for k in &res.report.kootas {
        writeln!(
            t,
            "- {:<14} {:>4.1} / {}",
            k.name, k.earned_points, k.max_points
        )?;
    }
    writeln!(
        t,
        "Mangal dosha: male={} female={} cancelled={}",
        res.report.male_mangal_dosha,
        res.report.female_mangal_dosha,
        res.report.mangal_dosha_cancelled
    )?;
    writeln!(t, "{}", res.report.explanation)?;
    Ok(Part {
        json: serde_json::to_value(&res)?,
        text: t,
    })
}

fn western_part(a: &BirthArgs, b: &BirthArgs, house_system: &str) -> Result<Part> {
    let (ia, ib) = (a.to_analysis_input()?, b.to_analysis_input()?);
    let res = facade::analyze_western_synastry(
        WesternAnalysisInput::new(ia, house_system.to_string()),
        WesternAnalysisInput::new(ib, house_system.to_string()),
        "Person A",
        "Person B",
    )?;

    let mut t = String::new();
    writeln!(t, "-- Western (Synastry & Composite) --")?;
    writeln!(t, "Harmony score: {:.1}", res.synastry.harmony_score)?;
    writeln!(t, "{}", res.synastry.summary)?;
    writeln!(
        t,
        "Inter-chart aspects: {}",
        res.synastry.inter_aspects.len()
    )?;
    let mut tightest: Vec<_> = res.synastry.inter_aspects.iter().collect();
    tightest.sort_by(|x, y| x.orb.total_cmp(&y.orb));
    for asp in tightest.into_iter().take(8) {
        writeln!(
            t,
            "- {} {:?} {} (orb {:.2}°)",
            asp.body_a_name, asp.aspect_type, asp.body_b_name, asp.orb
        )?;
    }
    writeln!(t, "Composite: {}", res.composite.summary)?;
    Ok(Part {
        json: serde_json::to_value(&res)?,
        text: t,
    })
}

fn hd_part(a: &BirthArgs, b: &BirthArgs) -> Result<Part> {
    let res = facade::analyze_hd_connection(
        HumanDesignAnalysisInput::new(a.to_analysis_input()?),
        HumanDesignAnalysisInput::new(b.to_analysis_input()?),
    )?;
    let mut t = String::new();
    writeln!(t, "-- Human Design (Connection Chart) --")?;
    write!(t, "{}", connection_text(&res))?;
    Ok(Part {
        json: serde_json::to_value(&res)?,
        text: t,
    })
}

fn numerology_part(a: &BirthArgs, b: &BirthArgs) -> Result<Part> {
    let input = |p: &BirthArgs| -> Result<NumerologyAnalysisInput> {
        Ok(NumerologyAnalysisInput::new(
            p.to_analysis_input()?,
            p.name.clone(),
            None,
        ))
    };
    let res = facade::analyze_numerology_compatibility(input(a)?, input(b)?)?;
    let c = &res.compatibility;

    let mut t = String::new();
    writeln!(t, "-- Numerology (heuristic) --")?;
    writeln!(t, "Total: {:.1} / 100", c.total_score)?;
    let pair = |label: &str, p: &eon_numerology::NumberPairScore| {
        format!(
            "- {label}: {} & {} -> {} ({}) {}\n",
            p.number_a, p.number_b, p.score, p.relation, p.description_en
        )
    };
    t.push_str(&pair("Life Path", &c.life_path));
    match &c.expression {
        Some(p) => t.push_str(&pair("Expression", p)),
        None => writeln!(t, "- Expression: N/A (pass --name and --partner-name)")?,
    }
    match &c.soul_urge {
        Some(p) => t.push_str(&pair("Soul Urge", p)),
        None => writeln!(t, "- Soul Urge: N/A (pass --name and --partner-name)")?,
    }
    writeln!(t, "{}", c.methodology)?;
    Ok(Part {
        json: serde_json::to_value(&res)?,
        text: t,
    })
}

pub fn execute(cmd: &CompatCommand) -> Result<Rendered> {
    let a = &cmd.birth;
    let b = cmd.partner.to_birth_args();

    let run_one = |system: CompatSystem| -> Result<Part> {
        match system {
            CompatSystem::Saju => saju_part(a, &b),
            CompatSystem::Vedic => vedic_part(a, &b),
            CompatSystem::Western => western_part(a, &b, &cmd.house_system),
            CompatSystem::Hd => hd_part(a, &b),
            CompatSystem::Numerology => numerology_part(a, &b),
            CompatSystem::All => Err(anyhow!("unreachable")),
        }
    };

    if cmd.system != CompatSystem::All {
        let part = run_one(cmd.system)?;
        let json = json!({ format!("{:?}", cmd.system).to_lowercase(): part.json });
        return Ok(Rendered::from_parts(json, part.text));
    }

    // `all`: engines that cannot run (e.g. no birth time of day) are reported, not fatal.
    let mut map = Map::new();
    let mut text = String::new();
    let mut skipped = Vec::new();
    for (key, system) in [
        ("saju", CompatSystem::Saju),
        ("vedic", CompatSystem::Vedic),
        ("western", CompatSystem::Western),
        ("hd", CompatSystem::Hd),
    ] {
        match run_one(system) {
            Ok(part) => {
                map.insert(key.to_string(), part.json);
                text.push_str(&part.text);
                text.push('\n');
            }
            Err(e) => {
                writeln!(text, "-- {key} -- skipped: {e}\n")?;
                skipped.push(json!({ "system": key, "reason": e.to_string() }));
            }
        }
    }
    if map.is_empty() {
        return Err(anyhow!(
            "No compatibility system could run: {}",
            skipped
                .iter()
                .filter_map(|s| s["reason"].as_str())
                .collect::<Vec<_>>()
                .join("; ")
        ));
    }
    map.insert("skipped".to_string(), Value::Array(skipped));
    Ok(Rendered::from_parts(Value::Object(map), text))
}

pub fn run(cmd: CompatCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
