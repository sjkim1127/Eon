use anyhow::Result;
use clap::Args;
use eon_service::dto::SajuAnalysisInput;
use eon_service::facade;

use super::{BirthArgs, OutputFormat};

#[derive(Args, Debug)]
pub struct IchingCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: OutputFormat,
}

pub fn run(cmd: IchingCommand) -> Result<()> {
    let input = cmd.birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male()?;
    let saju_in = SajuAnalysisInput::new(input, is_male, false, None);
    let res = facade::analyze_iching(saju_in)?;

    match cmd.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        _ => {
            println!("== I Ching / HeLuo Destiny Hexagrams (河洛理數) ==");
            println!(
                "Pre-natal Hexagram (先天卦): #{}",
                res.result.pre_natal_hexagram
            );
            println!(
                "Post-natal Hexagram (後天卦): #{}",
                res.result.post_natal_hexagram
            );
            println!(
                "Principal Governing Line (元堂爻): Line {}",
                res.result.yuan_dang_yao
            );
            println!("Ti-Yong Relation (體用): {:?}", res.result.ti_yong);
            println!(
                "Lifetime Transformation Cycles: {} periods",
                res.result.lifetime_cycles.len()
            );
            for cycle in res.result.lifetime_cycles.iter().take(5) {
                println!(
                    "- Ages {}~{}: Hexagram #{}",
                    cycle.start_age, cycle.end_age, cycle.hexagram_index
                );
            }
        }
    }

    Ok(())
}
