//! 위치(Location) 및 지역시 보정
//!
//! 모든 운명 시스템에서 공통으로 사용되는 위치 정보와
//! 진태양시(True Solar Time) 계산을 제공합니다.

use serde::{Deserialize, Serialize};

/// 지리적 위치
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// 도시/지역 이름
    pub name: String,
    /// 위도 (북위 +, 남위 -)
    pub latitude: f64,
    /// 경도 (동경 +, 서경 -)
    pub longitude: f64,
    /// 표준시 기준 경도 (예: 한국 = 135.0, 중국 = 120.0)
    pub standard_longitude: f64,
}

impl Location {
    /// 새 위치 생성
    pub fn new(
        name: impl Into<String>,
        latitude: f64,
        longitude: f64,
        standard_longitude: f64,
    ) -> Self {
        Self {
            name: name.into(),
            latitude,
            longitude,
            standard_longitude,
        }
    }

    /// 한국 도시 생성 (표준경도 135도)
    pub fn korea(name: impl Into<String>, latitude: f64, longitude: f64) -> Self {
        Self::new(name, latitude, longitude, 135.0)
    }

    /// 중국 도시 생성 (표준경도 120도)
    pub fn china(name: impl Into<String>, latitude: f64, longitude: f64) -> Self {
        Self::new(name, latitude, longitude, 120.0)
    }

    /// 지역시 보정값 계산 (분 단위)
    ///
    /// 표준시와 진태양시의 차이를 계산합니다.
    /// 양수: 표준시보다 빠름 (동쪽)
    /// 음수: 표준시보다 느림 (서쪽)
    ///
    /// # 공식
    /// 보정값(분) = (실제경도 - 표준경도) × 4분/도
    ///
    /// # 예시
    /// - 서울(126.98°): (126.98 - 135) × 4 = -32분
    /// - 안산(126.83°): (126.83 - 135) × 4 = -33분
    pub fn time_offset_minutes(&self) -> i32 {
        let diff = self.longitude - self.standard_longitude;
        (diff * 4.0).round() as i32
    }

    /// 진태양시로 변환된 시간 오프셋(초 단위)
    pub fn time_offset_seconds(&self) -> i32 {
        self.time_offset_minutes() * 60
    }
}

// ============================================
// 한국 주요 도시 상수
// ============================================

impl Location {
    /// 서울
    pub fn seoul() -> Self {
        Self::korea("서울", 37.5665, 126.9780)
    }

    /// 안산
    pub fn ansan() -> Self {
        Self::korea("안산", 37.3219, 126.8309)
    }

    /// 인천
    pub fn incheon() -> Self {
        Self::korea("인천", 37.4563, 126.7052)
    }

    /// 부산
    pub fn busan() -> Self {
        Self::korea("부산", 35.1796, 129.0756)
    }

    /// 대구
    pub fn daegu() -> Self {
        Self::korea("대구", 35.8714, 128.6014)
    }

    /// 대전
    pub fn daejeon() -> Self {
        Self::korea("대전", 36.3504, 127.3845)
    }

    /// 광주
    pub fn gwangju() -> Self {
        Self::korea("광주", 35.1595, 126.8526)
    }

    /// 제주
    pub fn jeju() -> Self {
        Self::korea("제주", 33.4996, 126.5312)
    }

    /// 울산
    pub fn ulsan() -> Self {
        Self::korea("울산", 35.5384, 129.3114)
    }

    /// 수원
    pub fn suwon() -> Self {
        Self::korea("수원", 37.2636, 127.0286)
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({:.2}°N, {:.2}°E)",
            self.name, self.latitude, self.longitude
        )
    }
}

impl Default for Location {
    /// 기본값: 서울
    fn default() -> Self {
        Self::seoul()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seoul_offset() {
        let seoul = Location::seoul();
        // 서울: (126.98 - 135) × 4 ≈ -32분
        assert_eq!(seoul.time_offset_minutes(), -32);
    }

    #[test]
    fn test_ansan_offset() {
        let ansan = Location::ansan();
        // 안산: (126.83 - 135) × 4 ≈ -33분
        assert_eq!(ansan.time_offset_minutes(), -33);
    }

    #[test]
    fn test_busan_offset() {
        let busan = Location::busan();
        // 부산: (129.08 - 135) × 4 ≈ -24분
        assert_eq!(busan.time_offset_minutes(), -24);
    }

    #[test]
    fn test_custom_location() {
        // 도쿄 (표준경도 135도로 거의 정확)
        let tokyo = Location::new("도쿄", 35.6762, 139.6503, 135.0);
        // (139.65 - 135) × 4 ≈ +19분
        assert_eq!(tokyo.time_offset_minutes(), 19);
    }
}
