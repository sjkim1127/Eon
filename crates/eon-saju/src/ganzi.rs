//! 간지(干支, GanZi) - 60갑자 사이클
//!
//! 천간(10)과 지지(12)의 조합으로 60갑자를 구성합니다.
//! 甲子부터 癸亥까지 60개의 조합이 순환합니다.

use serde::{Deserialize, Serialize};
use crate::stem::HeavenlyStem;
use crate::branch::EarthlyBranch;
use crate::element::{Element, Polarity};

/// 간지(干支) - 천간과 지지의 조합
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GanZi {
    /// 천간
    pub stem: HeavenlyStem,
    /// 지지
    pub branch: EarthlyBranch,
}

impl GanZi {
    /// 60갑자 배열 (甲子 ~ 癸亥)
    pub const SIXTY_CYCLE: [(HeavenlyStem, EarthlyBranch); 60] = Self::generate_sixty_cycle();

    /// 60갑자 배열 생성 (상수 함수)
    const fn generate_sixty_cycle() -> [(HeavenlyStem, EarthlyBranch); 60] {
        let mut result = [(HeavenlyStem::Jia, EarthlyBranch::Zi); 60];
        let mut i = 0;
        while i < 60 {
            result[i] = (
                HeavenlyStem::from_index(i as i32),
                EarthlyBranch::from_index(i as i32),
            );
            i += 1;
        }
        result
    }

    /// 새 간지 생성
    #[inline]
    pub const fn new(stem: HeavenlyStem, branch: EarthlyBranch) -> Self {
        Self { stem, branch }
    }

    /// 60갑자 인덱스로부터 간지 생성 (0-59, 순환)
    #[inline]
    pub const fn from_index(idx: i32) -> Self {
        let normalized = idx.rem_euclid(60);
        let (stem, branch) = Self::SIXTY_CYCLE[normalized as usize];
        Self { stem, branch }
    }

    /// 60갑자 내에서의 인덱스 반환 (0-59)
    /// 
    /// 간지가 60갑자 중 몇 번째인지 계산합니다.
    /// 예: 甲子=0, 乙丑=1, ..., 癸亥=59
    pub const fn index(self) -> u8 {
        // 천간과 지지의 최소공배수 순환에서 위치 계산
        // stem_idx = n mod 10
        // branch_idx = n mod 12
        // n을 찾아야 함 (중국 나머지 정리)
        let stem_idx = self.stem.index() as i32;
        let branch_idx = self.branch.index() as i32;
        
        // 60갑자에서 유효한 조합인지 확인 (음양이 일치해야 함)
        // stem_idx % 2 == branch_idx % 2 이어야 유효한 간지
        
        // n ≡ stem_idx (mod 10)
        // n ≡ branch_idx (mod 12)
        // 해는 n = (6 * stem_idx - 5 * branch_idx) mod 60
        let n = (6 * stem_idx - 5 * branch_idx).rem_euclid(60);
        n as u8
    }

    /// 천간의 오행
    #[inline]
    pub const fn stem_element(self) -> Element {
        self.stem.element()
    }

    /// 지지의 오행 (정기 기준)
    #[inline]
    pub const fn branch_element(self) -> Element {
        self.branch.element()
    }

    /// 음양 (천간과 지지의 음양은 항상 일치)
    #[inline]
    pub const fn polarity(self) -> Polarity {
        self.stem.polarity()
    }

    /// 한자 표기 (예: "甲子")
    pub fn hanja(&self) -> String {
        format!("{}{}", self.stem.hanja(), self.branch.hanja())
    }

    /// 한글 표기 (예: "갑자")
    pub fn hangul(&self) -> String {
        format!("{}{}", self.stem.hangul(), self.branch.hangul())
    }

    /// 다음 간지 (순환)
    #[inline]
    pub const fn next(self) -> Self {
        Self::from_index(self.index() as i32 + 1)
    }

    /// 이전 간지 (순환)
    #[inline]
    pub const fn prev(self) -> Self {
        Self::from_index(self.index() as i32 - 1)
    }

    /// 두 간지 사이의 거리 계산
    #[inline]
    pub const fn distance_to(self, other: Self) -> i32 {
        (other.index() as i32 - self.index() as i32).rem_euclid(60)
    }
}

impl std::fmt::Display for GanZi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.stem, self.branch)
    }
}

impl Default for GanZi {
    fn default() -> Self {
        Self::new(HeavenlyStem::Jia, EarthlyBranch::Zi)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ganzi_index() {
        // 甲子 = 0
        let jiazi = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        assert_eq!(jiazi.index(), 0);

        // 乙丑 = 1
        let yichou = GanZi::new(HeavenlyStem::Yi, EarthlyBranch::Chou);
        assert_eq!(yichou.index(), 1);

        // 癸亥 = 59
        let guihai = GanZi::new(HeavenlyStem::Gui, EarthlyBranch::Hai);
        assert_eq!(guihai.index(), 59);

        // 甲寅 = 50
        let jiayin = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Yin);
        assert_eq!(jiayin.index(), 50);
    }

    #[test]
    fn test_ganzi_from_index() {
        assert_eq!(GanZi::from_index(0), GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi));
        assert_eq!(GanZi::from_index(59), GanZi::new(HeavenlyStem::Gui, EarthlyBranch::Hai));
        assert_eq!(GanZi::from_index(60), GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi)); // 순환
        assert_eq!(GanZi::from_index(-1), GanZi::new(HeavenlyStem::Gui, EarthlyBranch::Hai)); // 음수
    }

    #[test]
    fn test_ganzi_sixty_cycle() {
        // 60갑자 전체 순환 테스트
        for i in 0..60 {
            let ganzi = GanZi::from_index(i);
            assert_eq!(ganzi.index() as i32, i);
        }
    }

    #[test]
    fn test_ganzi_next_prev() {
        let jiazi = GanZi::from_index(0);
        assert_eq!(jiazi.next(), GanZi::from_index(1));
        assert_eq!(jiazi.prev(), GanZi::from_index(59));
    }

    #[test]
    fn test_ganzi_display() {
        let jiazi = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        assert_eq!(jiazi.to_string(), "甲子");
        assert_eq!(jiazi.hangul(), "갑자");
    }

    #[test]
    fn test_ganzi_polarity() {
        // 양간 + 양지
        let jiazi = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        assert_eq!(jiazi.polarity(), Polarity::Yang);

        // 음간 + 음지
        let yichou = GanZi::new(HeavenlyStem::Yi, EarthlyBranch::Chou);
        assert_eq!(yichou.polarity(), Polarity::Yin);
    }
}
