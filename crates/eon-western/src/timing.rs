//! Timing techniques that relate a moment in time to the natal chart: transits, solar and lunar
//! returns, solar arc directions and annual profections.

use crate::techniques::{natal_points, Point};
use crate::{
    aspect_dynamics, calculate_western, is_angle_between, AspectType, WesternAspectData,
    WesternError, WesternResult,
};
use chrono::{DateTime, Duration, Utc};
use eon_astro::AstroEngine;
use serde::{Deserialize, Serialize};

/// Days per tropical year, used for the "one year of life = one step" scalings.
const YEAR_DAYS: f64 = 365.2425;
/// Mean daily motion of the Moon (degrees per day), used only to seed the lunar return search.
const MOON_MEAN_MOTION: f64 = 13.176_358;

const MAJOR_ASPECTS: [AspectType; 5] = [
    AspectType::Conjunction,
    AspectType::Sextile,
    AspectType::Square,
    AspectType::Trine,
    AspectType::Opposition,
];

/// Orb (degrees) for a moving body to a natal point. Faster bodies get tighter orbs; sextiles are
/// 25% tighter than the other major aspects. This is a school-dependent convention.
fn moving_orb(body: &str, aspect: AspectType) -> f64 {
    let base = match body {
        "Moon" => 1.0,
        "Sun" | "Mercury" | "Venus" | "Mars" => 2.0,
        "Jupiter" | "Saturn" | "Uranus" | "Neptune" | "Pluto" => 3.0,
        _ => 1.5,
    };
    if aspect == AspectType::Sextile {
        base * 0.75
    } else {
        base
    }
}

/// Orb for solar arc directions, which move about one degree per year.
const SOLAR_ARC_ORB: f64 = 1.0;

/// Aspects from moving points to natal points. `moving` carries its speed (degrees per day for
/// transits; the arc rate for directions) so applying/separating can be told.
fn aspects_to_natal(
    prefix_moving: &str,
    moving: &[(String, f64, f64)],
    natal: &[Point],
    orb_for: impl Fn(&str, AspectType) -> f64,
) -> Vec<WesternAspectData> {
    let mut found = Vec::new();
    for (name, longitude, speed) in moving {
        for point in natal {
            let signed_delta = (longitude - point.longitude + 540.0).rem_euclid(360.0) - 180.0;
            let angle = signed_delta.abs();
            for aspect in MAJOR_ASPECTS {
                let orb = (angle - aspect.angle()).abs();
                if orb <= orb_for(name, aspect) {
                    found.push(WesternAspectData {
                        body_a_name: format!("{prefix_moving} {name}"),
                        body_b_name: format!("Natal {}", point.name),
                        aspect_type: aspect,
                        angle_diff: angle,
                        orb,
                        dynamics: aspect_dynamics(angle, aspect.angle(), signed_delta, *speed),
                        is_major: true,
                    });
                }
            }
        }
    }
    found.sort_by(|a, b| a.orb.total_cmp(&b.orb));
    found
}

fn house_of(longitude: f64, natal: &WesternResult) -> usize {
    for (i, house) in natal.houses.iter().enumerate() {
        let next = natal.houses[(i + 1) % natal.houses.len()].cusp_longitude;
        if is_angle_between(longitude, house.cusp_longitude, next) {
            return house.house_number;
        }
    }
    1
}

// ───────────────────────────── transits ─────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransitPlanetData {
    pub name: String,
    pub longitude: f64,
    pub speed: f64,
    pub is_retrograde: bool,
    pub sign_index: usize,
    pub degree_in_sign: f64,
    /// House of the natal chart the transiting body currently occupies
    pub natal_house: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransitResult {
    pub target_time: DateTime<Utc>,
    pub natal_chart: WesternResult,
    pub planets: Vec<TransitPlanetData>,
    /// Transiting body to natal planet/angle, tightest orb first
    pub aspects: Vec<WesternAspectData>,
}

