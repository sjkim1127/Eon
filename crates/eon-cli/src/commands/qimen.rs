use anyhow::Result;
use clap::{Args, ValueEnum};
use eon_qimen::analysis::insights::PalaceReading;
use eon_qimen::analysis::question::QimenQuestion;
use eon_qimen::core::PalaceState;
use eon_service::dto::QimenAnalysisInput;
use eon_service::facade;
use std::fmt::Write as _;

use super::{BirthArgs, EngineFormat, Rendered};

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuestionArg {
    General,
    Wealth,
    Career,
    Marriage,
    Health,
    Travel,
    Lawsuit,
    Study,
}

impl From<QuestionArg> for QimenQuestion {
    fn from(value: QuestionArg) -> Self {
        match value {
            QuestionArg::General => QimenQuestion::General,
            QuestionArg::Wealth => QimenQuestion::Wealth,
            QuestionArg::Career => QimenQuestion::Career,
            QuestionArg::Marriage => QimenQuestion::Marriage,
            QuestionArg::Health => QimenQuestion::Health,
            QuestionArg::Travel => QimenQuestion::Travel,
            QuestionArg::Lawsuit => QimenQuestion::Lawsuit,
            QuestionArg::Study => QimenQuestion::Study,
        }
    }
}

impl QuestionArg {
    /// Parses the MCP spelling of a question.
    pub fn parse(text: &str) -> Option<Self> {
        Self::from_str(text, true).ok()
    }
}

