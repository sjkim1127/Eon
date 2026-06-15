//! 출생 정보 (BirthInfo) - 히스토리컬 타임존 지원
//!
//! 모든 운명 시스템에서 공통으로 사용되는 출생 정보입니다.
//! IANA 타임존 데이터베이스(chrono-tz)를 활용하여
//! 썸머타임(DST) 등 역사적 시간대 변경을 자동으로 처리합니다.

use crate::location::Location;
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};
use chrono_tz::{OffsetComponents, Tz};
use serde::{Deserialize, Serialize};

/// 성별
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    /// 남성
    Male,
    /// 여성
    Female,
}

impl Gender {
    pub fn is_male(&self) -> bool {
        matches!(self, Gender::Male)
    }

    pub fn is_female(&self) -> bool {
        matches!(self, Gender::Female)
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "남"),
            Gender::Female => write!(f, "여"),
        }
    }
}

/// 달력 유형
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CalendarType {
    /// 양력 (그레고리력)
    #[default]
    Solar,
    /// 음력
    Lunar {
        /// 윤달 여부
        is_leap_month: bool,
    },
}

impl CalendarType {
    pub fn is_solar(&self) -> bool {
        matches!(self, CalendarType::Solar)
    }

    pub fn is_lunar(&self) -> bool {
        matches!(self, CalendarType::Lunar { .. })
    }
}

/// 출생 정보
///
/// 모든 운명 시스템(사주, 베딕, 자미두수 등)에서 공통으로 사용됩니다.
///
/// ## 타임존 지원
///
/// IANA 타임존 데이터베이스를 사용하여 역사적 썸머타임(DST)을 자동 처리합니다.
///
/// ### 한국 썸머타임 역사
/// - 1948-1951년: 일광절약시간 시행
/// - 1955-1960년: 일광절약시간 시행
/// - 1987-1988년: 서울 올림픽 전후 일광절약시간 시행
///
/// ## 예시
/// ```rust
/// use eon_core::{BirthInfo, Location};
///
/// // 1988년 올림픽 시절 (썸머타임 적용 기간)
/// let birth = BirthInfo::solar(1988, 5, 15, 10, 0)
///     .with_timezone("Asia/Seoul");
///
/// // chrono-tz가 자동으로 +1시간(KDT) 보정
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirthInfo {
    /// 년도
    pub year: i32,
    /// 월 (1-12)
    pub month: u32,
    /// 일 (1-31)
    pub day: u32,
    /// 시 (0-23)
    pub hour: u32,
    /// 분 (0-59)
    pub minute: u32,
    /// 달력 유형 (양력/음력)
    pub calendar: CalendarType,
    /// 출생 위치 (선택적)
    pub location: Option<Location>,
    /// IANA 타임존 (예: "Asia/Seoul", "America/New_York")
    pub timezone: Option<String>,
    /// 진태양시 사용 여부 (경도 기반 보정)
    pub use_true_solar_time: bool,
    /// 성별 (선택적)
    pub gender: Option<Gender>,
}

