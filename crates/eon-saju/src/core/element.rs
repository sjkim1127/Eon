//! 오행(五行, Five Elements) 및 음양(陰陽, Yin-Yang)
//!
//! 木火土金水의 상생상극 관계를 정의합니다.

use serde::{Deserialize, Serialize};

/// 오행(五行)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    /// 木 (목) - 나무
    Wood,
    /// 火 (화) - 불
    Fire,
    /// 土 (토) - 흙
    Earth,
    /// 金 (금) - 쇠
    Metal,
    /// 水 (수) - 물
    Water,
}

/// 음양(陰陽)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Polarity {
    /// 陽 (양)
    Yang,
    /// 陰 (음)
    Yin,
}

/// 오행 관계 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementRelation {
    /// 같은 오행
    Same,
    /// 생(生): 내가 생해준다
    Generates,
    /// 생(生): 내가 생을 받는다
    GeneratedBy,
    /// 극(克): 내가 극한다
    Controls,
    /// 극(克): 내가 극을 받는다
    ControlledBy,
}

impl Element {
    /// 모든 오행 배열
    pub const ALL: [Element; 5] = [
        Self::Wood,
        Self::Fire,
        Self::Earth,
        Self::Metal,
        Self::Water,
    ];

    /// 한자 표기
    pub const HANJA: [&'static str; 5] = ["木", "火", "土", "金", "水"];

    /// 한글 표기
    pub const HANGUL: [&'static str; 5] = ["목", "화", "토", "금", "수"];

    /// 인덱스 (0-4)
    #[inline]
    pub const fn index(self) -> u8 {
        match self {
            Self::Wood => 0,
            Self::Fire => 1,
            Self::Earth => 2,
            Self::Metal => 3,
            Self::Water => 4,
        }
    }

    /// 인덱스로부터 오행 생성
    #[inline]
    pub const fn from_index(idx: i32) -> Self {
        Self::ALL[idx.rem_euclid(5) as usize]
    }

    /// 상생(相生): 내가 생해주는 오행
    ///
    /// 木→火→土→金→水→木
    #[inline]
    pub const fn generates(self) -> Element {
        match self {
            Self::Wood => Self::Fire,
            Self::Fire => Self::Earth,
            Self::Earth => Self::Metal,
            Self::Metal => Self::Water,
            Self::Water => Self::Wood,
        }
    }

    /// 상생(相生): 나를 생해주는 오행
    #[inline]
    pub const fn generated_by(self) -> Element {
        match self {
            Self::Wood => Self::Water,
            Self::Fire => Self::Wood,
            Self::Earth => Self::Fire,
            Self::Metal => Self::Earth,
            Self::Water => Self::Metal,
        }
    }

    /// 상극(相克): 내가 극하는 오행
    ///
    /// 木→土→水→火→金→木
    #[inline]
    pub const fn controls(self) -> Element {
        match self {
            Self::Wood => Self::Earth,
            Self::Fire => Self::Metal,
            Self::Earth => Self::Water,
            Self::Metal => Self::Wood,
            Self::Water => Self::Fire,
        }
    }

    /// 상극(相克): 나를 극하는 오행
    #[inline]
    pub const fn controlled_by(self) -> Element {
        match self {
            Self::Wood => Self::Metal,
            Self::Fire => Self::Water,
            Self::Earth => Self::Wood,
            Self::Metal => Self::Fire,
            Self::Water => Self::Earth,
        }
    }

    /// 다른 오행과의 관계 판정
    #[inline]
    pub const fn relation_to(self, other: Element) -> ElementRelation {
        if self.index() == other.index() {
            ElementRelation::Same
        } else if self.generates().index() == other.index() {
            ElementRelation::Generates
        } else if self.generated_by().index() == other.index() {
            ElementRelation::GeneratedBy
        } else if self.controls().index() == other.index() {
            ElementRelation::Controls
        } else {
            ElementRelation::ControlledBy
        }
    }

    /// 한자 표기
    #[inline]
    pub const fn hanja(self) -> &'static str {
        Self::HANJA[self.index() as usize]
    }

    /// 한글 표기
    #[inline]
    pub const fn hangul(self) -> &'static str {
        Self::HANGUL[self.index() as usize]
    }
}

impl Polarity {
    /// 한자 표기
    #[inline]
    pub const fn hanja(self) -> &'static str {
        match self {
            Self::Yang => "陽",
            Self::Yin => "陰",
        }
    }

    /// 한글 표기
    #[inline]
    pub const fn hangul(self) -> &'static str {
        match self {
            Self::Yang => "양",
            Self::Yin => "음",
        }
    }

    /// 반대 음양
    #[inline]
    pub const fn opposite(self) -> Self {
        match self {
            Self::Yang => Self::Yin,
            Self::Yin => Self::Yang,
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hanja())
    }
}

impl std::fmt::Display for Polarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hanja())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_generates() {
        assert_eq!(Element::Wood.generates(), Element::Fire);
        assert_eq!(Element::Fire.generates(), Element::Earth);
        assert_eq!(Element::Earth.generates(), Element::Metal);
        assert_eq!(Element::Metal.generates(), Element::Water);
        assert_eq!(Element::Water.generates(), Element::Wood);
    }

    #[test]
    fn test_element_controls() {
        assert_eq!(Element::Wood.controls(), Element::Earth);
        assert_eq!(Element::Fire.controls(), Element::Metal);
        assert_eq!(Element::Earth.controls(), Element::Water);
        assert_eq!(Element::Metal.controls(), Element::Wood);
        assert_eq!(Element::Water.controls(), Element::Fire);
    }

    #[test]
    fn test_element_relation() {
        assert_eq!(
            Element::Wood.relation_to(Element::Wood),
            ElementRelation::Same
        );
        assert_eq!(
            Element::Wood.relation_to(Element::Fire),
            ElementRelation::Generates
        );
        assert_eq!(
            Element::Wood.relation_to(Element::Water),
            ElementRelation::GeneratedBy
        );
        assert_eq!(
            Element::Wood.relation_to(Element::Earth),
            ElementRelation::Controls
        );
        assert_eq!(
            Element::Wood.relation_to(Element::Metal),
            ElementRelation::ControlledBy
        );
    }

    #[test]
    fn test_polarity_opposite() {
        assert_eq!(Polarity::Yang.opposite(), Polarity::Yin);
        assert_eq!(Polarity::Yin.opposite(), Polarity::Yang);
    }
}
