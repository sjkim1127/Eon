//! 십성(十神, Ten Gods) 분석
//!
//! 일간(日干)을 기준으로 다른 천간/지지와의 관계를 분석합니다.
//!
//! ## 십성 관계표
//!
//! | 관계 | 같은 음양 | 다른 음양 |
//! |-----|----------|----------|
//! | 같은 오행 | 비견 | 겁재 |
//! | 내가 생 | 식신 | 상관 |
//! | 내가 극 | 편재 | 정재 |
//! | 나를 극 | 편관 | 정관 |
//! | 나를 생 | 편인 | 정인 |

use serde::{Deserialize, Serialize};
use crate::stem::HeavenlyStem;
use crate::branch::EarthlyBranch;
use crate::element::{Element, ElementRelation, Polarity};
use crate::ganzi::GanZi;
use crate::pillars::FourPillars;

/// 십성(十神)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TenGod {
    /// 비견(比肩) - 같은 오행, 같은 음양
    /// 동료, 경쟁자, 형제자매
    Bijian,
    /// 겁재(劫財) - 같은 오행, 다른 음양
    /// 형제, 갈등, 손재
    Jiecai,
    /// 식신(食神) - 내가 생하는 오행, 같은 음양
    /// 재능, 표현, 식복
    Shishen,
    /// 상관(傷官) - 내가 생하는 오행, 다른 음양
    /// 반항, 창의, 예술
    Shangguan,
    /// 편재(偏財) - 내가 극하는 오행, 같은 음양
    /// 투기, 부친, 횡재
    Piancai,
    /// 정재(正財) - 내가 극하는 오행, 다른 음양
    /// 재물, 아내, 정당한 수입
    Zhengcai,
    /// 편관(偏官) - 나를 극하는 오행, 같은 음양
    /// 권위, 압박, 칠살
    Pianguan,
    /// 정관(正官) - 나를 극하는 오행, 다른 음양
    /// 직장, 남편, 명예
    Zhengguan,
    /// 편인(偏印) - 나를 생하는 오행, 같은 음양
    /// 편모, 특기, 효신
    Pianyin,
    /// 정인(正印) - 나를 생하는 오행, 다른 음양
    /// 모친, 학문, 인덕
    Zhengyin,
}

impl TenGod {
    /// 모든 십성 배열
    pub const ALL: [TenGod; 10] = [
        Self::Bijian, Self::Jiecai,
        Self::Shishen, Self::Shangguan,
        Self::Piancai, Self::Zhengcai,
        Self::Pianguan, Self::Zhengguan,
        Self::Pianyin, Self::Zhengyin,
    ];

    /// 한자 표기
    pub const HANJA: [&'static str; 10] = [
        "比肩", "劫財", "食神", "傷官", "偏財",
        "正財", "偏官", "正官", "偏印", "正印",
    ];

    /// 한글 표기
    pub const HANGUL: [&'static str; 10] = [
        "비견", "겁재", "식신", "상관", "편재",
        "정재", "편관", "정관", "편인", "정인",
    ];

    /// 약어 (2글자)
    pub const ABBREV: [&'static str; 10] = [
        "비", "겁", "식", "상", "편재",
        "정재", "편관", "정관", "편인", "정인",
    ];

    /// 일간(日干)과 다른 천간의 관계로 십성 계산
    /// 
    /// # Arguments
    /// * `day_master` - 일간 (기준)
    /// * `other` - 비교할 천간
    pub fn from_stems(day_master: HeavenlyStem, other: HeavenlyStem) -> Self {
        let same_polarity = day_master.polarity() == other.polarity();
        let relation = day_master.element().relation_to(other.element());

        match (relation, same_polarity) {
            (ElementRelation::Same, true) => Self::Bijian,
            (ElementRelation::Same, false) => Self::Jiecai,
            (ElementRelation::Generates, true) => Self::Shishen,
            (ElementRelation::Generates, false) => Self::Shangguan,
            (ElementRelation::Controls, true) => Self::Piancai,
            (ElementRelation::Controls, false) => Self::Zhengcai,
            (ElementRelation::ControlledBy, true) => Self::Pianguan,
            (ElementRelation::ControlledBy, false) => Self::Zhengguan,
            (ElementRelation::GeneratedBy, true) => Self::Pianyin,
            (ElementRelation::GeneratedBy, false) => Self::Zhengyin,
        }
    }

