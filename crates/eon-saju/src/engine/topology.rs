use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use serde::{Deserialize, Serialize};

/// 에너지 노드 상태
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QiNode {
    pub element: Element,
    pub capacity: f32, // 노드가 수용 가능한 에너지량 (원국의 글자 수 기반)
    pub output: f32,   // 실제로 다음 노드로 흘려보내는 에너지량
}

/// 에너지 흐름 분석 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyAnalysis {
    pub nodes: Vec<QiNode>,
    pub throughput: f32,             // 시스템 전체 에너지 유동 효율 (0.0 ~ 1.0)
    pub bottleneck: Option<Element>, // 가장 흐름이 정체되는 구간
}

pub struct QiTopology;

impl QiTopology {
    pub fn analyze(pillars: &FourPillars) -> TopologyAnalysis {
        let counts = pillars.element_counts();
        let mut nodes = Vec::new();

        // 1. 각 노드(오행)의 Capacity 계산
        for i in 0..5 {
            let el = Element::from_index(i);
            let count = counts
                .iter()
                .find(|(e, _)| *e == el)
                .map(|(_, c)| *c)
                .unwrap_or(0);
            nodes.push(QiNode {
                element: el,
                capacity: count as f32 * 10.0, // 한 글자당 10의 대역폭
                output: 0.0,
            });
        }

        // 2. 상생(Flow) 흐름 계산 (Simulated Network Flow)
        // 목 -> 화 -> 토 -> 금 -> 수 -> 목
        let mut total_output = 0.0;
        let mut min_efficiency = 1.0;
        let mut bottleneck = None;

        for i in 0..5 {
            let current_idx = i;
            let next_idx = (i + 1) % 5;

            let cap_current = nodes[current_idx].capacity;
            let cap_next = nodes[next_idx].capacity;

            // Flow(Throughput) = min(Source_Bandwidth, Drain_Bandwidth)
            // 에너지는 현재 노드에 충분히 있고(Source), 다음 노드에서 수용 가능해야(Drain) 흐름
            let flow = if cap_current > 0.0 && cap_next > 0.0 {
                cap_current.min(cap_next)
            } else if cap_current > 0.0 {
                // 다음 노드가 없으면 에너지가 고임 (Congestion)
                cap_current * 0.2 // 최소한의 유동성만 유지
            } else {
                0.0
            };

            nodes[current_idx].output = flow;
            total_output += flow;

            // 효율성 계산 및 병목 탐지
            if cap_current > 0.0 {
                let efficiency = flow / cap_current;
                if efficiency < min_efficiency {
                    min_efficiency = efficiency;
                    bottleneck = Some(nodes[current_idx].element);
                }
            }
        }

        // 3. 상극(Resistance)에 의한 손실 반영
        // 극(Control)을 심하게 받으면 지연 시간(Latency) 증가로 인한 효율 감소
        for i in 0..5 {
            let el = nodes[i].element;
            let controller = el.controlled_by();
            let controller_idx = controller.index() as usize;
            let resistance = nodes[controller_idx].capacity * 0.15; // 상극 노드의 크기만큼 저항 발생

            nodes[i].output = (nodes[i].output - resistance).max(0.0);
        }

        let system_throughput = (total_output / 40.0).min(1.0); // 최대 8글자 완벽 유동 시 1.0 기준

        TopologyAnalysis {
            nodes,
            throughput: system_throughput,
            bottleneck,
        }
    }
}

impl std::fmt::Display for TopologyAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【Qi Flow Network Topology】")?;
        writeln!(
            f,
            "Throughput: {:.1}% (System Efficiency)",
            self.throughput * 100.0
        )?;

        if let Some(bn) = self.bottleneck {
            writeln!(
                f,
                "Bottleneck: {} Node (Bandwidth Limitation Detected)",
                bn.hangul()
            )?;
        }

        writeln!(f, "Node Traffic Status:")?;
        for node in &self.nodes {
            let bar_len = (node.output / 2.0) as usize;
            let bar = "⚡".repeat(bar_len);
            writeln!(
                f,
                "  {:<4}: {:<15} (Output: {:.1})",
                node.element.hangul(),
                bar,
                node.output
            )?;
        }
        Ok(())
    }
}
