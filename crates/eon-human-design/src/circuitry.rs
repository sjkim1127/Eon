//! Circuitry: every one of the 36 channels belongs to exactly one sub-circuit group, and every
//! group belongs to one of the three main circuits (Tribal, Individual, Collective).
//!
//! | Circuit    | Group         | Channels                                                  |
//! |------------|---------------|-----------------------------------------------------------|
//! | Tribal     | Ego           | 19-49, 21-45, 26-44, 37-40                                |
//! | Tribal     | Defense       | 6-59, 27-50, 32-54                                        |
//! | Individual | Knowing       | 3-60, 12-22, 23-43, 24-61, 28-38, 39-55                   |
//! | Individual | Centering     | 1-8, 2-14, 25-51                                          |
//! | Individual | Integration   | 10-20, 10-34, 10-57, 20-34, 20-57, 34-57                  |
//! | Collective | Understanding | 4-63, 5-15, 7-31, 9-52, 16-48, 17-62, 18-58               |
//! | Collective | Sensing       | 11-56, 13-33, 29-46, 30-41, 35-36, 42-53, 47-64           |

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Circuit {
    Tribal,
    Individual,
    Collective,
}

impl Circuit {
    pub fn name_en(&self) -> &'static str {
        match self {
            Circuit::Tribal => "Tribal",
            Circuit::Individual => "Individual",
            Circuit::Collective => "Collective",
        }
    }

    pub fn name_ko(&self) -> &'static str {
        match self {
            Circuit::Tribal => "부족 회로",
            Circuit::Individual => "개인 회로",
            Circuit::Collective => "집단 회로",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CircuitGroup {
    Ego,
    Defense,
    Knowing,
    Centering,
    Integration,
    Understanding,
    Sensing,
}

impl CircuitGroup {
    pub const ALL: [CircuitGroup; 7] = [
        CircuitGroup::Ego,
        CircuitGroup::Defense,
        CircuitGroup::Knowing,
        CircuitGroup::Centering,
        CircuitGroup::Integration,
        CircuitGroup::Understanding,
        CircuitGroup::Sensing,
    ];

    pub fn circuit(&self) -> Circuit {
        match self {
            CircuitGroup::Ego | CircuitGroup::Defense => Circuit::Tribal,
            CircuitGroup::Knowing | CircuitGroup::Centering | CircuitGroup::Integration => {
                Circuit::Individual
            }
            CircuitGroup::Understanding | CircuitGroup::Sensing => Circuit::Collective,
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            CircuitGroup::Ego => "Ego",
            CircuitGroup::Defense => "Defense",
            CircuitGroup::Knowing => "Knowing",
            CircuitGroup::Centering => "Centering",
            CircuitGroup::Integration => "Integration",
            CircuitGroup::Understanding => "Understanding",
            CircuitGroup::Sensing => "Sensing",
        }
    }

    pub fn name_ko(&self) -> &'static str {
        match self {
            CircuitGroup::Ego => "에고 회로",
            CircuitGroup::Defense => "방어 회로",
            CircuitGroup::Knowing => "앎의 회로",
            CircuitGroup::Centering => "중심잡기 회로",
            CircuitGroup::Integration => "통합 회로",
            CircuitGroup::Understanding => "이해(논리) 회로",
            CircuitGroup::Sensing => "감각(추상) 회로",
        }
    }
}

/// Sub-circuit group of a channel; `None` when `(g1, g2)` is not one of the 36 channels.
/// The order of the two gates does not matter.
pub fn channel_circuit_group(g1: u8, g2: u8) -> Option<CircuitGroup> {
    let pair = if g1 < g2 { (g1, g2) } else { (g2, g1) };
    Some(match pair {
        (19, 49) | (21, 45) | (26, 44) | (37, 40) => CircuitGroup::Ego,
        (6, 59) | (27, 50) | (32, 54) => CircuitGroup::Defense,
        (3, 60) | (12, 22) | (23, 43) | (24, 61) | (28, 38) | (39, 55) => CircuitGroup::Knowing,
        (1, 8) | (2, 14) | (25, 51) => CircuitGroup::Centering,
        (10, 20) | (10, 34) | (10, 57) | (20, 34) | (20, 57) | (34, 57) => {
            CircuitGroup::Integration
        }
        (4, 63) | (5, 15) | (7, 31) | (9, 52) | (16, 48) | (17, 62) | (18, 58) => {
            CircuitGroup::Understanding
        }
        (11, 56) | (13, 33) | (29, 46) | (30, 41) | (35, 36) | (42, 53) | (47, 64) => {
            CircuitGroup::Sensing
        }
        _ => return None,
    })
}

