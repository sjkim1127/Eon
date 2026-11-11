//! Geographic location, True Solar Time (TST), and offline city database

use serde::{Deserialize, Serialize};

/// Geographic Location
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// City / region name
    pub name: String,
    /// Latitude (North +, South -)
    pub latitude: f64,
    /// Longitude (East +, West -)
    pub longitude: f64,
    /// Standard timezone meridian longitude (e.g. Korea = 135.0, China = 120.0, Eastern US = -75.0)
    pub standard_longitude: f64,
}

/// Detailed offline city location and timezone entry
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CityInfo {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: &'static str,
    pub country: &'static str,
}

impl CityInfo {
    pub fn to_location(&self) -> Location {
        let std_meridian = crate::timezone::standard_meridian_from_tz(self.timezone);
        Location::new(self.name, self.latitude, self.longitude, std_meridian)
    }

    /// True Solar Time offset in minutes from the standard timezone meridian
    pub fn time_offset_minutes(&self) -> i32 {
        let std_meridian = crate::timezone::standard_meridian_from_tz(self.timezone);
        ((self.longitude - std_meridian) * 4.0).round() as i32
    }
}

pub static BUILTIN_CITIES: &[CityInfo] = &[
    // South Korea (All major metropolitan cities & provinces)
    CityInfo {
        name: "Seoul",
        aliases: &["seoul", "서울", "서울특별시", "seoul-si", "sel"],
        latitude: 37.5665,
        longitude: 126.9780,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Busan",
        aliases: &["busan", "부산", "부산광역시", "pusan"],
        latitude: 35.1796,
        longitude: 129.0756,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Incheon",
        aliases: &["incheon", "인천", "인천광역시"],
        latitude: 37.4563,
        longitude: 126.7052,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Daegu",
        aliases: &["daegu", "대구", "대구광역시", "taegu"],
        latitude: 35.8714,
        longitude: 128.6014,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Daejeon",
        aliases: &["daejeon", "대전", "대전광역시", "taejon"],
        latitude: 36.3504,
        longitude: 127.3845,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Gwangju",
        aliases: &["gwangju", "광주", "광주광역시", "kwangju"],
        latitude: 35.1595,
        longitude: 126.8526,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Ulsan",
        aliases: &["ulsan", "울산", "울산광역시"],
        latitude: 35.5384,
        longitude: 129.3114,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Suwon",
        aliases: &["suwon", "수원", "수원시"],
        latitude: 37.2636,
        longitude: 127.0286,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Changwon",
        aliases: &[
            "changwon",
            "창원",
            "창원시",
            "masan",
            "마산",
            "jinhae",
            "진해",
        ],
        latitude: 35.2281,
        longitude: 128.6811,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Seongnam",
        aliases: &[
            "seongnam",
            "성남",
            "성남시",
            "bundang",
            "분당",
            "pangyo",
            "판교",
        ],
        latitude: 37.4201,
        longitude: 127.1265,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Goyang",
        aliases: &["goyang", "고양", "고양시", "ilsan", "일산"],
        latitude: 37.6584,
        longitude: 126.8320,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Yongin",
        aliases: &["yongin", "용인", "용인시", "suji", "수지"],
        latitude: 37.2411,
        longitude: 127.1776,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Bucheon",
        aliases: &["bucheon", "부천", "부천시"],
        latitude: 37.5034,
        longitude: 126.7660,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Ansan",
        aliases: &["ansan", "안산", "안산시"],
        latitude: 37.3219,
        longitude: 126.8309,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Cheongju",
        aliases: &["cheongju", "청주", "청주시"],
        latitude: 36.6424,
        longitude: 127.4890,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Jeonju",
        aliases: &["jeonju", "전주", "전주시"],
        latitude: 35.8242,
        longitude: 127.1480,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Cheonan",
        aliases: &["cheonan", "천안", "천안시"],
        latitude: 36.8151,
        longitude: 127.1139,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Pohang",
        aliases: &["pohang", "포항", "포항시"],
        latitude: 36.0190,
        longitude: 129.3435,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Gimhae",
        aliases: &["gimhae", "김해", "김해시"],
        latitude: 35.2285,
        longitude: 128.8894,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Jeju",
        aliases: &["jeju", "제주", "제주시", "제주도", "seogwipo", "서귀포"],
        latitude: 33.4996,
        longitude: 126.5312,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Chuncheon",
        aliases: &["chuncheon", "춘천", "춘천시"],
        latitude: 37.8813,
        longitude: 127.7298,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Wonju",
        aliases: &["wonju", "원주", "원주시"],
        latitude: 37.3422,
        longitude: 127.9202,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Gangneung",
        aliases: &["gangneung", "강릉", "강릉시"],
        latitude: 37.7519,
        longitude: 128.8761,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Sejong",
        aliases: &["sejong", "세종", "세종특별자치시"],
        latitude: 36.4800,
        longitude: 127.2890,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Pyeongyang",
        aliases: &["pyongyang", "평양", "pyeongyang"],
        latitude: 39.0392,
        longitude: 125.7625,
        timezone: "Asia/Pyongyang",
        country: "North Korea",
    },
    // Asia
    CityInfo {
        name: "Tokyo",
        aliases: &["tokyo", "도쿄", "동경"],
        latitude: 35.6762,
        longitude: 139.6503,
        timezone: "Asia/Tokyo",
        country: "Japan",
    },
    CityInfo {
        name: "Osaka",
        aliases: &["osaka", "오사카"],
        latitude: 34.6937,
        longitude: 135.5023,
        timezone: "Asia/Tokyo",
        country: "Japan",
    },
    CityInfo {
        name: "Kyoto",
        aliases: &["kyoto", "교토"],
        latitude: 35.0116,
        longitude: 135.7681,
        timezone: "Asia/Tokyo",
        country: "Japan",
    },
    CityInfo {
        name: "Beijing",
        aliases: &["beijing", "베이징", "북경", "peking"],
        latitude: 39.9042,
        longitude: 116.4074,
        timezone: "Asia/Shanghai",
        country: "China",
    },
    CityInfo {
        name: "Shanghai",
        aliases: &["shanghai", "상하이", "상해"],
        latitude: 31.2304,
        longitude: 121.4737,
        timezone: "Asia/Shanghai",
        country: "China",
    },
    CityInfo {
        name: "Guangzhou",
        aliases: &["guangzhou", "광저우", "광주(중국)", "canton"],
        latitude: 23.1291,
        longitude: 113.2644,
        timezone: "Asia/Shanghai",
        country: "China",
    },
    CityInfo {
        name: "Shenzhen",
        aliases: &["shenzhen", "선전", "심천"],
        latitude: 22.5431,
        longitude: 114.0579,
        timezone: "Asia/Shanghai",
        country: "China",
    },
    CityInfo {
        name: "Hong Kong",
        aliases: &["hong kong", "hongkong", "홍콩", "hk"],
        latitude: 22.3193,
        longitude: 114.1694,
        timezone: "Asia/Hong_Kong",
        country: "Hong Kong",
    },
    CityInfo {
        name: "Taipei",
        aliases: &["taipei", "타이베이", "대만", "taiwan"],
        latitude: 25.0330,
        longitude: 121.5654,
        timezone: "Asia/Taipei",
        country: "Taiwan",
    },
    CityInfo {
        name: "Singapore",
        aliases: &["singapore", "싱가포르", "싱가폴", "sg"],
        latitude: 1.3521,
        longitude: 103.8198,
        timezone: "Asia/Singapore",
        country: "Singapore",
    },
    CityInfo {
        name: "Bangkok",
        aliases: &["bangkok", "방콕"],
        latitude: 13.7563,
        longitude: 100.5018,
        timezone: "Asia/Bangkok",
        country: "Thailand",
    },
    CityInfo {
        name: "Hanoi",
        aliases: &["hanoi", "하노이"],
        latitude: 21.0285,
        longitude: 105.8542,
        timezone: "Asia/Ho_Chi_Minh",
        country: "Vietnam",
    },
    CityInfo {
        name: "Ho Chi Minh City",
        aliases: &["ho chi minh", "호치민", "saigon", "사이공"],
        latitude: 10.8231,
        longitude: 106.6297,
        timezone: "Asia/Ho_Chi_Minh",
        country: "Vietnam",
    },
    CityInfo {
        name: "Jakarta",
        aliases: &["jakarta", "자카르타"],
        latitude: -6.2088,
        longitude: 106.8456,
        timezone: "Asia/Jakarta",
        country: "Indonesia",
    },
    CityInfo {
        name: "Kuala Lumpur",
        aliases: &["kuala lumpur", "쿠알라룸푸르", "kl"],
        latitude: 3.1390,
        longitude: 101.6869,
        timezone: "Asia/Kuala_Lumpur",
        country: "Malaysia",
    },
    CityInfo {
        name: "Manila",
        aliases: &["manila", "마닐라"],
        latitude: 14.5995,
        longitude: 120.9842,
        timezone: "Asia/Manila",
        country: "Philippines",
    },
    CityInfo {
        name: "New Delhi",
        aliases: &["new delhi", "delhi", "뉴델리", "델리"],
        latitude: 28.6139,
        longitude: 77.2090,
        timezone: "Asia/Kolkata",
        country: "India",
    },
    CityInfo {
        name: "Mumbai",
        aliases: &["mumbai", "뭄바이", "bombay", "봄베이"],
        latitude: 19.0760,
        longitude: 72.8777,
        timezone: "Asia/Kolkata",
        country: "India",
    },
    // Middle East
    CityInfo {
        name: "Dubai",
        aliases: &["dubai", "두바이"],
        latitude: 25.2048,
        longitude: 55.2708,
        timezone: "Asia/Dubai",
        country: "United Arab Emirates",
    },
    CityInfo {
        name: "Riyadh",
        aliases: &["riyadh", "리야드"],
        latitude: 24.7136,
        longitude: 46.6753,
        timezone: "Asia/Riyadh",
        country: "Saudi Arabia",
    },
    CityInfo {
        name: "Istanbul",
        aliases: &["istanbul", "이스탄불"],
        latitude: 41.0082,
        longitude: 28.9784,
        timezone: "Europe/Istanbul",
        country: "Turkey",
    },
    // Europe
    CityInfo {
        name: "London",
        aliases: &["london", "런던"],
        latitude: 51.5074,
        longitude: -0.1278,
        timezone: "Europe/London",
        country: "United Kingdom",
    },
    CityInfo {
        name: "Paris",
        aliases: &["paris", "파리"],
        latitude: 48.8566,
        longitude: 2.3522,
        timezone: "Europe/Paris",
        country: "France",
    },
    CityInfo {
        name: "Berlin",
        aliases: &["berlin", "베를린"],
        latitude: 52.5200,
        longitude: 13.4050,
        timezone: "Europe/Berlin",
        country: "Germany",
    },
    CityInfo {
        name: "Rome",
        aliases: &["rome", "로마"],
        latitude: 41.9028,
        longitude: 12.4964,
        timezone: "Europe/Rome",
        country: "Italy",
    },
    CityInfo {
        name: "Madrid",
        aliases: &["madrid", "마드리드"],
        latitude: 40.4168,
        longitude: -3.7038,
        timezone: "Europe/Madrid",
        country: "Spain",
    },
    CityInfo {
        name: "Amsterdam",
        aliases: &["amsterdam", "암스테르담"],
        latitude: 52.3676,
        longitude: 4.9041,
        timezone: "Europe/Amsterdam",
        country: "Netherlands",
    },
    CityInfo {
        name: "Zurich",
        aliases: &["zurich", "취리히"],
        latitude: 47.3769,
        longitude: 8.5417,
        timezone: "Europe/Zurich",
        country: "Switzerland",
    },
    CityInfo {
        name: "Vienna",
        aliases: &["vienna", "비엔나", "wien"],
        latitude: 48.2082,
        longitude: 16.3738,
        timezone: "Europe/Vienna",
        country: "Austria",
    },
    // North America
    CityInfo {
        name: "New York",
        aliases: &["new york", "new york city", "nyc", "뉴욕"],
        latitude: 40.7128,
        longitude: -74.0060,
        timezone: "America/New_York",
        country: "United States",
    },
    CityInfo {
        name: "Los Angeles",
        aliases: &["los angeles", "la", "로스앤젤레스", "엘에이"],
        latitude: 34.0522,
        longitude: -118.2437,
        timezone: "America/Los_Angeles",
        country: "United States",
    },
    CityInfo {
        name: "Chicago",
        aliases: &["chicago", "시카고"],
        latitude: 41.8781,
        longitude: -87.6298,
        timezone: "America/Chicago",
        country: "United States",
    },
    CityInfo {
        name: "San Francisco",
        aliases: &["san francisco", "sf", "샌프란시스코", "bay area"],
        latitude: 37.7749,
        longitude: -122.4194,
        timezone: "America/Los_Angeles",
        country: "United States",
    },
    CityInfo {
        name: "Seattle",
        aliases: &["seattle", "시애틀"],
        latitude: 47.6062,
        longitude: -122.3321,
        timezone: "America/Los_Angeles",
        country: "United States",
    },
    CityInfo {
        name: "Boston",
        aliases: &["boston", "보스턴"],
        latitude: 42.3601,
        longitude: -71.0589,
        timezone: "America/New_York",
        country: "United States",
    },
    CityInfo {
        name: "Washington DC",
        aliases: &["washington", "washington dc", "dc", "워싱턴"],
        latitude: 38.9072,
        longitude: -77.0369,
        timezone: "America/New_York",
        country: "United States",
    },
    CityInfo {
        name: "Atlanta",
        aliases: &["atlanta", "애틀랜타"],
        latitude: 33.7490,
        longitude: -84.3880,
        timezone: "America/New_York",
        country: "United States",
    },
    CityInfo {
        name: "Dallas",
        aliases: &["dallas", "달라스", "dfw"],
        latitude: 32.7767,
        longitude: -96.7970,
        timezone: "America/Chicago",
        country: "United States",
    },
    CityInfo {
        name: "Houston",
        aliases: &["houston", "휴스턴"],
        latitude: 29.7604,
        longitude: -95.3698,
        timezone: "America/Chicago",
        country: "United States",
    },
    CityInfo {
        name: "Miami",
        aliases: &["miami", "마이애미"],
        latitude: 25.7617,
        longitude: -80.1918,
        timezone: "America/New_York",
        country: "United States",
    },
    CityInfo {
        name: "Toronto",
        aliases: &["toronto", "토론토"],
        latitude: 43.6532,
        longitude: -79.3832,
        timezone: "America/Toronto",
        country: "Canada",
    },
    CityInfo {
        name: "Vancouver",
        aliases: &["vancouver", "밴쿠버"],
        latitude: 49.2827,
        longitude: -123.1207,
        timezone: "America/Vancouver",
        country: "Canada",
    },
    // Oceania & South America
    CityInfo {
        name: "Sydney",
        aliases: &["sydney", "시드니"],
        latitude: -33.8688,
        longitude: 151.2093,
        timezone: "Australia/Sydney",
        country: "Australia",
    },
    CityInfo {
        name: "Melbourne",
        aliases: &["melbourne", "멜버른"],
        latitude: -37.8136,
        longitude: 144.9631,
        timezone: "Australia/Melbourne",
        country: "Australia",
    },
    CityInfo {
        name: "Auckland",
        aliases: &["auckland", "오클랜드"],
        latitude: -36.8485,
        longitude: 174.7633,
        timezone: "Pacific/Auckland",
        country: "New Zealand",
    },
    CityInfo {
        name: "São Paulo",
        aliases: &["sao paulo", "são paulo", "상파울루"],
        latitude: -23.5505,
        longitude: -46.6333,
        timezone: "America/Sao_Paulo",
        country: "Brazil",
    },
    CityInfo {
        name: "Buenos Aires",
        aliases: &["buenos aires", "부에노스아이레스"],
        latitude: -34.6037,
        longitude: -58.3816,
        timezone: "America/Argentina/Buenos_Aires",
        country: "Argentina",
    },
];

