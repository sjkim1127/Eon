//! 출생 정보 (BirthInfo)
//!
//! 모든 운명 시스템에서 공통으로 사용되는 출생 정보입니다.

use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};
use serde::{Deserialize, Serialize};
use crate::location::Location;

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
    /// 진태양시 사용 여부
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
            calendar: CalendarType::Lunar { is_leap_month: is_leap },
            location: None,
            use_true_solar_time: false,
            gender: None,
        }
    }

    /// 위치 설정
    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
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

    /// 지역시 보정이 적용된 시간 반환 (분 단위 오프셋)
    /// 
    /// 진태양시를 사용하는 경우, 위치 기반 보정을 적용합니다.
    pub fn time_offset_minutes(&self) -> i32 {
        if self.use_true_solar_time {
            self.location.as_ref().map(|l| l.time_offset_minutes()).unwrap_or(0)
        } else {
            0
        }
    }

    /// 보정된 시간 (시, 분)
    /// 
    /// 진태양시를 사용하는 경우, 지역시 보정이 적용된 시간을 반환합니다.
    pub fn corrected_time(&self) -> (u32, u32) {
        let offset = self.time_offset_minutes();
        if offset == 0 {
            return (self.hour, self.minute);
        }

        // 분 단위로 변환 후 보정
        let total_minutes = (self.hour as i32 * 60 + self.minute as i32 + offset).rem_euclid(24 * 60);
        let corrected_hour = (total_minutes / 60) as u32;
        let corrected_minute = (total_minutes % 60) as u32;

        (corrected_hour, corrected_minute)
    }

    /// 보정된 날짜와 시간
    /// 
    /// 지역시 보정으로 인해 날짜가 바뀌는 경우를 처리합니다.
    pub fn corrected_datetime(&self) -> (i32, u32, u32, u32, u32) {
        let offset = self.time_offset_minutes();
        if offset == 0 {
            return (self.year, self.month, self.day, self.hour, self.minute);
        }

        // NaiveDateTime을 사용하여 날짜 경계 처리
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
            CalendarType::Lunar { is_leap_month: false } => "음력",
            CalendarType::Lunar { is_leap_month: true } => "음력(윤달)",
        };
        
        write!(f, "{} {}년 {}월 {}일 {:02}:{:02}", 
            cal, self.year, self.month, self.day, self.hour, self.minute)?;
        
        if let Some(ref loc) = self.location {
            write!(f, " ({})", loc.name)?;
        }
        
        if let Some(gender) = self.gender {
            write!(f, " {}", gender)?;
        }
        
        if self.use_true_solar_time {
            let offset = self.time_offset_minutes();
            if offset != 0 {
                write!(f, " [지역시 {:+}분]", offset)?;
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birth_info_creation() {
        let info = BirthInfo::solar(2004, 11, 27, 22, 0)
            .with_location(Location::ansan())
            .with_true_solar_time(true)
            .with_gender(Gender::Male);

        assert_eq!(info.year, 2004);
        assert_eq!(info.month, 11);
        assert_eq!(info.day, 27);
        assert_eq!(info.hour, 22);
        assert_eq!(info.gender, Some(Gender::Male));
        assert_eq!(info.time_offset_minutes(), -33);
    }

    #[test]
    fn test_corrected_time() {
        // 안산, 22:00 → 21:27 (-33분)
        let info = BirthInfo::solar(2004, 11, 27, 22, 0)
            .with_location(Location::ansan())
            .with_true_solar_time(true);

        let (hour, minute) = info.corrected_time();
        assert_eq!(hour, 21);
        assert_eq!(minute, 27);
    }

    #[test]
    fn test_corrected_datetime_day_change() {
        // 00:30에 -33분 보정 → 전날 23:57
        let info = BirthInfo::solar(2004, 11, 27, 0, 30)
            .with_location(Location::ansan())
            .with_true_solar_time(true);

        let (year, month, day, hour, minute) = info.corrected_datetime();
        assert_eq!(day, 26); // 날짜가 바뀜
        assert_eq!(hour, 23);
        assert_eq!(minute, 57);
    }

    #[test]
    fn test_no_correction_without_flag() {
        let info = BirthInfo::solar(2004, 11, 27, 22, 0)
            .with_location(Location::ansan())
            .with_true_solar_time(false);

        // 진태양시 미사용시 보정 없음
        assert_eq!(info.time_offset_minutes(), 0);
        assert_eq!(info.corrected_time(), (22, 0));
    }
}
