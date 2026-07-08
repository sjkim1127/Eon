use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DreamCenter {
    LightField,
    DemonRealm,
    EarthPlane,
    Ocean,
    Chaos,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DreamRaveResult {
    pub active_gates: HashSet<u8>,
    pub defined_centers: HashSet<DreamCenter>,
    pub active_channels: Vec<(u8, u8)>,
}

pub fn calculate_dream_rave(
    personality: &std::collections::HashMap<String, crate::HdPlanetData>,
    design: &std::collections::HashMap<String, crate::HdPlanetData>,
) -> DreamRaveResult {
    let mut all_waking_gates = HashSet::new();
    for p in personality.values() {
        all_waking_gates.insert(p.gate);
    }
    for p in design.values() {
        all_waking_gates.insert(p.gate);
    }

    let dream_gates: HashSet<u8> = [
        62, 20, 12, // Earth Plane
        50, 57, 53, // Demon Realm
        27, 42, 19, // Chaos
        60, 38, 58, // Ocean
        1, 2, 8, // Light Field (approx)
    ]
    .iter()
    .cloned()
    .collect();

    let active_dream_gates: HashSet<u8> = all_waking_gates
        .intersection(&dream_gates)
        .cloned()
        .collect();

    let dream_channels = vec![(62, 20), (20, 12)];

    let mut defined_centers = HashSet::new();
    let mut active_channels = Vec::new();

    for &(g1, g2) in &dream_channels {
        if active_dream_gates.contains(&g1) && active_dream_gates.contains(&g2) {
            active_channels.push((g1, g2));
            defined_centers.insert(DreamCenter::EarthPlane);
        }
    }

    DreamRaveResult {
        active_gates: active_dream_gates,
        defined_centers,
        active_channels,
    }
}
