//! # Eon Saju (四柱)
//!
//! 사주(四柱, Four Pillars of Destiny) 계산 라이브러리입니다.
//!
//! ## 주요 기능
//!
//! - 천간(天干) / 지지(地支) 타입
//! - 오행(五行) 상생상극 로직
//! - 60갑자 사이클
//! - 사주 팔자 계산
//!
//! ## 사용 예시
//!
//! ```rust
//! use eon_saju::{FourPillars, SajuInput};
//! use chrono::{TimeZone, Local};
//!
//! let input = SajuInput {
//!     year: 1990,
//!     month: 5,
//!     day: 15,
//!     hour: 14,
//!     minute: 30,
//!     is_lunar: false,
//!     is_leap_month: false,
//! };
//!
//! let pillars = FourPillars::calculate(&input).unwrap();
//! println!("{}", pillars);
//! ```

pub mod stem;
pub mod branch;
pub mod element;
pub mod ganzi;
pub mod pillars;
pub mod calendar;

pub use stem::HeavenlyStem;
pub use branch::EarthlyBranch;
pub use element::{Element, Polarity};
pub use ganzi::GanZi;
pub use pillars::{FourPillars, SajuInput};