/// The sky at `target_time` laid over the natal chart (houses stay natal).
pub fn calculate_transits(
    birth_time: DateTime<Utc>,
    lat: f64,
    lon: f64,
    target_time: DateTime<Utc>,
    house_sys: char,
) -> Result<TransitResult, WesternError> {
    let natal = calculate_western(birth_time, lat, lon, house_sys)?;
    // Houses of the transit chart are not used; only its planet positions matter.
    let sky = calculate_western(target_time, lat, lon, 'W')?;

    let planets: Vec<TransitPlanetData> = sky
        .planets
        .iter()
        .filter(|p| p.name != "South Node")
        .map(|p| TransitPlanetData {
            name: p.name.clone(),
            longitude: p.longitude,
            speed: p.speed,
            is_retrograde: p.is_retrograde,
            sign_index: p.sign_index,
            degree_in_sign: p.degree_in_sign,
            natal_house: house_of(p.longitude, &natal),
        })
        .collect();

    let moving: Vec<(String, f64, f64)> = planets
        .iter()
        .map(|p| (p.name.clone(), p.longitude, p.speed))
        .collect();
    let targets = natal_points(&natal.planets, natal.ascendant, natal.midheaven);
    let aspects = aspects_to_natal("Transit", &moving, &targets, moving_orb);

    Ok(TransitResult {
        target_time,
        natal_chart: natal,
        planets,
        aspects,
    })
}

// ───────────────────────────── returns ─────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReturnKind {
    Solar,
    Lunar,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReturnResult {
    pub kind: ReturnKind,
    /// The instant the Sun (or Moon) is back on its natal longitude
    pub exact_time: DateTime<Utc>,
    pub natal_chart: WesternResult,
    /// Chart of the return moment, cast for the requested location
    pub return_chart: WesternResult,
    /// Return-chart planets to natal planets/angles, tightest orb first
    pub aspects_to_natal: Vec<WesternAspectData>,
}

/// Solar return of the calendar `year` (the Sun back on its natal longitude near the birthday).
/// The return chart is cast for `lat`/`lon`, which may be a relocation.
pub fn calculate_solar_return(
    birth_time: DateTime<Utc>,
    birth_lat: f64,
    birth_lon: f64,
    year: i32,
    return_lat: f64,
    return_lon: f64,
    house_sys: char,
) -> Result<ReturnResult, WesternError> {
    use chrono::Datelike;
    let years = year - birth_time.year();
    if years <= 0 {
        return Err(WesternError::InvalidInput(
            "a solar return needs a year after the birth year".to_string(),
        ));
    }

    let natal = calculate_western(birth_time, birth_lat, birth_lon, house_sys)?;
    let natal_sun = natal
        .planets
        .iter()
        .find(|p| p.name == "Sun")
        .map(|p| p.longitude)
        .unwrap_or(0.0);

    let engine = AstroEngine::new();
    let start = birth_time + Duration::seconds((f64::from(years) * YEAR_DAYS * 86_400.0) as i64);
    let exact_time = engine.find_time_for_planet_longitude(start, natal_sun, 0)?;

    finish_return(
        ReturnKind::Solar,
        natal,
        exact_time,
        return_lat,
        return_lon,
        house_sys,
    )
}

/// First lunar return on or after `after` (the Moon back on its natal longitude).
pub fn calculate_lunar_return(
    birth_time: DateTime<Utc>,
    birth_lat: f64,
    birth_lon: f64,
    after: DateTime<Utc>,
    return_lat: f64,
    return_lon: f64,
    house_sys: char,
) -> Result<ReturnResult, WesternError> {
    let natal = calculate_western(birth_time, birth_lat, birth_lon, house_sys)?;
    let natal_moon = natal
        .planets
        .iter()
        .find(|p| p.name == "Moon")
        .map(|p| p.longitude)
        .unwrap_or(0.0);

    let engine = AstroEngine::new();
    let (current, _) = engine.get_planet_full(after, 1, 256)?;
    let ahead = (natal_moon - current).rem_euclid(360.0);
    let seed = after + Duration::seconds((ahead / MOON_MEAN_MOTION * 86_400.0) as i64);
    let mut exact_time = engine.find_time_for_planet_longitude(seed, natal_moon, 1)?;

    // Newton refinement may land on the previous pass when the Moon is almost on the point.
    if exact_time < after {
        let next_seed = exact_time + Duration::seconds((27.32 * 86_400.0) as i64);
        exact_time = engine.find_time_for_planet_longitude(next_seed, natal_moon, 1)?;
    }

    finish_return(
        ReturnKind::Lunar,
        natal,
        exact_time,
        return_lat,
        return_lon,
        house_sys,
    )
}

