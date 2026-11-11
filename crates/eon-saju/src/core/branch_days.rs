//! Hidden Stem Command Days (月令分金, Saryeong) Analysis
//!
//! Analyzes which hidden stem of the month branch currently commands (司令) energy.

use crate::core::branch::EarthlyBranch;
use crate::core::calendar::{get_solar_term_time, SolarTerm};
use crate::core::pillars::{FourPillars, SajuError};
use crate::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

/// Hidden Stem Command Information (司令)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaryeongAnalysis {
    /// Month Branch
    pub month_branch: EarthlyBranch,
    /// Days elapsed since solar term entry
    pub days_passed: u32,
    /// Currently commanding Heavenly Stem
    pub commanding_stem: HeavenlyStem,
    /// Duration of commanding period (days)
    pub duration_days: u32,
    /// Part of hidden stems (Remnant, Middle, Main)
    pub part: String,
}

impl SaryeongAnalysis {
    /// Analyze commanding stem from Four Pillars
    pub fn from_pillars(pillars: &FourPillars) -> Result<Self, SajuError> {
        let month_branch = pillars.month.branch;
        let start_term = SolarTerm::from_month_branch_index(month_branch.index());
        let term_time = get_solar_term_time(pillars.birth_time, start_term)?;

        let duration = pillars.birth_time.signed_duration_since(term_time);
        let seconds = duration.num_seconds();
        let days_passed = if seconds < 0 {
            1
        } else {
            (seconds / 86400) as u32 + 1
        };

        let (stem, duration_days, part) = calculate_saryeong(month_branch, days_passed);

        Ok(Self {
            month_branch,
            days_passed,
            commanding_stem: stem,
            duration_days,
            part,
        })
    }
}

/// Calculate commanding stem based on month branch and elapsed days
pub fn calculate_saryeong(branch: EarthlyBranch, days: u32) -> (HeavenlyStem, u32, String) {
    use EarthlyBranch as B;
    use HeavenlyStem as S;

    match branch {
        B::Yin => {
            if days <= 7 {
                (S::Wu, 7, "Remnant (餘氣)".to_string())
            } else if days <= 14 {
                (S::Bing, 7, "Middle (中氣)".to_string())
            } else {
                (S::Jia, 16, "Main (正氣)".to_string())
            }
        }
        B::Mao => {
            if days <= 10 {
                (S::Jia, 10, "Remnant (餘氣)".to_string())
            } else {
                (S::Yi, 20, "Main (正氣)".to_string())
            }
        }
        B::Chen => {
            if days <= 9 {
                (S::Yi, 9, "Remnant (餘氣)".to_string())
            } else if days <= 12 {
                (S::Gui, 3, "Middle (中氣)".to_string())
            } else {
                (S::Wu, 18, "Main (正氣)".to_string())
            }
        }
        B::Si => {
            if days <= 7 {
                (S::Wu, 7, "Remnant (餘氣)".to_string())
            } else if days <= 14 {
                (S::Geng, 7, "Middle (中氣)".to_string())
            } else {
                (S::Bing, 16, "Main (正氣)".to_string())
            }
        }
        B::Wu => {
            if days <= 10 {
                (S::Bing, 10, "Remnant (餘氣)".to_string())
            } else if days <= 19 {
                (S::Ji, 9, "Middle (中氣)".to_string())
            } else {
                (S::Ding, 11, "Main (正氣)".to_string())
            }
        }
        B::Wei => {
            if days <= 9 {
                (S::Ding, 9, "Remnant (餘氣)".to_string())
            } else if days <= 12 {
                (S::Yi, 3, "Middle (中氣)".to_string())
            } else {
                (S::Ji, 18, "Main (正氣)".to_string())
            }
        }
        B::Shen => {
            if days <= 7 {
                (S::Wu, 7, "Remnant (餘氣)".to_string())
            } else if days <= 14 {
                (S::Ren, 7, "Middle (中氣)".to_string())
            } else {
                (S::Geng, 16, "Main (正氣)".to_string())
            }
        }
        B::You => {
            if days <= 10 {
                (S::Geng, 10, "Remnant (餘氣)".to_string())
            } else {
                (S::Xin, 20, "Main (正氣)".to_string())
            }
        }
        B::Xu => {
            if days <= 9 {
                (S::Xin, 9, "Remnant (餘氣)".to_string())
            } else if days <= 12 {
                (S::Ding, 3, "Middle (中氣)".to_string())
            } else {
                (S::Wu, 18, "Main (正氣)".to_string())
            }
        }
        B::Hai => {
            if days <= 7 {
                (S::Wu, 7, "Remnant (餘氣)".to_string())
            } else if days <= 14 {
                (S::Jia, 7, "Middle (中氣)".to_string())
            } else {
                (S::Ren, 16, "Main (正氣)".to_string())
            }
        }
        B::Zi => {
            if days <= 10 {
                (S::Ren, 10, "Remnant (餘氣)".to_string())
            } else {
                (S::Gui, 20, "Main (正氣)".to_string())
            }
        }
        B::Chou => {
            if days <= 9 {
                (S::Gui, 9, "Remnant (餘氣)".to_string())
            } else if days <= 12 {
                (S::Xin, 3, "Middle (中氣)".to_string())
            } else {
                (S::Ji, 18, "Main (正氣)".to_string())
            }
        }
    }
}

impl std::fmt::Display for SaryeongAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【Commanding Hidden Stem (司令) Analysis】")?;
        writeln!(f, "─────────────────────────────────")?;
        writeln!(
            f,
            "Month Branch: {} ({})",
            self.month_branch.hangul(),
            self.month_branch.hanja()
        )?;
        writeln!(f, "Days Elapsed: {} days", self.days_passed)?;
        writeln!(
            f,
            "▶ Current Command: {} ({}) - {}",
            self.commanding_stem.hangul(),
            self.commanding_stem.hanja(),
            self.part
        )?;
        writeln!(
            f,
            "  (This energy commands seasonal authority in this window)"
        )?;
        Ok(())
    }
}

impl FourPillars {
    /// Command (Saryeong) Analysis
    pub fn saryeong(&self) -> Result<SaryeongAnalysis, SajuError> {
        SaryeongAnalysis::from_pillars(self)
    }
}
