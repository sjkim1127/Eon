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
        aliases: &["seoul", "seoul-si", "sel"],
        latitude: 37.5665,
        longitude: 126.9780,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Busan",
        aliases: &["busan", "pusan"],
        latitude: 35.1796,
        longitude: 129.0756,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Incheon",
        aliases: &["incheon"],
        latitude: 37.4563,
        longitude: 126.7052,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Daegu",
        aliases: &["daegu", "taegu"],
        latitude: 35.8714,
        longitude: 128.6014,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Daejeon",
        aliases: &["daejeon", "taejon"],
        latitude: 36.3504,
        longitude: 127.3845,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Gwangju",
        aliases: &["gwangju", "kwangju"],
        latitude: 35.1595,
        longitude: 126.8526,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Ulsan",
        aliases: &["ulsan"],
        latitude: 35.5384,
        longitude: 129.3114,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Suwon",
        aliases: &["suwon"],
        latitude: 37.2636,
        longitude: 127.0286,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Changwon",
        aliases: &["changwon", "masan", "jinhae"],
        latitude: 35.2281,
        longitude: 128.6811,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Seongnam",
        aliases: &["seongnam", "bundang", "pangyo"],
        latitude: 37.4201,
        longitude: 127.1265,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Goyang",
        aliases: &["goyang", "ilsan"],
        latitude: 37.6584,
        longitude: 126.8320,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Yongin",
        aliases: &["yongin", "suji"],
        latitude: 37.2411,
        longitude: 127.1776,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Bucheon",
        aliases: &["bucheon"],
        latitude: 37.5034,
        longitude: 126.7660,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Ansan",
        aliases: &["ansan"],
        latitude: 37.3219,
        longitude: 126.8309,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Cheongju",
        aliases: &["cheongju"],
        latitude: 36.6424,
        longitude: 127.4890,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Jeonju",
        aliases: &["jeonju"],
        latitude: 35.8242,
        longitude: 127.1480,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Cheonan",
        aliases: &["cheonan"],
        latitude: 36.8151,
        longitude: 127.1139,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Pohang",
        aliases: &["pohang"],
        latitude: 36.0190,
        longitude: 129.3435,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Gimhae",
        aliases: &["gimhae"],
        latitude: 35.2285,
        longitude: 128.8894,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Jeju",
        aliases: &["jeju", "seogwipo"],
        latitude: 33.4996,
        longitude: 126.5312,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Chuncheon",
        aliases: &["chuncheon"],
        latitude: 37.8813,
        longitude: 127.7298,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Wonju",
        aliases: &["wonju"],
        latitude: 37.3422,
        longitude: 127.9202,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Gangneung",
        aliases: &["gangneung"],
        latitude: 37.7519,
        longitude: 128.8761,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Sejong",
        aliases: &["sejong"],
        latitude: 36.4800,
        longitude: 127.2890,
        timezone: "Asia/Seoul",
        country: "South Korea",
    },
    CityInfo {
        name: "Pyeongyang",
        aliases: &["pyongyang", "pyeongyang"],
        latitude: 39.0392,
        longitude: 125.7625,
        timezone: "Asia/Pyongyang",
        country: "North Korea",
    },
    // Asia
    CityInfo {
        name: "Tokyo",
        aliases: &["tokyo"],
        latitude: 35.6762,
        longitude: 139.6503,
        timezone: "Asia/Tokyo",
        country: "Japan",
    },
    CityInfo {
        name: "Osaka",
        aliases: &["osaka"],
        latitude: 34.6937,
        longitude: 135.5023,
        timezone: "Asia/Tokyo",
        country: "Japan",
    },
    CityInfo {
        name: "Kyoto",
        aliases: &["kyoto"],
        latitude: 35.0116,
        longitude: 135.7681,
        timezone: "Asia/Tokyo",
        country: "Japan",
    },
    CityInfo {
        name: "Beijing",
        aliases: &["beijing", "peking"],
        latitude: 39.9042,
        longitude: 116.4074,
        timezone: "Asia/Shanghai",
        country: "China",
    },
    CityInfo {
        name: "Shanghai",
        aliases: &["shanghai"],
        latitude: 31.2304,
        longitude: 121.4737,
        timezone: "Asia/Shanghai",
        country: "China",
    },
    CityInfo {
        name: "Guangzhou",
        aliases: &["guangzhou", "canton"],
        latitude: 23.1291,
        longitude: 113.2644,
        timezone: "Asia/Shanghai",
        country: "China",
    },
    CityInfo {
        name: "Shenzhen",
        aliases: &["shenzhen"],
        latitude: 22.5431,
        longitude: 114.0579,
        timezone: "Asia/Shanghai",
        country: "China",
    },
    CityInfo {
        name: "Hong Kong",
        aliases: &["hong kong", "hongkong", "hk"],
        latitude: 22.3193,
        longitude: 114.1694,
        timezone: "Asia/Hong_Kong",
        country: "Hong Kong",
    },
    CityInfo {
        name: "Taipei",
        aliases: &["taipei", "taiwan"],
        latitude: 25.0330,
        longitude: 121.5654,
        timezone: "Asia/Taipei",
        country: "Taiwan",
    },
    CityInfo {
        name: "Singapore",
        aliases: &["singapore", "sg"],
        latitude: 1.3521,
        longitude: 103.8198,
        timezone: "Asia/Singapore",
        country: "Singapore",
    },
    CityInfo {
        name: "Bangkok",
        aliases: &["bangkok"],
        latitude: 13.7563,
        longitude: 100.5018,
        timezone: "Asia/Bangkok",
        country: "Thailand",
    },
    CityInfo {
        name: "Hanoi",
        aliases: &["hanoi"],
        latitude: 21.0285,
        longitude: 105.8542,
        timezone: "Asia/Ho_Chi_Minh",
        country: "Vietnam",
    },
    CityInfo {
        name: "Ho Chi Minh City",
        aliases: &["ho chi minh", "saigon"],
        latitude: 10.8231,
        longitude: 106.6297,
        timezone: "Asia/Ho_Chi_Minh",
        country: "Vietnam",
    },
    CityInfo {
        name: "Jakarta",
        aliases: &["jakarta"],
        latitude: -6.2088,
        longitude: 106.8456,
        timezone: "Asia/Jakarta",
        country: "Indonesia",
    },
    CityInfo {
        name: "Kuala Lumpur",
        aliases: &["kuala lumpur", "kl"],
        latitude: 3.1390,
        longitude: 101.6869,
        timezone: "Asia/Kuala_Lumpur",
        country: "Malaysia",
    },
    CityInfo {
        name: "Manila",
        aliases: &["manila"],
        latitude: 14.5995,
        longitude: 120.9842,
        timezone: "Asia/Manila",
        country: "Philippines",
    },
    CityInfo {
        name: "New Delhi",
        aliases: &["new delhi", "delhi"],
        latitude: 28.6139,
        longitude: 77.2090,
        timezone: "Asia/Kolkata",
        country: "India",
    },
    CityInfo {
        name: "Mumbai",
        aliases: &["mumbai", "bombay"],
        latitude: 19.0760,
        longitude: 72.8777,
        timezone: "Asia/Kolkata",
        country: "India",
    },
    // Middle East
    CityInfo {
        name: "Dubai",
        aliases: &["dubai"],
        latitude: 25.2048,
        longitude: 55.2708,
        timezone: "Asia/Dubai",
        country: "United Arab Emirates",
    },
    CityInfo {
        name: "Riyadh",
        aliases: &["riyadh"],
        latitude: 24.7136,
        longitude: 46.6753,
        timezone: "Asia/Riyadh",
        country: "Saudi Arabia",
    },
    CityInfo {
        name: "Istanbul",
        aliases: &["istanbul"],
        latitude: 41.0082,
        longitude: 28.9784,
        timezone: "Europe/Istanbul",
        country: "Turkey",
    },
    // Europe
    CityInfo {
        name: "London",
        aliases: &["london"],
        latitude: 51.5074,
        longitude: -0.1278,
        timezone: "Europe/London",
        country: "United Kingdom",
    },
    CityInfo {
        name: "Paris",
        aliases: &["paris"],
        latitude: 48.8566,
        longitude: 2.3522,
        timezone: "Europe/Paris",
        country: "France",
    },
    CityInfo {
        name: "Berlin",
        aliases: &["berlin"],
        latitude: 52.5200,
        longitude: 13.4050,
        timezone: "Europe/Berlin",
        country: "Germany",
    },
    CityInfo {
        name: "Rome",
        aliases: &["rome"],
        latitude: 41.9028,
        longitude: 12.4964,
        timezone: "Europe/Rome",
        country: "Italy",
    },
    CityInfo {
        name: "Madrid",
        aliases: &["madrid"],
        latitude: 40.4168,
        longitude: -3.7038,
        timezone: "Europe/Madrid",
        country: "Spain",
    },
    CityInfo {
        name: "Amsterdam",
        aliases: &["amsterdam"],
        latitude: 52.3676,
        longitude: 4.9041,
        timezone: "Europe/Amsterdam",
        country: "Netherlands",
    },
    CityInfo {
        name: "Zurich",
        aliases: &["zurich"],
        latitude: 47.3769,
        longitude: 8.5417,
        timezone: "Europe/Zurich",
        country: "Switzerland",
    },
    CityInfo {
        name: "Vienna",
        aliases: &["vienna", "wien"],
        latitude: 48.2082,
        longitude: 16.3738,
        timezone: "Europe/Vienna",
        country: "Austria",
    },
    // North America
    CityInfo {
        name: "New York",
        aliases: &["new york", "new york city", "nyc"],
        latitude: 40.7128,
        longitude: -74.0060,
        timezone: "America/New_York",
        country: "United States",
    },
    CityInfo {
        name: "Los Angeles",
        aliases: &["los angeles", "la"],
        latitude: 34.0522,
        longitude: -118.2437,
        timezone: "America/Los_Angeles",
        country: "United States",
    },
    CityInfo {
        name: "Chicago",
        aliases: &["chicago"],
        latitude: 41.8781,
        longitude: -87.6298,
        timezone: "America/Chicago",
        country: "United States",
    },
    CityInfo {
        name: "San Francisco",
        aliases: &["san francisco", "sf", "bay area"],
        latitude: 37.7749,
        longitude: -122.4194,
        timezone: "America/Los_Angeles",
        country: "United States",
    },
    CityInfo {
        name: "Seattle",
        aliases: &["seattle"],
        latitude: 47.6062,
        longitude: -122.3321,
        timezone: "America/Los_Angeles",
        country: "United States",
    },
    CityInfo {
        name: "Boston",
        aliases: &["boston"],
        latitude: 42.3601,
        longitude: -71.0589,
        timezone: "America/New_York",
        country: "United States",
    },
    CityInfo {
        name: "Washington DC",
        aliases: &["washington", "washington dc", "dc"],
        latitude: 38.9072,
        longitude: -77.0369,
        timezone: "America/New_York",
        country: "United States",
    },
    CityInfo {
        name: "Atlanta",
        aliases: &["atlanta"],
        latitude: 33.7490,
        longitude: -84.3880,
        timezone: "America/New_York",
        country: "United States",
    },
    CityInfo {
        name: "Dallas",
        aliases: &["dallas", "dfw"],
        latitude: 32.7767,
        longitude: -96.7970,
        timezone: "America/Chicago",
        country: "United States",
    },
    CityInfo {
        name: "Houston",
        aliases: &["houston"],
        latitude: 29.7604,
        longitude: -95.3698,
        timezone: "America/Chicago",
        country: "United States",
    },
    CityInfo {
        name: "Miami",
        aliases: &["miami"],
        latitude: 25.7617,
        longitude: -80.1918,
        timezone: "America/New_York",
        country: "United States",
    },
    CityInfo {
        name: "Toronto",
        aliases: &["toronto"],
        latitude: 43.6532,
        longitude: -79.3832,
        timezone: "America/Toronto",
        country: "Canada",
    },
    CityInfo {
        name: "Vancouver",
        aliases: &["vancouver"],
        latitude: 49.2827,
        longitude: -123.1207,
        timezone: "America/Vancouver",
        country: "Canada",
    },
    // Oceania & South America
    CityInfo {
        name: "Sydney",
        aliases: &["sydney"],
        latitude: -33.8688,
        longitude: 151.2093,
        timezone: "Australia/Sydney",
        country: "Australia",
    },
    CityInfo {
        name: "Melbourne",
        aliases: &["melbourne"],
        latitude: -37.8136,
        longitude: 144.9631,
        timezone: "Australia/Melbourne",
        country: "Australia",
    },
    CityInfo {
        name: "Auckland",
        aliases: &["auckland"],
        latitude: -36.8485,
        longitude: 174.7633,
        timezone: "Pacific/Auckland",
        country: "New Zealand",
    },
    CityInfo {
        name: "São Paulo",
        aliases: &["sao paulo", "são paulo"],
        latitude: -23.5505,
        longitude: -46.6333,
        timezone: "America/Sao_Paulo",
        country: "Brazil",
    },
    CityInfo {
        name: "Buenos Aires",
        aliases: &["buenos aires"],
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
    /// Create new location
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

    /// Create Korean city location (Standard meridian 135.0 deg)
    pub fn korea(name: impl Into<String>, latitude: f64, longitude: f64) -> Self {
        Self::new(name, latitude, longitude, 135.0)
    }

    /// Create Chinese city location (Standard meridian 120.0 deg)
    pub fn china(name: impl Into<String>, latitude: f64, longitude: f64) -> Self {
        Self::new(name, latitude, longitude, 120.0)
    }

    /// Calculate local solar time offset in minutes
    ///
    /// Computes the difference between standard clock time and true solar time.
    /// Positive: Ahead of standard time (East)
    /// Negative: Behind standard time (West)
    ///
    /// # Formula
    /// Offset (min) = (Longitude - Standard Meridian) * 4 min/deg
    ///
    /// # Examples
    /// - Seoul (126.98°): (126.98 - 135) * 4 = -32 min
    /// - Ansan (126.83°): (126.83 - 135) * 4 = -33 min
    pub fn time_offset_minutes(&self) -> i32 {
        let diff = self.longitude - self.standard_longitude;
        (diff * 4.0).round() as i32
    }

    /// Time offset in seconds for true solar time conversion
    pub fn time_offset_seconds(&self) -> i32 {
        self.time_offset_minutes() * 60
    }
}

