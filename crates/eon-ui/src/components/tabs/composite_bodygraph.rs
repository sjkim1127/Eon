#![allow(non_snake_case)]
use dioxus::prelude::*;
use eon_human_design::connection::HumanDesignConnectionResult;
use eon_human_design::HdCenter;

#[derive(Props, PartialEq, Clone)]
pub struct CompositeBodyGraphProps {
    pub result: HumanDesignConnectionResult,
}

#[component]
pub fn CompositeBodyGraph(props: CompositeBodyGraphProps) -> Element {
    let defined_fill = "#f8fafc";
    let open_fill = "#1e293b";
    let outline_color = "#334155";
    let gate_text_color = "fill-slate-500";

    let has_center = |c: HdCenter| -> &'static str {
        if props.result.defined_centers.contains(&c) {
            defined_fill
        } else {
            open_fill
        }
    };

    let get_channel_color = |g1: u8, g2: u8| -> &'static str {
        if props.result.electromagnetic_channels.contains(&(g1, g2))
            || props.result.electromagnetic_channels.contains(&(g2, g1))
        {
            "#facc15" // Yellow/Spark for Electromagnetic
        } else if props.result.compromise_channels.contains(&(g1, g2))
            || props.result.compromise_channels.contains(&(g2, g1))
        {
            "#a855f7" // Purple for Compromise
        } else if props.result.dominance_channels.contains(&(g1, g2))
            || props.result.dominance_channels.contains(&(g2, g1))
        {
            "#3b82f6" // Blue for Dominance (A over B or B over A)
        } else if props.result.companionship_channels.contains(&(g1, g2))
            || props.result.companionship_channels.contains(&(g2, g1))
        {
            "#22c55e" // Green for Companionship
        } else {
            "#e2e8f0" // Default undefined
        }
    };

    rsx! {
        div { class: "w-full flex justify-center items-center py-6 bg-slate-900/30 rounded-2xl border border-slate-800",
            svg {
                width: "100%", height: "auto", view_box: "0 0 400 600", class: "drop-shadow-2xl max-w-sm",
                // Example Single Channel to demonstrate the concept.

                // Channel 1-8
                line { x1: "200", y1: "310", x2: "200", y2: "270", stroke: get_channel_color(1, 8), stroke_width: "6", stroke_linecap: "round" }
                text { x: "196", y: "308", class: "text-[8px] font-bold {gate_text_color}", "1" }
                text { x: "196", y: "276", class: "text-[8px] font-bold {gate_text_color}", "8" }

                // Head
                polygon { points: "200,30 260,80 140,80", fill: has_center(HdCenter::Head), stroke: outline_color, stroke_width: "4" }
                // Ajna
                polygon { points: "200,160 260,110 140,110", fill: has_center(HdCenter::Ajna), stroke: outline_color, stroke_width: "4" }
                // Throat
                rect { x: "170", y: "210", width: "60", height: "60", rx: "8", fill: has_center(HdCenter::Throat), stroke: outline_color, stroke_width: "4" }
                // Self
                polygon { points: "200,310 240,350 200,390 160,350", fill: has_center(HdCenter::SelfG), stroke: outline_color, stroke_width: "4" }
                // Heart
                polygon { points: "250,280 290,260 290,300", fill: has_center(HdCenter::Heart), stroke: outline_color, stroke_width: "4" }
                // Sacral
                rect { x: "170", y: "410", width: "60", height: "60", rx: "8", fill: has_center(HdCenter::Sacral), stroke: outline_color, stroke_width: "4" }
                // Root
                rect { x: "170", y: "510", width: "60", height: "60", rx: "8", fill: has_center(HdCenter::Root), stroke: outline_color, stroke_width: "4" }
                // Spleen
                polygon { points: "90,440 130,420 130,480", fill: has_center(HdCenter::Spleen), stroke: outline_color, stroke_width: "4" }
                // Solar Plexus
                polygon { points: "310,440 270,420 270,480", fill: has_center(HdCenter::SolarPlexus), stroke: outline_color, stroke_width: "4" }
            }
        }
    }
}
