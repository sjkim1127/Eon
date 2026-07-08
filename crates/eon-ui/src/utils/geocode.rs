use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CityRecord {
    pub name: String,
    pub name_ko: Option<String>,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
    pub tz: String,
    pub population: u64,
}

const CITIES_BIN: &[u8] = include_bytes!("cities.bin");

lazy_static::lazy_static! {
    static ref CITIES: Vec<CityRecord> = {
        bincode::deserialize(CITIES_BIN).unwrap_or_default()
    };
}

pub fn search_city(query: &str, limit: usize) -> Vec<CityRecord> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for city in CITIES.iter() {
        let name_lower = city.name.to_lowercase();
        let match_en = name_lower.starts_with(&query) || name_lower.contains(&query);
        let match_ko = city.name_ko.as_ref().map(|k| k.starts_with(&query) || k.contains(&query)).unwrap_or(false);
        
        if match_en || match_ko {
            results.push(city.clone());
            if results.len() >= limit {
                break;
            }
        }
    }
    
    // Sort logic is already handled because CITIES is sorted by population descending!
    results
}
