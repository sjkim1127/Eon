//! 지장간 일수 (월령분금, Won-ryeong-bun-geum) 분석
//!
//! 월지(月支) 내 지장간 중 현재 시점에 어떤 기운이 주도하는지(사령, 司令) 분석합니다.

use crate::core::branch::EarthlyBranch;
use crate::core::calendar::{get_solar_term_time, SolarTerm};
use crate::core::pillars::FourPillars;
use crate::core::pillars::SajuError;
use crate::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

/// 지장간 사령(司令) 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaryeongAnalysis {
    /// 월지
    pub month_branch: EarthlyBranch,
    /// 절기 진입 후 경과 일수
    pub days_passed: u32,
    /// 현재 사령하고 있는 천간
    pub commanding_stem: HeavenlyStem,
    /// 해당 천간의 주도 기간 (일)
    pub duration: u32,
    /// 해당 천간이 지장간 중 어느 부분인지 (여기, 중기, 정기)
    pub part: String,
}

impl SaryeongAnalysis {
    /// 사주 팔자로부터 사령 분석
    pub fn from_pillars(pillars: &FourPillars) -> Result<Self, SajuError> {
        let branch = pillars.month.branch;

        // 해당 월의 시작 절기 찾기
        let term = SolarTerm::from_month_branch_index(branch.index());

        // 해당 절기의 정확한 시작 시각 계산
        let term_time = get_solar_term_time(pillars.birth_time, term)?;

        // 경과 일수 계산 (실제 시간차 기준)
        let diff = pillars.birth_time - term_time;
        let diff_seconds = diff.num_seconds().abs();

        // 1일 = 86400초. 0.5일 이상이면 올림하는 방식이 아닌, 단순히 며칠째인지 계산 (1일차부터 시작)
        let days_passed = (diff_seconds as f64 / 86400.0) as u32 + 1;

        let (stem, duration, part) = get_saryeong_data(branch, days_passed);

        Ok(Self {
            month_branch: branch,
            days_passed,
            commanding_stem: stem,
            duration,
            part,
        })
    }
}

/// 월지와 경과 일수에 따른 사령 천간 산출
fn get_saryeong_data(branch: EarthlyBranch, days: u32) -> (HeavenlyStem, u32, String) {
    use EarthlyBranch as B;
    use HeavenlyStem as S;

    match branch {
        B::Yin => {
            if days <= 7 {
                (S::Wu, 7, "여기(餘氣)".to_string())
            } else if days <= 14 {
                (S::Bing, 7, "중기(中氣)".to_string())
            } else {
                (S::Jia, 16, "정기(正氣)".to_string())
            }
        }
        B::Mao => {
            if days <= 10 {
                (S::Jia, 10, "여기(餘氣)".to_string())
            } else {
                (S::Yi, 20, "정기(正氣)".to_string())
            }
        }
        B::Chen => {
            if days <= 9 {
                (S::Yi, 9, "여기(餘氣)".to_string())
            } else if days <= 12 {
                (S::Gui, 3, "중기(中氣)".to_string())
            } else {
                (S::Wu, 18, "정기(正氣)".to_string())
            }
        }
        B::Si => {
            if days <= 7 {
                (S::Wu, 7, "여기(餘氣)".to_string())
            } else if days <= 14 {
                (S::Geng, 7, "중기(中氣)".to_string())
            } else {
                (S::Bing, 16, "정기(正氣)".to_string())
            }
        }
        B::Wu => {
            if days <= 10 {
                (S::Bing, 10, "여기(餘氣)".to_string())
            } else if days <= 19 {
                (S::Ji, 9, "중기(中氣)".to_string())
            } else {
                (S::Ding, 11, "정기(正氣)".to_string())
            }
        }
        B::Wei => {
            if days <= 9 {
                (S::Ding, 9, "여기(餘氣)".to_string())
            } else if days <= 12 {
                (S::Yi, 3, "중기(中氣)".to_string())
            } else {
                (S::Ji, 18, "정기(正氣)".to_string())
            }
        }
        B::Shen => {
            if days <= 7 {
                (S::Wu, 7, "여기(餘氣)".to_string())
            } else if days <= 14 {
                (S::Ren, 7, "중기(中氣)".to_string())
            } else {
                (S::Geng, 16, "정기(正氣)".to_string())
            }
        }
        B::You => {
            if days <= 10 {
                (S::Geng, 10, "여기(餘氣)".to_string())
            } else {
                (S::Xin, 20, "정기(正氣)".to_string())
            }
        }
        B::Xu => {
            if days <= 9 {
                (S::Xin, 9, "여기(餘氣)".to_string())
            } else if days <= 12 {
                (S::Ding, 3, "중기(中氣)".to_string())
            } else {
                (S::Wu, 18, "정기(正氣)".to_string())
            }
        }
        B::Hai => {
            if days <= 7 {
                (S::Wu, 7, "여기(餘氣)".to_string())
            } else if days <= 14 {
                (S::Jia, 7, "중기(中氣)".to_string())
            } else {
                (S::Ren, 16, "정기(正氣)".to_string())
            }
        }
        B::Zi => {
            if days <= 10 {
                (S::Ren, 10, "여기(餘氣)".to_string())
            } else {
                (S::Gui, 20, "정기(正氣)".to_string())
            }
        }
        B::Chou => {
            if days <= 9 {
                (S::Gui, 9, "여기(餘氣)".to_string())
            } else if days <= 12 {
                (S::Xin, 3, "중기(中氣)".to_string())
            } else {
                (S::Ji, 18, "정기(正氣)".to_string())
            }
        }
    }
}

impl std::fmt::Display for SaryeongAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【월령분금(사령) 분석】")?;
        writeln!(f, "─────────────────────────────────")?;
        writeln!(
            f,
            "월지: {} ({})",
            self.month_branch.hangul(),
            self.month_branch.hanja()
        )?;
        writeln!(f, "경과 일수: {}일", self.days_passed)?;
        writeln!(
            f,
            "▶ 현재 사령: {} ({}) - {}",
            self.commanding_stem.hangul(),
            self.commanding_stem.hanja(),
            self.part
        )?;
        writeln!(f, "  (해당 기운이 이 기간의 주도권을 가짐)")?;
        Ok(())
    }
}

impl FourPillars {
    /// 월령분금(사령) 분석
    pub fn saryeong(&self) -> Result<SaryeongAnalysis, SajuError> {
        SaryeongAnalysis::from_pillars(self)
    }
}
