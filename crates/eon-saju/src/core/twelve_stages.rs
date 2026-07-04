//! 12운성(十二運星, Twelve Life Stages) 분석
//!
//! 일간(日干)을 기준으로 각 지지(地支)의 에너지 상태를 계산합니다.
//!
//! ## 12운성
//!
//! 1. 장생(長生) - 생명의 시작
//! 2. 목욕(沐浴) - 성장, 불안정
//! 3. 관대(冠帶) - 성인, 자립
//! 4. 건록(建祿) - 녹봉, 전성기 진입
//! 5. 제왕(帝旺) - 정점, 최대 에너지
//! 6. 쇠(衰) - 하강 시작
//! 7. 병(病) - 약화
//! 8. 사(死) - 정지
//! 9. 묘(墓) - 저장, 창고
//! 10. 절(絶) - 단절
//! 11. 태(胎) - 잉태
//! 12. 양(養) - 양육

use crate::core::branch::EarthlyBranch;
use crate::core::pillars::FourPillars;
use crate::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

/// 12운성 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TwelveStage {
    /// 장생(長生) - 태어남
    Changsheng = 0,
    /// 목욕(沐浴) - 목욕
    Muyu = 1,
    /// 관대(冠帶) - 관을 씀
    Guandai = 2,
    /// 건록(建祿) - 녹봉
    Jianlu = 3,
    /// 제왕(帝旺) - 왕성
    Diwang = 4,
    /// 쇠(衰) - 쇠약
    Shuai = 5,
    /// 병(病) - 아픔
    Bing = 6,
    /// 사(死) - 죽음
    Si = 7,
    /// 묘(墓) - 무덤
    Mu = 8,
    /// 절(絶) - 끊어짐
    Jue = 9,
    /// 태(胎) - 잉태
    Tai = 10,
    /// 양(養) - 양육
    Yang = 11,
}

impl TwelveStage {
    /// 모든 12운성 배열
    pub const ALL: [TwelveStage; 12] = [
        Self::Changsheng,
        Self::Muyu,
        Self::Guandai,
        Self::Jianlu,
        Self::Diwang,
        Self::Shuai,
        Self::Bing,
        Self::Si,
        Self::Mu,
        Self::Jue,
        Self::Tai,
        Self::Yang,
    ];

