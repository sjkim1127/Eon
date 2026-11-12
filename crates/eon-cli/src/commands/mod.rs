use anyhow::{anyhow, Result};
use clap::{Args, ValueEnum};
use eon_service::dto::AnalysisInput;

pub mod analyze;
pub mod hd;
pub mod iching;
pub mod mcp;
pub mod numerology;
pub mod prompt;
pub mod qimen;
pub mod saju;
pub mod tier;
pub mod vedic;
pub mod western;
pub mod zwds;

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OutputFormat {
    #[default]
    Agent,
    Json,
    Markdown,
    Text,
}

#[derive(Args, Debug, Clone)]
pub struct BirthArgs {
    /// Birth datetime: 'YYYY-MM-DD HH:MM' or 'YYYY-MM-DD' (e.g. '1990-05-15 10:30')
    #[arg(short, long)]
    pub birth: String,

    /// Gender: 'm', 'male', 'f', 'female' (default: male)
    #[arg(short, long, default_value = "m")]
    pub gender: String,

    /// Birth city or region (e.g. 'seoul', 'busan', 'tokyo', 'new york', 'la', 'london')
    /// Automatically resolves latitude, longitude, and timezone.
    #[arg(short, long)]
    pub city: Option<String>,

    /// Latitude (default: 37.5665 for Seoul, or automatically resolved from --city)
    #[arg(long)]
    pub lat: Option<f64>,

    /// Longitude (default: 126.9780 for Seoul, or automatically resolved from --city)
    #[arg(long)]
    pub lon: Option<f64>,

    /// Timezone (default: Asia/Seoul, or automatically resolved from --city)
    #[arg(long)]
    pub tz: Option<String>,

    /// Whether the birth date is Lunar calendar
    #[arg(long)]
    pub lunar: bool,

    /// Whether the lunar birth month is a leap month
    #[arg(long)]
    pub leap: bool,

    /// Full name in English/Latin letters for Numerology
    #[arg(long)]
    pub name: Option<String>,
}

impl BirthArgs {
    pub fn parse_gender(&self) -> Result<eon_core::Gender> {
        let g = self.gender.trim().to_lowercase();
        match g.as_str() {
            "m" | "male" | "1" | "3" => Ok(eon_core::Gender::Male),
            "f" | "female" | "2" | "4" => Ok(eon_core::Gender::Female),
            _ => Err(anyhow!(
                "Invalid gender: '{}'. Expected 'm', 'male', 'f', or 'female'",
                self.gender
            )),
        }
    }

    pub fn is_male(&self) -> Result<bool> {
        self.parse_gender().map(|g| g == eon_core::Gender::Male)
    }

