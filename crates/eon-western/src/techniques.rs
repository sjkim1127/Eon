//! Natal-chart techniques that need no second moment in time: antiscia, mutual reception and
//! midpoint contacts.

use crate::{get_traditional_ruler, WesternPlanetData};
use serde::{Deserialize, Serialize};

/// Orb (degrees) for antiscia and midpoint contacts.
const CONTACT_ORB: f64 = 1.0;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AntisciaKind {
    /// Mirror image across the Cancer/Capricorn (solstice) axis
    Antiscion,
    /// Mirror image across the Aries/Libra (equinox) axis
    ContraAntiscion,
}

/// `body_a`'s (contra-)antiscion falls within the orb of `body_b`. The relation is symmetric, so
/// every pair is listed once.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntisciaContact {
    pub body_a: String,
    pub body_b: String,
    pub kind: AntisciaKind,
    pub orb: f64,
}

/// Two planets each placed in a sign ruled (by domicile) by the other.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MutualReception {
    pub planet_a: String,
    pub planet_b: String,
    pub sign_a_index: usize,
    pub sign_b_index: usize,
}

/// `contact_body` sits on the midpoint of `body_a` and `body_b` on the 90-degree dial.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MidpointContact {
    pub body_a: String,
    pub body_b: String,
    pub midpoint_longitude: f64,
    pub contact_body: String,
    pub orb: f64,
}

/// A natal point: a planet or one of the two angles.
#[derive(Debug, Clone)]
pub(crate) struct Point {
    pub name: String,
    pub longitude: f64,
}

/// Planets (without the derived South Node) followed by the Ascendant and Midheaven.
pub(crate) fn natal_points(planets: &[WesternPlanetData], asc: f64, mc: f64) -> Vec<Point> {
    let mut points: Vec<Point> = planets
        .iter()
        .filter(|p| p.name != "South Node")
        .map(|p| Point {
            name: p.name.clone(),
            longitude: p.longitude,
        })
        .collect();
    points.push(Point {
        name: "Ascendant".to_string(),
        longitude: asc,
    });
    points.push(Point {
        name: "Midheaven".to_string(),
        longitude: mc,
    });
    points
}

/// Shortest angular distance between two longitudes, 0..=180.
pub(crate) fn separation(a: f64, b: f64) -> f64 {
    ((a - b + 540.0).rem_euclid(360.0) - 180.0).abs()
}

pub fn calculate_antiscia(
    planets: &[WesternPlanetData],
    asc: f64,
    mc: f64,
) -> Vec<AntisciaContact> {
    let points = natal_points(planets, asc, mc);
    let mut contacts = Vec::new();

    for (i, a) in points.iter().enumerate() {
        let antiscion = (180.0 - a.longitude).rem_euclid(360.0);
        let contra = (360.0 - a.longitude).rem_euclid(360.0);
        for b in points.iter().skip(i + 1) {
            for (kind, mirror) in [
                (AntisciaKind::Antiscion, antiscion),
                (AntisciaKind::ContraAntiscion, contra),
            ] {
                let orb = separation(mirror, b.longitude);
                if orb <= CONTACT_ORB {
                    contacts.push(AntisciaContact {
                        body_a: a.name.clone(),
                        body_b: b.name.clone(),
                        kind,
                        orb,
                    });
                }
            }
        }
    }

    contacts.sort_by(|x, y| x.orb.total_cmp(&y.orb));
    contacts
}

pub fn calculate_mutual_receptions(planets: &[WesternPlanetData]) -> Vec<MutualReception> {
    const CLASSICAL: [&str; 7] = [
        "Sun", "Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn",
    ];
    let classical: Vec<&WesternPlanetData> = planets
        .iter()
        .filter(|p| CLASSICAL.contains(&p.name.as_str()))
        .collect();

    let mut result = Vec::new();
    for (i, a) in classical.iter().enumerate() {
        for b in classical.iter().skip(i + 1) {
            if get_traditional_ruler(a.sign_index) == b.name
                && get_traditional_ruler(b.sign_index) == a.name
            {
                result.push(MutualReception {
                    planet_a: a.name.clone(),
                    planet_b: b.name.clone(),
                    sign_a_index: a.sign_index,
                    sign_b_index: b.sign_index,
                });
            }
        }
    }
    result
}

