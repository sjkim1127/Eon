use crate::i18n::{t, TK};
use crate::store::{AnalysisState, FormState, TaskStatus};
use dioxus::prelude::*;
use eon_service::dto::HumanDesignAnalysisInput;
use eon_service::facade;

#[component]
pub fn HdPentaTab() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();
    let mut analysis_trigger = use_signal(|| 0);
    let mut copied_feedback = use_signal(|| false);

    let mut forms = state.penta_forms;

    let state_cloned = state.clone();
    use_effect(move || {
        let current_forms = forms.read().clone();
        let _trig = *analysis_trigger.read();

        let all_valid = current_forms.iter().all(|f| f.year > 0);
        if all_valid && _trig > 0 && current_forms.len() >= 3 {
            let mut state = state_cloned.clone();
            spawn(async move {
                state.hd_penta.write().status = TaskStatus::Loading;

                let inputs: Vec<HumanDesignAnalysisInput> = current_forms
                    .into_iter()
                    .map(|f| HumanDesignAnalysisInput::new(f.to_analysis_input()))
                    .collect();

                match facade::analyze_hd_penta(inputs) {
                    Ok(res) => {
                        state.hd_penta.write().data = Some(res);
                        state.hd_penta.write().status = TaskStatus::Success;
                    }
                    Err(e) => {
                        state.hd_penta.write().error = Some(e.to_string());
                        state.hd_penta.write().status = TaskStatus::Error(e.to_string());
                    }
                }
            });
        }
    });

    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-emerald-400 via-teal-400 to-indigo-400 bg-clip-text text-transparent flex items-center gap-2",
                    span { "🌀" }
                    if forms.read().len() >= 6 {
                        "WA Group Dynamics (6-16)"
                    } else {
                        "Penta Dynamics (3-5)"
                    }
                }
                div { class: "flex gap-2",
                    button {
                        class: if *copied_feedback.read() {
                            "px-4 py-2 bg-emerald-650 hover:bg-emerald-600 text-white border border-emerald-500/50 rounded-lg transition-all cursor-pointer flex items-center justify-center active:scale-95 gap-1 text-sm font-medium"
                        } else {
                            "px-4 py-2 bg-slate-800 hover:bg-slate-700 border border-slate-700/50 rounded-lg text-slate-300 transition-all cursor-pointer flex items-center justify-center active:scale-95 gap-1 text-sm font-medium"
                        },
                        title: "Copy Report",
                        onclick: move |_| {
                            if let Some(ref data) = state.hd_penta.read().data.as_ref() {
                                let txt = crate::components::shared::export_markdown::export_hd_penta_to_markdown(data, locale);
                                crate::components::shared::export_markdown::copy_to_clipboard(&txt);
                                copied_feedback.set(true);
                                spawn(async move {
                                    gloo_timers::future::TimeoutFuture::new(2000).await;
                                    copied_feedback.set(false);
                                });
                            }
                        },
                        if *copied_feedback.read() {
                            span { class: "text-xs font-bold", "✓ Copied" }
                        } else {
                            span { class: "text-xs font-medium", "📋 Copy" }
                        }
                    }
                    button {
                        class: "px-4 py-2 bg-slate-800 hover:bg-slate-700 text-slate-300 rounded-lg text-sm font-medium transition-colors disabled:opacity-50",
                        disabled: forms.read().len() <= 3,
                        onclick: move |_| {
                            let mut f = forms.write();
                            f.pop();
                        },
                        "Remove Person"
                    }
                    button {
                        class: "px-4 py-2 bg-slate-800 hover:bg-slate-700 text-slate-300 rounded-lg text-sm font-medium transition-colors disabled:opacity-50",
                        disabled: forms.read().len() >= 16,
                        onclick: move |_| {
                            let mut f = forms.write();
                            f.push(FormState::default());
                        },
                        "Add Person"
                    }
                }
            }

            // Forms grid
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                {
                    (0..forms.read().len()).map(|idx| {
                        let year_val = forms.read()[idx].year;
                        let month_val = forms.read()[idx].month;
                        let day_val = forms.read()[idx].day;
                        let hour_val = forms.read()[idx].hour;
                        let min_val = forms.read()[idx].minute;

                        rsx! {
                            div {
                                key: "{idx}",
                                class: "p-4 bg-slate-900/50 border border-slate-800/80 rounded-xl space-y-3",
                                h3 { class: "font-semibold text-teal-400 text-sm", "Person {idx + 1}" }
                                div { class: "grid grid-cols-3 gap-2",
                                    input {
                                        class: "p-2 bg-slate-950/50 border border-slate-700/50 rounded-lg text-sm text-slate-200 outline-none focus:border-teal-500",
                                        r#type: "number", placeholder: "YYYY", value: if year_val > 0 { "{year_val}" } else { "".to_string() },
                                        oninput: move |e| { if let Ok(v) = e.value().parse() { forms.write()[idx].year = v; } }
                                    }
                                    input {
                                        class: "p-2 bg-slate-950/50 border border-slate-700/50 rounded-lg text-sm text-slate-200 outline-none focus:border-teal-500",
                                        r#type: "number", placeholder: "MM", value: if month_val > 0 { "{month_val}" } else { "".to_string() },
                                        oninput: move |e| { if let Ok(v) = e.value().parse() { forms.write()[idx].month = v; } }
                                    }
                                    input {
                                        class: "p-2 bg-slate-950/50 border border-slate-700/50 rounded-lg text-sm text-slate-200 outline-none focus:border-teal-500",
                                        r#type: "number", placeholder: "DD", value: if day_val > 0 { "{day_val}" } else { "".to_string() },
                                        oninput: move |e| { if let Ok(v) = e.value().parse() { forms.write()[idx].day = v; } }
                                    }
                                }
                                div { class: "grid grid-cols-2 gap-2",
                                    input {
                                        class: "p-2 bg-slate-950/50 border border-slate-700/50 rounded-lg text-sm text-slate-200 outline-none focus:border-teal-500",
                                        r#type: "number", placeholder: "HH", value: "{hour_val}",
                                        oninput: move |e| { if let Ok(v) = e.value().parse() { forms.write()[idx].hour = v; } }
                                    }
                                    input {
                                        class: "p-2 bg-slate-950/50 border border-slate-700/50 rounded-lg text-sm text-slate-200 outline-none focus:border-teal-500",
                                        r#type: "number", placeholder: "Min", value: "{min_val}",
                                        oninput: move |e| { if let Ok(v) = e.value().parse() { forms.write()[idx].minute = v; } }
                                    }
                                }
                            }
                        }
                    })
                }
            }

            div { class: "flex justify-center items-center mt-4",
                button {
                    class: "px-6 py-3 bg-gradient-to-r from-teal-500 to-indigo-500 hover:from-teal-400 hover:to-indigo-400 text-white font-bold rounded-xl shadow-lg transition-all active:scale-95 cursor-pointer disabled:opacity-50",
                    disabled: forms.read().iter().any(|f| f.year == 0),
                    onclick: move |_| {
                        let current = *analysis_trigger.peek();
                        analysis_trigger.set(current + 1);
                    },
                    "Analyze Penta"
                }
            }

            // Results Section
            match &state.hd_penta.read().status {
                TaskStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500 bg-slate-900/20 border border-slate-800/40 rounded-2xl backdrop-blur-sm",
                        span { class: "text-5xl animate-bounce", "👥" }
                        p { class: "text-lg font-medium", "Enter 3 to 5 birth details to see group Penta dynamics." }
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
                    if let Some(res) = &state.hd_penta.read().data {
                        rsx! {
                            div { class: "space-y-6",
                                // Summary Cards
                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                    div { class: "p-5 bg-gradient-to-br from-slate-900/80 to-slate-950/90 border border-slate-800/60 rounded-2xl shadow-xl",
                                        span { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "Fully Defined Channels" }
                                        p { class: "text-3xl font-bold text-teal-300 mt-2", "{res.fully_defined_channels} / 6" }
                                    }
                                    div { class: "p-5 bg-gradient-to-br from-slate-900/80 to-slate-950/90 border border-slate-800/60 rounded-2xl shadow-xl",
                                        span { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "Gaps (Missing Gates)" }
                                        p { class: "text-xl font-bold text-rose-400 mt-2",
                                            if res.gaps.is_empty() {
                                                "None! Perfect Synergy."
                                            } else {
                                                "{res.gaps.iter().map(|g| g.to_string()).collect::<Vec<_>>().join(\", \")}"
                                            }
                                        }
                                    }
                                }

                                // Penta Channels
                                div { class: "p-6 bg-slate-950/40 border border-slate-800/50 rounded-2xl backdrop-blur-md space-y-4",
                                    h3 { class: "text-lg font-bold text-slate-200 flex items-center gap-2",
                                        span { "⚡" }
                                        if res.participants >= 6 {
                                            "WA Vortex Channels"
                                        } else {
                                            "Penta Vortex Channels"
                                        }
                                    }
                                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                                        {
                                            res.channels.iter().map(|ch| {
                                                let is_active = ch.is_active;
                                                let bg_cls = if is_active { "bg-teal-900/30 border-teal-500/50" } else { "bg-slate-900/30 border-slate-800/50" };
                                                let text_cls = if is_active { "text-teal-300" } else { "text-slate-500" };

                                                rsx! {
                                                    div {
                                                        key: "{ch.channel.0}-{ch.channel.1}",
                                                        class: "p-4 rounded-xl border flex flex-col gap-2 {bg_cls}",
                                                        div { class: "flex justify-between items-start",
                                                            span { class: "font-bold {text_cls}", "{ch.channel.0} - {ch.channel.1}" }
                                                            if is_active {
                                                                span { class: "text-xs px-2 py-1 bg-teal-500/20 text-teal-300 rounded-full font-bold", "Active" }
                                                            }
                                                        }
                                                        span { class: "text-sm {text_cls}", "{ch.name}" }

                                                        if !is_active && !ch.missing_gates.is_empty() {
                                                            div { class: "mt-2 pt-2 border-t border-slate-800/50",
                                                                span { class: "text-xs text-rose-400/80 font-medium", "Missing: {ch.missing_gates.iter().map(|g| g.to_string()).collect::<Vec<_>>().join(\", \")}" }
                                                            }
                                                        }
                                                    }
                                                }
                                            })
                                        }
                                    }
                                }
                            }
                        }
                    } else { rsx! { div {} } }
                }
            }
        }
    }
}