    /// 한글 이름
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Changsheng => "장생",
            Self::Muyu => "목욕",
            Self::Guandai => "관대",
            Self::Jianlu => "건록",
            Self::Diwang => "제왕",
            Self::Shuai => "쇠",
            Self::Bing => "병",
            Self::Si => "사",
            Self::Mu => "묘",
            Self::Jue => "절",
            Self::Tai => "태",
            Self::Yang => "양",
        }
    }

    /// 한자 이름
    pub const fn hanja(&self) -> &'static str {
        match self {
            Self::Changsheng => "長生",
            Self::Muyu => "沐浴",
            Self::Guandai => "冠帶",
            Self::Jianlu => "建祿",
            Self::Diwang => "帝旺",
            Self::Shuai => "衰",
            Self::Bing => "病",
            Self::Si => "死",
            Self::Mu => "墓",
            Self::Jue => "絶",
            Self::Tai => "胎",
            Self::Yang => "養",
        }
    }

    /// 에너지 수준 (0-100)
    pub const fn energy_level(&self) -> u8 {
        match self {
            Self::Diwang => 100,    // 제왕 - 최고
            Self::Jianlu => 90,     // 건록
            Self::Guandai => 70,    // 관대
            Self::Changsheng => 60, // 장생
            Self::Muyu => 50,       // 목욕
            Self::Yang => 40,       // 양
            Self::Tai => 30,        // 태
            Self::Shuai => 20,      // 쇠
            Self::Bing => 15,       // 병
            Self::Si => 10,         // 사
            Self::Mu => 5,          // 묘
            Self::Jue => 0,         // 절 - 최저
        }
    }

    /// 통근(通根) 가중치 - 득지 판정용
    ///
    /// 12운성을 세 등급으로 분류하여 차등 가중치 적용:
    /// - **A급 (왕성)**: 건록, 제왕, 관대 → 1.0 (완전한 통근)
    /// - **B급 (생조)**: 장생, 목욕 → 0.5 (생조는 받지만 뿌리 약함)
    /// - **C급 (쇠약)**: 쇠, 병, 사, 묘, 절, 태, 양 → 0.0 (통근 없음)
    ///
    /// # 이론적 배경
    ///
    /// 장생(長生)은 지지가 일간을 '생(生)'해주는 것이지,
    /// 일간이 지지에 '뿌리(根)'를 내린 것과는 다릅니다.
    /// 예: 甲일간이 亥월(장생)에 태어나면, 水가 木을 생하지만
    /// 甲목의 뿌리는 亥 속 甲목 지장간이 약하여 '부목(浮木)'이 될 수 있습니다.
    pub const fn root_weight(&self) -> f32 {
        match self {
            // A급: 완전한 통근 (건록, 제왕, 관대)
            Self::Jianlu | Self::Diwang | Self::Guandai => 1.0,

            // B급: 생조는 받지만 뿌리 약함 (장생, 목욕)
            Self::Changsheng | Self::Muyu => 0.5,

            // C급: 통근 없음 (나머지)
            Self::Shuai | Self::Bing | Self::Si | Self::Mu | Self::Jue | Self::Tai | Self::Yang => {
                0.0
            }
        }
    }

    /// 강한 12운성인지 확인 (A급 또는 B급)
    pub const fn is_strong(&self) -> bool {
        matches!(
            self,
            Self::Jianlu | Self::Diwang | Self::Guandai | Self::Changsheng | Self::Muyu
        )
    }

    /// 길흉 판단
    /// true = 길(吉), false = 흉(凶), None = 중립
    pub const fn is_auspicious(&self) -> Option<bool> {
        match self {
            Self::Changsheng | Self::Guandai | Self::Jianlu | Self::Diwang => Some(true),
            Self::Bing | Self::Si | Self::Jue => Some(false),
            Self::Muyu | Self::Shuai | Self::Mu | Self::Tai | Self::Yang => None,
        }
    }

    /// 설명
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Changsheng => "생명의 시작, 새로운 희망",
            Self::Muyu => "정화와 성장, 다소 불안정",
            Self::Guandai => "자립과 성인, 사회 진출",
            Self::Jianlu => "녹봉과 권위, 전성기 진입",
            Self::Diwang => "최고점, 최대 에너지와 성공",
            Self::Shuai => "하강 시작, 절제 필요",
            Self::Bing => "약화, 건강 주의",
            Self::Si => "정지, 휴식기",
            Self::Mu => "저장, 숨겨진 에너지",
            Self::Jue => "단절, 재시작 준비",
            Self::Tai => "잉태, 새로운 가능성",
            Self::Yang => "양육, 성장 준비",
        }
    }

    /// 인덱스로부터 생성
    pub const fn from_index(idx: i32) -> Self {
        Self::ALL[idx.rem_euclid(12) as usize]
    }
}

impl std::fmt::Display for TwelveStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hangul())
    }
}

// ============================================
// 12운성 계산 로직
// ============================================

/// 양간(陽干)의 장생(長生) 지지
/// 甲→亥, 丙→寅, 戊→寅, 庚→巳, 壬→申
const YANG_STEM_CHANGSHENG: [EarthlyBranch; 5] = [
    EarthlyBranch::Hai,  // 甲 → 亥
    EarthlyBranch::Yin,  // 丙 → 寅
    EarthlyBranch::Yin,  // 戊 → 寅
    EarthlyBranch::Si,   // 庚 → 巳
    EarthlyBranch::Shen, // 壬 → 申
];

/// 음간(陰干)의 장생(長生) 지지 (역행)
/// 乙→午, 丁→酉, 己→酉, 辛→子, 癸→卯
const YIN_STEM_CHANGSHENG: [EarthlyBranch; 5] = [
    EarthlyBranch::Wu,  // 乙 → 午
    EarthlyBranch::You, // 丁 → 酉
    EarthlyBranch::You, // 己 → 酉
    EarthlyBranch::Zi,  // 辛 → 子
    EarthlyBranch::Mao, // 癸 → 卯
];

