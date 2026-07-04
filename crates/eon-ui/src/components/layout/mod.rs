use dioxus::prelude::*;
use crate::router::Route;
use crate::store::AnalysisState;
use crate::i18n::{t, Locale, TK};

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "flex h-screen w-full bg-brand-950 text-slate-100 relative overflow-hidden",
            // Celestial background nebula glows
            div { class: "absolute inset-0 bg-[radial-gradient(circle_at_20%_20%,rgba(99,102,241,0.15)_0%,transparent_50%)] pointer-events-none" }
            div { class: "absolute inset-0 bg-[radial-gradient(circle_at_80%_80%,rgba(6,182,212,0.12)_0%,transparent_50%)] pointer-events-none" }
            
            Sidebar {}
            main { class: "flex-1 overflow-auto bg-transparent relative flex flex-col z-10",
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
        aside { class: "w-64 glass-premium bg-[#0d0f22]/45 border-r border-white/5 flex flex-col hidden md:flex z-20",
            // ── Logo ──────────────────────────────────────────────────────────
            div { class: "h-16 flex items-center px-6 border-b border-white/5",
                h1 { class: "text-2xl font-bold bg-gradient-to-r from-violet-400 via-indigo-400 to-cyan-400 bg-clip-text text-transparent tracking-wider",
                    "EON"
                }
            }

            // ── Navigation ────────────────────────────────────────────────────
            nav { class: "flex-1 p-4 space-y-1.5 overflow-y-auto",
                SidebarLink { to: Route::SajuTab {}, icon: "📝", label: t(locale, TK::NavSaju) }
                SidebarLink { to: Route::VedicTab {}, icon: "✨", label: t(locale, TK::NavVedic) }
                SidebarLink { to: Route::StrengthTab {}, icon: "💪", label: t(locale, TK::NavStrength) }
                SidebarLink { to: Route::TransitTab {}, icon: "⏳", label: t(locale, TK::NavTransit) }
                SidebarLink { to: Route::SimulationTab {}, icon: "🧪", label: t(locale, TK::NavSimulation) }
                SidebarLink { to: Route::TierTab {}, icon: "🏆", label: t(locale, TK::NavTier) }
                SidebarLink { to: Route::ZwdsTab {}, icon: "🔮", label: t(locale, TK::NavZwds) }
                SidebarLink { to: Route::IChingTab {}, icon: "☯️", label: t(locale, TK::NavIChing) }
                SidebarLink { to: Route::WesternTab {}, icon: "🪐", label: t(locale, TK::NavWestern) }
                SidebarLink { to: Route::HumanDesignTab {}, icon: "🧬", label: t(locale, TK::NavHumanDesign) }
            }

            // ── Export Results ────────────────────────────────────────────────
            crate::components::shared::export_markdown::ExportWidget {}

            // ── Language Switcher ─────────────────────────────────────────────
            div { class: "px-4 pb-4 pt-2 border-t border-white/5",
                p { class: "text-[10px] text-slate-500 uppercase tracking-widest mb-2 font-semibold", "Language" }
                div { class: "grid grid-cols-4 gap-1.5",
                    {Locale::all().iter().map(|&loc| {
                        let is_active = locale == loc;
                        let active_cls = if is_active {
                            "bg-violet-600/20 border-violet-500/40 text-violet-300 scale-105 shadow-md shadow-violet-950/30"
                        } else {
                            "bg-white/5 border-white/5 text-slate-400 hover:bg-white/10 hover:text-slate-200 hover:border-white/10"
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
        "bg-violet-600/10 text-violet-300 border-violet-500/35 shadow-sm shadow-violet-950/20 glass-glow-hover"
    } else {
        "text-slate-400 hover:bg-white/5 hover:text-slate-200 border-transparent hover:border-white/5"
    };

    rsx! {
        Link {
            to: to,
            class: "flex items-center gap-3 px-4 py-3 rounded-xl border transition-all duration-200 cursor-pointer {active_class}",
            span { class: "text-xl", "{icon}" }
            span { class: "font-medium", "{label}" }
        }
    }
}