impl BirthInfo {
    /// 양력 출생 정보 생성
    pub fn solar(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            calendar: CalendarType::Solar,
            location: None,
            timezone: None,
            use_true_solar_time: false,
            gender: None,
        }
    }

    /// 음력 출생 정보 생성
    pub fn lunar(year: i32, month: u32, day: u32, hour: u32, minute: u32, is_leap: bool) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            calendar: CalendarType::Lunar {
                is_leap_month: is_leap,
            },
            location: None,
            timezone: None,
            use_true_solar_time: false,
            gender: None,
        }
    }

    /// 위치 설정
    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    /// 타임존 설정 (IANA 형식: "Asia/Seoul", "America/New_York" 등)
    pub fn with_timezone(mut self, tz: impl Into<String>) -> Self {
        self.timezone = Some(tz.into());
        self
    }

    /// 한국 타임존 설정 (편의 함수)
    pub fn with_korea_timezone(self) -> Self {
        self.with_timezone("Asia/Seoul")
    }

    /// 진태양시 사용 설정
    pub fn with_true_solar_time(mut self, use_tst: bool) -> Self {
        self.use_true_solar_time = use_tst;
        self
    }

    /// 성별 설정
    pub fn with_gender(mut self, gender: Gender) -> Self {
        self.gender = Some(gender);
        self
    }

    /// 타임존 파싱
    fn parsed_timezone(&self) -> Option<Tz> {
        self.timezone.as_ref().and_then(|tz| tz.parse().ok())
    }

    /// 해당 시간대의 로컬 DateTime 생성
    fn local_datetime(&self) -> Option<NaiveDateTime> {
        NaiveDate::from_ymd_opt(self.year, self.month, self.day)
            .and_then(|d| d.and_hms_opt(self.hour, self.minute, 0))
    }

    /// 썸머타임(DST)이 적용된 UTC 시간 반환
    ///
    /// IANA 타임존 데이터베이스를 사용하여 역사적 DST를 자동 처리합니다.
    pub fn to_utc(&self) -> Result<DateTime<Utc>, crate::error::CoreError> {
        let naive = self.local_datetime()
            .ok_or(crate::error::CoreError::InvalidDateTime)?;

        let tz = if let Some(ref tz_str) = self.timezone {
            tz_str.parse::<chrono_tz::Tz>()
                .map_err(|_| crate::error::CoreError::InvalidTimezone(tz_str.clone()))?
        } else {
            chrono_tz::Asia::Seoul
        };

        // Handle ambiguous times (DST fall-back) by picking the earlier (DST) variant
        match tz.from_local_datetime(&naive) {
            chrono::LocalResult::Single(dt) => Ok(dt.with_timezone(&Utc)),
            chrono::LocalResult::Ambiguous(dt1, _dt2) => Ok(dt1.with_timezone(&Utc)),
            chrono::LocalResult::None => Err(crate::error::CoreError::NonExistentLocalTime),
        }
    }

    /// 썸머타임(DST) 적용 여부 확인
    pub fn is_dst(&self) -> bool {
        if let (Some(tz), Some(naive)) = (self.parsed_timezone(), self.local_datetime()) {
            let local_dt = match tz.from_local_datetime(&naive) {
                chrono::LocalResult::Single(dt) => Some(dt),
                chrono::LocalResult::Ambiguous(dt1, _) => Some(dt1),
                chrono::LocalResult::None => None,
            };
            if let Some(dt) = local_dt {
                let dst = dt.offset().dst_offset();
                return dst.num_hours() > 0;
            }
        }
        false
    }

    /// DST 오프셋 정보 반환 (시간 단위)
    ///
    /// 전체 UTC 오프셋(base + dst)을 반환합니다.
    pub fn dst_offset_hours(&self) -> Option<i32> {
        if let (Some(tz), Some(naive)) = (self.parsed_timezone(), self.local_datetime()) {
            let local_dt = match tz.from_local_datetime(&naive) {
                chrono::LocalResult::Single(dt) => Some(dt),
                chrono::LocalResult::Ambiguous(dt1, _) => Some(dt1),
                chrono::LocalResult::None => None,
            };
            if let Some(dt) = local_dt {
                let base = dt.offset().base_utc_offset();
                let dst = dt.offset().dst_offset();
                return Some((base.num_hours() + dst.num_hours()) as i32);
            }
        }
        None
    }

    /// 지역시 보정값 (분 단위) - 경도 기반
    pub fn longitude_offset_minutes(&self) -> i32 {
        if self.use_true_solar_time {
            self.location
                .as_ref()
                .map(|l| l.time_offset_minutes())
                .unwrap_or(0)
        } else {
            0
        }
    }

    /// 보정된 시간 (시, 분)
    ///
    /// 진태양시를 사용하는 경우, 경도 기반 보정이 적용됩니다.
    /// 주의: DST는 `to_utc()` 메서드에서 자동 처리됩니다.
    pub fn corrected_time(&self) -> (u32, u32) {
        let offset = self.longitude_offset_minutes();
        if offset == 0 {
            return (self.hour, self.minute);
        }

        let total_minutes =
            (self.hour as i32 * 60 + self.minute as i32 + offset).rem_euclid(24 * 60);
        let corrected_hour = (total_minutes / 60) as u32;
        let corrected_minute = (total_minutes % 60) as u32;

        (corrected_hour, corrected_minute)
    }

    /// 보정된 날짜와 시간 (경도 기반 진태양시)
    pub fn corrected_datetime(&self) -> (i32, u32, u32, u32, u32) {
        let offset = self.longitude_offset_minutes();
        if offset == 0 {
            return (self.year, self.month, self.day, self.hour, self.minute);
        }

        let dt = NaiveDate::from_ymd_opt(self.year, self.month, self.day)
            .and_then(|d| d.and_hms_opt(self.hour, self.minute, 0))
            .map(|dt| dt + Duration::minutes(offset as i64));

        match dt {
            Some(corrected) => (
                corrected.year(),
                corrected.month(),
                corrected.day(),
                corrected.hour(),
                corrected.minute(),
            ),
            None => (self.year, self.month, self.day, self.hour, self.minute),
        }
    }

    /// 사주 계산용 데이터 반환 (년, 월, 일, 시)
    ///
    /// 진태양시 보정이 적용된 값을 반환합니다.
    pub fn for_saju(&self) -> (i32, u32, u32, u32) {
        let (year, month, day, hour, _) = self.corrected_datetime();
        (year, month, day, hour)
    }
}

