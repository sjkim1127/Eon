use dioxus::prelude::*;
use eon_human_design::{HdCenter, HumanDesignResult};

#[component]
pub fn HdBodyGraph(result: HumanDesignResult) -> Element {
    let has_center = move |c: HdCenter| result.defined_centers.contains(&c);

    // Color definitions
    let defined_fill = "#0d9488"; // teal-600
    let open_fill = "#f8fafc"; // slate-50
    let outline_color = "#334155"; // slate-700

    rsx! {
        div { class: "w-full flex justify-center items-center py-6 bg-slate-900/30 rounded-2xl border border-slate-800",
            svg {
                width: "100%",
                height: "auto",
                view_box: "0 0 400 600",
                class: "drop-shadow-2xl max-w-sm",

                // Head (200, 50) to Ajna (200, 150)
                line { x1: "200", y1: "70", x2: "200", y2: "130", stroke: outline_color, stroke_width: "6" }
                // Ajna to Throat
                line { x1: "200", y1: "170", x2: "200", y2: "230", stroke: outline_color, stroke_width: "6" }
                // Throat to Self
                line { x1: "200", y1: "270", x2: "200", y2: "330", stroke: outline_color, stroke_width: "6" }
                // Self to Sacral
                line { x1: "200", y1: "370", x2: "200", y2: "430", stroke: outline_color, stroke_width: "6" }
                // Sacral to Root
                line { x1: "200", y1: "470", x2: "200", y2: "530", stroke: outline_color, stroke_width: "6" }
                // Heart connections
                line { x1: "270", y1: "300", x2: "220", y2: "270", stroke: outline_color, stroke_width: "6" }
                line { x1: "270", y1: "300", x2: "220", y2: "330", stroke: outline_color, stroke_width: "6" }
                // Spleen connections
                line { x1: "100", y1: "400", x2: "180", y2: "430", stroke: outline_color, stroke_width: "6" }
                line { x1: "100", y1: "400", x2: "180", y2: "530", stroke: outline_color, stroke_width: "6" }
                line { x1: "100", y1: "400", x2: "180", y2: "330", stroke: outline_color, stroke_width: "6" }
                // SP connections
                line { x1: "300", y1: "400", x2: "220", y2: "430", stroke: outline_color, stroke_width: "6" }
                line { x1: "300", y1: "400", x2: "220", y2: "530", stroke: outline_color, stroke_width: "6" }
                line { x1: "300", y1: "400", x2: "250", y2: "310", stroke: outline_color, stroke_width: "6" }

                // Centers
                // Head
                polygon { points: "200,20 230,70 170,70", fill: if has_center(HdCenter::Head) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // Ajna
                polygon { points: "170,130 230,130 200,180", fill: if has_center(HdCenter::Ajna) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // Throat
                rect { x: "175", y: "225", width: "50", height: "50", rx: "8", fill: if has_center(HdCenter::Throat) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // Self
                polygon { points: "200,320 230,350 200,380 170,350", fill: if has_center(HdCenter::SelfG) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // Heart (Right side)
                polygon { points: "270,280 250,320 290,320", fill: if has_center(HdCenter::Heart) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // Sacral
                rect { x: "175", y: "425", width: "50", height: "50", rx: "8", fill: if has_center(HdCenter::Sacral) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // Root
                rect { x: "175", y: "525", width: "50", height: "50", rx: "8", fill: if has_center(HdCenter::Root) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // Spleen (Left side)
                polygon { points: "80,380 120,400 80,420", fill: if has_center(HdCenter::Spleen) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // Solar Plexus (Right side)
                polygon { points: "320,380 280,400 320,420", fill: if has_center(HdCenter::SolarPlexus) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
            }
        }
    }
}
