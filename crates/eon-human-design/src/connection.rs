use crate::{HdCenter, HumanDesignResult};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HumanDesignConnectionResult {
    pub electromagnetic_channels: Vec<(u8, u8)>, // Each has half
    pub compromise_channels: Vec<(u8, u8)>,      // One has whole, one has half
    pub dominance_channels: Vec<(u8, u8)>,       // One has whole, one has none
    pub companionship_channels: Vec<(u8, u8)>,   // Both have the whole channel
    pub defined_centers: HashSet<HdCenter>,      // Combined defined centers
}

pub fn calculate_connection_chart(
    person1: &HumanDesignResult,
    person2: &HumanDesignResult,
) -> HumanDesignConnectionResult {
    let mut electromagnetic = Vec::new();
    let mut compromise = Vec::new();
    let mut dominance = Vec::new();
    let mut companionship = Vec::new();

    let channels = vec![
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

    let has_gate = |res: &HumanDesignResult, gate: u8| -> bool {
        res.personality.values().any(|p| p.gate == gate)
            || res.design.values().any(|p| p.gate == gate)
    };

    let has_channel = |res: &HumanDesignResult, g1: u8, g2: u8| -> bool {
        has_gate(res, g1) && has_gate(res, g2)
    };

    for &(g1, g2) in &channels {
        let p1_has_c = has_channel(person1, g1, g2);
        let p2_has_c = has_channel(person2, g1, g2);

        let p1_has_half_1 = has_gate(person1, g1) && !has_gate(person1, g2);
        let p1_has_half_2 = !has_gate(person1, g1) && has_gate(person1, g2);

        let p2_has_half_1 = has_gate(person2, g1) && !has_gate(person2, g2);
        let p2_has_half_2 = !has_gate(person2, g1) && has_gate(person2, g2);

        let p1_has_half = p1_has_half_1 || p1_has_half_2;
        let p2_has_half = p2_has_half_1 || p2_has_half_2;
        let p1_has_none = !has_gate(person1, g1) && !has_gate(person1, g2);
        let p2_has_none = !has_gate(person2, g1) && !has_gate(person2, g2);

        if p1_has_c && p2_has_c {
            companionship.push((g1, g2));
        } else if (p1_has_c && p2_has_half) || (p2_has_c && p1_has_half) {
            compromise.push((g1, g2));
        } else if (p1_has_c && p2_has_none) || (p2_has_c && p1_has_none) {
            dominance.push((g1, g2));
        } else if (p1_has_half_1 && p2_has_half_2) || (p1_has_half_2 && p2_has_half_1) {
            electromagnetic.push((g1, g2));
        }
    }

    let mut combined_active_channels = Vec::new();
    combined_active_channels.extend(&companionship);
    combined_active_channels.extend(&compromise);
    combined_active_channels.extend(&dominance);
    combined_active_channels.extend(&electromagnetic);

    let combined_defined_set = crate::determine_defined_centers(&combined_active_channels);

    HumanDesignConnectionResult {
        electromagnetic_channels: electromagnetic,
        compromise_channels: compromise,
        dominance_channels: dominance,
        companionship_channels: companionship,
        defined_centers: combined_defined_set,
    }
}