/// 일간과 지지로부터 12운성 계산
pub fn calculate_twelve_stage(day_stem: HeavenlyStem, branch: EarthlyBranch) -> TwelveStage {
    // 양간과 음간 구분
    let is_yang = day_stem.index().is_multiple_of(2);

    // 오행별 인덱스 (甲乙=0, 丙丁=1, 戊己=2, 庚辛=3, 壬癸=4)
    let element_idx = (day_stem.index() / 2) as usize;

    // 장생 지지
    let changsheng_branch = if is_yang {
        YANG_STEM_CHANGSHENG[element_idx]
    } else {
        YIN_STEM_CHANGSHENG[element_idx]
    };

    // 장생 지지부터의 거리 계산
    let branch_idx = branch.index() as i32;
    let changsheng_idx = changsheng_branch.index() as i32;

    let distance = if is_yang {
        // 양간: 순행 (시계방향)
        (branch_idx - changsheng_idx).rem_euclid(12)
    } else {
        // 음간: 역행 (반시계방향)
        (changsheng_idx - branch_idx).rem_euclid(12)
    };

    TwelveStage::from_index(distance)
}

// ============================================
// 12운성 분석 결과
// ============================================

/// 사주 전체의 12운성 분석
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwelveStageAnalysis {
    /// 일간 (기준)
    pub day_master: HeavenlyStem,
    /// 년지 12운성 (일간 기준)
    pub year_stage: TwelveStage,
    /// 월지 12운성 (일간 기준)
    pub month_stage: TwelveStage,
    /// 일지 12운성 (일간 기준)
    pub day_stage: TwelveStage,
    /// 시지 12운성 (일간 기준)
    pub hour_stage: TwelveStage,
    /// 전체 에너지 수준 (평균)
    pub total_energy: u8,
    /// --- 자좌(Self) 12운성 ---
    /// 년간이 년지에서 가지는 상태
    pub year_self: TwelveStage,
    /// 월간이 월지에서 가지는 상태
    pub month_self: TwelveStage,
    /// 일간이 일지에서 가지는 상태
    pub day_self: TwelveStage,
    /// 시간이 시지에서 가지는 상태
    pub hour_self: TwelveStage,
}

impl TwelveStageAnalysis {
    /// 사주로부터 12운성 분석
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        let day_master = pillars.day_master();

        let year_stage = calculate_twelve_stage(day_master, pillars.year.branch);
        let month_stage = calculate_twelve_stage(day_master, pillars.month.branch);
        let day_stage = calculate_twelve_stage(day_master, pillars.day.branch);
        let hour_stage = calculate_twelve_stage(day_master, pillars.hour.branch);

        // 자좌 12운성 (각 기둥의 천간 vs 지지)
        let year_self = calculate_twelve_stage(pillars.year.stem, pillars.year.branch);
        let month_self = calculate_twelve_stage(pillars.month.stem, pillars.month.branch);
        let day_self = calculate_twelve_stage(pillars.day.stem, pillars.day.branch);
        let hour_self = calculate_twelve_stage(pillars.hour.stem, pillars.hour.branch);

        let total_energy = (year_stage.energy_level() as u32
            + month_stage.energy_level() as u32
            + day_stage.energy_level() as u32
            + hour_stage.energy_level() as u32)
            / 4;

        Self {
            day_master,
            year_stage,
            month_stage,
            day_stage,
            hour_stage,
            year_self,
            month_self,
            day_self,
            hour_self,
            total_energy: total_energy as u8,
        }
    }

    /// 가장 강한 위치
    pub fn strongest_position(&self) -> (&'static str, TwelveStage) {
        let positions = [
            ("년지", self.year_stage),
            ("월지", self.month_stage),
            ("일지", self.day_stage),
            ("시지", self.hour_stage),
        ];

        positions
            .into_iter()
            .max_by_key(|(_, stage)| stage.energy_level())
            .unwrap()
    }

    /// 가장 약한 위치
    pub fn weakest_position(&self) -> (&'static str, TwelveStage) {
        let positions = [
            ("년지", self.year_stage),
            ("월지", self.month_stage),
            ("일지", self.day_stage),
            ("시지", self.hour_stage),
        ];

        positions
            .into_iter()
            .min_by_key(|(_, stage)| stage.energy_level())
            .unwrap()
    }

    /// 특정 운성이 있는지 확인
    pub fn has_stage(&self, stage: TwelveStage) -> bool {
        self.year_stage == stage
            || self.month_stage == stage
            || self.day_stage == stage
            || self.hour_stage == stage
    }
}

