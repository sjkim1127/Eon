use crate::i18n::{t, TK};
use crate::store::AnalysisState;
use dioxus::prelude::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;

#[component]
pub fn TimelineTab() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    use_effect(move || {
        // Draw the chart on mount
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(canvas_el) = document.get_element_by_id("timeline-canvas") {
                    use wasm_bindgen::JsCast;
                    if let Ok(canvas) = canvas_el.dyn_into::<HtmlCanvasElement>() {
                        draw_timeline(&canvas);
                    }
                }
            }
        }
    });

    rsx! {
        div {
            class: "w-full h-full flex flex-col p-4 overflow-y-auto overflow-x-hidden",
            h2 { class: "text-2xl font-bold text-white mb-4 tracking-wider", "{t(locale, TK::NavTimeline)}" }
            div {
                class: "flex flex-col space-y-4 w-full h-[600px]",
                canvas {
                    id: "timeline-canvas",
                    width: "1200",
                    height: "600",
                    class: "w-full h-full border border-zinc-700 bg-zinc-800 rounded-xl shadow-lg"
                }
            }
        }
    }
}

fn draw_timeline(canvas: &HtmlCanvasElement) {
    let backend =
        CanvasBackend::with_canvas_object(canvas.clone()).expect("Cannot attach to canvas");
    let root = backend.into_drawing_area();
    root.fill(&RGBColor(39, 39, 42)).unwrap(); // zinc-800

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Destiny Timeline",
            ("sans-serif", 30).into_font().color(&WHITE),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(100)
        .build_cartesian_2d(0..100, 0..50)
        .unwrap();

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(10)
        .y_labels(5)
        .axis_style(RGBColor(113, 113, 122)) // zinc-500
        .label_style(("sans-serif", 15).into_font().color(&WHITE))
        .draw()
        .unwrap();

    // Draw dummy Gantt bars for Daewun
    let style = RGBColor(16, 185, 129).filled(); // emerald-500
    chart
        .draw_series(std::iter::once(Rectangle::new([(10, 10), (20, 15)], style)))
        .unwrap();

    root.present().unwrap();
}