fn finish_return(
    kind: ReturnKind,
    natal: WesternResult,
    exact_time: DateTime<Utc>,
    lat: f64,
    lon: f64,
    house_sys: char,
) -> Result<ReturnResult, WesternError> {
    let return_chart = calculate_western(exact_time, lat, lon, house_sys)?;

    let moving: Vec<(String, f64, f64)> = return_chart
        .planets
        .iter()
        .filter(|p| p.name != "South Node")
        .map(|p| (p.name.clone(), p.longitude, p.speed))
        .collect();
    let targets = natal_points(&natal.planets, natal.ascendant, natal.midheaven);
    // Charts are compared as static pictures: no applying/separating motion.
    let static_moving: Vec<(String, f64, f64)> =
        moving.into_iter().map(|(n, l, _)| (n, l, 0.0)).collect();
    let aspects = aspects_to_natal("Return", &static_moving, &targets, |_, aspect| {
        if aspect == AspectType::Sextile {
            4.0
        } else {
            5.0
        }
    });

    Ok(ReturnResult {
        kind,
        exact_time,
        natal_chart: natal,
        return_chart,
        aspects_to_natal: aspects,
    })
}

// ───────────────────────────── solar arc ─────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DirectedPoint {
    pub name: String,
    pub longitude: f64,
    pub sign_index: usize,
    pub degree_in_sign: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SolarArcResult {
    pub target_time: DateTime<Utc>,
    pub age_years: f64,
    /// Secondary-progressed Sun minus natal Sun: added to every natal point
    pub arc_degrees: f64,
    pub natal_chart: WesternResult,
    pub directed: Vec<DirectedPoint>,
    /// Directed points to natal planets/angles (orb 1 degree), tightest first
    pub aspects: Vec<WesternAspectData>,
}

pub fn calculate_solar_arc(
    birth_time: DateTime<Utc>,
    lat: f64,
    lon: f64,
    target_time: DateTime<Utc>,
    house_sys: char,
) -> Result<SolarArcResult, WesternError> {
    let natal = calculate_western(birth_time, lat, lon, house_sys)?;
    let natal_sun = natal
        .planets
        .iter()
        .find(|p| p.name == "Sun")
        .map(|p| p.longitude)
        .unwrap_or(0.0);

    let age_years = (target_time - birth_time).num_seconds() as f64 / 86_400.0 / YEAR_DAYS;
    let progressed_time = birth_time + Duration::seconds((age_years * 86_400.0) as i64);
    let engine = AstroEngine::new();
    let (progressed_sun, _) = engine.get_planet_full(progressed_time, 0, 4 | 256)?;
    let arc_degrees = (progressed_sun - natal_sun).rem_euclid(360.0);

    let points = natal_points(&natal.planets, natal.ascendant, natal.midheaven);
    let directed: Vec<DirectedPoint> = points
        .iter()
        .map(|p| {
            let longitude = (p.longitude + arc_degrees).rem_euclid(360.0);
            DirectedPoint {
                name: p.name.clone(),
                longitude,
                sign_index: (longitude / 30.0).floor() as usize,
                degree_in_sign: longitude % 30.0,
            }
        })
        .collect();

    // A directed point advances about one degree per year; speeds here are degrees per year.
    let moving: Vec<(String, f64, f64)> = directed
        .iter()
        .map(|d| (d.name.clone(), d.longitude, 1.0))
        .collect();
    let aspects = aspects_to_natal("Directed", &moving, &points, |_, _| SOLAR_ARC_ORB)
        .into_iter()
        // A directed point trivially conjoins its own natal position only when the arc is ~0.
        .filter(|a| {
            let own = a.body_a_name.trim_start_matches("Directed ");
            let target = a.body_b_name.trim_start_matches("Natal ");
            !(own == target && a.aspect_type == AspectType::Conjunction && arc_degrees < 0.5)
        })
        .collect();

    Ok(SolarArcResult {
        target_time,
        age_years,
        arc_degrees,
        natal_chart: natal,
        directed,
        aspects,
    })
}

