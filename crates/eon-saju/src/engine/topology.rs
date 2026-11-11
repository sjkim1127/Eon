use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use serde::{Deserialize, Serialize};

/// Qi network node status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QiNode {
    pub element: Element,
    pub capacity: f32, // Node capacity based on chart element counts
    pub output: f32,   // Energy output routed to next node
}

/// Network flow topology analysis outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyAnalysis {
    pub nodes: Vec<QiNode>,
    pub throughput: f32, // System-wide energy circulation efficiency (0.0 ~ 1.0)
    pub bottleneck: Option<Element>, // Bottleneck element with greatest congestion
}

pub struct QiTopology;

impl QiTopology {
    pub fn analyze(pillars: &FourPillars) -> TopologyAnalysis {
        let counts = pillars.element_counts();
        let mut nodes = Vec::new();

        // 1. Calculate capacity for each node (10 bandwidth per character)
        for i in 0..5 {
            let el = Element::from_index(i);
            let count = counts
                .iter()
                .find(|(e, _)| *e == el)
                .map(|(_, c)| *c)
                .unwrap_or(0);
            nodes.push(QiNode {
                element: el,
                capacity: count as f32 * 10.0,
                output: 0.0,
            });
        }

        // 2. Compute generating flow (Simulated Network Flow: Wood -> Fire -> Earth -> Metal -> Water -> Wood)
        let mut total_output = 0.0;
        let mut min_efficiency = 1.0;
        let mut bottleneck = None;

        for i in 0..5 {
            let current_idx = i;
            let next_idx = (i + 1) % 5;

            let cap_current = nodes[current_idx].capacity;
            let cap_next = nodes[next_idx].capacity;

            // Flow(Throughput) = min(Source_Bandwidth, Drain_Bandwidth)
            let flow = if cap_current > 0.0 && cap_next > 0.0 {
                cap_current.min(cap_next)
            } else if cap_current > 0.0 {
                cap_current * 0.2 // Maintain minimal baseline liquidity
            } else {
                0.0
            };

            nodes[current_idx].output = flow;
            total_output += flow;

            // Efficiency calculation and bottleneck detection
            if cap_current > 0.0 {
                let efficiency = flow / cap_current;
                if efficiency < min_efficiency {
                    min_efficiency = efficiency;
                    bottleneck = Some(nodes[current_idx].element);
                }
            }
        }

        // 3. Reflect attenuation from controlling relationships (Resistance)
        for i in 0..5 {
            let el = nodes[i].element;
            let controller = el.controlled_by();
            let controller_idx = controller.index() as usize;
            let resistance = nodes[controller_idx].capacity * 0.15;

            nodes[i].output = (nodes[i].output - resistance).max(0.0);
        }

        let system_throughput = (total_output / 40.0).min(1.0);

        TopologyAnalysis {
            nodes,
            throughput: system_throughput,
            bottleneck,
        }
    }
}

impl std::fmt::Display for TopologyAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[Qi Flow Network Topology]")?;
        writeln!(
            f,
            "Throughput: {:.1}% (System Efficiency)",
            self.throughput * 100.0
        )?;

        if let Some(bn) = self.bottleneck {
            writeln!(
                f,
                "Bottleneck: {:?} Node (Bandwidth Limitation Detected)",
                bn
            )?;
        }

        writeln!(f, "Node Traffic Status:")?;
        for node in &self.nodes {
            let bar_len = (node.output / 2.0) as usize;
            let bar = "#".repeat(bar_len);
            writeln!(
                f,
                "  {:<6}: {:<15} (Output: {:.1})",
                format!("{:?}", node.element),
                bar,
                node.output
            )?;
        }
        Ok(())
    }
}
