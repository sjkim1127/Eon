use dioxus::prelude::*;
use eon_human_design::phs::{ArrowDirection, PhsVariablesResult};

#[derive(Props, PartialEq, Clone)]
pub struct PhsVariablesProps {
    pub data: PhsVariablesResult,
}

#[component]
pub fn PhsVariables(props: PhsVariablesProps) -> Element {
    let get_arrow_icon = |d: &ArrowDirection| match d {
        ArrowDirection::Left => "←",
        ArrowDirection::Right => "→",
    };

    let dig_arrow = get_arrow_icon(&props.data.digestion.direction);
    let env_arrow = get_arrow_icon(&props.data.environment.direction);
    let mot_arrow = get_arrow_icon(&props.data.motivation.direction);
    let per_arrow = get_arrow_icon(&props.data.perspective.direction);

    rsx! {
        div { class: "p-6 bg-gradient-to-br from-slate-900/80 to-slate-950/90 border border-slate-800/60 rounded-2xl shadow-xl backdrop-blur-md relative overflow-hidden group col-span-1 md:col-span-2 lg:col-span-4",
            div { class: "absolute -right-6 -bottom-6 text-9xl opacity-5 group-hover:scale-110 transition-transform duration-300 pointer-events-none", "⬆️" }
            h3 { class: "text-lg font-bold text-slate-200 flex items-center gap-2 mb-4",
                span { "🎯" }
                "Primary Health System & Variables"
            }

            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 relative z-10",
                // 1. Digestion (Design Sun/Earth)
                div { class: "bg-slate-950/60 p-4 rounded-xl border border-rose-900/50 flex flex-col gap-2 relative",
                    div { class: "absolute top-4 right-4 text-3xl font-black text-rose-500/80 drop-shadow-md", "{dig_arrow}" }
                    span { class: "text-xs font-bold text-rose-400 uppercase tracking-widest", "Digestion" }
                    div { class: "flex items-baseline gap-2 mt-2",
                        span { class: "text-2xl font-bold text-slate-100", "{props.data.digestion.color:?}" }
                        span { class: "text-xs text-slate-400", "(Design Sun/Earth)" }
                    }
                    div { class: "mt-auto pt-4 flex flex-col gap-1 border-t border-slate-800/60",
                        div { class: "flex justify-between items-center text-sm",
                            span { class: "text-slate-500", "Cognition (Tone)" }
                            span { class: "font-semibold text-rose-300", "{props.data.digestion.tone:?}" }
                        }
                        div { class: "flex justify-between items-center text-xs",
                            span { class: "text-slate-600", "Base" }
                            span { class: "font-semibold text-rose-400/70", "{props.data.digestion.base:?}" }
                        }
                    }
                }

                // 2. Environment (Design Nodes)
                div { class: "bg-slate-950/60 p-4 rounded-xl border border-emerald-900/50 flex flex-col gap-2 relative",
                    div { class: "absolute top-4 right-4 text-3xl font-black text-emerald-500/80 drop-shadow-md", "{env_arrow}" }
                    span { class: "text-xs font-bold text-emerald-400 uppercase tracking-widest", "Environment" }
                    div { class: "flex items-baseline gap-2 mt-2",
                        span { class: "text-2xl font-bold text-slate-100", "{props.data.environment.color:?}" }
                        span { class: "text-xs text-slate-400", "(Design Nodes)" }
                    }
                    div { class: "mt-auto pt-4 flex flex-col gap-1 border-t border-slate-800/60",
                        div { class: "flex justify-between items-center text-sm",
                            span { class: "text-slate-500", "Style (Tone)" }
                            span { class: "font-semibold text-emerald-300", "{props.data.environment.tone:?}" }
                        }
                        div { class: "flex justify-between items-center text-xs",
                            span { class: "text-slate-600", "Base" }
                            span { class: "font-semibold text-emerald-400/70", "{props.data.environment.base:?}" }
                        }
                    }
                }

                // 3. Motivation (Personality Sun/Earth)
                div { class: "bg-slate-950/60 p-4 rounded-xl border border-blue-900/50 flex flex-col gap-2 relative",
                    div { class: "absolute top-4 right-4 text-3xl font-black text-blue-500/80 drop-shadow-md", "{mot_arrow}" }
                    span { class: "text-xs font-bold text-blue-400 uppercase tracking-widest", "Motivation" }
                    div { class: "flex items-baseline gap-2 mt-2",
                        span { class: "text-2xl font-bold text-slate-100", "{props.data.motivation.color:?}" }
                        span { class: "text-xs text-slate-400", "(Personality Sun/Earth)" }
                    }
                    div { class: "text-xs text-slate-400 mb-2 flex items-center gap-1",
                        "Transference ➔ ",
                        span { class: "text-slate-300 font-medium", "{props.data.motivation.transference:?}" }
                    }
                    div { class: "mt-auto pt-2 flex flex-col gap-1 border-t border-slate-800/60",
                        div { class: "flex justify-between items-center text-sm",
                            span { class: "text-slate-500", "Cognition (Tone)" }
                            span { class: "font-semibold text-blue-300", "{props.data.motivation.tone:?}" }
                        }
                        div { class: "flex justify-between items-center text-xs",
                            span { class: "text-slate-600", "Base" }
                            span { class: "font-semibold text-blue-400/70", "{props.data.motivation.base:?}" }
                        }
                    }
                }

                // 4. Perspective/View (Personality Nodes)
                div { class: "bg-slate-950/60 p-4 rounded-xl border border-amber-900/50 flex flex-col gap-2 relative",
                    div { class: "absolute top-4 right-4 text-3xl font-black text-amber-500/80 drop-shadow-md", "{per_arrow}" }
                    span { class: "text-xs font-bold text-amber-400 uppercase tracking-widest", "Perspective" }
                    div { class: "flex items-baseline gap-2 mt-2",
                        span { class: "text-2xl font-bold text-slate-100", "{props.data.perspective.color:?}" }
                        span { class: "text-xs text-slate-400", "(Personality Nodes)" }
                    }
                    div { class: "text-xs text-slate-400 mb-2 flex items-center gap-1",
                        "Transference ➔ ",
                        span { class: "text-slate-300 font-medium", "{props.data.perspective.transference:?}" }
                    }
                    div { class: "mt-auto pt-2 flex flex-col gap-1 border-t border-slate-800/60",
                        div { class: "flex justify-between items-center text-sm",
                            span { class: "text-slate-500", "Focus (Tone)" }
                            span { class: "font-semibold text-amber-300", "{props.data.perspective.tone:?}" }
                        }
                        div { class: "flex justify-between items-center text-xs",
                            span { class: "text-slate-600", "Base" }
                            span { class: "font-semibold text-amber-400/70", "{props.data.perspective.base:?}" }
                        }
                    }
                }
            }
        }
    }
}