#[derive(Args, Debug)]
pub struct QimenCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Cast the plate for this moment instead of the birth moment ('YYYY-MM-DD HH:MM', local time
    /// of the birthplace). The place, timezone and gender still come from the birth options.
    #[arg(long)]
    pub at: Option<String>,

    /// Read the palaces that stand for a question: general, wealth, career, marriage, health,
    /// travel, lawsuit or study
    #[arg(long, value_enum)]
    pub question: Option<QuestionArg>,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

fn verdict_text(v: eon_qimen::analysis::insights::Verdict) -> String {
    format!("{:?}/{}", v, v.korean())
}

fn palace_line(state: &PalaceState, reading: Option<&PalaceReading>) -> String {
    let stem = |s: Option<eon_saju::core::stem::HeavenlyStem>| {
        s.map(|s| s.hanja().to_string())
            .unwrap_or_else(|| "-".into())
    };
    let door = state.door.map(|d| d.hanja()).unwrap_or("--");
    let star = state.star.map(|s| s.hanja()).unwrap_or("--");
    let deity = state.deity.map(|d| d.hanja()).unwrap_or("--");
    let mut flags = Vec::new();
    if let Some(r) = reading {
        if r.hour_void {
            flags.push("時空");
        }
        if r.day_void {
            flags.push("日空");
        }
        if r.horse {
            flags.push("馬星");
        }
        if r.door_oppresses_palace {
            flags.push("門迫");
        }
    }
    format!(
        "{} {}  天{}/地{}  {} {} {}  {}  score {:+} ({})",
        state.palace.as_u8(),
        state.palace.trigram(),
        stem(state.heaven_stem),
        stem(state.earth_stem),
        door,
        star,
        deity,
        flags.join(" "),
        reading.map(|r| r.score).unwrap_or(0),
        reading.map(|r| verdict_text(r.verdict)).unwrap_or_default()
    )
}

pub fn execute(cmd: &QimenCommand) -> Result<Rendered> {
    // A divination chart reuses the birth options for place and timezone but not its date.
    let mut chart_birth = cmd.birth.clone();
    if let Some(at) = &cmd.at {
        chart_birth.birth = at.clone();
        chart_birth.lunar = false;
        chart_birth.leap = false;
    }
    let input = chart_birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male()?;
    let qimen_in = QimenAnalysisInput::new(input, is_male);

    let question = cmd.question.map(QimenQuestion::from);
    let (report, reading, json) = match question {
        Some(q) => {
            let out = facade::analyze_qimen_question(qimen_in, q)?;
            let json = serde_json::to_value(&out)?;
            (out.report, Some(out.reading), json)
        }
        None => {
            let out = facade::analyze_qimen(qimen_in)?;
            let json = serde_json::to_value(&out)?;
            (out.report, None, json)
        }
    };
    let (pan, insights) = (&report.pan, &report.insights);

    let mut t = String::new();
    writeln!(t, "== QiMen DunJia ==")?;
    if let Some(at) = &cmd.at {
        writeln!(t, "Divination chart for {at}")?;
    }
    writeln!(
        t,
        "Ju (Structure): {} Ju {}",
        if pan.is_yin_ju {
            "Yin Dun (陰遁)"
        } else {
            "Yang Dun (陽遁)"
        },
        pan.ju_number
    )?;
    if let Some(chief) = &pan.value_chief_star {
        writeln!(
            t,
            "Chief Star (Zhi Fu): {} {}",
            chief.hanja(),
            chief.english()
        )?;
    }
    if let Some(envoy) = &pan.value_envoy_door {
        writeln!(
            t,
            "Envoy Door (Zhi Shi): {} {}",
            envoy.hanja(),
            envoy.english()
        )?;
    }
    writeln!(t, "Palaces Deployed: {} palaces", pan.palaces.len())?;

    let branches = |v: &[eon_saju::core::branch::EarthlyBranch]| {
        v.iter().map(|b| b.hanja()).collect::<Vec<_>>().join("")
    };
    let palaces =
        |v: &[eon_qimen::core::Palace]| v.iter().map(|p| p.trigram()).collect::<Vec<_>>().join("");
    writeln!(
        t,
        "Void (空亡): hour {} -> {} | day {} -> {}",
        branches(&insights.hour_void.branches),
        palaces(&insights.hour_void.palaces),
        branches(&insights.day_void.branches),
        palaces(&insights.day_void.palaces)
    )?;
    if let (Some(b), Some(p)) = (insights.horse_branch, insights.horse_palace) {
        writeln!(t, "Horse star (馬星): {} -> {}", b.hanja(), p.trigram())?;
    }
    if let (Some(me), Some(event)) = (insights.self_palace, insights.event_palace) {
        writeln!(
            t,
            "Querent (日干宮): {} {} | Matter of the hour (時干宮): {} {}",
            me.as_u8(),
            me.trigram(),
            event.as_u8(),
            event.trigram()
        )?;
    }

    if !insights.patterns.is_empty() {
        writeln!(t, "Formations (格局): {}", insights.patterns.len())?;
        for p in &insights.patterns {
            let place = p
                .palace
                .map(|p| format!("{} {}", p.as_u8(), p.trigram()))
                .unwrap_or_else(|| "whole plate".to_string());
            writeln!(
                t,
                "- {} {} / {} [{:?}] @ {}: {}",
                p.hanja, p.english, p.korean, p.kind, place, p.detail
            )?;
        }
    }

    writeln!(t, "Palaces:")?;
    for state in &pan.palaces {
        let reading = insights.palace(state.palace);
        writeln!(t, "  {}", palace_line(state, reading))?;
    }

    if let Some(r) = &reading {
        writeln!(t, "Question: {:?}", r.question)?;
        if let Some(me) = r.self_palace {
            writeln!(
                t,
                "  Querent palace {} {}: score {:+}",
                me.as_u8(),
                me.trigram(),
                r.self_score
            )?;
        }
        for y in &r.yongshen {
            writeln!(
                t,
                "  {} -> {} {}: score {:+} ({})",
                y.label,
                y.palace.as_u8(),
                y.palace.trigram(),
                y.score,
                verdict_text(y.verdict)
            )?;
        }
        writeln!(
            t,
            "  Overall {:+.1}: {}",
            r.overall_score,
            verdict_text(r.verdict)
        )?;
        writeln!(t, "  {}", r.advice_en)?;
        writeln!(t, "  {}", r.advice_ko)?;
    }
    writeln!(
        t,
        "(Scores are a heuristic over classical tables; see docs/CALCULATION.md section 7a.)"
    )?;

    Ok(Rendered::from_parts(json, t))
}

pub fn run(cmd: QimenCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
