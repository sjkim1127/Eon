use anyhow::Result;
use clap::Args;
use eon_service::dto::ZwdsAnalysisInput;
use eon_service::facade;
use std::fmt::Write as _;

use super::{BirthArgs, EngineFormat, Rendered};

#[derive(Args, Debug)]
pub struct ZwdsCommand {
    #[command(flatten)]
    pub birth: BirthArgs,

    /// Year for the annual (流年) palace, 小限, 斗君 and four transformations (defaults to the current year)
    #[arg(long)]
    pub target_year: Option<i32>,

    /// Output format: json (default), text
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: EngineFormat,
}

pub fn execute(cmd: &ZwdsCommand) -> Result<Rendered> {
    let input = cmd.birth.to_analysis_input()?;
    let is_male = cmd.birth.is_male()?;
    let zwds_in = ZwdsAnalysisInput::new(input, is_male, cmd.target_year);
    let res = facade::analyze_zwds(zwds_in)?;

    let mut t = String::new();
    let soul = &res.chart.palaces[res.chart.soul_idx];
    let body = &res.chart.palaces[res.chart.body_idx];
    writeln!(t, "== Zi Wei Dou Shu (Purple Star Astrology) ==")?;
    writeln!(
        t,
        "Life Palace: {} | Body Palace: {}",
        soul.name.english(),
        body.name.english()
    )?;
    writeln!(t, "Five Elements Bureau: {:?}", res.chart.five_elements)?;
    writeln!(t, "Stars in Life Palace:")?;
    for s in &soul.stars {
        writeln!(t, "- {:?} ({:?})", s.star, s.brightness)?;
    }

    // Current luck periods: 大限 / 流年 / 小限 / 斗君 and the 大限四化.
    let dx = &res.current_daxian;
    writeln!(
        t,
        "Current 大限: ages {}-{} in {}{} ({})",
        dx.age_start,
        dx.age_end,
        dx.stem_hanja,
        dx.branch_hanja,
        res.chart.palace_name_at(dx.palace_idx).english()
    )?;
    if let Some(sh) = &res.daxian_si_hua {
        writeln!(t, "大限四化 (stem {}):", sh.stem_hanja)?;
        for l in &sh.landings {
            writeln!(
                t,
                "- {} {} -> {}",
                l.sihua_type.emoji(),
                l.star.hanja(),
                l.palace_name.english()
            )?;
        }
    }
    let ln = &res.current_liu_nian;
    writeln!(
        t,
        "流年 {}: {}{} in {}",
        ln.year,
        ln.stem_hanja,
        ln.branch_hanja,
        res.chart.palace_name_at(ln.palace_idx).english()
    )?;
    if let Some(x) = &res.minor_limit {
        writeln!(
            t,
            "小限 (age {}): {}{} in {}",
            x.age,
            x.stem_hanja,
            x.branch_hanja,
            x.palace_name.english()
        )?;
    }
    if let Some(d) = &res.dou_jun {
        writeln!(
            t,
            "斗君 ({}): {} in {}",
            d.year,
            d.branch_hanja,
            d.palace_name.english()
        )?;
    }
    if let Some(g) = &res.soul_san_fang_si_zheng {
        writeln!(
            t,
            "命宮 三方四正: {} (opposite), {} + {} (trine)",
            g.opposite.english(),
            g.trine[0].english(),
            g.trine[1].english()
        )?;
    }

    writeln!(t, "Twelve gods (長生 / 博士 / 將前 / 歲前) by palace:")?;
    for p in &res.chart.palaces {
        if let Some(g) = &p.twelve_gods {
            writeln!(
                t,
                "- {}{} {}: {} / {} / {} / {}",
                p.heavenly_stem,
                p.earthly_branch,
                p.name.hanja(),
                g.chang_sheng.hanja(),
                g.bo_shi.hanja(),
                g.jiang_qian.hanja(),
                g.sui_qian.hanja()
            )?;
        }
    }
    if !res.chart.destiny_patterns.is_empty() {
        writeln!(
            t,
            "Key Destiny Formations (Total {}):",
            res.chart.destiny_patterns.len()
        )?;
        for p in &res.chart.destiny_patterns {
            writeln!(t, "- {}: {}", p.name_english, p.description_english)?;
        }
    }

    Rendered::new(&res, t)
}

pub fn run(cmd: ZwdsCommand) -> Result<()> {
    println!("{}", execute(&cmd)?.render(cmd.format)?);
    Ok(())
}
