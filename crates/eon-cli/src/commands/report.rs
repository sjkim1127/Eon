use anyhow::Result;
use clap::{Args, ValueEnum};
use eon_service::dto::{ReportTheme, SajuAnalysisInput, ThemedReportInput};
use eon_service::facade;

use super::{BirthArgs, EngineFormat, Rendered};

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeArg {
    /// Wealth and career
    Wealth,
    /// Love and marriage
    Love,
    /// Health and vitality
    Health,
}

impl From<ThemeArg> for ReportTheme {
    fn from(value: ThemeArg) -> Self {
        match value {
            ThemeArg::Wealth => ReportTheme::WealthAndCareer,
            ThemeArg::Love => ReportTheme::LoveAndMarriage,
            ThemeArg::Health => ReportTheme::HealthAndVitality,
        }
    }
}

#[derive(Args, Debug)]
pub struct ReportCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Report theme
    #[arg(long, value_enum)]
    pub theme: ThemeArg,

    /// Output format: json (default), text (the report as markdown)
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

pub fn execute(cmd: &ReportCommand) -> Result<Rendered> {
    let input = cmd.birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male()?;
    let base = SajuAnalysisInput::new(input.clone(), is_male, false, Some(input.unknown_time));
    let res = facade::generate_themed_report(ThemedReportInput {
        base,
        theme: cmd.theme.into(),
        user_name: cmd.birth.name.clone().unwrap_or_else(|| "You".to_string()),
    })?;

    let text = format!("# {}\n\n{}", res.title, res.content);
    Rendered::new(&res, text)
}

pub fn run(cmd: ReportCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