// ============================================
// Built-in major city constants
// ============================================

impl Location {
    /// Resolve location from city name using the built-in offline city database
    pub fn from_city(query: &str) -> Option<Self> {
        lookup_city(query).map(|c| c.to_location())
    }

    /// Seoul
    pub fn seoul() -> Self {
        Self::korea("Seoul", 37.5665, 126.9780)
    }

    /// Ansan
    pub fn ansan() -> Self {
        Self::korea("Ansan", 37.3219, 126.8309)
    }

    /// Incheon
    pub fn incheon() -> Self {
        Self::korea("Incheon", 37.4563, 126.7052)
    }

    /// Busan
    pub fn busan() -> Self {
        Self::korea("Busan", 35.1796, 129.0756)
    }

    /// Daegu
    pub fn daegu() -> Self {
        Self::korea("Daegu", 35.8714, 128.6014)
    }

    /// Daejeon
    pub fn daejeon() -> Self {
        Self::korea("Daejeon", 36.3504, 127.3845)
    }

    /// Gwangju
    pub fn gwangju() -> Self {
        Self::korea("Gwangju", 35.1595, 126.8526)
    }

    /// Jeju
    pub fn jeju() -> Self {
        Self::korea("Jeju", 33.4996, 126.5312)
    }

    /// Ulsan
    pub fn ulsan() -> Self {
        Self::korea("Ulsan", 35.5384, 129.3114)
    }