impl std::fmt::Display for BirthInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cal = match self.calendar {
            CalendarType::Solar => "양력",
            CalendarType::Lunar {
                is_leap_month: false,
            } => "음력",
            CalendarType::Lunar {
                is_leap_month: true,
            } => "음력(윤달)",
        };

        write!(
            f,
            "{} {}년 {}월 {}일 {:02}:{:02}",
            cal, self.year, self.month, self.day, self.hour, self.minute
        )?;

        if let Some(ref loc) = self.location {
            write!(f, " ({})", loc.name)?;
        }

        if let Some(ref tz) = self.timezone {
            write!(f, " [{}]", tz)?;
        }

        if let Some(gender) = self.gender {
            write!(f, " {}", gender)?;
        }

        if self.is_dst() {
            write!(f, " (썸머타임)")?;
        }

        if self.use_true_solar_time {
            let offset = self.longitude_offset_minutes();
            if offset != 0 {
                write!(f, " [지역시 {:+}분]", offset)?;
            }
        }

        Ok(())
    }
}

// ============================================
// 한국 주요 타임존 상수
// ============================================

/// 미리 정의된 타임존 상수
pub mod timezones {
    /// 한국 (Asia/Seoul)
    pub const KOREA: &str = "Asia/Seoul";
    /// 일본 (Asia/Tokyo)
    pub const JAPAN: &str = "Asia/Tokyo";
    /// 중국 (Asia/Shanghai)
    pub const CHINA: &str = "Asia/Shanghai";
    /// 대만 (Asia/Taipei)
    pub const TAIWAN: &str = "Asia/Taipei";
    /// 홍콩 (Asia/Hong_Kong)
    pub const HONG_KONG: &str = "Asia/Hong_Kong";
    /// 인도 (Asia/Kolkata)
    pub const INDIA: &str = "Asia/Kolkata";
    /// 미국 동부 (America/New_York)
    pub const US_EAST: &str = "America/New_York";
    /// 미국 서부 (America/Los_Angeles)
    pub const US_WEST: &str = "America/Los_Angeles";
    /// 영국 (Europe/London)
    pub const UK: &str = "Europe/London";
    /// 독일 (Europe/Berlin)
    pub const GERMANY: &str = "Europe/Berlin";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birth_info_creation() {
        let info = BirthInfo::solar(2004, 11, 27, 22, 0)
            .with_location(Location::ansan())
            .with_korea_timezone()
            .with_true_solar_time(true)
            .with_gender(Gender::Male);

        assert_eq!(info.year, 2004);
        assert_eq!(info.month, 11);
        assert_eq!(info.day, 27);
        assert_eq!(info.hour, 22);
        assert_eq!(info.gender, Some(Gender::Male));
        assert_eq!(info.timezone, Some("Asia/Seoul".to_string()));
    }

    #[test]
    fn test_to_utc_normal() {
        // 2004년 11월 27일 22:00 KST (썸머타임 아님)
        let info = BirthInfo::solar(2004, 11, 27, 22, 0).with_korea_timezone();

        let utc = info.to_utc().unwrap();
        // KST = UTC+9, 22:00 KST = 13:00 UTC
        assert_eq!(utc.hour(), 13);
    }

    #[test]
    fn test_to_utc_with_dst_1988() {
        // 1988년 5월 15일 10:00 - 서울 올림픽 전 썸머타임 기간
        let info = BirthInfo::solar(1988, 5, 15, 10, 0).with_korea_timezone();

        let utc = info.to_utc().unwrap();
        // KDT(썸머타임) = UTC+10, 10:00 KDT = 00:00 UTC
        assert_eq!(utc.hour(), 0);

        // DST 확인
        assert!(info.is_dst());
        assert_eq!(info.dst_offset_hours(), Some(10));
    }

    #[test]
    fn test_no_dst_winter() {
        // 2004년 1월 (겨울, 썸머타임 없음)
        let info = BirthInfo::solar(2004, 1, 15, 10, 0).with_korea_timezone();

        assert!(!info.is_dst());
        assert_eq!(info.dst_offset_hours(), Some(9));
    }

    #[test]
    fn test_corrected_time_with_location() {
        // 안산, 22:00 → 21:27 (-33분)
        let info = BirthInfo::solar(2004, 11, 27, 22, 0)
            .with_location(Location::ansan())
            .with_true_solar_time(true);

        let (hour, minute) = info.corrected_time();
        assert_eq!(hour, 21);
        assert_eq!(minute, 27);
    }

    #[test]
    fn test_display_with_dst() {
        let info = BirthInfo::solar(1988, 5, 15, 10, 0)
            .with_korea_timezone()
            .with_gender(Gender::Male);

        let display = format!("{}", info);
        assert!(display.contains("썸머타임"));
    }
}
