use dioxus::prelude::*;
use crate::router::Route;
use crate::store::AnalysisState;
use crate::i18n::{t, Locale, TK};

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "flex h-screen w-full bg-slate-950 text-slate-100",
            Sidebar {}
            main { class: "flex-1 overflow-auto bg-slate-900/50 backdrop-blur-xl relative flex flex-col",
                div { class: "p-6 w-full max-w-6xl mx-auto space-y-6 flex-1",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[component]
fn Sidebar() -> Element {
    let mut state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    rsx! {
        aside { class: "w-64 bg-slate-950/80 border-r border-slate-800/50 backdrop-blur-md flex flex-col hidden md:flex",
            // ── Logo ──────────────────────────────────────────────────────────
            div { class: "h-16 flex items-center px-6 border-b border-slate-800/50",
                h1 { class: "text-2xl font-bold bg-gradient-to-r from-violet-400 to-indigo-400 bg-clip-text text-transparent",
                    "EON"
                }
            }

            // ── Navigation ────────────────────────────────────────────────────
            nav { class: "flex-1 p-4 space-y-2 overflow-y-auto",
                SidebarLink { to: Route::SajuTab {}, icon: "📝", label: t(locale, TK::NavSaju) }
                SidebarLink { to: Route::VedicTab {}, icon: "✨", label: t(locale, TK::NavVedic) }
                SidebarLink { to: Route::StrengthTab {}, icon: "💪", label: t(locale, TK::NavStrength) }
                SidebarLink { to: Route::TransitTab {}, icon: "⏳", label: t(locale, TK::NavTransit) }
                SidebarLink { to: Route::SimulationTab {}, icon: "🧪", label: t(locale, TK::NavSimulation) }
                SidebarLink { to: Route::TierTab {}, icon: "🏆", label: t(locale, TK::NavTier) }
                SidebarLink { to: Route::AiTab {}, icon: "🤖", label: t(locale, TK::NavAi) }
                SidebarLink { to: Route::ZwdsTab {}, icon: "🔮", label: t(locale, TK::NavZwds) }
                SidebarLink { to: Route::IChingTab {}, icon: "☯️", label: t(locale, TK::NavIChing) }
                SidebarLink { to: Route::WesternTab {}, icon: "🪐", label: t(locale, TK::NavWestern) }
            }

            // ── Export Results ────────────────────────────────────────────────
            crate::components::shared::export_markdown::ExportWidget {}

            // ── Language Switcher ─────────────────────────────────────────────
            div { class: "px-4 pb-4 pt-2 border-t border-slate-800/50",
                p { class: "text-[10px] text-slate-500 uppercase tracking-widest mb-2 font-semibold", "Language" }
                div { class: "grid grid-cols-4 gap-1.5",
                    {Locale::all().iter().map(|&loc| {
                        let is_active = locale == loc;
                        let active_cls = if is_active {
                            "bg-violet-600/25 border-violet-500/60 text-violet-300 scale-105 shadow-lg shadow-violet-900/20"
                        } else {
                            "bg-slate-800/40 border-slate-700/50 text-slate-400 hover:bg-slate-700/50 hover:text-slate-200 hover:border-slate-600"
                        };
                        rsx! {
                            button {
                                key: "{loc.code()}",
                                class: "flex flex-col items-center justify-center py-2 px-1 rounded-xl border transition-all duration-200 cursor-pointer {active_cls}",
                                title: "{loc.label()}",
                                onclick: move |_| {
                                    state.locale.set(loc);
                                    // Persist to localStorage
                                    let _ = web_sys_set_locale(loc.code());
                                },
                                span { class: "text-lg leading-none", "{loc.flag()}" }
                                span { class: "text-[9px] font-bold mt-0.5 tracking-wide", "{loc.code().to_uppercase()}" }
                            }
                        }
                    })}
                }
            }
        }
    }
}

/// Write locale code to localStorage (best-effort — no-op on error).
fn web_sys_set_locale(code: &str) -> Result<(), ()> {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = localStorage, js_name = setItem)]
        fn ls_set(key: &str, value: &str);
    }
    ls_set("eon_locale", code);
    Ok(())
}

#[component]
fn SidebarLink(to: Route, icon: &'static str, label: &'static str) -> Element {
    let route: Route = use_route();
    let is_active = route == to;

    let active_class = if is_active {
        "bg-violet-500/10 text-violet-300 border-violet-500/50"
    } else {
        "text-slate-400 hover:bg-slate-800/50 hover:text-slate-200 border-transparent"
    };

    rsx! {
        Link {
            to: to,
            class: "flex items-center gap-3 px-4 py-3 rounded-xl border transition-all duration-200 {active_class}",
            span { class: "text-xl", "{icon}" }
            span { class: "font-medium", "{label}" }
        }
    }
}
