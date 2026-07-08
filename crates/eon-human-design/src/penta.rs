use crate::HumanDesignResult;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PentaGateInfo {
    pub gate: u8,
    pub activated_by: Vec<usize>, // indices of the people in the input array
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PentaChannelInfo {
    pub channel: (u8, u8),
    pub name: String,
    pub is_active: bool,
    pub missing_gates: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PentaResult {
    pub participants: usize,
    pub active_gates: HashMap<u8, PentaGateInfo>,
    pub channels: Vec<PentaChannelInfo>,
    pub fully_defined_channels: usize,
    pub gaps: Vec<u8>, // Gates needed to complete any partially defined channel
}

pub const PENTA_CHANNELS: [((u8, u8), &str); 6] = [
    ((5, 15), "Flow / Rhythm"),
    ((14, 2), "Capacity / Resources"),
    ((29, 46), "Discovery / Commitment"),
    ((7, 31), "Role / Future"),
    ((1, 8), "Direction / Present"),
    ((13, 33), "Memory / Past"),
];

pub fn calculate_penta(charts: &[HumanDesignResult]) -> PentaResult {
    let mut active_gates: HashMap<u8, PentaGateInfo> = HashMap::new();

    // Collect all active gates from all participants
    for (i, chart) in charts.iter().enumerate() {
        for &gate in &chart.active_gates {
            active_gates
                .entry(gate)
                .or_insert(PentaGateInfo {
                    gate,
                    activated_by: Vec::new(),
                })
                .activated_by
                .push(i);
        }
    }

    let mut penta_channels = Vec::new();
    let mut fully_defined_channels = 0;
    let mut gaps = HashSet::new();

    for &((g1, g2), name) in &PENTA_CHANNELS {
        let has_g1 = active_gates.contains_key(&g1);
        let has_g2 = active_gates.contains_key(&g2);
        let is_active = has_g1 && has_g2;

        let mut missing = Vec::new();
        if !is_active {
            if has_g1 && !has_g2 {
                missing.push(g2);
                gaps.insert(g2);
            } else if !has_g1 && has_g2 {
                missing.push(g1);
                gaps.insert(g1);
            }
            // If neither is present, it's not a gap, it's just an empty channel.
        } else {
            fully_defined_channels += 1;
        }

        penta_channels.push(PentaChannelInfo {
            channel: (g1, g2),
            name: name.to_string(),
            is_active,
            missing_gates: missing,
        });
    }

    let mut gaps_vec: Vec<u8> = gaps.into_iter().collect();
    gaps_vec.sort();

    PentaResult {
        participants: charts.len(),
        active_gates,
        channels: penta_channels,
        fully_defined_channels,
        gaps: gaps_vec,
    }
}
