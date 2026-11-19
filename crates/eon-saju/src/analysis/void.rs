//! Void / Emptiness (空亡) Analysis
//!
//! Analyzes the two remaining earthly branches when pairing 10 stems with 12 branches
//! using the Day Pillar as reference.

use crate::core::branch::EarthlyBranch;
use crate::core::ganzi::GanZi;
use crate::core::pillars::FourPillars;
use crate::core::ten_gods::TenGod;
use serde::{Deserialize, Serialize};

/// Void Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoidAnalysis {
    /// The two void branches for this chart
    pub void_branches: [EarthlyBranch; 2],
    /// Positions where void occurs (Year, Month, Hour)
    pub void_positions: Vec<String>,
    /// Ten Gods associated with void branches
    pub void_ten_gods: Vec<TenGod>,
    /// 10-day Xun group name (e.g. JiaZi Xun)
    pub xun_group: String,
    /// Detailed explainable DTOs
    pub mapped_voids: Vec<VoidDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoidDetail {
    pub branch: EarthlyBranch,
    pub position: String,
    pub ten_god: crate::core::ten_gods::TenGod,
    pub summary: String,
    pub description: String,
    pub reasons: Vec<String>,
    pub level: crate::analysis::supplementary_pillars::InterpretationLevel,
    #[serde(default)]
    pub is_dissolved: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dissolution_reason: Option<String>,
}

impl VoidAnalysis {
    /// Analyze void branches from Four Pillars
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        let day_pillar = pillars.day;
        let (void_branches, xun_group) = calculate_void_branches(day_pillar);

        let mut void_positions = Vec::new();
        let mut void_ten_gods = Vec::new();
        let mut mapped_voids = Vec::new();

        let mut checks = vec![("Year", pillars.year), ("Month", pillars.month)];
        if let Some(hour) = pillars.hour {
            checks.push(("Hour", hour));
        }

        let dm = pillars.day_master();

        for (pos, ganzi) in checks {
            if void_branches.contains(&ganzi.branch) {
                let tg = TenGod::from_stem_and_branch(dm, ganzi.branch);
                void_positions.push(pos.to_string());
                void_ten_gods.push(tg);

                let dissolution = check_void_dissolution(ganzi.branch, pos, pillars);
                let (is_dissolved, dissolution_reason) = match dissolution {
                    Some((diss, msg)) => (diss, Some(msg)),
                    None => (false, None),
                };

                let mut reasons = vec![
                    format!("Day Pillar Void: {}", ganzi.branch.hangul()),
                    format!("{} position match", pos),
                ];

                let (summary, level, desc_extra) = if is_dissolved {
                    let reason_str = dissolution_reason.clone().unwrap_or_default();
                    reasons.push(reason_str.clone());
                    (
                        format!(
                            "{} {} Void (Dissolved via Clash/Combination)",
                            pos,
                            tg.hangul()
                        ),
                        crate::analysis::supplementary_pillars::InterpretationLevel::Neutral,
                        format!(" (Rescued/resolved by {}.)", reason_str),
                    )
                } else {
                    (
                        format!("{} {} Void", pos, tg.hangul()),
                        crate::analysis::supplementary_pillars::InterpretationLevel::Caution,
                        "".to_string(),
                    )
                };

                let description = match pos {
                    "Year" => "Indicates weaker ancestral support or an unsettled early foundation in childhood.",
                    "Month" => "Indicates career foundation requires extra diligence or independent effort away from family.",
                    "Hour" => "Indicates late-life achievements or progeny fruits manifest later than expected.",
                    _ => "",
                };

                mapped_voids.push(VoidDetail {
                    branch: ganzi.branch,
                    position: pos.to_string(),
                    ten_god: tg,
                    summary,
                    description: format!("{}{}", description, desc_extra),
                    reasons,
                    level,
                    is_dissolved,
                    dissolution_reason,
                });
            }
        }

        Self {
            void_branches,
            void_positions,
            void_ten_gods,
            xun_group,
            mapped_voids,
        }
    }
}

