use anyhow::Result;
use clap::Args;
use eon_service::dto::VedicAnalysisInput;
use eon_service::facade;

use super::{BirthArgs, OutputFormat};

#[derive(Args, Debug)]
pub struct VedicCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: OutputFormat,
}

pub fn run(cmd: VedicCommand) -> Result<()> {
    let input = cmd.birth.to_analysis_input()?;
    let vedic_in = VedicAnalysisInput::new(input, Some(false), None);
    let res = facade::analyze_vedic(vedic_in)?;

    match cmd.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        _ => {
            println!("== 베딕 점성학 (Jyotish) ==");
            println!("상승궁 (Ascendant): {:?}", res.chart.ascendant);
            println!("달 Nakshatra 정보: {}", res.report.nakshatra_info);
            println!("사데 사티 (Sade Sati): {:?}", res.report.sade_sati);
            println!("하우스 종합 강도: {}", res.report.overall_strength_score);
            println!("감지된 요가 (총 {}개):", res.report.yogas.len());
            for y in res.report.yogas.iter().take(5) {
                println!("- {} ({:?}): {}", y.name, y.quality, y.description);
            }
        }
    }

    Ok(())
}