impl std::fmt::Display for TwelveStageAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【12운성 분석】")?;
        writeln!(f, "─────────────────────────────────")?;
        writeln!(
            f,
            "일간: {} ({})",
            self.day_master.hanja(),
            self.day_master.hangul()
        )?;
        writeln!(f)?;
        writeln!(
            f,
            "  년지: {} ({}) - {}",
            self.year_stage.hangul(),
            self.year_stage.hanja(),
            self.year_stage.description()
        )?;
        writeln!(
            f,
            "  월지: {} ({}) - {}",
            self.month_stage.hangul(),
            self.month_stage.hanja(),
            self.month_stage.description()
        )?;
        writeln!(
            f,
            "  일지: {} ({}) - {}",
            self.day_stage.hangul(),
            self.day_stage.hanja(),
            self.day_stage.description()
        )?;
        writeln!(
            f,
            "  시지: {} ({}) - {}",
            self.hour_stage.hangul(),
            self.hour_stage.hanja(),
            self.hour_stage.description()
        )?;
        writeln!(f)?;
        writeln!(f, "【자좌 12운성 (각 기둥 내부 에너지)】")?;
        writeln!(
            f,
            "  년주: {} | 월주: {} | 일주: {} | 시주: {}",
            self.year_self.hangul(),
            self.month_self.hangul(),
            self.day_self.hangul(),
            self.hour_self.hangul()
        )?;
        writeln!(f)?;
        writeln!(f, "총 에너지: {}%", self.total_energy)?;

        let (strongest_pos, strongest_stage) = self.strongest_position();
        let (weakest_pos, weakest_stage) = self.weakest_position();
        writeln!(
            f,
            "최강 위치: {} ({})",
            strongest_pos,
            strongest_stage.hangul()
        )?;
        writeln!(f, "최약 위치: {} ({})", weakest_pos, weakest_stage.hangul())?;

        Ok(())
    }
}

// ============================================
// FourPillars 편의 메서드
// ============================================

impl FourPillars {
    /// 12운성 분석
    pub fn twelve_stages(&self) -> TwelveStageAnalysis {
        TwelveStageAnalysis::from_pillars(self)
    }

    /// 특정 지지의 12운성
    pub fn stage_of(&self, branch: EarthlyBranch) -> TwelveStage {
        calculate_twelve_stage(self.day_master(), branch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pillars::SajuInput;

    #[test]
    fn test_user_twelve_stages() {
        // 김성주님 사주: 甲申年 乙亥月 庚戌日 丁亥時
        // 일간 庚(양금)
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        let analysis = pillars.twelve_stages();

        println!("{}", analysis);

        // 庚의 장생은 巳
        // 庚 → 巳(장생), 午(목욕), 未(관대), 申(건록), 酉(제왕), 戌(쇠)...

        // 년지 申 = 건록
        assert_eq!(analysis.year_stage, TwelveStage::Jianlu);

        // 일지 戌 = 쇠
        assert_eq!(analysis.day_stage, TwelveStage::Shuai);
    }

    #[test]
    fn test_jia_stem() {
        // 甲의 장생은 亥
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Jia, EarthlyBranch::Hai),
            TwelveStage::Changsheng
        );

        // 甲 → 子 = 목욕
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Jia, EarthlyBranch::Zi),
            TwelveStage::Muyu
        );

        // 甲 → 寅 = 건록
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Jia, EarthlyBranch::Yin),
            TwelveStage::Jianlu
        );
    }

    #[test]
    fn test_geng_stem() {
        // 庚의 장생은 巳
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Geng, EarthlyBranch::Si),
            TwelveStage::Changsheng
        );

        // 庚 → 申 = 건록
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Geng, EarthlyBranch::Shen),
            TwelveStage::Jianlu
        );

        // 庚 → 戌 = 쇠
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Geng, EarthlyBranch::Xu),
            TwelveStage::Shuai
        );
    }

    #[test]
    fn test_yin_stem() {
        // 乙(음목)의 장생은 午 (역행)
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Yi, EarthlyBranch::Wu),
            TwelveStage::Changsheng
        );

        // 乙 → 卯 = 건록
        assert_eq!(
            calculate_twelve_stage(HeavenlyStem::Yi, EarthlyBranch::Mao),
            TwelveStage::Jianlu
        );
    }

    #[test]
    fn test_energy_level() {
        assert_eq!(TwelveStage::Diwang.energy_level(), 100);
        assert_eq!(TwelveStage::Jue.energy_level(), 0);
    }
}
