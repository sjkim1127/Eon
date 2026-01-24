//! 신살(神煞, Spirit Markers) 분석
//!
//! 사주에서 특정 조합으로 나타나는 길신(吉神)과 흉살(凶煞)을 분석합니다.
//!
//! ## 주요 신살
//!
//! ### 길신 (吉神)
//! - 천을귀인, 문창귀인, 태극귀인, 월덕귀인, 정록 등
//!
//! ### 흉살 (凶煞)
//! - 역마살, 화개살, 괴강살, 도화살, 고신살 등

use serde::{Deserialize, Serialize};
use crate::stem::HeavenlyStem;
use crate::branch::EarthlyBranch;
use crate::ganzi::GanZi;
use crate::pillars::FourPillars;

/// 신살 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpiritMarker {
    // === 길신 (吉神) ===
    /// 천을귀인(天乙貴人) - 귀인의 도움
    Tianyi,
    /// 문창귀인(文昌貴人) - 학문, 시험운
    Wenchang,
    /// 태극귀인(太極貴人) - 영적 보호
    Taiji,
    /// 월덕귀인(月德貴人) - 월간의 덕
    Yuede,
    /// 천덕귀인(天德貴人) - 천간의 덕
    Tiande,
    /// 정록(正祿) - 녹성, 재물운
    Zhenglu,
    /// 금여록(金輿祿) - 귀하고 화려한 운
    Jinyu,
    /// 암록(暗祿) - 숨겨진 재물운
    Anlu,
    /// 학당귀인(學堂貴人) - 학업 성취
    Xuetang,
    /// 천의성(天醫星) - 의료, 치유 능력
    Tianyi_medical,
    /// 천문성(天文星) - 학문, 예술
    Tianwen,
    
    // === 흉살 (凶煞) ===
    /// 역마살(驛馬煞) - 이동, 변화
    Yima,
    /// 화개살(華蓋煞) - 고독, 예술성
    Huagai,
    /// 괴강살(魁罡煞) - 강한 성격
    Kuigang,
    /// 도화살(桃花煞) - 이성 관계
    Taohua,
    /// 홍염살(紅艶煞) - 색정, 매력
    Hongyan,
    /// 고신살(孤辰煞) - 고독
    Guchen,
    /// 과숙살(寡宿煞) - 독거
    Guasu,
    /// 현침살(懸針煞) - 날카로움
    Xuanzhen,
    /// 백호살(白虎煞) - 흉험
    Baihu,
    /// 망신살(亡身煞) - 손실
    Wangshen,
    /// 겁살(劫煞) - 겁탈
    Jiesha,
    /// 원진살(怨嗔煞) - 원한
    Yuanzhen,
}

impl SpiritMarker {
    /// 한글 이름
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Tianyi => "천을귀인",
            Self::Wenchang => "문창귀인",
            Self::Taiji => "태극귀인",
            Self::Yuede => "월덕귀인",
            Self::Tiande => "천덕귀인",
            Self::Zhenglu => "정록",
            Self::Jinyu => "금여록",
            Self::Anlu => "암록",
            Self::Xuetang => "학당귀인",
            Self::Tianyi_medical => "천의성",
            Self::Tianwen => "천문성",
            Self::Yima => "역마살",
            Self::Huagai => "화개살",
            Self::Kuigang => "괴강살",
            Self::Taohua => "도화살",
            Self::Hongyan => "홍염살",
            Self::Guchen => "고신살",
            Self::Guasu => "과숙살",
            Self::Xuanzhen => "현침살",
            Self::Baihu => "백호살",
            Self::Wangshen => "망신살",
            Self::Jiesha => "겁살",
            Self::Yuanzhen => "원진살",
        }
    }

    /// 한자 이름
    pub const fn hanja(&self) -> &'static str {
        match self {
            Self::Tianyi => "天乙貴人",
            Self::Wenchang => "文昌貴人",
            Self::Taiji => "太極貴人",
            Self::Yuede => "月德貴人",
            Self::Tiande => "天德貴人",
            Self::Zhenglu => "正祿",
            Self::Jinyu => "金輿祿",
            Self::Anlu => "暗祿",
            Self::Xuetang => "學堂貴人",
            Self::Tianyi_medical => "天醫星",
            Self::Tianwen => "天文星",
            Self::Yima => "驛馬煞",
            Self::Huagai => "華蓋煞",
            Self::Kuigang => "魁罡煞",
            Self::Taohua => "桃花煞",
            Self::Hongyan => "紅艶煞",
            Self::Guchen => "孤辰煞",
            Self::Guasu => "寡宿煞",
            Self::Xuanzhen => "懸針煞",
            Self::Baihu => "白虎煞",
            Self::Wangshen => "亡身煞",
            Self::Jiesha => "劫煞",
            Self::Yuanzhen => "怨嗔煞",
        }
    }

    /// 길신 여부
    pub const fn is_auspicious(&self) -> bool {
        matches!(self,
            Self::Tianyi | Self::Wenchang | Self::Taiji | Self::Yuede |
            Self::Tiande | Self::Zhenglu | Self::Jinyu | Self::Anlu |
            Self::Xuetang | Self::Tianyi_medical | Self::Tianwen
        )
    }

    /// 설명
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Tianyi => "귀인의 도움을 받아 어려움을 극복",
            Self::Wenchang => "학문과 시험에 유리, 문서 관련 길함",
            Self::Taiji => "영적 보호, 종교/철학에 재능",
            Self::Yuede => "월간의 덕으로 재난 회피",
            Self::Tiande => "천간의 덕으로 흉을 피함",
            Self::Zhenglu => "정당한 재물운, 녹봉",
            Self::Jinyu => "귀하고 화려한 운명",
            Self::Anlu => "숨겨진 재물운",
            Self::Xuetang => "학업 성취에 유리",
            Self::Tianyi_medical => "의료, 치유 분야에 재능",
            Self::Tianwen => "학문과 예술에 특별한 재능",
            Self::Yima => "이동수, 변화가 많음",
            Self::Huagai => "예술적 감각, 고독한 경향",
            Self::Kuigang => "강한 성격, 리더십",
            Self::Taohua => "이성에게 인기, 연애운",
            Self::Hongyan => "매력적, 색정 주의",
            Self::Guchen => "외로움, 독립적",
            Self::Guasu => "배우자운 불리",
            Self::Xuanzhen => "날카로운 성격",
            Self::Baihu => "사고, 부상 주의",
            Self::Wangshen => "손실, 실패 주의",
            Self::Jiesha => "재물 손실 주의",
            Self::Yuanzhen => "대인관계 갈등",
        }
    }
}