/// Determine whether void branch is dissolved via clash or combination
fn check_void_dissolution(
    target_branch: EarthlyBranch,
    target_pos: &str,
    pillars: &FourPillars,
) -> Option<(bool, String)> {
    let mut other_branches = vec![
        ("Year", pillars.year.branch),
        ("Month", pillars.month.branch),
        ("Day", pillars.day.branch),
    ];
    if let Some(hour) = pillars.hour {
        other_branches.push(("Hour", hour.branch));
    }
    let all_branches: Vec<EarthlyBranch> = other_branches.iter().map(|(_, b)| *b).collect();

    for (pos, b) in other_branches {
        if pos == target_pos {
            continue;
        }
        if crate::analysis::relationships::BranchClash::check(target_branch, b).is_some() {
            return Some((
                true,
                format!("Resolved by Clash (沖) with {} {}", pos, b.hangul()),
            ));
        }
        if crate::analysis::relationships::SixCombination::check(target_branch, b).is_some() {
            return Some((
                true,
                format!(
                    "Resolved by Six Combination (六合) with {} {}",
                    pos,
                    b.hangul()
                ),
            ));
        }
    }

    let triples = crate::analysis::relationships::TripleCombination::check(&all_branches);
    if triples
        .iter()
        .any(|c| c.branches().contains(&target_branch))
    {
        return Some((
            true,
            "Resolved by chart Triple Combination (三合) formation".to_string(),
        ));
    }
    let seasonals = crate::analysis::relationships::SeasonalCombination::check(&all_branches);
    if seasonals
        .iter()
        .any(|c| c.branches().contains(&target_branch))
    {
        return Some((
            true,
            "Resolved by chart Seasonal Combination (方合) formation".to_string(),
        ));
    }

    None
}

/// Calculate void branches and Xun group for given GanZi
pub fn calculate_void_branches(ganzi: GanZi) -> ([EarthlyBranch; 2], String) {
    let s_idx = ganzi.stem.index() as i32;
    let b_idx = ganzi.branch.index() as i32;

    let xun_start_idx = (b_idx - s_idx).rem_euclid(12);

    let xun_name = match xun_start_idx {
        0 => "JiaZi Xun (甲子旬)",
        10 => "JiaXu Xun (甲戌旬)",
        8 => "JiaShen Xun (甲申旬)",
        6 => "JiaWu Xun (甲午旬)",
        4 => "JiaChen Xun (甲辰旬)",
        2 => "JiaYin Xun (甲寅旬)",
        _ => "Other",
    };

    let v1_idx = (xun_start_idx - 2).rem_euclid(12);
    let v2_idx = (xun_start_idx - 1).rem_euclid(12);

    let v1 = EarthlyBranch::from_index(v1_idx);
    let v2 = EarthlyBranch::from_index(v2_idx);

    ([v1, v2], xun_name.to_string())
}

impl std::fmt::Display for VoidAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【Void (空亡) Analysis】")?;
        writeln!(f, "─────────────────────────────────")?;
        writeln!(f, "Day Pillar Basis: {}", self.xun_group)?;
        writeln!(
            f,
            "Void Branches: {} ({}), {} ({})",
            self.void_branches[0].hangul(),
            self.void_branches[0].hanja(),
            self.void_branches[1].hangul(),
            self.void_branches[1].hanja()
        )?;

        if self.void_positions.is_empty() {
            writeln!(f, "▶ No void branches found in natal chart.")?;
        } else {
            for (idx, pos) in self.void_positions.iter().enumerate() {
                writeln!(
                    f,
                    "▶ Void manifested in {} (Ten God: {})",
                    pos,
                    self.void_ten_gods[idx].hangul()
                )?;
            }
            writeln!(
                f,
                "  * Effectiveness of this social/relational energy may be attenuated."
            )?;
        }
        Ok(())
    }
}

impl FourPillars {
    /// Void Analysis
    pub fn void_analysis(&self) -> VoidAnalysis {
        VoidAnalysis::from_pillars(self)
    }
}