    /// Suwon
    pub fn suwon() -> Self {
        Self::korea("Suwon", 37.2636, 127.0286)
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
    /// Default: Seoul
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
        // Seoul: (126.98 - 135) * 4 ≈ -32m
        assert_eq!(seoul.time_offset_minutes(), -32);
    }

    #[test]
    fn test_ansan_offset() {
        let ansan = Location::ansan();
        // Ansan: (126.83 - 135) * 4 ≈ -33m
        assert_eq!(ansan.time_offset_minutes(), -33);
    }

    #[test]
    fn test_busan_offset() {
        let busan = Location::busan();
        // Busan: (129.08 - 135) * 4 ≈ -24m
        assert_eq!(busan.time_offset_minutes(), -24);
    }

    #[test]
    fn test_custom_location() {
        // Tokyo (Standard meridian 135.0 deg)
        let tokyo = Location::new("Tokyo", 35.6762, 139.6503, 135.0);
        // (139.65 - 135) * 4 ≈ +19m
        assert_eq!(tokyo.time_offset_minutes(), 19);
    }

    #[test]
    fn test_lookup_city() {
        let seoul = lookup_city("seoul").unwrap();
        assert_eq!(seoul.name, "Seoul");
        assert_eq!(seoul.timezone, "Asia/Seoul");
        assert!((seoul.latitude - 37.5665).abs() < 0.001);

        let busan = lookup_city("Busan").unwrap();
        assert_eq!(busan.name, "Busan");

        let tokyo = lookup_city("Tokyo").unwrap();
        assert_eq!(tokyo.name, "Tokyo");
        assert_eq!(tokyo.timezone, "Asia/Tokyo");

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