/// Main circuit of a channel.
pub fn channel_circuit(g1: u8, g2: u8) -> Option<Circuit> {
    channel_circuit_group(g1, g2).map(|g| g.circuit())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelCircuit {
    pub channel: (u8, u8),
    pub circuit: Circuit,
    pub group: CircuitGroup,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CircuitryResult {
    /// Circuit classification of every defined channel (same order as `active_channels`).
    pub channels: Vec<ChannelCircuit>,
    /// Number of defined channels per main circuit (circuits without a channel are omitted).
    pub circuit_counts: BTreeMap<Circuit, usize>,
    /// Number of defined channels per sub-circuit group (groups without a channel are omitted).
    pub group_counts: BTreeMap<CircuitGroup, usize>,
    /// The main circuit with the most defined channels; `None` without channels or on a tie.
    pub dominant_circuit: Option<Circuit>,
    /// The sub-circuit group with the most defined channels; `None` without channels or on a tie.
    pub dominant_group: Option<CircuitGroup>,
}

fn unique_max<K: Copy + Ord>(counts: &BTreeMap<K, usize>) -> Option<K> {
    let max = *counts.values().max()?;
    let mut top = counts.iter().filter(|(_, &n)| n == max);
    let first = top.next()?;
    if top.next().is_some() {
        None
    } else {
        Some(*first.0)
    }
}

/// Classify the defined channels of a chart.
pub fn analyze_circuitry(active_channels: &[(u8, u8)]) -> CircuitryResult {
    let mut result = CircuitryResult::default();
    for &(a, b) in active_channels {
        let Some(group) = channel_circuit_group(a, b) else {
            continue;
        };
        let circuit = group.circuit();
        result.channels.push(ChannelCircuit {
            channel: (a, b),
            circuit,
            group,
        });
        *result.circuit_counts.entry(circuit).or_insert(0) += 1;
        *result.group_counts.entry(group).or_insert(0) += 1;
    }
    result.dominant_circuit = unique_max(&result.circuit_counts);
    result.dominant_group = unique_max(&result.group_counts);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CHANNELS;

    #[test]
    fn all_36_channels_are_classified_exactly_once() {
        let mut seen = std::collections::HashSet::new();
        for &(a, b) in &CHANNELS {
            let group = channel_circuit_group(a, b).unwrap_or_else(|| panic!("{a}-{b}"));
            assert_eq!(channel_circuit_group(b, a), Some(group));
            assert_eq!(channel_circuit(a, b), Some(group.circuit()));
            assert!(seen.insert((a, b)));
        }
        assert_eq!(seen.len(), 36);

        // Nothing outside the 36 channels is classified.
        let mut classified = 0;
        for a in 1..=64u8 {
            for b in (a + 1)..=64u8 {
                if channel_circuit_group(a, b).is_some() {
                    classified += 1;
                    assert!(CHANNELS.contains(&(a, b)), "{a}-{b} is not a channel");
                }
            }
        }
        assert_eq!(classified, 36);
    }

    #[test]
    fn group_sizes_match_the_standard_tables() {
        let count = |g: CircuitGroup| {
            CHANNELS
                .iter()
                .filter(|&&(a, b)| channel_circuit_group(a, b) == Some(g))
                .count()
        };
        assert_eq!(count(CircuitGroup::Ego), 4);
        assert_eq!(count(CircuitGroup::Defense), 3);
        assert_eq!(count(CircuitGroup::Knowing), 6);
        assert_eq!(count(CircuitGroup::Centering), 3);
        assert_eq!(count(CircuitGroup::Integration), 6);
        assert_eq!(count(CircuitGroup::Understanding), 7);
        assert_eq!(count(CircuitGroup::Sensing), 7);
        // Tribal 7, Individual 15, Collective 14.
        let by = |c: Circuit| {
            CHANNELS
                .iter()
                .filter(|&&(a, b)| channel_circuit(a, b) == Some(c))
                .count()
        };
        assert_eq!(by(Circuit::Tribal), 7);
        assert_eq!(by(Circuit::Individual), 15);
        assert_eq!(by(Circuit::Collective), 14);
    }

    #[test]
    fn analyze_counts_and_dominant() {
        // 1990-05-15 01:30 UTC golden chart channels.
        let r = analyze_circuitry(&[(13, 33), (23, 43), (24, 61), (30, 41), (32, 54)]);
        assert_eq!(r.circuit_counts[&Circuit::Collective], 2);
        assert_eq!(r.circuit_counts[&Circuit::Individual], 2);
        assert_eq!(r.circuit_counts[&Circuit::Tribal], 1);
        assert_eq!(r.dominant_circuit, None, "2-2-1 is a tie");
        assert_eq!(r.group_counts[&CircuitGroup::Knowing], 2);
        assert_eq!(r.group_counts[&CircuitGroup::Sensing], 2);
        assert_eq!(r.group_counts[&CircuitGroup::Defense], 1);

        let r = analyze_circuitry(&[(7, 31), (16, 48), (21, 45)]);
        assert_eq!(r.dominant_circuit, Some(Circuit::Collective));
        assert_eq!(r.dominant_group, Some(CircuitGroup::Understanding));

        let r = analyze_circuitry(&[]);
        assert!(r.channels.is_empty() && r.dominant_circuit.is_none());
    }
}
