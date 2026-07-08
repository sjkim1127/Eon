use crate::components::shared::birth_form::BirthForm;
use crate::components::tabs::composite_bodygraph::CompositeBodyGraph;
use crate::i18n::{t, TK};
use crate::store::{AnalysisState, TaskStatus};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use eon_human_design::transit::ReturnType;
use eon_service::dto::HumanDesignAnalysisInput;
use eon_service::facade;

#[component]
pub fn HdTransitTab() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();
    
    // UI states
    let mut target_date = use_signal(|| chrono::Local::now().format("%Y-%m-%dT%H:%M").to_string());
    let mut target_year = use_signal(|| chrono::Local::now().format("%Y").to_string());
    
    // Trigger signals
    let mut analyze_transit_trigger = use_signal(|| false);
    let mut analyze_return_trigger = use_signal(|| None::<ReturnType>);

    let state_cloned = state.clone();

    // Effect for Transit (Arbitrary date)
    use_effect(move || {
        let trig = *analyze_transit_trigger.read();
        let form1 = state_cloned.form.read().clone();
        let date_str = target_date.read().clone();
        
        if trig && form1.year > 0 {
            let mut state = state_cloned.clone();
            spawn(async move {
                analyze_transit_trigger.set(false);
                state.hd_transit.write().status = TaskStatus::Loading;
                
                let base_input = form1.to_analysis_input();
                let hd_input = HumanDesignAnalysisInput::new(base_input);
                
                let parsed_dt = match chrono::NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M") {
                    Ok(ndt) => DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc),
                    Err(_) => Utc::now(), // Fallback
                };

                match facade::analyze_hd_transit(hd_input, parsed_dt) {
                    Ok(res) => {
                        state.hd_transit.write().data = Some(res);
                        state.hd_transit.write().status = TaskStatus::Success;
                    }
                    Err(e) => {
                        state.hd_transit.write().error = Some(e.to_string());
                        state.hd_transit.write().status = TaskStatus::Error(e.to_string());
                    }
                }
            });
        }
    });

    // Effect for Return (Automatic precise time search)
    let state_cloned_2 = state.clone();
    use_effect(move || {
        let return_type_opt = analyze_return_trigger.read().clone();
        let form1 = state_cloned_2.form.read().clone();
        let year_str = target_year.read().clone();
        
        if let Some(r_type) = return_type_opt {
            let mut state = state_cloned_2.clone();
            spawn(async move {
                analyze_return_trigger.set(None);
                state.hd_transit.write().status = TaskStatus::Loading;
                
                let base_input = form1.to_analysis_input();
                let hd_input = HumanDesignAnalysisInput::new(base_input);
                
                let year = year_str.parse::<i32>().unwrap_or(2026);

                match facade::analyze_hd_return(hd_input, r_type, year) {
                    Ok(res) => {
                        state.hd_transit.write().data = Some(res);
                        state.hd_transit.write().status = TaskStatus::Success;
                    }
                    Err(e) => {
                        state.hd_transit.write().error = Some(e.to_string());
                        state.hd_transit.write().status = TaskStatus::Error(e.to_string());
                    }
                }
            });
        }
    });

    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                // Left: Natal Data
                div {
                    h3 { class: "text-lg font-bold text-slate-300 mb-3", "Natal Chart (Person)" }
                    BirthForm {}
                }
                
                // Right: Transit Controls
                div { class: "space-y-4",
                    h3 { class: "text-lg font-bold text-slate-300 mb-3", "Transit & Return Controls" }
                    
                    // 1. Manual Transit
                    div { class: "p-4 bg-slate-900/50 border border-slate-700 rounded-xl space-y-3",
                        h4 { class: "font-semibold text-teal-400", "Custom Transit Date" }
                        input {
                            r#type: "datetime-local",
                            class: "w-full bg-slate-950/50 text-slate-200 p-2.5 rounded border border-slate-700 focus:border-teal-500 focus:ring-1 focus:ring-teal-500 outline-none",
                            value: "{target_date}",
                            oninput: move |e| target_date.set(e.value().clone()),
                        }
                        button {
                            class: "w-full px-4 py-2 bg-teal-600 hover:bg-teal-500 text-white font-bold rounded shadow transition-all active:scale-95",
                            onclick: move |_| analyze_transit_trigger.set(true),
                            "Calculate Transit Chart"
                        }
                    }
                    
                    // 2. Returns
                    div { class: "p-4 bg-slate-900/50 border border-slate-700 rounded-xl space-y-3",
                        h4 { class: "font-semibold text-purple-400", "Exact Returns (Astro Search)" }
                        
                        div { class: "flex items-center gap-2",
                            input {
                                r#type: "number",
                                class: "w-24 bg-slate-950/50 text-slate-200 p-2 rounded border border-slate-700 outline-none",
                                value: "{target_year}",
                                oninput: move |e| target_year.set(e.value().clone()),
                            }
                            button {
                                class: "flex-1 px-3 py-2 bg-yellow-600 hover:bg-yellow-500 text-white font-bold rounded transition-all active:scale-95 text-sm",
                                onclick: move |_| analyze_return_trigger.set(Some(ReturnType::Solar)),
                                "Solar Return"
                            }
                        }
                        
                        div { class: "grid grid-cols-2 gap-2",
                            button {
                                class: "px-3 py-2 bg-slate-700 hover:bg-slate-600 text-slate-200 font-bold rounded transition-all active:scale-95 text-sm",
                                onclick: move |_| analyze_return_trigger.set(Some(ReturnType::Saturn)),
                                "Saturn Return (~29y)"
                            }
                            button {
                                class: "px-3 py-2 bg-slate-700 hover:bg-slate-600 text-slate-200 font-bold rounded transition-all active:scale-95 text-sm",
                                onclick: move |_| analyze_return_trigger.set(Some(ReturnType::Chiron)),
                                "Chiron Return (~50y)"
                            }
                        }
                    }
                }
            }

            // Results Section
            match &state.hd_transit.read().status {
                TaskStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500 bg-slate-900/20 border border-slate-800/40 rounded-2xl backdrop-blur-sm",
                        span { class: "text-5xl animate-bounce", "🌌" }
                        p { class: "text-lg font-medium", "Calculate a Transit or Return chart to see conditioning." }
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
                    if let Some(res) = &state.hd_transit.read().data {
                        let is_return_str = match res.is_return {
                            Some(ReturnType::Solar) => "Solar Return",
                            Some(ReturnType::Saturn) => "Saturn Return",
                            Some(ReturnType::Chiron) => "Chiron Return",
                            None => "Transit",
                        };

                        rsx! {
                            div { class: "space-y-6",
                                div { class: "p-6 bg-slate-950/40 border border-slate-800/50 rounded-2xl backdrop-blur-md space-y-4",
                                    div { class: "flex justify-between items-center",
                                        h3 { class: "text-xl font-bold text-slate-200 flex items-center gap-2",
                                            span { "✨" }
                                            "{is_return_str} Chart"
                                        }
                                        span { class: "text-sm text-teal-300 font-mono bg-teal-900/30 px-3 py-1 rounded-full border border-teal-500/30",
                                            "Target Exact UTC: {res.target_date}"
                                        }
                                    }
                                    
                                    // Reuse the composite bodygraph, treating Person 1 as Natal, Person 2 as Transit
                                    CompositeBodyGraph { result: res.composite_connection.clone() }
                                }
                                
                                // Conditioning Info
                                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                                    div { class: "p-4 bg-slate-900/50 border border-yellow-500/30 rounded-xl flex flex-col gap-2",
                                        h4 { class: "font-bold text-yellow-400", "Electromagnetic Conditioning" }
                                        p { class: "text-sm text-slate-400", "Transit brings the missing half of your hanging gate." }
                                        p { class: "text-lg text-slate-200 font-semibold", "{res.composite_connection.electromagnetic_channels.len()} Channels Activated" }
                                    }
                                    div { class: "p-4 bg-slate-900/50 border border-blue-500/30 rounded-xl flex flex-col gap-2",
                                        h4 { class: "font-bold text-blue-400", "Dominance Conditioning" }
                                        p { class: "text-sm text-slate-400", "Transit activates a whole channel you don't have." }
                                        p { class: "text-lg text-slate-200 font-semibold", "{res.composite_connection.dominance_channels.len()} New Channels" }
                                    }
                                    div { class: "p-4 bg-slate-900/50 border border-purple-500/30 rounded-xl flex flex-col gap-2",
                                        h4 { class: "font-bold text-purple-400", "Compromise Conditioning" }
                                        p { class: "text-sm text-slate-400", "Friction between your whole channel and transit's hanging gate." }
                                        p { class: "text-lg text-slate-200 font-semibold", "{res.composite_connection.compromise_channels.len()} Channels" }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! { div {} }
                    }
                }
            }
        }
    }
}