impl std::fmt::Display for SpiritMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hangul())
    }
}

/// 신살 발견 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PillarPosition {
    Year,
    Month,
    Day,
    Hour,
}

impl PillarPosition {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Year => "년주",
            Self::Month => "월주",
            Self::Day => "일주",
            Self::Hour => "시주",
        }
    }
}

/// 발견된 신살 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundMarker {
    pub marker: SpiritMarker,
    pub position: PillarPosition,
    pub is_stem: bool, // true=천간, false=지지
}

impl std::fmt::Display for FoundMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let part = if self.is_stem { "천간" } else { "지지" };
        write!(f, "{} {} - {}", self.position.hangul(), part, self.marker)
    }
}

/// 신살 분석 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritMarkerAnalysis {
    /// 발견된 모든 신살
    pub markers: Vec<FoundMarker>,
    /// 길신 목록
    pub auspicious: Vec<SpiritMarker>,
    /// 흉살 목록
    pub inauspicious: Vec<SpiritMarker>,
}

impl SpiritMarkerAnalysis {
    /// 사주에서 신살 분석
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        let mut markers = Vec::new();
        
        let branches = [
            (pillars.year.branch, PillarPosition::Year),
            (pillars.month.branch, PillarPosition::Month),
            (pillars.day.branch, PillarPosition::Day),
            (pillars.hour.branch, PillarPosition::Hour),
        ];

        let day_branch = pillars.day.branch;
        let day_stem = pillars.day.stem;