// ───────────────────────────── profections ─────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfectionResult {
    pub target_time: DateTime<Utc>,
    /// Completed years of life at the target time
    pub age: u32,
    /// House the year profects to (age 0 = 1st house, 12 = 1st again)
    pub profected_house: usize,
    pub profected_sign_index: usize,
    /// Traditional ruler of the profected sign: the lord of the year
    pub year_lord: String,
    pub year_lord_natal_sign_index: usize,
    pub year_lord_natal_house: usize,
    /// Natal planets in the profected house
    pub planets_in_profected_house: Vec<String>,
}

/// Annual profection: the Ascendant advances one sign per year of life.
pub fn calculate_profection(
    birth_time: DateTime<Utc>,
    lat: f64,
    lon: f64,
    target_time: DateTime<Utc>,
    house_sys: char,
) -> Result<ProfectionResult, WesternError> {
    if target_time < birth_time {
        return Err(WesternError::InvalidInput(
            "profection target time is before the birth time".to_string(),
        ));
    }
    let natal = calculate_western(birth_time, lat, lon, house_sys)?;

    let age =
        ((target_time - birth_time).num_seconds() as f64 / 86_400.0 / YEAR_DAYS).floor() as u32;
    let steps = (age % 12) as usize;
    let asc_sign = (natal.ascendant / 30.0).floor() as usize;
    let profected_sign_index = (asc_sign + steps) % 12;
    let profected_house = steps + 1;

    let year_lord = crate::get_traditional_ruler(profected_sign_index).to_string();
    let lord = natal.planets.iter().find(|p| p.name == year_lord);

    let planets_in_profected_house = natal
        .planets
        .iter()
        .filter(|p| p.name != "South Node" && p.house_number == profected_house)
        .map(|p| p.name.clone())
        .collect();

    Ok(ProfectionResult {
        target_time,
        age,
        profected_house,
        profected_sign_index,
        year_lord,
        year_lord_natal_sign_index: lord.map(|p| p.sign_index).unwrap_or(0),
        year_lord_natal_house: lord.map(|p| p.house_number).unwrap_or(1),
        planets_in_profected_house,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn birth() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(1990, 5, 15, 1, 30, 0).unwrap()
    }

    const SEOUL: (f64, f64) = (37.5665, 126.978);

    fn longitude(chart: &WesternResult, body: &str) -> f64 {
        chart
            .planets
            .iter()
            .find(|p| p.name == body)
            .unwrap()
            .longitude
    }

    #[test]
    fn solar_return_puts_the_sun_on_its_natal_longitude() {
        let r =
            calculate_solar_return(birth(), SEOUL.0, SEOUL.1, 2027, SEOUL.0, SEOUL.1, 'P').unwrap();
        let natal = longitude(&r.natal_chart, "Sun");
        let back = longitude(&r.return_chart, "Sun");
        assert!(
            crate::techniques::separation(natal, back) < 0.001,
            "{natal} vs {back}"
        );
        // Within about a day of the birthday
        let birthday = Utc.with_ymd_and_hms(2027, 5, 15, 1, 30, 0).unwrap();
        assert!((r.exact_time - birthday).num_hours().abs() < 36);
    }

    #[test]
    fn solar_return_rejects_the_birth_year() {
        assert!(
            calculate_solar_return(birth(), SEOUL.0, SEOUL.1, 1990, SEOUL.0, SEOUL.1, 'P').is_err()
        );
    }

    #[test]
    fn lunar_return_is_the_next_one_after_the_date_and_within_a_sidereal_month() {
        let after = Utc.with_ymd_and_hms(2026, 7, 1, 0, 0, 0).unwrap();
        let r = calculate_lunar_return(birth(), SEOUL.0, SEOUL.1, after, SEOUL.0, SEOUL.1, 'P')
            .unwrap();
        assert!(r.exact_time >= after);
        assert!((r.exact_time - after).num_hours() <= 28 * 24);
        let natal = longitude(&r.natal_chart, "Moon");
        let back = longitude(&r.return_chart, "Moon");
        assert!(
            crate::techniques::separation(natal, back) < 0.01,
            "{natal} vs {back}"
        );
    }

    #[test]
    fn transit_of_the_birth_moment_conjoins_every_natal_planet() {
        // The sky at birth laid over the natal chart: each planet is conjunct itself.
        let t = calculate_transits(birth(), SEOUL.0, SEOUL.1, birth(), 'P').unwrap();
        let sun = t
            .aspects
            .iter()
            .find(|a| a.body_a_name == "Transit Sun" && a.body_b_name == "Natal Sun")
            .expect("Sun conjunct Sun");
        assert_eq!(sun.aspect_type, AspectType::Conjunction);
        assert!(sun.orb < 1e-6);
        assert_eq!(t.planets.len(), t.natal_chart.planets.len() - 1); // no South Node
        assert!(t.planets.iter().all(|p| (1..=12).contains(&p.natal_house)));
        // Results are ordered by orb
        assert!(t.aspects.windows(2).all(|w| w[0].orb <= w[1].orb));
    }

    #[test]
    fn solar_arc_is_about_one_degree_per_year_and_moves_every_point() {
        let target = Utc.with_ymd_and_hms(2020, 5, 15, 1, 30, 0).unwrap();
        let r = calculate_solar_arc(birth(), SEOUL.0, SEOUL.1, target, 'P').unwrap();
        assert!((r.age_years - 30.0).abs() < 0.01);
        assert!((r.arc_degrees - 30.0).abs() < 1.5, "arc {}", r.arc_degrees);

        let natal_mc = r.natal_chart.midheaven;
        let directed_mc = r.directed.iter().find(|d| d.name == "Midheaven").unwrap();
        let moved = (directed_mc.longitude - natal_mc).rem_euclid(360.0);
        assert!((moved - r.arc_degrees).abs() < 1e-9);
    }

    #[test]
    fn profection_advances_one_house_per_year_and_wraps_after_twelve() {
        let at = |years: i32| {
            let t = Utc.with_ymd_and_hms(1990 + years, 6, 20, 0, 0, 0).unwrap();
            calculate_profection(birth(), SEOUL.0, SEOUL.1, t, 'P').unwrap()
        };
        let a0 = at(0);
        assert_eq!((a0.age, a0.profected_house), (0, 1));
        let a5 = at(5);
        assert_eq!((a5.age, a5.profected_house), (5, 6));
        let a12 = at(12);
        assert_eq!((a12.age, a12.profected_house), (12, 1));
        assert_eq!(a12.profected_sign_index, a0.profected_sign_index);

        // The year lord is the traditional ruler of the profected sign.
        assert_eq!(
            a5.year_lord,
            crate::get_traditional_ruler(a5.profected_sign_index)
        );
    }

    #[test]
    fn profection_before_birth_is_rejected() {
        let before = Utc.with_ymd_and_hms(1980, 1, 1, 0, 0, 0).unwrap();
        assert!(calculate_profection(birth(), SEOUL.0, SEOUL.1, before, 'P').is_err());
    }
}
