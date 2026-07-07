// crates/eon-ui/src/components/tabs/human_design_tab.rs
use crate::components::shared::birth_form::BirthForm;
use crate::components::tabs::hd_bodygraph::HdBodyGraph;
use crate::i18n::{t, translate_hd_authority, translate_hd_center, translate_hd_type, TK};
use crate::store::{AnalysisState, TaskStatus};
use dioxus::prelude::*;
use eon_service::dto::HumanDesignAnalysisInput;
use eon_service::facade;

#[component]
pub fn HumanDesignTab() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();
    let mut copied_feedback = use_signal(|| false);

    // Reactive trigger for manual analysis runs
    let mut analysis_trigger = use_signal(|| 0);

    // Auto-run or manually triggered analysis when form or trigger changes
    let state_cloned = state.clone();
    use_effect(move || {
        let form = state_cloned.form.read().clone();
        let _trig = *analysis_trigger.read();

        if form.year > 0 {
            let mut state = state_cloned.clone();
            spawn(async move {
                state.human_design.write().status = TaskStatus::Loading;
                let base_input = form.to_analysis_input();
                let hd_input = HumanDesignAnalysisInput::new(base_input);

                match facade::analyze_human_design(hd_input) {
                    Ok(res) => {
                        state.human_design.write().data = Some(res);
                        state.human_design.write().status = TaskStatus::Success;
                    }
                    Err(e) => {
                        state.human_design.write().error = Some(e.to_string());
                        state.human_design.write().status = TaskStatus::Error(e.to_string());
                    }
                }
            });
        }
    });

    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            // ── Birth Form ──────────────────────────────────────────
            BirthForm {}

            // ── Title and Action buttons ────────────────────────────
            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-teal-400 via-emerald-400 to-indigo-400 bg-clip-text text-transparent",
                    "{t(locale, TK::HdTitle)}"
                }
                div { class: "flex items-center gap-3",
                    if let TaskStatus::Success = &state.human_design.read().status {
                        if let Some(output) = &state.human_design.read().data {
                            {
                                let data_cloned = output.clone();
                                let form_cloned = state.form.read().clone();
                                let btn_text = if *copied_feedback.read() {
                                    t(locale, TK::HdReportCopiedBtn)
                                } else {
                                    t(locale, TK::HdReportCopyBtn)
                                };
                                let btn_cls = if *copied_feedback.read() {
                                    "px-5 py-2.5 bg-emerald-650 text-white rounded-xl font-semibold shadow-lg transition-all duration-200 active:scale-95 cursor-pointer text-sm"
                                } else {
                                    "px-5 py-2.5 bg-slate-800 hover:bg-slate-700/80 text-slate-200 rounded-xl font-semibold border border-slate-700/60 shadow-lg transition-all duration-200 active:scale-95 cursor-pointer text-sm"
                                };
                                rsx! {
                                    button {
                                        class: "{btn_cls}",
                                        onclick: move |_| {
                                            let txt = crate::components::shared::export_markdown::export_human_design_to_markdown(&data_cloned, &form_cloned, locale);
                                            crate::components::shared::export_markdown::copy_to_clipboard(&txt);
                                            copied_feedback.set(true);
                                            spawn(async move {
                                                gloo_timers::future::TimeoutFuture::new(2000).await;
                                                copied_feedback.set(false);
                                            });
                                        },
                                        "{btn_text}"
                                    }
                                }
                            }
                        }
                    }

                    button {
                        class: "p-2.5 bg-slate-800 hover:bg-slate-700 active:bg-slate-600 border border-slate-700/50 rounded-xl text-slate-300 hover:text-white transition-all cursor-pointer flex items-center justify-center active:scale-95",
                        onclick: move |_| {
                            let current = *analysis_trigger.peek();
                            analysis_trigger.set(current + 1);
                        },
                        title: "{t(locale, TK::FormAnalyzeBtn)}",
                        svg {
                            class: "w-5 h-5",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M4 4v5h.582m15.356 2A8.001 8.001 0 1121.21 7.89M21 3v5h-5"
                            }
                        }
                    }
                }
            }

            // ── Content depending on status ─────────────────────────
            match &state.human_design.read().status {
                TaskStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500 bg-slate-900/20 border border-slate-800/40 rounded-2xl backdrop-blur-sm",
                        span { class: "text-5xl animate-bounce", "🧬" }
                        p { class: "text-lg font-medium", "{t(locale, TK::StatusIdleHint)}" }
                    }
                },
                TaskStatus::Loading => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3",
                        div { class: "w-12 h-12 rounded-full border-4 border-teal-500/30 border-t-teal-400 animate-spin" }
                        p { class: "text-teal-400 font-medium animate-pulse", "{t(locale, TK::StatusLoading)}" }
                    }
                },
                TaskStatus::Error(err) => rsx! {
                    div { class: "p-6 bg-rose-500/10 border border-rose-500/30 rounded-2xl text-rose-200 text-center space-y-2",
                        h4 { class: "font-bold text-lg", "{t(locale, TK::StatusError)}" }
                        p { class: "text-sm", "{err}" }
                    }
                },
                TaskStatus::Success => {
                    if let Some(out) = &state.human_design.read().data {
                        let res = &out.result;
                        rsx! {
                            div { class: "space-y-6",
                                // 1. Summary Cards
                                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4",
                                    // Type Card
                                    div { class: "p-5 bg-gradient-to-br from-slate-900/80 to-slate-950/90 border border-slate-800/60 rounded-2xl shadow-xl flex flex-col justify-between backdrop-blur-md relative overflow-hidden group",
                                        div { class: "absolute -right-6 -bottom-6 text-7xl opacity-5 group-hover:scale-110 transition-transform duration-300", "🧩" }
                                        span { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "{t(locale, TK::HdType)}" }
                                        p { class: "text-lg font-bold text-teal-300 mt-2", "{translate_hd_type(locale, &res.chart_type)}" }
                                    }
                                    // Profile Card
                                    div { class: "p-5 bg-gradient-to-br from-slate-900/80 to-slate-950/90 border border-slate-800/60 rounded-2xl shadow-xl flex flex-col justify-between backdrop-blur-md relative overflow-hidden group",
                                        div { class: "absolute -right-6 -bottom-6 text-7xl opacity-5 group-hover:scale-110 transition-transform duration-300", "📐" }
                                        span { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "{t(locale, TK::HdProfile)}" }
                                        p { class: "text-lg font-bold text-purple-300 mt-2", "{res.profile}" }
                                    }
                                    // Authority Card
                                    div { class: "p-5 bg-gradient-to-br from-slate-900/80 to-slate-950/90 border border-slate-800/60 rounded-2xl shadow-xl flex flex-col justify-between backdrop-blur-md relative overflow-hidden group",
                                        div { class: "absolute -right-6 -bottom-6 text-7xl opacity-5 group-hover:scale-110 transition-transform duration-300", "⚖️" }
                                        span { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "{t(locale, TK::HdAuthority)}" }
                                        p { class: "text-lg font-bold text-amber-300 mt-2", "{translate_hd_authority(locale, &res.authority)}" }
                                    }
                                    // Strategy & Not-Self
                                    div { class: "p-5 bg-gradient-to-br from-slate-900/80 to-slate-950/90 border border-slate-800/60 rounded-2xl shadow-xl flex flex-col justify-between backdrop-blur-md relative overflow-hidden group lg:col-span-1",
                                        div { class: "absolute -right-6 -bottom-6 text-7xl opacity-5 group-hover:scale-110 transition-transform duration-300", "🧭" }
                                        span { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "Strategy & Not-Self" }
                                        p { class: "text-base font-bold text-blue-300 mt-2", "{res.strategy} / {res.not_self_theme}" }
                                    }
                                    // Definition Type
                                    div { class: "p-5 bg-gradient-to-br from-slate-900/80 to-slate-950/90 border border-slate-800/60 rounded-2xl shadow-xl flex flex-col justify-between backdrop-blur-md relative overflow-hidden group lg:col-span-1",
                                        div { class: "absolute -right-6 -bottom-6 text-7xl opacity-5 group-hover:scale-110 transition-transform duration-300", "🔗" }
                                        span { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "Definition" }
                                        p { class: "text-base font-bold text-pink-300 mt-2", "{res.definition_type}" }
                                    }
                                    // Incarnation Cross
                                    div { class: "p-5 bg-gradient-to-br from-slate-900/80 to-slate-950/90 border border-slate-800/60 rounded-2xl shadow-xl flex flex-col justify-between backdrop-blur-md relative overflow-hidden group lg:col-span-1",
                                        div { class: "absolute -right-6 -bottom-6 text-7xl opacity-5 group-hover:scale-110 transition-transform duration-300", "✝️" }
                                        span { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "Incarnation Cross" }
                                        p { class: "text-base font-bold text-yellow-300 mt-2", "{res.incarnation_cross}" }
                                    }
                                    // Variables (Arrows)
                                    div { class: "p-5 bg-gradient-to-br from-slate-900/80 to-slate-950/90 border border-slate-800/60 rounded-2xl shadow-xl flex flex-col justify-between backdrop-blur-md relative overflow-hidden group lg:col-span-1",
                                        div { class: "absolute -right-6 -bottom-6 text-7xl opacity-5 group-hover:scale-110 transition-transform duration-300", "⬆️" }
                                        span { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider mb-2", "Variables (PHS)" }
                                        div { class: "flex justify-center items-center gap-6 mt-1",
                                            {
                                                let get_arrow = |c: u8| if c <= 3 { "←" } else { "→" };
                                                let pl_sun = res.personality.get("Sun");
                                                let pl_node = res.personality.get("NorthNode");
                                                let ds_sun = res.design.get("Sun");
                                                let ds_node = res.design.get("NorthNode");

                                                let awar = pl_sun.map(|p| get_arrow(p.color)).unwrap_or("-");
                                                let pers = pl_node.map(|p| get_arrow(p.color)).unwrap_or("-");
                                                let dige = ds_sun.map(|p| get_arrow(p.color)).unwrap_or("-");
                                                let envi = ds_node.map(|p| get_arrow(p.color)).unwrap_or("-");

                                                rsx! {
                                                    div { class: "flex flex-col gap-2 text-2xl font-black text-rose-400",
                                                        span { class: "drop-shadow-md", "{dige}" }
                                                        span { class: "drop-shadow-md", "{envi}" }
                                                    }
                                                    div { class: "flex flex-col gap-2 text-2xl font-black text-slate-200",
                                                        span { class: "drop-shadow-md", "{awar}" }
                                                        span { class: "drop-shadow-md", "{pers}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // 2. Energy Centers (Defined / Open)
                                div { class: "p-6 bg-slate-950/40 border border-slate-800/50 rounded-2xl backdrop-blur-md space-y-4",
                                    h3 { class: "text-lg font-bold text-slate-200 flex items-center gap-2",
                                        span { "💎" }
                                        "Energy Centers"
                                    }
                                    div { class: "grid grid-cols-2 md:grid-cols-3 lg:grid-cols-9 gap-3",
                                        {
                                            let centers = vec![
                                                eon_human_design::HdCenter::Head,
                                                eon_human_design::HdCenter::Ajna,
                                                eon_human_design::HdCenter::Throat,
                                                eon_human_design::HdCenter::SelfG,
                                                eon_human_design::HdCenter::Heart,
                                                eon_human_design::HdCenter::Sacral,
                                                eon_human_design::HdCenter::Root,
                                                eon_human_design::HdCenter::Spleen,
                                                eon_human_design::HdCenter::SolarPlexus,
                                            ];
                                            centers.into_iter().map(|c| {
                                                let is_defined = res.defined_centers.contains(&c);
                                                let card_cls = if is_defined {
                                                    "bg-gradient-to-br from-teal-900/35 to-emerald-950/45 border-teal-500/40 text-teal-200 font-semibold"
                                                } else {
                                                    "bg-slate-900/30 border-slate-800/80 text-slate-500"
                                                };
                                                rsx! {
                                                    div {
                                                        key: "{c.name_en()}",
                                                        class: "p-3 rounded-xl border flex flex-col items-center justify-center text-center gap-1.5 transition-all duration-300 hover:scale-[1.02] {card_cls}",
                                                        span { class: "text-xs font-medium leading-tight", "{translate_hd_center(locale, c)}" }
                                                        span {
                                                            class: "text-[9px] px-1.5 py-0.5 rounded-full uppercase tracking-wider font-bold",
                                                            class: if is_defined { "bg-teal-500/20 text-teal-300" } else { "bg-slate-800/40 text-slate-600" },
                                                            if is_defined { "Defined" } else { "Open" }
                                                        }
                                                    }
                                                }
                                            })
                                        }
                                    }
                                }

                                // 2.5 BodyGraph UI
                                div { class: "p-6 bg-slate-950/40 border border-slate-800/50 rounded-2xl backdrop-blur-md space-y-4",
                                    h3 { class: "text-lg font-bold text-slate-200 flex items-center gap-2",
                                        span { "🧘" }
                                        "BodyGraph"
                                    }
                                    HdBodyGraph { result: res.clone() }
                                }

                                // 3. Activations and Channels Grid
                                div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                                    // Personality (Black) Activations
                                    div { class: "p-5 bg-slate-950/30 border border-slate-800/50 rounded-2xl space-y-4",
                                        h3 { class: "text-base font-bold text-slate-300 flex items-center gap-2 border-b border-slate-800/60 pb-2",
                                            span { class: "w-2.5 h-2.5 rounded-full bg-slate-100" }
                                            "{t(locale, TK::HdPersonalitySide)}"
                                        }
                                        div { class: "divide-y divide-slate-800/40 text-xs",
                                            {
                                                let planets_ordered = vec![
                                                    "Sun", "Earth", "Moon", "NorthNode", "SouthNode",
                                                    "Mercury", "Venus", "Mars", "Jupiter", "Saturn",
                                                    "Uranus", "Neptune", "Pluto", "Chiron"
                                                ];
                                                planets_ordered.into_iter().map(|p| {
                                                    let val_str = if let Some(p_data) = res.personality.get(p) {
                                                        format!("{}.{}.{}.{}.{}", p_data.gate, p_data.line, p_data.color, p_data.tone, p_data.base)
                                                    } else {
                                                        "-".to_string()
                                                    };
                                                    let p_name = match p {
                                                        "Sun" => t(locale, TK::WestPlanetSun),
                                                        "Earth" => t(locale, TK::WestPlanetEarth),
                                                        "Moon" => t(locale, TK::WestPlanetMoon),
                                                        "NorthNode" => t(locale, TK::WestNodeNorth),
                                                        "SouthNode" => t(locale, TK::WestNodeSouth),
                                                        "Mercury" => t(locale, TK::WestPlanetMercury),
                                                        "Venus" => t(locale, TK::WestPlanetVenus),
                                                        "Mars" => t(locale, TK::WestPlanetMars),
                                                        "Jupiter" => t(locale, TK::WestPlanetJupiter),
                                                        "Saturn" => t(locale, TK::WestPlanetSaturn),
                                                        "Uranus" => t(locale, TK::WestPlanetUranus),
                                                        "Neptune" => t(locale, TK::WestPlanetNeptune),
                                                        "Pluto" => t(locale, TK::WestPlanetPluto),
                                                        _ => p,
                                                    };
                                                    rsx! {
                                                        div { key: "{p}", class: "flex justify-between items-center py-2",
                                                            span { class: "text-slate-400 font-medium", "{p_name}" }
                                                            span { class: "font-mono font-bold text-slate-200 bg-slate-900/50 px-2 py-0.5 rounded", "{val_str}" }
                                                        }
                                                    }
                                                })
                                            }
                                        }
                                    }

                                    // Design (Red) Activations
                                    div { class: "p-5 bg-slate-950/30 border border-slate-800/50 rounded-2xl space-y-4",
                                        h3 { class: "text-base font-bold text-rose-400 flex items-center gap-2 border-b border-slate-800/60 pb-2",
                                            span { class: "w-2.5 h-2.5 rounded-full bg-rose-500" }
                                            "{t(locale, TK::HdDesignSide)}"
                                        }
                                        div { class: "divide-y divide-slate-800/40 text-xs",
                                            {
                                                let planets_ordered = vec![
                                                    "Sun", "Earth", "Moon", "NorthNode", "SouthNode",
                                                    "Mercury", "Venus", "Mars", "Jupiter", "Saturn",
                                                    "Uranus", "Neptune", "Pluto", "Chiron"
                                                ];
                                                planets_ordered.into_iter().map(|p| {
                                                    let val_str = if let Some(p_data) = res.design.get(p) {
                                                        format!("{}.{}.{}.{}.{}", p_data.gate, p_data.line, p_data.color, p_data.tone, p_data.base)
                                                    } else {
                                                        "-".to_string()
                                                    };
                                                    let p_name = match p {
                                                        "Sun" => t(locale, TK::WestPlanetSun),
                                                        "Earth" => t(locale, TK::WestPlanetEarth),
                                                        "Moon" => t(locale, TK::WestPlanetMoon),
                                                        "NorthNode" => t(locale, TK::WestNodeNorth),
                                                        "SouthNode" => t(locale, TK::WestNodeSouth),
                                                        "Mercury" => t(locale, TK::WestPlanetMercury),
                                                        "Venus" => t(locale, TK::WestPlanetVenus),
                                                        "Mars" => t(locale, TK::WestPlanetMars),
                                                        "Jupiter" => t(locale, TK::WestPlanetJupiter),
                                                        "Saturn" => t(locale, TK::WestPlanetSaturn),
                                                        "Uranus" => t(locale, TK::WestPlanetUranus),
                                                        "Neptune" => t(locale, TK::WestPlanetNeptune),
                                                        "Pluto" => t(locale, TK::WestPlanetPluto),
                                                        _ => p,
                                                    };
                                                    rsx! {
                                                        div { key: "{p}", class: "flex justify-between items-center py-2",
                                                            span { class: "text-slate-400 font-medium", "{p_name}" }
                                                            span { class: "font-mono font-bold text-rose-400 bg-rose-950/20 border border-rose-500/20 px-2 py-0.5 rounded", "{val_str}" }
                                                        }
                                                    }
                                                })
                                            }
                                        }
                                    }

                                    // Channels & Active Gates
                                    div { class: "space-y-6",
                                        // Active Channels Card
                                        div { class: "p-5 bg-slate-950/30 border border-slate-800/50 rounded-2xl space-y-4",
                                            h3 { class: "text-base font-bold text-slate-300 flex items-center gap-2",
                                                span { "🔀" }
                                                "{t(locale, TK::HdActiveChannels)}"
                                            }
                                            if res.active_channels.is_empty() {
                                                p { class: "text-xs text-slate-500 italic", "No active channels." }
                                            } else {
                                                div { class: "flex flex-wrap gap-2 text-xs",
                                                    {
                                                        res.active_channels.iter().map(|&(g1, g2)| {
                                                            rsx! {
                                                                span {
                                                                    key: "{g1}-{g2}",
                                                                    class: "px-3 py-1.5 bg-indigo-900/35 border border-indigo-500/35 text-indigo-200 rounded-xl font-bold tracking-wider",
                                                                    "Channel {g1} - {g2}"
                                                                }
                                                            }
                                                        })
                                                    }
                                                }
                                            }
                                        }

                                        // Active Gates Card
                                        div { class: "p-5 bg-slate-950/30 border border-slate-800/50 rounded-2xl space-y-4",
                                            h3 { class: "text-base font-bold text-slate-300 flex items-center gap-2",
                                                span { "🔑" }
                                                "{t(locale, TK::HdActiveGates)}"
                                            }
                                            if res.active_gates.is_empty() {
                                                p { class: "text-xs text-slate-500 italic", "No active gates." }
                                            } else {
                                                div { class: "grid grid-cols-8 gap-2 text-xs font-mono",
                                                    {
                                                        let mut sorted_gates = res.active_gates.clone();
                                                        sorted_gates.sort();
                                                        sorted_gates.into_iter().map(|g| {
                                                            rsx! {
                                                                span {
                                                                    key: "{g}",
                                                                    class: "p-1.5 bg-slate-900/50 border border-slate-800 text-slate-300 rounded text-center font-semibold",
                                                                    "{g}"
                                                                }
                                                            }
                                                        })
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {
                            div { class: "text-slate-500 text-sm italic", "No data loaded." }
                        }
                    }
                }
            }
        }
    }
}