    /// 일간(日干)과 지지의 정기(正氣)로 십성 계산
    /// 
    /// 지지는 장간(藏干)을 가지고 있지만, 간단히 정기만 사용합니다.
    pub fn from_stem_and_branch(day_master: HeavenlyStem, branch: EarthlyBranch) -> Self {
        // 지지의 정기 천간 구하기
        let branch_stem = branch.primary_stem();
        Self::from_stems(day_master, branch_stem)
    }

    /// 인덱스 (0-9)
    pub const fn index(self) -> usize {
        match self {
            Self::Bijian => 0,
            Self::Jiecai => 1,
            Self::Shishen => 2,
            Self::Shangguan => 3,
            Self::Piancai => 4,
            Self::Zhengcai => 5,
            Self::Pianguan => 6,
            Self::Zhengguan => 7,
            Self::Pianyin => 8,
            Self::Zhengyin => 9,
        }
    }

    /// 한자 표기
    pub const fn hanja(self) -> &'static str {
        Self::HANJA[self.index()]
    }

    /// 한글 표기
    pub const fn hangul(self) -> &'static str {
        Self::HANGUL[self.index()]
    }

    /// 길신/흉신 여부 (일반적 해석, 상황에 따라 달라질 수 있음)
    pub const fn is_auspicious(self) -> bool {
        matches!(self, 
            Self::Shishen | Self::Zhengcai | Self::Zhengguan | Self::Zhengyin
        )
    }
}

impl std::fmt::Display for TenGod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hangul())
    }
}

// ============================================
// 지지의 정기(正氣) - 지지에 숨겨진 천간
// ============================================

impl EarthlyBranch {
    /// 지지의 정기(正氣) 천간 반환
    /// 
    /// 각 지지에는 1~3개의 장간(藏干)이 있으며,
    /// 여기서는 가장 강한 정기만 반환합니다.
    pub const fn primary_stem(self) -> HeavenlyStem {
        match self {
            Self::Zi => HeavenlyStem::Gui,   // 子 → 癸 (계수)
            Self::Chou => HeavenlyStem::Ji,  // 丑 → 己 (기토)
            Self::Yin => HeavenlyStem::Jia,  // 寅 → 甲 (갑목)
            Self::Mao => HeavenlyStem::Yi,   // 卯 → 乙 (을목)
            Self::Chen => HeavenlyStem::Wu,  // 辰 → 戊 (무토)
            Self::Si => HeavenlyStem::Bing,  // 巳 → 丙 (병화)
            Self::Wu => HeavenlyStem::Ding,  // 午 → 丁 (정화)
            Self::Wei => HeavenlyStem::Ji,   // 未 → 己 (기토)
            Self::Shen => HeavenlyStem::Geng,// 申 → 庚 (경금)
            Self::You => HeavenlyStem::Xin,  // 酉 → 辛 (신금)
            Self::Xu => HeavenlyStem::Wu,    // 戌 → 戊 (무토)
            Self::Hai => HeavenlyStem::Ren,  // 亥 → 壬 (임수)
        }
    }

