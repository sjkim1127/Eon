use anyhow::Result;
use clap::Args;
use eon_service::dto::*;
use eon_service::facade;

use super::{BirthArgs, OutputFormat};

#[derive(Args, Debug)]
pub struct TierCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: OutputFormat,
}

pub fn run(cmd: TierCommand) -> Result<()> {
    let input = cmd.birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male()?;

    let saju_in = SajuAnalysisInput::new(input.clone(), is_male, false, Some(input.unknown_time));
    let vedic_in = VedicAnalysisInput::new(input.clone(), Some(input.unknown_time), None);
    let western_in = WesternAnalysisInput::new(input.clone(), "Placidus".to_string());
    let zwds_in = ZwdsAnalysisInput::new(input.clone(), is_male, None);
    let hd_in = HumanDesignAnalysisInput::new(input.clone());
    let qimen_in = QimenAnalysisInput::new(input.clone(), is_male);
    let num_in = NumerologyAnalysisInput::new(input, cmd.birth.name.clone(), None);

    let saju_res = facade::analyze_saju(saju_in)?;
    // Vedic needs the ascendant and Moon, so it is skipped when the birth time is unknown
    let vedic_res = match facade::analyze_vedic(vedic_in) {
        Ok(v) => Some(v),
        Err(eon_service::error::ServiceError::BirthTimeRequired(_)) => None,
        Err(e) => return Err(e.into()),
    };
    let western_res = facade::analyze_western(western_in).ok();
    let zwds_res = facade::analyze_zwds(zwds_in).ok();
    let hd_res = facade::analyze_human_design(hd_in).ok();
    let qimen_res = facade::analyze_qimen(qimen_in).ok();
    let num_res = facade::analyze_numerology(num_in).ok();

    let omni_input = OmniDestinyTierInput {
        saju: saju_res,
        vedic: vedic_res,
        western: western_res,
        zwds: zwds_res,
        human_design: hd_res,
        qimen: qimen_res,
        numerology: num_res,
        transit: None,
    };

    let tier_res = facade::analyze_destiny_tier_omni(omni_input)?;

    match cmd.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&tier_res)?);
        }
        _ => {
            println!("== Destiny Tier 7.0 (8-Engine Omni Synthesis) ==");
            println!(
                "Overall Destiny Tier: {} ({:.1} pts)",
                tier_res.destiny_tier.grade, tier_res.destiny_tier_score
            );
            println!(
                "Birth Chart Potential: {} ({:.1} pts)",
                tier_res.potential_tier.grade, tier_res.potential_score
            );
            println!(
                "Growth Gap: {:+.1} pts | Profile: {}",
                tier_res.growth_gap, tier_res.profile
            );
            println!(
                "8D Quantum Synergies (Total {}):",
                tier_res.quantum_synergies.len()
            );
            for s in &tier_res.quantum_synergies {
                println!("- [{}] {:+.0}pt: {}", s.title, s.score_delta, s.description);
            }
        }
    }

    Ok(())
}
