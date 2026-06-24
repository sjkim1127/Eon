//! # eon-zwds — 자미두수 (紫微斗數) 엔진
//!
//! 중국 전통 성반 명리학 자미두수를 Rust로 구현한 엔진입니다.
//! 알고리즘 레퍼런스: [SylarLong/iztro](https://github.com/SylarLong/iztro) (TypeScript, MIT)
//!
//! ## 주요 기능
//! - 12궁(宮) 성반 배치 계산
//! - 14주성(主星) + 6보조성 + 중잡성 ~50성 배치
//! - 사화(四化: 化祿·化權·化科·化忌) 비행
//! - 대한(大限, 10년 운) / 유년(流年, 1년 운)
//! - 명주(命主) / 신주(身主)

pub mod error;
pub mod types;
pub mod calendar;
pub mod palace;
pub mod stars;
pub mod transformations;
pub mod decadal;
pub mod annual;
pub mod brightness;
pub mod chart;
pub mod destiny_patterns;

pub use error::ZwdsError;
pub use types::*;
pub use chart::build_chart;
