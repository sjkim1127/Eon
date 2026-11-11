use anyhow::Result;
use clap::Args;
use eon_service::dto::SajuAnalysisInput;
use eon_service::facade;

use super::{BirthArgs, OutputFormat};

#[derive(Args, Debug)]
pub struct SajuCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Night rat hour option (야자시 사용 여부)
    #[arg(long)]
    pub night_rat: bool,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: OutputFormat,
}

pub fn run(cmd: SajuCommand) -> Result<()> {
    let input = cmd.birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male();
    let saju_in = SajuAnalysisInput::new(input, is_male, cmd.night_rat, Some(false));

    let res = facade::analyze_saju(saju_in)?;

    match cmd.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        _ => {
            let p = &res.report.pillars;
            println!("== 사주 명리학 원국 ==");
            println!(
                "년주: {}({}{}) | 월주: {}({}{}) | 일주: {}({}{}) | 시주: {}({}{})",
                p.year,
                p.year.stem.hangul(),
                p.year.branch.hangul(),
                p.month,
                p.month.stem.hangul(),
                p.month.branch.hangul(),
                p.day,
                p.day.stem.hangul(),
                p.day.branch.hangul(),
                p.hour,
                p.hour.stem.hangul(),
                p.hour.branch.hangul()
            );
            println!(
                "신강약: {:?} (신강 지수: {:.1})",
                res.report.strength.strength_type, res.report.strength.strength_score
            );
            println!("격국: {:?}", res.report.structure.structure);
            println!("용신(주용신): {:?}", res.report.yongshin.primary);
            println!("오행 유통성: {:.1}%", res.qi_topology.throughput * 100.0);
            if let Some(gt) = &res.report.golden_time {
                println!(
                    "골든타임: {}~{}세 (평균 {:.1}점)",
                    gt.start_age, gt.end_age, gt.average_score
                );
            }
        }
    }

    Ok(())
}