    /// 지지의 모든 장간(藏干) 반환 (정기, 중기, 여기 순)
    pub fn hidden_stems(self) -> Vec<HeavenlyStem> {
        match self {
            Self::Zi => vec![HeavenlyStem::Gui],
            Self::Chou => vec![HeavenlyStem::Ji, HeavenlyStem::Gui, HeavenlyStem::Xin],
            Self::Yin => vec![HeavenlyStem::Jia, HeavenlyStem::Bing, HeavenlyStem::Wu],
            Self::Mao => vec![HeavenlyStem::Yi],
            Self::Chen => vec![HeavenlyStem::Wu, HeavenlyStem::Yi, HeavenlyStem::Gui],
            Self::Si => vec![HeavenlyStem::Bing, HeavenlyStem::Geng, HeavenlyStem::Wu],
            Self::Wu => vec![HeavenlyStem::Ding, HeavenlyStem::Ji],
            Self::Wei => vec![HeavenlyStem::Ji, HeavenlyStem::Ding, HeavenlyStem::Yi],
            Self::Shen => vec![HeavenlyStem::Geng, HeavenlyStem::Ren, HeavenlyStem::Wu],
            Self::You => vec![HeavenlyStem::Xin],
            Self::Xu => vec![HeavenlyStem::Wu, HeavenlyStem::Xin, HeavenlyStem::Ding],
            Self::Hai => vec![HeavenlyStem::Ren, HeavenlyStem::Jia],
        }
    }
}

// ============================================
// 사주 십성 분석
// ============================================

/// 사주 십성 분석 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenGodAnalysis {
    /// 일간 (기준)
    pub day_master: HeavenlyStem,
    /// 년간 십성
    pub year_stem: TenGod,
    /// 월간 십성
    pub month_stem: TenGod,
    /// 일간 십성 (항상 비견)
    pub day_stem: TenGod,
    /// 시간 십성
    pub hour_stem: TenGod,
    /// 년지 십성 (정기 기준)
    pub year_branch: TenGod,
    /// 월지 십성 (정기 기준)
    pub month_branch: TenGod,
    /// 일지 십성 (정기 기준)
    pub day_branch: TenGod,
    /// 시지 십성 (정기 기준)
    pub hour_branch: TenGod,
}

impl TenGodAnalysis {
    /// 사주로부터 십성 분석
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        let dm = pillars.day_master();

        Self {
            day_master: dm,
            year_stem: TenGod::from_stems(dm, pillars.year.stem),
            month_stem: TenGod::from_stems(dm, pillars.month.stem),
            day_stem: TenGod::Bijian, // 일간은 항상 비견
            hour_stem: TenGod::from_stems(dm, pillars.hour.stem),
            year_branch: TenGod::from_stem_and_branch(dm, pillars.year.branch),
            month_branch: TenGod::from_stem_and_branch(dm, pillars.month.branch),
            day_branch: TenGod::from_stem_and_branch(dm, pillars.day.branch),
            hour_branch: TenGod::from_stem_and_branch(dm, pillars.hour.branch),
        }
    }

    /// 십성별 개수 집계
    pub fn counts(&self) -> [(TenGod, u32); 10] {
        let mut counts = [
            (TenGod::Bijian, 0), (TenGod::Jiecai, 0),
            (TenGod::Shishen, 0), (TenGod::Shangguan, 0),
            (TenGod::Piancai, 0), (TenGod::Zhengcai, 0),
            (TenGod::Pianguan, 0), (TenGod::Zhengguan, 0),
            (TenGod::Pianyin, 0), (TenGod::Zhengyin, 0),
        ];

        let all = [
            self.year_stem, self.month_stem, self.day_stem, self.hour_stem,
            self.year_branch, self.month_branch, self.day_branch, self.hour_branch,
        ];

        for god in all {
            counts[god.index()].1 += 1;
        }

        counts
    }

    /// 가장 많은 십성
    pub fn dominant(&self) -> TenGod {
        let counts = self.counts();
        counts.iter()
            .max_by_key(|(_, count)| count)
            .map(|(god, _)| *god)
            .unwrap_or(TenGod::Bijian)
    }
}

