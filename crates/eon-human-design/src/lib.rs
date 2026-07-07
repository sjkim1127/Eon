pub mod db;
pub mod connection;
pub mod dream_rave;
use chrono::{DateTime, Utc};
use eon_astro::{AstroEngine, AstroError};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, thiserror::Error)]
pub enum HdError {
    #[error("Astronomical calculation error: {0}")]
    Astro(#[from] AstroError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum HdCenter {
    Head,
    Ajna,
    Throat,
    SelfG,
    Heart,
    Sacral,
    Root,
    Spleen,
    SolarPlexus,
}

impl HdCenter {
    pub fn name_en(&self) -> &'static str {
        match self {
            HdCenter::Head => "Head",
            HdCenter::Ajna => "Ajna",
            HdCenter::Throat => "Throat",
            HdCenter::SelfG => "Self",
            HdCenter::Heart => "Heart",
            HdCenter::Sacral => "Sacral",
            HdCenter::Root => "Root",
            HdCenter::Spleen => "Spleen",
            HdCenter::SolarPlexus => "Solar Plexus",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HdPlanetData {
    pub name: String,
    pub degree: f64,
    pub gate: u8,
    pub line: u8,
    pub color: u8,
    pub tone: u8,
    pub base: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HumanDesignResult {
    pub chart_type: String,
    pub profile: String,
    pub authority: String,
    pub defined_centers: Vec<HdCenter>,
    pub undefined_centers: Vec<HdCenter>,
    pub personality: HashMap<String, HdPlanetData>,
    pub design: HashMap<String, HdPlanetData>,
    pub active_gates: Vec<u8>,
    pub active_channels: Vec<(u8, u8)>,
    pub definition_type: String,
    pub strategy: String,
    pub not_self_theme: String,
    pub incarnation_cross: String,
}

pub const GATE_SEQUENCE: [u8; 64] = [
    25, 17, 21, 51, 42, 3, // Aries
    27, 24, 2, 23, 8, 20, // Taurus
    16, 35, 45, 12, 15, 52, // Gemini
    39, 53, 62, 56, 31, 33, // Cancer
    7, 4, 29, 59, 40, 64, // Leo
    47, 6, 46, 18, 48, 57, // Virgo+Libra
    32, 50, 28, 44, 1, 43, // Libra+Scorpio
    14, 34, 9, 5, 26, 11, // Scorpio+Sag
    10, 58, 38, 54, 61, 60, // Cap
    41, 19, 13, 49, 30, 55, // Aquarius
    37, 63, 22, 36, // Pisces
];

pub const HD_START_DEGREE: f64 = 358.25;

pub const CHANNELS: [(u8, u8); 36] = [
    (1, 8),
    (2, 14),
    (3, 60),
    (4, 63),
    (5, 15),
    (6, 59),
    (7, 31),
    (9, 52),
    (10, 20),
    (10, 34),
    (10, 57),
    (11, 56),
    (12, 22),
    (13, 33),
    (16, 48),
    (17, 62),
    (18, 58),
    (19, 49),
    (20, 34),
    (20, 57),
    (21, 45),
    (23, 43),
    (24, 61),
    (25, 51),
    (26, 44),
    (27, 50),
    (28, 38),
    (29, 46),
    (30, 41),
    (32, 54),
    (34, 57),
    (35, 36),
    (37, 40),
    (39, 55),
    (42, 53),
    (47, 64),
];

pub fn get_channel_centers(g1: u8, g2: u8) -> Option<(HdCenter, HdCenter)> {
    let pair = if g1 < g2 { (g1, g2) } else { (g2, g1) };
    match pair {
        (1, 8) => Some((HdCenter::SelfG, HdCenter::Throat)),
        (2, 14) => Some((HdCenter::SelfG, HdCenter::Sacral)),
        (3, 60) => Some((HdCenter::Sacral, HdCenter::Root)),
        (4, 63) => Some((HdCenter::Ajna, HdCenter::Head)),
        (5, 15) => Some((HdCenter::Sacral, HdCenter::SelfG)),
        (6, 59) => Some((HdCenter::SolarPlexus, HdCenter::Sacral)),
        (7, 31) => Some((HdCenter::SelfG, HdCenter::Throat)),
        (9, 52) => Some((HdCenter::Sacral, HdCenter::Root)),
        (10, 20) => Some((HdCenter::SelfG, HdCenter::Throat)),
        (10, 34) => Some((HdCenter::SelfG, HdCenter::Sacral)),
        (10, 57) => Some((HdCenter::SelfG, HdCenter::Spleen)),
        (11, 56) => Some((HdCenter::Ajna, HdCenter::Throat)),
        (12, 22) => Some((HdCenter::Throat, HdCenter::SolarPlexus)),
        (13, 33) => Some((HdCenter::SelfG, HdCenter::Throat)),
        (16, 48) => Some((HdCenter::Throat, HdCenter::Spleen)),
        (17, 62) => Some((HdCenter::Ajna, HdCenter::Throat)),
        (18, 58) => Some((HdCenter::Spleen, HdCenter::Root)),
        (19, 49) => Some((HdCenter::Root, HdCenter::SolarPlexus)),
        (20, 34) => Some((HdCenter::Throat, HdCenter::Sacral)),
        (20, 57) => Some((HdCenter::Throat, HdCenter::Spleen)),
        (21, 45) => Some((HdCenter::Heart, HdCenter::Throat)),
        (23, 43) => Some((HdCenter::Throat, HdCenter::Ajna)),
        (24, 61) => Some((HdCenter::Ajna, HdCenter::Head)),
        (25, 51) => Some((HdCenter::SelfG, HdCenter::Heart)),
        (26, 44) => Some((HdCenter::Heart, HdCenter::Spleen)),
        (27, 50) => Some((HdCenter::Sacral, HdCenter::Spleen)),
        (28, 38) => Some((HdCenter::Spleen, HdCenter::Root)),
        (29, 46) => Some((HdCenter::Sacral, HdCenter::SelfG)),
        (30, 41) => Some((HdCenter::SolarPlexus, HdCenter::Root)),
        (32, 54) => Some((HdCenter::Spleen, HdCenter::Root)),
        (34, 57) => Some((HdCenter::Sacral, HdCenter::Spleen)),
        (35, 36) => Some((HdCenter::Throat, HdCenter::SolarPlexus)),
        (37, 40) => Some((HdCenter::SolarPlexus, HdCenter::Heart)),
        (39, 55) => Some((HdCenter::Root, HdCenter::SolarPlexus)),
        (42, 53) => Some((HdCenter::Sacral, HdCenter::Root)),
        (47, 64) => Some((HdCenter::Ajna, HdCenter::Head)),
        _ => None,
    }
}

pub fn degree_to_gate_line(degree: f64) -> (u8, u8, u8, u8, u8) {
    let gate_size = 360.0 / 64.0;
    let line_size = gate_size / 6.0;
    let color_size = line_size / 6.0;
    let tone_size = color_size / 6.0;
    let base_size = tone_size / 5.0;

    let adjusted = (degree - HD_START_DEGREE + 360.0) % 360.0;
    let gate_idx = (adjusted / gate_size).floor() as usize;
    let rem_gate = adjusted % gate_size;

    let line = (rem_gate / line_size).floor() as u8 + 1;
    let rem_line = rem_gate % line_size;

    let color = (rem_line / color_size).floor() as u8 + 1;
    let rem_color = rem_line % color_size;

    let tone = (rem_color / tone_size).floor() as u8 + 1;
    let rem_tone = rem_color % tone_size;

    let base = (rem_tone / base_size).floor() as u8 + 1;

    let gate = GATE_SEQUENCE[gate_idx % 64];
    (gate, line, color, tone, base)
}

pub fn get_planet_positions(
    engine: &AstroEngine,
    datetime: DateTime<Utc>,
) -> Result<HashMap<String, HdPlanetData>, AstroError> {
    let mut results = HashMap::new();

    // 1. Sun & Earth
    // Flag: SEFLG_SWIEPH = 2, but get_sun_longitude uses FFI internally or we can use swe_calc_ut.
    // Let's use get_sun_longitude or swe_calc_ut.
    let sun_deg = engine.get_sun_longitude(datetime)?;
    let (gate, line, color, tone, base) = degree_to_gate_line(sun_deg);
    results.insert(
        "Sun".to_string(),
        HdPlanetData {
            name: "Sun".to_string(),
            degree: sun_deg,
            gate,
            line,
            color,
            tone,
            base,
        },
    );

    let earth_deg = (sun_deg + 180.0) % 360.0;
    let (gate, line, color, tone, base) = degree_to_gate_line(earth_deg);
    results.insert(
        "Earth".to_string(),
        HdPlanetData {
            name: "Earth".to_string(),
            degree: earth_deg,
            gate,
            line,
            color,
            tone,
            base,
        },
    );

    // 2. Nodes (True Node)
    // Planet id: 11 is SE_TRUE_NODE
    let nn_deg = engine.get_planet_position(datetime, 11, 2)?; // SEFLG_SWIEPH = 2
    let (gate, line, color, tone, base) = degree_to_gate_line(nn_deg);
    results.insert(
        "N.Node".to_string(),
        HdPlanetData {
            name: "N.Node".to_string(),
            degree: nn_deg,
            gate,
            line,
            color,
            tone,
            base,
        },
    );

    let sn_deg = (nn_deg + 180.0) % 360.0;
    let (gate, line, color, tone, base) = degree_to_gate_line(sn_deg);
    results.insert(
        "S.Node".to_string(),
        HdPlanetData {
            name: "S.Node".to_string(),
            degree: sn_deg,
            gate,
            line,
            color,
            tone,
            base,
        },
    );

    // 3. Remaining Planets
    let planets = vec![
        ("Moon", 1),
        ("Mercury", 2),
        ("Venus", 3),
        ("Mars", 4),
        ("Jupiter", 5),
        ("Saturn", 6),
        ("Uranus", 7),
        ("Neptune", 8),
        ("Pluto", 9),
        ("Chiron", 15),
    ];

    for (name, planet_id) in planets {
        let pos = engine.get_planet_position(datetime, planet_id, 2)?;
        let (gate, line, color, tone, base) = degree_to_gate_line(pos);
        results.insert(
            name.to_string(),
            HdPlanetData {
                name: name.to_string(),
                degree: pos,
                gate,
                line,
                color,
                tone,
                base,
            },
        );
    }

    Ok(results)
}

pub fn get_defined_centers_and_channels(
    all_gates: &HashSet<u8>,
) -> (HashSet<HdCenter>, Vec<(u8, u8)>) {
    let mut defined_centers = HashSet::new();
    let mut active_channels = Vec::new();

    for &(g1, g2) in &CHANNELS {
        if all_gates.contains(&g1) && all_gates.contains(&g2) {
            if let Some((c1, c2)) = get_channel_centers(g1, g2) {
                defined_centers.insert(c1);
                defined_centers.insert(c2);
                active_channels.push((g1, g2));
            }
        }
    }

    (defined_centers, active_channels)
}

pub fn determine_defined_centers(active_channels: &[(u8, u8)]) -> HashSet<HdCenter> {
    let mut defined_centers = HashSet::new();
    for &(g1, g2) in active_channels {
        if let Some((c1, c2)) = get_channel_centers(g1, g2) {
            defined_centers.insert(c1);
            defined_centers.insert(c2);
        }
    }
    defined_centers
}

pub fn determine_type(defined_centers: &HashSet<HdCenter>, active_channels: &[(u8, u8)]) -> String {
    let has_sacral = defined_centers.contains(&HdCenter::Sacral);
    let has_throat = defined_centers.contains(&HdCenter::Throat);

    let motor_centers: HashSet<HdCenter> = [
        HdCenter::Sacral,
        HdCenter::Heart,
        HdCenter::SolarPlexus,
        HdCenter::Root,
    ]
    .iter()
    .cloned()
    .collect();

    let mut motor_to_throat = false;

    if has_throat {
        // Build graph of defined centers
        let mut adj = HashMap::new();
        for &center in &[
            HdCenter::Head,
            HdCenter::Ajna,
            HdCenter::Throat,
            HdCenter::SelfG,
            HdCenter::Heart,
            HdCenter::Sacral,
            HdCenter::Root,
            HdCenter::Spleen,
            HdCenter::SolarPlexus,
        ] {
            adj.insert(center, HashSet::new());
        }

        for &(g1, g2) in active_channels {
            if let Some((c1, c2)) = get_channel_centers(g1, g2) {
                adj.entry(c1).or_insert_with(HashSet::new).insert(c2);
                adj.entry(c2).or_insert_with(HashSet::new).insert(c1);
            }
        }

        // BFS to find if a motor center connects to Throat
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(HdCenter::Throat);

        while let Some(current) = queue.pop_front() {
            if !visited.contains(&current) {
                visited.insert(current);
                if motor_centers.contains(&current) {
                    motor_to_throat = true;
                    break;
                }
                if let Some(neighbors) = adj.get(&current) {
                    for &neigh in neighbors {
                        if !visited.contains(&neigh) {
                            queue.push_back(neigh);
                        }
                    }
                }
            }
        }
    }

    if defined_centers.is_empty() {
        "Reflector".to_string()
    } else if has_sacral && motor_to_throat {
        "Manifesting Generator".to_string()
    } else if has_sacral {
        "Generator".to_string()
    } else if motor_to_throat {
        "Manifestor".to_string()
    } else {
        "Projector".to_string()
    }
}

pub fn determine_authority(defined_centers: &HashSet<HdCenter>) -> String {
    let priority = vec![
        (HdCenter::SolarPlexus, "Emotional"),
        (HdCenter::Sacral, "Sacral"),
        (HdCenter::Spleen, "Splenic"),
        (HdCenter::Heart, "Ego"),
        (HdCenter::SelfG, "Self-Projected"),
    ];

    for (center, authority) in priority {
        if defined_centers.contains(&center) {
            return authority.to_string();
        }
    }

    if defined_centers.contains(&HdCenter::Throat)
        || defined_centers.contains(&HdCenter::Ajna)
        || defined_centers.contains(&HdCenter::Head)
    {
        "Mental/Outer".to_string()
    } else {
        "None/Outer".to_string()
    }
}

pub fn determine_strategy(chart_type: &str) -> String {
    match chart_type {
        "Manifestor" => "To Inform",
        "Generator" => "To Respond",
        "Manifesting Generator" => "To Respond",
        "Projector" => "To Wait for the Invitation",
        "Reflector" => "To Wait a Lunar Cycle",
        _ => "Unknown",
    }
    .to_string()
}

pub fn determine_not_self_theme(chart_type: &str) -> String {
    match chart_type {
        "Manifestor" => "Anger",
        "Generator" => "Frustration",
        "Manifesting Generator" => "Frustration / Anger",
        "Projector" => "Bitterness",
        "Reflector" => "Disappointment",
        _ => "Unknown",
    }
    .to_string()
}

pub fn determine_definition_type(
    defined_centers: &HashSet<HdCenter>,
    active_channels: &[(u8, u8)],
) -> String {
    if defined_centers.is_empty() {
        return "No Definition".to_string();
    }

    let mut adj = HashMap::new();
    for &center in defined_centers {
        adj.insert(center, HashSet::new());
    }

    for &(g1, g2) in active_channels {
        if let Some((c1, c2)) = get_channel_centers(g1, g2) {
            if defined_centers.contains(&c1) && defined_centers.contains(&c2) {
                adj.entry(c1).or_insert_with(HashSet::new).insert(c2);
                adj.entry(c2).or_insert_with(HashSet::new).insert(c1);
            }
        }
    }

    let mut visited = HashSet::new();
    let mut components = 0;

    for &center in defined_centers {
        if !visited.contains(&center) {
            components += 1;
            let mut queue = VecDeque::new();
            queue.push_back(center);

            while let Some(current) = queue.pop_front() {
                if !visited.contains(&current) {
                    visited.insert(current);
                    if let Some(neighbors) = adj.get(&current) {
                        for &neigh in neighbors {
                            if !visited.contains(&neigh) {
                                queue.push_back(neigh);
                            }
                        }
                    }
                }
            }
        }
    }

    match components {
        1 => "Single Definition".to_string(),
        2 => "Split Definition".to_string(),
        3 => "Triple Split Definition".to_string(),
        4 => "Quadruple Split Definition".to_string(),
        _ => format!("{} Splits", components),
    }
}

pub fn determine_incarnation_cross(sun_gate: u8, profile: &str) -> String {
    let parts: Vec<&str> = profile.split('/').collect();
    if parts.len() == 2 {
        let p_line: u8 = parts[0].parse().unwrap_or(1);
        let angle = match p_line {
            1 | 2 | 3 | 4 => "Right Angle",
            5 | 6 => "Left Angle",
            _ => "Right Angle",
        };
        let final_angle = if p_line == 4 && parts[1] == "1" {
            "Juxtaposition"
        } else {
            angle
        };
        db::get_incarnation_cross_name(sun_gate, final_angle)
    } else {
        format!("Unknown Cross of Gate {}", sun_gate)
    }
}

pub fn calculate_human_design(
    engine: &AstroEngine,
    birth_time: DateTime<Utc>,
) -> Result<HumanDesignResult, HdError> {
    // 1. Personality Positions
    let personality = get_planet_positions(engine, birth_time)?;

    // 2. Design Date Calculation
    // Design time is when the Sun was exactly 88 degrees prior to its birth position.
    let p_sun = personality.get("Sun").unwrap();
    let target_design_deg = (p_sun.degree - 88.0 + 360.0) % 360.0;

    // Use AstroEngine's find_time_for_longitude starting approx 88 days before birth
    use chrono::Duration;
    let approx_design_time = birth_time - Duration::days(88);
    let design_time = engine.find_time_for_longitude(approx_design_time, target_design_deg)?;

    // Calculate Design Positions
    let design = get_planet_positions(engine, design_time)?;

    // 3. Collect active gates
    let mut all_gates = HashSet::new();
    for p in personality.values() {
        all_gates.insert(p.gate);
    }
    for p in design.values() {
        all_gates.insert(p.gate);
    }

    // 4. Calculate centers and channels
    let (defined_set, active_channels) = get_defined_centers_and_channels(&all_gates);

    let mut defined_centers: Vec<HdCenter> = defined_set.iter().cloned().collect();
    defined_centers.sort_by_key(|c| *c as u8);

    let mut undefined_centers: Vec<HdCenter> = [
        HdCenter::Head,
        HdCenter::Ajna,
        HdCenter::Throat,
        HdCenter::SelfG,
        HdCenter::Heart,
        HdCenter::Sacral,
        HdCenter::Root,
        HdCenter::Spleen,
        HdCenter::SolarPlexus,
    ]
    .iter()
    .filter(|c| !defined_set.contains(c))
    .cloned()
    .collect();
    undefined_centers.sort_by_key(|c| *c as u8);

    // 5. Profile
    let p_sun = personality.get("Sun").unwrap();
    let p_sun_line = p_sun.line;
    let p_sun_gate = p_sun.gate;
    let d_sun_line = design.get("Sun").unwrap().line;
    let profile = format!("{}/{}", p_sun_line, d_sun_line);

    // 6. Type and Authority
    let chart_type = determine_type(&defined_set, &active_channels);
    let authority = determine_authority(&defined_set);
    let strategy = determine_strategy(&chart_type);
    let not_self_theme = determine_not_self_theme(&chart_type);
    let definition_type = determine_definition_type(&defined_set, &active_channels);
    let incarnation_cross = determine_incarnation_cross(p_sun_gate, &profile);

    let active_gates: Vec<u8> = all_gates.into_iter().collect();
    let mut active_gates = active_gates;
    active_gates.sort();

    Ok(HumanDesignResult {
        chart_type,
        profile,
        authority,
        defined_centers,
        undefined_centers,
        personality,
        design,
        active_gates,
        active_channels,
        definition_type,
        strategy,
        not_self_theme,
        incarnation_cross,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_degree_to_gate_line() {
        // Gate 25 starts at 358.25. Line 1 spans 358.25 to 359.1875.
        let (gate, line, _, _, _) = degree_to_gate_line(358.5);
        assert_eq!(gate, 25);
        assert_eq!(line, 1);

        let (gate, line, _, _, _) = degree_to_gate_line(359.25);
        assert_eq!(gate, 25);
        assert_eq!(line, 2);
    }

    #[test]
    fn test_calculate_human_design_basic() {
        let engine = AstroEngine::new();
        // May 15, 1990 at 10:00 AM UTC
        let birth = Utc.with_ymd_and_hms(1990, 5, 15, 10, 0, 0).unwrap();
        let res = calculate_human_design(&engine, birth);
        assert!(res.is_ok());
        let result = res.unwrap();

        assert!(!result.profile.is_empty());
        assert!(!result.chart_type.is_empty());
        assert!(!result.authority.is_empty());
        assert!(!result.personality.is_empty());
        assert!(!result.design.is_empty());
    }
}