        // === 역마살 (驛馬煞) ===
        // 寅午戌 → 申, 申子辰 → 寅, 巳酉丑 → 亥, 亥卯未 → 巳
        let yima_branch = Self::get_yima_branch(day_branch);
        for (branch, pos) in &branches {
            if *branch == yima_branch {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Yima,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 화개살 (華蓋煞) ===
        // 寅午戌 → 戌, 申子辰 → 辰, 巳酉丑 → 丑, 亥卯未 → 未
        let huagai_branch = Self::get_huagai_branch(day_branch);
        for (branch, pos) in &branches {
            if *branch == huagai_branch {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Huagai,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 괴강살 (魁罡煞) ===
        // 庚辰, 庚戌, 壬辰, 戊戌 일주
        if Self::is_kuigang(pillars.day) {
            markers.push(FoundMarker {
                marker: SpiritMarker::Kuigang,
                position: PillarPosition::Day,
                is_stem: false,
            });
        }

        // === 도화살 (桃花煞) ===
        // 寅午戌 → 卯, 申子辰 → 酉, 巳酉丑 → 午, 亥卯未 → 子
        let taohua_branch = Self::get_taohua_branch(day_branch);
        for (branch, pos) in &branches {
            if *branch == taohua_branch {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Taohua,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 문창귀인 (文昌貴人) ===
        // 甲→巳, 乙→午, 丙→申, 丁→酉, 戊→申, 己→酉, 庚→亥, 辛→子, 壬→寅, 癸→卯
        let wenchang_branch = Self::get_wenchang_branch(day_stem);
        for (branch, pos) in &branches {
            if *branch == wenchang_branch {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Wenchang,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 태극귀인 (太極貴人) ===
        let taiji_branches = Self::get_taiji_branches(day_stem);
        for (branch, pos) in &branches {
            if taiji_branches.contains(branch) {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Taiji,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 고신살 (孤辰煞) ===
        let guchen_branch = Self::get_guchen_branch(day_branch);
        for (branch, pos) in &branches {
            if *branch == guchen_branch {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Guchen,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 홍염살 (紅艶煞) ===
        let hongyan_branch_opt = Self::get_hongyan_branch(day_stem);
        if let Some(hongyan_branch) = hongyan_branch_opt {
            for (branch, pos) in &branches {
                if *branch == hongyan_branch {
                    markers.push(FoundMarker {
                        marker: SpiritMarker::Hongyan,
                        position: *pos,
                        is_stem: false,
                    });
                }
            }
        }

        // === 천의성 (天醫星) ===
        // 월지를 기준으로 계산
        let tianyi_branch = Self::get_tianyi_medical_branch(pillars.month.branch);
        for (branch, pos) in &branches {
            if *branch == tianyi_branch {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Tianyi_medical,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 금여록 (金輿祿) ===
        let jinyu_branch_opt = Self::get_jinyu_branch(day_stem);
        if let Some(jinyu_branch) = jinyu_branch_opt {
            for (branch, pos) in &branches {
                if *branch == jinyu_branch {
                    markers.push(FoundMarker {
                        marker: SpiritMarker::Jinyu,
                        position: *pos,
                        is_stem: false,
                    });
                }
            }
        }

        // 길신/흉살 분류
        let auspicious: Vec<_> = markers.iter()
            .filter(|m| m.marker.is_auspicious())
            .map(|m| m.marker)
            .collect();
        
        let inauspicious: Vec<_> = markers.iter()
            .filter(|m| !m.marker.is_auspicious())
            .map(|m| m.marker)
            .collect();

        Self {
            markers,
            auspicious,
            inauspicious,
        }
    }

    // === 신살 계산 헬퍼 함수들 ===

    fn get_yima_branch(day_branch: EarthlyBranch) -> EarthlyBranch {
        match day_branch {
            EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Shen,
            EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::Yin,
            EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Hai,
            EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Si,
        }
    }

    fn get_huagai_branch(day_branch: EarthlyBranch) -> EarthlyBranch {
        match day_branch {
            EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Xu,
            EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::Chen,
            EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Chou,
            EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Wei,
        }
    }

    fn is_kuigang(day: GanZi) -> bool {
        matches!(
            (day.stem, day.branch),
            (HeavenlyStem::Geng, EarthlyBranch::Chen) |
            (HeavenlyStem::Geng, EarthlyBranch::Xu) |
            (HeavenlyStem::Ren, EarthlyBranch::Chen) |
            (HeavenlyStem::Wu, EarthlyBranch::Xu)
        )
    }

    fn get_taohua_branch(day_branch: EarthlyBranch) -> EarthlyBranch {
        match day_branch {
            EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Mao,
            EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::You,
            EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Wu,
            EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Zi,
        }
    }

    fn get_wenchang_branch(day_stem: HeavenlyStem) -> EarthlyBranch {
        match day_stem {
            HeavenlyStem::Jia => EarthlyBranch::Si,
            HeavenlyStem::Yi => EarthlyBranch::Wu,
            HeavenlyStem::Bing | HeavenlyStem::Wu => EarthlyBranch::Shen,
            HeavenlyStem::Ding | HeavenlyStem::Ji => EarthlyBranch::You,
            HeavenlyStem::Geng => EarthlyBranch::Hai,
            HeavenlyStem::Xin => EarthlyBranch::Zi,
            HeavenlyStem::Ren => EarthlyBranch::Yin,
            HeavenlyStem::Gui => EarthlyBranch::Mao,
        }
    }

    fn get_taiji_branches(day_stem: HeavenlyStem) -> Vec<EarthlyBranch> {
        match day_stem {
            HeavenlyStem::Jia | HeavenlyStem::Ji => vec![EarthlyBranch::Zi, EarthlyBranch::Wu],
            HeavenlyStem::Yi | HeavenlyStem::Geng => vec![EarthlyBranch::Mao, EarthlyBranch::You],
            HeavenlyStem::Bing | HeavenlyStem::Xin => vec![EarthlyBranch::Yin, EarthlyBranch::Hai],
            HeavenlyStem::Ding | HeavenlyStem::Ren => vec![EarthlyBranch::Si, EarthlyBranch::You],
            HeavenlyStem::Wu | HeavenlyStem::Gui => vec![EarthlyBranch::Chen, EarthlyBranch::Xu],
        }
    }

    fn get_guchen_branch(day_branch: EarthlyBranch) -> EarthlyBranch {
        match day_branch {
            EarthlyBranch::Yin | EarthlyBranch::Mao | EarthlyBranch::Chen => EarthlyBranch::Si,
            EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei => EarthlyBranch::Shen,
            EarthlyBranch::Shen | EarthlyBranch::You | EarthlyBranch::Xu => EarthlyBranch::Hai,
            EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou => EarthlyBranch::Yin,
        }
    }

    fn get_hongyan_branch(day_stem: HeavenlyStem) -> Option<EarthlyBranch> {
        match day_stem {
            HeavenlyStem::Jia => Some(EarthlyBranch::Wu),
            HeavenlyStem::Yi => Some(EarthlyBranch::Shen),
            HeavenlyStem::Bing => Some(EarthlyBranch::Yin),
            HeavenlyStem::Ding => Some(EarthlyBranch::Wei),
            HeavenlyStem::Wu => Some(EarthlyBranch::Chen),
            HeavenlyStem::Ji => Some(EarthlyBranch::Chen),
            HeavenlyStem::Geng => Some(EarthlyBranch::Xu),
            HeavenlyStem::Xin => Some(EarthlyBranch::You),
            HeavenlyStem::Ren => Some(EarthlyBranch::Zi),
            HeavenlyStem::Gui => Some(EarthlyBranch::Shen),
        }
    }

    fn get_tianyi_medical_branch(month_branch: EarthlyBranch) -> EarthlyBranch {
        // 월지의 다음 지지
        EarthlyBranch::from_index((month_branch.index() as i32 + 1) % 12)
    }

    fn get_jinyu_branch(day_stem: HeavenlyStem) -> Option<EarthlyBranch> {
        match day_stem {
            HeavenlyStem::Jia => Some(EarthlyBranch::Chen),
            HeavenlyStem::Yi => Some(EarthlyBranch::Si),
            HeavenlyStem::Bing | HeavenlyStem::Wu => Some(EarthlyBranch::Wei),
            HeavenlyStem::Ding | HeavenlyStem::Ji => Some(EarthlyBranch::Shen),
            HeavenlyStem::Geng => Some(EarthlyBranch::Xu),
            HeavenlyStem::Xin => Some(EarthlyBranch::Hai),
            HeavenlyStem::Ren => Some(EarthlyBranch::Chou),
            HeavenlyStem::Gui => Some(EarthlyBranch::Yin),
        }
    }
}

impl std::fmt::Display for SpiritMarkerAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【신살 분석】")?;
        writeln!(f, "─────────────────────────────────")?;
        
        if !self.auspicious.is_empty() {
            write!(f, "길신: ")?;
            for (i, marker) in self.auspicious.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", marker.hangul())?;
            }
            writeln!(f)?;
        }
        
        if !self.inauspicious.is_empty() {
            write!(f, "흉살: ")?;
            for (i, marker) in self.inauspicious.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", marker.hangul())?;
            }
            writeln!(f)?;
        }

        writeln!(f, "\n상세:")?;
        for marker in &self.markers {
            writeln!(f, "  • {}", marker)?;
        }
        
        Ok(())
    }
}

// ============================================
// FourPillars 편의 메서드
// ============================================

impl FourPillars {
    /// 신살 분석
    pub fn spirit_markers(&self) -> SpiritMarkerAnalysis {
        SpiritMarkerAnalysis::from_pillars(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pillars::SajuInput;

    #[test]
    fn test_user_spirit_markers() {
        // 김성주님 사주: 甲申年 乙亥月 庚戌日 丁亥時
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        
        let analysis = pillars.spirit_markers();
        
        println!("{}", analysis);
        
        // 괴강살 확인 (庚戌日)
        assert!(analysis.markers.iter().any(|m| m.marker == SpiritMarker::Kuigang));
    }

    #[test]
    fn test_kuigang() {
        // 庚戌日 = 괴강살
        let ganzi = GanZi {
            stem: HeavenlyStem::Geng,
            branch: EarthlyBranch::Xu,
        };
        assert!(SpiritMarkerAnalysis::is_kuigang(ganzi));

        // 庚辰日 = 괴강살
        let ganzi2 = GanZi {
            stem: HeavenlyStem::Geng,
            branch: EarthlyBranch::Chen,
        };
        assert!(SpiritMarkerAnalysis::is_kuigang(ganzi2));
    }

    #[test]
    fn test_yima() {
        // 戌日 → 申이 역마
        assert_eq!(
            SpiritMarkerAnalysis::get_yima_branch(EarthlyBranch::Xu),
            EarthlyBranch::Shen
        );
    }
}