impl std::fmt::Display for TenGodAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┌────────┬────────┬────────┬────────┐")?;
        writeln!(f, "│  時柱  │  日柱  │  月柱  │  年柱  │")?;
        writeln!(f, "├────────┼────────┼────────┼────────┤")?;
        writeln!(f, "│ {:^6} │ {:^6} │ {:^6} │ {:^6} │",
            self.hour_stem.hangul(),
            self.day_stem.hangul(),
            self.month_stem.hangul(),
            self.year_stem.hangul()
        )?;
        writeln!(f, "│ {:^6} │ {:^6} │ {:^6} │ {:^6} │",
            self.hour_branch.hangul(),
            self.day_branch.hangul(),
            self.month_branch.hangul(),
            self.year_branch.hangul()
        )?;
        writeln!(f, "└────────┴────────┴────────┴────────┘")?;
        writeln!(f, "일간(日干): {} ({})", self.day_master, self.day_master.element())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pillars::{FourPillars, SajuInput};

    #[test]
    fn test_ten_god_from_stems() {
        // 庚(경금)이 일간인 경우
        let dm = HeavenlyStem::Geng; // 庚 양금

        // 같은 오행, 같은 음양 = 비견
        assert_eq!(TenGod::from_stems(dm, HeavenlyStem::Geng), TenGod::Bijian);
        
        // 같은 오행, 다른 음양 = 겁재
        assert_eq!(TenGod::from_stems(dm, HeavenlyStem::Xin), TenGod::Jiecai);
        
        // 내가 생하는 오행(水), 같은 음양 = 식신
        assert_eq!(TenGod::from_stems(dm, HeavenlyStem::Ren), TenGod::Shishen);
        
        // 내가 생하는 오행(水), 다른 음양 = 상관
        assert_eq!(TenGod::from_stems(dm, HeavenlyStem::Gui), TenGod::Shangguan);
        
        // 내가 극하는 오행(木), 같은 음양 = 편재
        assert_eq!(TenGod::from_stems(dm, HeavenlyStem::Jia), TenGod::Piancai);
        
        // 내가 극하는 오행(木), 다른 음양 = 정재
        assert_eq!(TenGod::from_stems(dm, HeavenlyStem::Yi), TenGod::Zhengcai);
        
        // 나를 극하는 오행(火), 같은 음양 = 편관
        assert_eq!(TenGod::from_stems(dm, HeavenlyStem::Bing), TenGod::Pianguan);
        
        // 나를 극하는 오행(火), 다른 음양 = 정관
        assert_eq!(TenGod::from_stems(dm, HeavenlyStem::Ding), TenGod::Zhengguan);
        
        // 나를 생하는 오행(土), 같은 음양 = 편인
        assert_eq!(TenGod::from_stems(dm, HeavenlyStem::Wu), TenGod::Pianyin);
        
        // 나를 생하는 오행(土), 다른 음양 = 정인
        assert_eq!(TenGod::from_stems(dm, HeavenlyStem::Ji), TenGod::Zhengyin);
    }

    #[test]
    fn test_user_saju_ten_gods() {
        // 김성주님 사주: 甲申年 乙亥月 庚戌日 丁亥時
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        let analysis = TenGodAnalysis::from_pillars(&pillars);

        // 일간: 庚 (경금)
        assert_eq!(analysis.day_master, HeavenlyStem::Geng);

        // 년간 甲 = 편재 (내가 극하는 木, 같은 양)
        assert_eq!(analysis.year_stem, TenGod::Piancai);

        // 월간 乙 = 정재 (내가 극하는 木, 다른 음양)
        assert_eq!(analysis.month_stem, TenGod::Zhengcai);

        // 시간 丁 = 정관 (나를 극하는 火, 다른 음양)
        assert_eq!(analysis.hour_stem, TenGod::Zhengguan);

        println!("{}", analysis);
    }

    #[test]
    fn test_hidden_stems() {
        // 寅 = 甲(정기), 丙(중기), 戊(여기)
        let stems = EarthlyBranch::Yin.hidden_stems();
        assert_eq!(stems.len(), 3);
        assert_eq!(stems[0], HeavenlyStem::Jia);
    }
}
