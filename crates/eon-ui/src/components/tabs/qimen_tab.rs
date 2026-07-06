use crate::i18n::{t, TK};
use crate::store::{AnalysisState, TaskStatus};
use dioxus::prelude::*;

#[component]
pub fn QimenTab() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = state.locale.read().clone();
    let qimen = state.qimen.read().clone();

    rsx! {
        div { class: "w-full max-w-7xl mx-auto space-y-6 md:space-y-8 animate-fade-in",
            match qimen.status {
                TaskStatus::Idle | TaskStatus::Loading => rsx! {
                    div { class: "flex justify-center items-center h-64",
                        div { class: "text-slate-400 font-medium animate-pulse",
                            "{t(locale, TK::QimenAnalyzing)}"
                        }
                    }
                },
                TaskStatus::Error(e) => rsx! {
                    div { class: "bg-red-950/30 border border-red-500/50 rounded-2xl p-6 text-red-200",
                        div { class: "font-bold mb-2 flex items-center gap-2",
                            "{t(locale, TK::QimenError)}"
                        }
                        div { class: "text-sm opacity-90", "{e}" }
                    }
                },
                TaskStatus::Success => rsx! {
                    if let Some(data) = qimen.data {
                        div { class: "bg-slate-900/50 backdrop-blur-md rounded-3xl border border-slate-700/50 p-6 md:p-8 shadow-2xl",
                            h2 { class: "text-2xl font-bold text-slate-100 mb-6 flex items-center gap-3",
                                span { class: "text-3xl", "🧭" }
                                "{t(locale, TK::QimenTitle)}"
                            }
                            div { class: "prose prose-invert max-w-none prose-p:leading-relaxed prose-pre:bg-slate-800/50",
                                p { class: "text-slate-300 text-lg",
                                    "{data.report.summary}"
                                }
                                div { class: "mt-8 p-4 bg-indigo-950/30 border border-indigo-500/30 rounded-xl",
                                    h3 { class: "text-indigo-300 font-bold mb-2", "{t(locale, TK::QimenScaffolding)}" }
                                    p { class: "text-indigo-200/70 text-sm",
                                        "{t(locale, TK::QimenPlaceholder)}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