    pub fn to_analysis_input(&self) -> Result<AnalysisInput> {
        // Validate gender strictly
        let _ = self.parse_gender()?;

        let raw = self.birth.trim();
        let (date_part, time_part) = if let Some((d, t)) = raw.split_once(' ') {
            (d, t)
        } else if let Some((d, t)) = raw.split_once('T') {
            (d, t)
        } else {
            (raw, "12:00")
        };

        let date_seps = ['-', '.', '/'];
        let parts: Vec<&str> = date_part
            .split(|c| date_seps.contains(&c))
            .filter(|s| !s.is_empty())
            .collect();

        if parts.len() != 3 {
            return Err(anyhow!(
                "Invalid date format: '{}'. Expected 'YYYY-MM-DD [HH:MM]'",
                self.birth
            ));
        }

        let year: i32 = parts[0]
            .parse()
            .map_err(|_| anyhow!("Invalid year: {}", parts[0]))?;
        let month: u32 = parts[1]
            .parse()
            .map_err(|_| anyhow!("Invalid month: {}", parts[1]))?;
        let day: u32 = parts[2]
            .parse()
            .map_err(|_| anyhow!("Invalid day: {}", parts[2]))?;

        if !(1..=12).contains(&month) {
            return Err(anyhow!("Invalid month: {}. Month must be 1-12", month));
        }
        if !(1..=31).contains(&day) {
            return Err(anyhow!("Invalid day: {}. Day must be 1-31", day));
        }

        let time_parts: Vec<&str> = time_part.split(':').collect();
        if time_parts.is_empty() || time_parts.len() > 3 {
            return Err(anyhow!(
                "Invalid time format: '{}'. Expected 'HH:MM'",
                time_part
            ));
        }
        let hour: u32 = time_parts[0].parse().map_err(|_| {
            anyhow!(
                "Invalid hour '{}' in birth time '{}'",
                time_parts[0],
                self.birth
            )
        })?;
        let minute: u32 = if time_parts.len() >= 2 {
            time_parts[1].parse().map_err(|_| {
                anyhow!(
                    "Invalid minute '{}' in birth time '{}'",
                    time_parts[1],
                    self.birth
                )
            })?
        } else {
            0
        };

        if hour > 23 || minute > 59 {
            return Err(anyhow!(
                "Birth time out of range: '{:02}:{:02}'. Hour must be 0-23, minute 0-59",
                hour,
                minute
            ));
        }

        let (lat, lon, tz, resolved_city) = if let Some(ref city_query) = self.city {
            match eon_core::lookup_city(city_query) {
                Some(info) => {
                    let lat = self.lat.unwrap_or(info.latitude);
                    let lon = self.lon.unwrap_or(info.longitude);
                    let tz = self.tz.clone().unwrap_or_else(|| info.timezone.to_string());
                    (lat, lon, tz, Some(info.name.to_string()))
                }
                None => {
                    if let (Some(lat), Some(lon)) = (self.lat, self.lon) {
                        let tz = self.tz.clone().unwrap_or_else(|| "Asia/Seoul".to_string());
                        (lat, lon, tz, Some(city_query.clone()))
                    } else {
                        return Err(anyhow!(
                            "Unknown city '{}'. Please provide a recognized city (e.g. 'Seoul', 'Busan', 'Tokyo', 'New York', 'London') or specify explicit coordinates with --lat and --lon.",
                            city_query
                        ));
                    }
                }
            }
        } else {
            let lat = self.lat.unwrap_or(37.5665);
            let lon = self.lon.unwrap_or(126.9780);
            let tz = self.tz.clone().unwrap_or_else(|| "Asia/Seoul".to_string());
            (lat, lon, tz, None)
        };

        Ok(AnalysisInput {
            year,
            month,
            day,
            hour,
            minute,
            is_lunar: self.lunar,
            is_leap_month: self.leap,
            lat,
            lon,
            timezone: tz,
            city: resolved_city,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_birth_args() {
        let args = BirthArgs {
            birth: "1990-05-15 10:30".to_string(),
            gender: "m".to_string(),
            city: Some("Seoul".to_string()),
            lat: None,
            lon: None,
            tz: None,
            lunar: false,
            leap: false,
            name: None,
        };
        let input = args.to_analysis_input().unwrap();
        assert_eq!(input.year, 1990);
        assert_eq!(input.month, 5);
        assert_eq!(input.day, 15);
        assert_eq!(input.hour, 10);
        assert_eq!(input.minute, 30);
        assert!(args.is_male().unwrap());
    }

    #[test]
    fn test_invalid_birth_time_fails() {
        let args = BirthArgs {
            birth: "1990-05-15 xx:yy".to_string(),
            gender: "m".to_string(),
            city: None,
            lat: None,
            lon: None,
            tz: None,
            lunar: false,
            leap: false,
            name: None,
        };
        assert!(args.to_analysis_input().is_err());
    }

    #[test]
    fn test_out_of_range_time_fails() {
        let args = BirthArgs {
            birth: "1990-05-15 25:99".to_string(),
            gender: "m".to_string(),
            city: None,
            lat: None,
            lon: None,
            tz: None,
            lunar: false,
            leap: false,
            name: None,
        };
        assert!(args.to_analysis_input().is_err());
    }

    #[test]
    fn test_invalid_gender_fails() {
        let args = BirthArgs {
            birth: "1990-05-15 10:30".to_string(),
            gender: "alien".to_string(),
            city: None,
            lat: None,
            lon: None,
            tz: None,
            lunar: false,
            leap: false,
            name: None,
        };
        assert!(args.parse_gender().is_err());
        assert!(args.to_analysis_input().is_err());
    }
}