pub fn calculate_midpoints(
    planets: &[WesternPlanetData],
    asc: f64,
    mc: f64,
) -> Vec<MidpointContact> {
    // The midpoint tree is read on personal planets, the outer planets, the nodes and the angles.
    const BODIES: [&str; 11] = [
        "Sun",
        "Moon",
        "Mercury",
        "Venus",
        "Mars",
        "Jupiter",
        "Saturn",
        "Uranus",
        "Neptune",
        "Pluto",
        "True Node",
    ];
    let points: Vec<Point> = natal_points(planets, asc, mc)
        .into_iter()
        .filter(|p| {
            BODIES.contains(&p.name.as_str()) || p.name == "Ascendant" || p.name == "Midheaven"
        })
        .collect();

    let mut contacts = Vec::new();
    for (i, a) in points.iter().enumerate() {
        for b in points.iter().skip(i + 1) {
            let delta = (b.longitude - a.longitude + 540.0).rem_euclid(360.0) - 180.0;
            let midpoint = (a.longitude + delta / 2.0).rem_euclid(360.0);

            for c in &points {
                if c.name == a.name || c.name == b.name {
                    continue;
                }
                // Hard aspects to a midpoint (0/45/90/135/180) all coincide on the 90-degree dial.
                let on_dial = (c.longitude - midpoint).rem_euclid(90.0);
                let orb = on_dial.min(90.0 - on_dial);
                if orb <= CONTACT_ORB {
                    contacts.push(MidpointContact {
                        body_a: a.name.clone(),
                        body_b: b.name.clone(),
                        midpoint_longitude: midpoint,
                        contact_body: c.name.clone(),
                        orb,
                    });
                }
            }
        }
    }

    contacts.sort_by(|x, y| x.orb.total_cmp(&y.orb));
    contacts.truncate(40);
    contacts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn planet(name: &str, longitude: f64) -> WesternPlanetData {
        WesternPlanetData {
            id: 0,
            name: name.to_string(),
            longitude,
            speed: 1.0,
            is_retrograde: false,
            sign_index: (longitude / 30.0).floor() as usize,
            degree_in_sign: longitude % 30.0,
            house_number: 1,
        }
    }

    #[test]
    fn antiscia_mirror_across_the_solstice_axis() {
        // 10 Gemini (70) mirrors to 20 Cancer (110); 20 Cancer is also where Venus sits.
        let planets = [planet("Sun", 70.0), planet("Venus", 110.0)];
        let found = calculate_antiscia(&planets, 200.0, 290.0);
        let hit = found
            .iter()
            .find(|c| c.body_a == "Sun" && c.body_b == "Venus")
            .expect("antiscion contact");
        assert_eq!(hit.kind, AntisciaKind::Antiscion);
        assert!(hit.orb < 1e-9);
    }

    #[test]
    fn contra_antiscia_mirror_across_the_equinox_axis() {
        // 10 Aries (10) mirrors to 20 Pisces (350).
        let planets = [planet("Mars", 10.0), planet("Moon", 350.5)];
        let found = calculate_antiscia(&planets, 200.0, 290.0);
        let hit = found
            .iter()
            .find(|c| c.kind == AntisciaKind::ContraAntiscion)
            .expect("contra-antiscion contact");
        assert!((hit.orb - 0.5).abs() < 1e-9);
    }

    #[test]
    fn mutual_reception_requires_both_planets_in_each_others_signs() {
        // Sun in Cancer (ruled by the Moon) and Moon in Leo (ruled by the Sun).
        let planets = [planet("Sun", 100.0), planet("Moon", 125.0)];
        let found = calculate_mutual_receptions(&planets);
        assert_eq!(found.len(), 1);
        assert_eq!(
            (found[0].planet_a.as_str(), found[0].planet_b.as_str()),
            ("Sun", "Moon")
        );

        // Moon in Leo but the Sun in Gemini: no reception.
        let none = calculate_mutual_receptions(&[planet("Sun", 70.0), planet("Moon", 125.0)]);
        assert!(none.is_empty());
    }

    #[test]
    fn midpoint_contact_uses_the_shorter_arc_and_the_ninety_degree_dial() {
        // Sun 0, Moon 40: midpoint 20. Mars at 110 (20 + 90) is on the 90-dial.
        let planets = [
            planet("Sun", 0.0),
            planet("Moon", 40.0),
            planet("Mars", 110.0),
        ];
        let found = calculate_midpoints(&planets, 200.0, 290.0);
        assert!(
            found.iter().any(|c| c.body_a == "Sun"
                && c.body_b == "Moon"
                && c.contact_body == "Mars"
                && c.orb < 1e-9),
            "{found:?}"
        );

        // The midpoint of 350 and 10 is 0, not 180.
        let wrap = calculate_midpoints(
            &[
                planet("Sun", 350.0),
                planet("Moon", 10.0),
                planet("Mars", 0.2),
            ],
            200.0,
            290.0,
        );
        assert!(wrap.iter().any(|c| c.body_a == "Sun"
            && c.contact_body == "Mars"
            && (c.midpoint_longitude - 0.0).abs() < 1e-9));
    }
}