/// Looks up a city in the built-in offline city database by name or alias (case-insensitive)
pub fn lookup_city(query: &str) -> Option<&'static CityInfo> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return None;
    }

    // 1. Exact match on name or alias
    if let Some(c) = BUILTIN_CITIES
        .iter()
        .find(|c| c.name.to_lowercase() == q || c.aliases.iter().any(|&a| a == q))
    {
        return Some(c);
    }

    // 2. Substring match (only for queries >= 4 chars to avoid false positives like "la")
    if q.len() >= 4 {
        BUILTIN_CITIES.iter().find(|c| {
            c.name.to_lowercase().contains(&q) || c.aliases.iter().any(|&a| a.contains(&q))
        })
    } else {
        None
    }
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
    /// Resolve location from city name using the built-in offline city database
    pub fn from_city(query: &str) -> Option<Self> {
        lookup_city(query).map(|c| c.to_location())
    }

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

    #[test]
    fn test_lookup_city() {
        let seoul = lookup_city("seoul").unwrap();
        assert_eq!(seoul.name, "Seoul");
        assert_eq!(seoul.timezone, "Asia/Seoul");
        assert!((seoul.latitude - 37.5665).abs() < 0.001);

        let busan_kr = lookup_city("부산").unwrap();
        assert_eq!(busan_kr.name, "Busan");

        let tokyo_kr = lookup_city("도쿄").unwrap();
        assert_eq!(tokyo_kr.name, "Tokyo");
        assert_eq!(tokyo_kr.timezone, "Asia/Tokyo");

        let ny = lookup_city("nyc").unwrap();
        assert_eq!(ny.name, "New York");
        assert_eq!(ny.timezone, "America/New_York");

        let la = lookup_city("LA").unwrap();
        assert_eq!(la.name, "Los Angeles");
        assert_eq!(la.timezone, "America/Los_Angeles");

        let london = lookup_city("London").unwrap();
        assert_eq!(london.name, "London");
        assert_eq!(london.timezone, "Europe/London");

        assert!(lookup_city("NonExistentCity12345").is_none());
    }
}
