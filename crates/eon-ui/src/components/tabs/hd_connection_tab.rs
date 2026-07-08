use crate::components::shared::birth_form::BirthForm;
use crate::components::shared::birth_form2::BirthForm2;
use crate::components::tabs::composite_bodygraph::CompositeBodyGraph;
use crate::i18n::{t, TK};
use crate::store::{AnalysisState, TaskStatus};
use dioxus::prelude::*;
use eon_service::dto::HumanDesignAnalysisInput;
use eon_service::facade;

#[component]
pub fn HdConnectionTab() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();
    let mut analysis_trigger = use_signal(|| 0);

    let state_cloned = state.clone();
    use_effect(move || {
        let form1 = state_cloned.form.read().clone();
        let form2 = state_cloned.form2.read().clone();
        let _trig = *analysis_trigger.read();

        if form1.year > 0 && form2.year > 0 && _trig > 0 {
            let mut state = state_cloned.clone();
            spawn(async move {
                state.hd_connection.write().status = TaskStatus::Loading;
                let base_input1 = form1.to_analysis_input();
                let base_input2 = form2.to_analysis_input();
                let hd_input1 = HumanDesignAnalysisInput::new(base_input1);
                let hd_input2 = HumanDesignAnalysisInput::new(base_input2);

                match facade::analyze_hd_connection(hd_input1, hd_input2) {
                    Ok(res) => {
                        state.hd_connection.write().data = Some(res);
                        state.hd_connection.write().status = TaskStatus::Success;
                    }
                    Err(e) => {
                        state.hd_connection.write().error = Some(e.to_string());
                        state.hd_connection.write().status = TaskStatus::Error(e.to_string());
                    }
                }
            });
        }
    });

    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                div {
                    h3 { class: "text-lg font-bold text-slate-300 mb-3", "Person 1" }
                    BirthForm {}
                }
                div {
                    h3 { class: "text-lg font-bold text-slate-300 mb-3", "Person 2" }
                    BirthForm2 {}
                }
            }

            div { class: "flex justify-center items-center mt-4",
                button {
                    class: "px-6 py-3 bg-gradient-to-r from-teal-500 to-indigo-500 hover:from-teal-400 hover:to-indigo-400 text-white font-bold rounded-xl shadow-lg transition-all active:scale-95 cursor-pointer",
                    onclick: move |_| {
                        let current = *analysis_trigger.peek();
                        analysis_trigger.set(current + 1);
                    },
                    "Analyze Connection"
                }
            }

            match &state.hd_connection.read().status {
                TaskStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500 bg-slate-900/20 border border-slate-800/40 rounded-2xl backdrop-blur-sm",
                        span { class: "text-5xl animate-bounce", "💞" }
                        p { class: "text-lg font-medium", "Enter both birth details to see composite chart." }
                    }
                },
                TaskStatus::Loading => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3",
                        div { class: "w-12 h-12 rounded-full border-4 border-indigo-500/30 border-t-indigo-400 animate-spin" }
                        p { class: "text-indigo-400 font-medium animate-pulse", "{t(locale, TK::StatusLoading)}" }
                    }
                },
                TaskStatus::Error(err) => rsx! {
                    div { class: "p-6 bg-rose-500/10 border border-rose-500/30 rounded-2xl text-rose-200 text-center space-y-2",
                        h4 { class: "font-bold text-lg", "{t(locale, TK::StatusError)}" }
                        p { class: "text-sm", "{err}" }
                    }
                },
                TaskStatus::Success => {
                    if let Some(res) = &state.hd_connection.read().data {
                        rsx! {
                            div { class: "space-y-6",
                                div { class: "p-6 bg-slate-950/40 border border-slate-800/50 rounded-2xl backdrop-blur-md space-y-4",
                                    h3 { class: "text-lg font-bold text-slate-200 flex items-center gap-2",
                                        span { "⚡" }
                                        "Composite BodyGraph"
                                    }
                                    CompositeBodyGraph { result: res.clone() }
                                }
                                
                                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4",
                                    div { class: "p-4 bg-slate-900/50 border border-yellow-500/30 rounded-xl flex flex-col gap-2",
                                        h4 { class: "font-bold text-yellow-400", "Electromagnetic" }
                                        p { class: "text-sm text-slate-400", "Spark & Attraction" }
                                        p { class: "text-lg text-slate-200 font-semibold", "{res.electromagnetic_channels.len()} Channels" }
                                    }
                                    div { class: "p-4 bg-slate-900/50 border border-purple-500/30 rounded-xl flex flex-col gap-2",
                                        h4 { class: "font-bold text-purple-400", "Compromise" }
                                        p { class: "text-sm text-slate-400", "Friction & Growth" }
                                        p { class: "text-lg text-slate-200 font-semibold", "{res.compromise_channels.len()} Channels" }
                                    }
                                    div { class: "p-4 bg-slate-900/50 border border-blue-500/30 rounded-xl flex flex-col gap-2",
                                        h4 { class: "font-bold text-blue-400", "Dominance" }
                                        p { class: "text-sm text-slate-400", "Learning & Absorption" }
                                        p { class: "text-lg text-slate-200 font-semibold", "{res.dominance_channels.len()} Channels" }
                                    }
                                    div { class: "p-4 bg-slate-900/50 border border-green-500/30 rounded-xl flex flex-col gap-2",
                                        h4 { class: "font-bold text-green-400", "Companionship" }
                                        p { class: "text-sm text-slate-400", "Similarity & Comfort" }
                                        p { class: "text-lg text-slate-200 font-semibold", "{res.companionship_channels.len()} Channels" }
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
