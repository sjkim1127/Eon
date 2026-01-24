//! 지장간 일수 (월령분금, Won-ryeong-bun-geum) 분석
//! 
//! 월지(月支) 내 지장간 중 현재 시점에 어떤 기운이 주도하는지(사령, 司令) 분석합니다.

use serde::{Deserialize, Serialize};
use crate::branch::EarthlyBranch;
use crate::stem::HeavenlyStem;
use crate::pillars::FourPillars;
use crate::calendar::{SolarTerm, approximate_solar_term_day};

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
    /// 사주 팔자와 출생 정보로부터 사령 분석
    pub fn from_pillars(pillars: &FourPillars, birth_day: u32) -> Self {
        let branch = pillars.month.branch;
        
        // 해당 월의 절기 찾기
        let term = match branch {
            EarthlyBranch::Yin => SolarTerm::Lichun,
            EarthlyBranch::Mao => SolarTerm::Jingzhe,
            EarthlyBranch::Chen => SolarTerm::Qingming,
            EarthlyBranch::Si => SolarTerm::Lixia,
            EarthlyBranch::Wu => SolarTerm::Mangzhong,
            EarthlyBranch::Wei => SolarTerm::Xiaoshu,
            EarthlyBranch::Shen => SolarTerm::Liqiu,
            EarthlyBranch::You => SolarTerm::Bailu,
            EarthlyBranch::Xu => SolarTerm::Hanlu,
            EarthlyBranch::Hai => SolarTerm::Lidong,
            EarthlyBranch::Zi => SolarTerm::Daxue,
            EarthlyBranch::Chou => SolarTerm::Xiaohan,
        };

        let (_, term_day) = approximate_solar_term_day(0, term); // 연도는 현재 무시 (근사치)
        
        // 경과 일수 계산 (간단화: 30일 주기 기준)
        let days_passed = if birth_day >= term_day {
            birth_day - term_day + 1
        } else {
            // 이전 달에서 넘어온 경우 (보통은 pillars 계산 시 이미 조정되지만 안전책)
            birth_day + 30 - term_day + 1
        };

        let (stem, duration, part) = get_saryeong_data(branch, days_passed);

        Self {
            month_branch: branch,
            days_passed,
            commanding_stem: stem,
            duration,
            part,
        }
    }
}

/// 월지와 경과 일수에 따른 사령 천간 산출
fn get_saryeong_data(branch: EarthlyBranch, days: u32) -> (HeavenlyStem, u32, String) {
    use EarthlyBranch as B;
    use HeavenlyStem as S;

    match branch {
        B::Yin => {
            if days <= 7 { (S::Wu, 7, "여기(餘氣)".to_string()) }
            else if days <= 14 { (S::Bing, 7, "중기(中氣)".to_string()) }
            else { (S::Jia, 16, "정기(正氣)".to_string()) }
        },
        B::Mao => {
            if days <= 10 { (S::Jia, 10, "여기(餘氣)".to_string()) }
            else { (S::Yi, 20, "정기(正氣)".to_string()) }
        },
        B::Chen => {
            if days <= 9 { (S::Yi, 9, "여기(餘氣)".to_string()) }
            else if days <= 12 { (S::Gui, 3, "중기(中氣)".to_string()) }
            else { (S::Wu, 18, "정기(正氣)".to_string()) }
        },
        B::Si => {
            if days <= 7 { (S::Wu, 7, "여기(餘氣)".to_string()) }
            else if days <= 14 { (S::Geng, 7, "중기(中氣)".to_string()) }
            else { (S::Bing, 16, "정기(正氣)".to_string()) }
        },
        B::Wu => {
            if days <= 10 { (S::Bing, 10, "여기(餘氣)".to_string()) }
            else if days <= 19 { (S::Ji, 9, "중기(中氣)".to_string()) }
            else { (S::Ding, 11, "정기(正氣)".to_string()) }
        },
        B::Wei => {
            if days <= 9 { (S::Ding, 9, "여기(餘氣)".to_string()) }
            else if days <= 12 { (S::Yi, 3, "중기(中氣)".to_string()) }
            else { (S::Ji, 18, "정기(正氣)".to_string()) }
        },
        B::Shen => {
            if days <= 7 { (S::Wu, 7, "여기(餘氣)".to_string()) }
            else if days <= 14 { (S::Ren, 7, "중기(中氣)".to_string()) }
            else { (S::Geng, 16, "정기(正氣)".to_string()) }
        },
        B::You => {
            if days <= 10 { (S::Geng, 10, "여기(餘氣)".to_string()) }
            else { (S::Xin, 20, "정기(正氣)".to_string()) }
        },
        B::Xu => {
            if days <= 9 { (S::Xin, 9, "여기(餘氣)".to_string()) }
            else if days <= 12 { (S::Ding, 3, "중기(中氣)".to_string()) }
            else { (S::Wu, 18, "정기(正氣)".to_string()) }
        },
        B::Hai => {
            if days <= 7 { (S::Wu, 7, "여기(餘氣)".to_string()) }
            else if days <= 14 { (S::Jia, 7, "중기(中氣)".to_string()) }
            else { (S::Ren, 16, "정기(正氣)".to_string()) }
        },
        B::Zi => {
            if days <= 10 { (S::Ren, 10, "여기(餘氣)".to_string()) }
            else { (S::Gui, 20, "정기(正氣)".to_string()) }
        },
        B::Chou => {
            if days <= 9 { (S::Gui, 9, "여기(餘氣)".to_string()) }
            else if days <= 12 { (S::Xin, 3, "중기(中氣)".to_string()) }
            else { (S::Ji, 18, "정기(正氣)".to_string()) }
        },
    }
}

impl std::fmt::Display for SaryeongAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【월령분금(사령) 분석】")?;
        writeln!(f, "─────────────────────────────────")?;
        writeln!(f, "월지: {} ({})", self.month_branch.hangul(), self.month_branch.hanja())?;
        writeln!(f, "경과 일수: {}일", self.days_passed)?;
        writeln!(f, "▶ 현재 사령: {} ({}) - {}", 
            self.commanding_stem.hangul(), self.commanding_stem.hanja(), self.part)?;
        writeln!(f, "  (해당 기운이 이 기간의 주도권을 가짐)")?;
        Ok(())
    }
}

impl FourPillars {
    /// 월령분금(사령) 분석
    pub fn saryeong(&self, birth_day: u32) -> SaryeongAnalysis {
        SaryeongAnalysis::from_pillars(self, birth_day)
    }
}
