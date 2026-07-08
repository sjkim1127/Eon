#![allow(non_snake_case)]
use dioxus::prelude::*;
use std::collections::HashSet;
use eon_human_design::dream_rave::DreamCenter;

#[derive(Props, PartialEq, Clone)]
pub struct DreamRaveBodyGraphProps {
    pub active_gates: HashSet<u8>,
    pub defined_centers: HashSet<DreamCenter>,
    pub active_channels: Vec<(u8, u8)>,
}

#[component]
pub fn DreamRaveBodyGraph(props: DreamRaveBodyGraphProps) -> Element {
    let defined_fill = "#fef08a"; // Soft yellow for dream
    let open_fill = "#1e293b";
    let outline_color = "#334155";
    let gate_text_color = "fill-slate-500";

    let has_center = |c: DreamCenter| -> &'static str {
        if props.defined_centers.contains(&c) {
            defined_fill
        } else {
            open_fill
        }
    };

    let has_gate = |g: u8| -> bool {
        props.active_gates.contains(&g)
    };

    let get_gate_color = |g: u8| -> &'static str {
        if has_gate(g) {
            "#818cf8" // Dreamy purple for active
        } else {
            "#e2e8f0"
        }
    };

    rsx! {
        div { class: "w-full flex justify-center items-center py-6 bg-slate-900/50 rounded-2xl border border-indigo-900/50",
            svg {
                width: "100%", height: "auto", view_box: "0 0 400 600", class: "drop-shadow-2xl max-w-sm",
                
                // Earth Plane (Center)
                rect { x: "160", y: "260", width: "80", height: "80", rx: "8", fill: has_center(DreamCenter::EarthPlane), stroke: outline_color, stroke_width: "4" }
                text { x: "185", y: "305", class: "text-xs font-bold {gate_text_color}", "Earth" }
                
                // Demon Realm (Left)
                rect { x: "40", y: "260", width: "80", height: "80", rx: "8", fill: has_center(DreamCenter::DemonRealm), stroke: outline_color, stroke_width: "4" }
                text { x: "65", y: "305", class: "text-xs font-bold {gate_text_color}", "Demon" }
                
                // Light Field (Top)
                rect { x: "160", y: "140", width: "80", height: "80", rx: "8", fill: has_center(DreamCenter::LightField), stroke: outline_color, stroke_width: "4" }
                text { x: "185", y: "185", class: "text-xs font-bold {gate_text_color}", "Light" }

                // Chaos (Right)
                rect { x: "280", y: "260", width: "80", height: "80", rx: "8", fill: has_center(DreamCenter::Chaos), stroke: outline_color, stroke_width: "4" }
                text { x: "305", y: "305", class: "text-xs font-bold {gate_text_color}", "Chaos" }

                // Ocean (Bottom)
                rect { x: "160", y: "380", width: "80", height: "80", rx: "8", fill: has_center(DreamCenter::Ocean), stroke: outline_color, stroke_width: "4" }
                text { x: "185", y: "425", class: "text-xs font-bold {gate_text_color}", "Ocean" }

                // Connectors (simplified)
                line { x1: "120", y1: "300", x2: "160", y2: "300", stroke: get_gate_color(57), stroke_width: "6" }
                line { x1: "240", y1: "300", x2: "280", y2: "300", stroke: get_gate_color(27), stroke_width: "6" }
                line { x1: "200", y1: "220", x2: "200", y2: "260", stroke: get_gate_color(20), stroke_width: "6" }
                line { x1: "200", y1: "340", x2: "200", y2: "380", stroke: get_gate_color(60), stroke_width: "6" }
            }
        }
    }
}
