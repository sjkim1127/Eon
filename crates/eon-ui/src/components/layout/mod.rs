use crate::i18n::{t, Locale, TK};
use crate::router::Route;
use crate::store::{AnalysisState, TaskStatus};
use dioxus::prelude::*;
use eon_service::facade;

#[component]
pub fn AppLayout() -> Element {
    let mut state = use_context::<AnalysisState>();
    let mut show_mobile_drawer = use_signal(|| false);
    let locale = *state.locale.read();

    // state.form 이 변경되면 모든 분석 실시간 자동 비동기 수행
    let effect_state = state.clone();
    use_effect(move || {
        let form = effect_state.form.read().clone();

        // 1. Saju (Simulation 포함)
        spawn({
            let mut state = effect_state.clone();
            let form = form.clone();
            async move {
                state.saju.write().status = TaskStatus::Loading;
                let saju_input = eon_service::dto::SajuAnalysisInput::new(
                    form.to_analysis_input(),
                    form.is_male,
                    form.use_night_rat_hour,
                    Some(false),
                );
                match facade::analyze_saju(saju_input) {
                    Ok(res) => {
                        state.saju.write().data = Some(res.clone());
                        state.saju.write().status = TaskStatus::Success;
                        check_and_run_tier(state.clone(), Some(res), None);
                    }
                    Err(e) => {
                        state.saju.write().error = Some(e.to_string());
                        state.saju.write().status = TaskStatus::Error(e.to_string());
                    }
                }
            }
        });

        // 2. Vedic
        spawn({
            let mut state = effect_state.clone();
            let form = form.clone();
            async move {
                state.vedic.write().status = TaskStatus::Loading;
                let vedic_input = eon_service::dto::VedicAnalysisInput::new(
                    form.to_analysis_input(),
                    Some(false),
                    None,
                );
                match facade::analyze_vedic(vedic_input) {
                    Ok(res) => {
                        state.vedic.write().data = Some(res.clone());
                        state.vedic.write().status = TaskStatus::Success;
                        check_and_run_tier(state.clone(), None, Some(res));
                    }
                    Err(e) => {
                        state.vedic.write().error = Some(e.to_string());
                        state.vedic.write().status = TaskStatus::Error(e.to_string());
                    }
                }
            }
        });

        // 3. Transit
        spawn({
            let mut state = effect_state.clone();
            let form = form.clone();
            async move {
                state.transit.write().status = TaskStatus::Loading;
                let saju_input = eon_service::dto::SajuAnalysisInput::new(
                    form.to_analysis_input(),
                    form.is_male,
                    form.use_night_rat_hour,
                    Some(false),
                );
                let transit_input = eon_service::dto::TransitAnalysisInput::new(saju_input, None);
                match facade::analyze_transit(transit_input) {
                    Ok(res) => {
                        state.transit.write().data = Some(res);
                        state.transit.write().status = TaskStatus::Success;
                    }
                    Err(e) => {
                        state.transit.write().error = Some(e.to_string());
                        state.transit.write().status = TaskStatus::Error(e.to_string());
                    }
                }
            }
        });

        // 4. ZWDS
        spawn({
            let mut state = effect_state.clone();
            let form = form.clone();
            async move {
                state.zwds.write().status = TaskStatus::Loading;
                let base = form.to_analysis_input();
                let zwds_input = eon_service::dto::ZwdsAnalysisInput::new(base, form.is_male, None);
                match facade::analyze_zwds(zwds_input) {
                    Ok(res) => {
                        state.zwds.write().data = Some(res);
                        state.zwds.write().status = TaskStatus::Success;
                    }
                    Err(e) => {
                        state.zwds.write().error = Some(e.to_string());
                        state.zwds.write().status = TaskStatus::Error(e.to_string());
                    }
                }
            }
        });

        // 5. IChing (주역 하락수)
        spawn({
            let mut state = effect_state.clone();
            let form = form.clone();
            async move {
                state.iching.write().status = TaskStatus::Loading;
                let saju_input = eon_service::dto::SajuAnalysisInput::new(
                    form.to_analysis_input(),
                    form.is_male,
                    form.use_night_rat_hour,
                    Some(false),
                );
                match facade::analyze_iching(saju_input) {
                    Ok(res) => {
                        state.iching.write().data = Some(res);
                        state.iching.write().status = TaskStatus::Success;
                    }
                    Err(e) => {
                        state.iching.write().error = Some(e.to_string());
                        state.iching.write().status = TaskStatus::Error(e.to_string());
                    }
                }
            }
        });

        // 6. Western (서양 점성학)
        spawn({
            let mut state = effect_state.clone();
            let form = form.clone();
            async move {
                state.western.write().status = TaskStatus::Loading;
                let base = form.to_analysis_input();
                let western_input =
                    eon_service::dto::WesternAnalysisInput::new(base, "Placidus".to_string());
                match facade::analyze_western(western_input) {
                    Ok(res) => {
                        state.western.write().data = Some(res);
                        state.western.write().status = TaskStatus::Success;
                    }
                    Err(e) => {
                        state.western.write().error = Some(e.to_string());
                        state.western.write().status = TaskStatus::Error(e.to_string());
                    }
                }
            }
        });

        // 7. Human Design
        spawn({
            let mut state = effect_state.clone();
            let form = form.clone();
            async move {
                state.human_design.write().status = TaskStatus::Loading;
                let base = form.to_analysis_input();
                let hd_input = eon_service::dto::HumanDesignAnalysisInput::new(base);
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
            }
        });
        // 8. Qimen Dunjia
        spawn({
            let mut state = effect_state.clone();
            let form = form.clone();
            async move {
                state.qimen.write().status = TaskStatus::Loading;
                let base = form.to_analysis_input();
                let qimen_input = eon_service::dto::QimenAnalysisInput::new(base, form.is_male);
                match facade::analyze_qimen(qimen_input) {
                    Ok(res) => {
                        state.qimen.write().data = Some(res);
                        state.qimen.write().status = TaskStatus::Success;
                    }
                    Err(e) => {
                        state.qimen.write().error = Some(e.to_string());
                        state.qimen.write().status = TaskStatus::Error(e.to_string());
                    }
                }
            }
        });
    });

    rsx! {
        div { class: "flex flex-col md:flex-row h-screen w-full bg-brand-950 text-slate-100 relative overflow-hidden",
            // Celestial background nebula glows
            div { class: "absolute inset-0 bg-[radial-gradient(circle_at_20%_20%,rgba(99,102,241,0.15)_0%,transparent_50%)] pointer-events-none" }
            div { class: "absolute inset-0 bg-[radial-gradient(circle_at_80%_80%,rgba(6,182,212,0.12)_0%,transparent_50%)] pointer-events-none" }

            // Desktop Sidebar
            Sidebar {}

            // Mobile Top Header Bar
            div { class: "md:hidden flex items-center justify-between px-4 py-3 bg-[#0d0f22]/90 border-b border-white/10 backdrop-blur-xl z-30 shrink-0",
                h1 { class: "text-xl font-bold bg-gradient-to-r from-violet-400 via-indigo-400 to-cyan-400 bg-clip-text text-transparent tracking-wider",
                    "EON"
                }
                div { class: "flex items-center gap-2",
                    button {
                        class: "px-3 py-1.5 rounded-xl bg-violet-600/20 text-violet-300 text-xs font-semibold border border-violet-500/30 flex items-center gap-1.5 active:scale-95 transition-transform cursor-pointer",
                        onclick: move |_| state.show_export_modal.set(true),
                        "📥 내보내기"
                    }
                    button {
                        class: "p-2 rounded-xl bg-white/5 border border-white/10 text-slate-200 active:scale-95 transition-transform cursor-pointer text-sm font-bold",
                        onclick: move |_| {
                            let is_open = *show_mobile_drawer.read();
                            show_mobile_drawer.set(!is_open);
                        },
                        if *show_mobile_drawer.read() { "✕" } else { "☰" }
                    }
                }
            }

            // Mobile Drawer Overlay
            if *show_mobile_drawer.read() {
                div {
                    class: "md:hidden fixed inset-0 bg-black/70 backdrop-blur-md z-40 flex flex-col justify-end animate-in fade-in duration-200",
                    onclick: move |_| show_mobile_drawer.set(false),
                    div {
                        class: "bg-[#0d0f22] border-t border-white/10 rounded-t-3xl p-5 space-y-4 max-h-[85vh] overflow-y-auto shadow-2xl",
                        onclick: move |e| e.stop_propagation(),
                        div { class: "flex justify-between items-center pb-2 border-b border-white/5",
                            h2 { class: "text-sm font-bold text-slate-300 uppercase tracking-widest", "분석 탐색 및 메뉴" }
                            button {
                                class: "text-slate-400 text-lg p-1 cursor-pointer",
                                onclick: move |_| show_mobile_drawer.set(false),
                                "✕"
                            }
                        }
                        nav { class: "grid grid-cols-2 gap-2 py-2",
                            MobileLink { to: Route::SajuTab {}, icon: "📝", label: t(locale, TK::NavSaju), close_drawer: move || show_mobile_drawer.set(false) }
                            MobileLink { to: Route::VedicTab {}, icon: "✨", label: t(locale, TK::NavVedic), close_drawer: move || show_mobile_drawer.set(false) }
                            MobileLink { to: Route::StrengthTab {}, icon: "💪", label: t(locale, TK::NavStrength), close_drawer: move || show_mobile_drawer.set(false) }
                            MobileLink { to: Route::TransitTab {}, icon: "⏳", label: t(locale, TK::NavTransit), close_drawer: move || show_mobile_drawer.set(false) }
                            MobileLink { to: Route::SimulationTab {}, icon: "🧪", label: t(locale, TK::NavSimulation), close_drawer: move || show_mobile_drawer.set(false) }
                            MobileLink { to: Route::TierTab {}, icon: "🏆", label: t(locale, TK::NavTier), close_drawer: move || show_mobile_drawer.set(false) }
                            MobileLink { to: Route::ZwdsTab {}, icon: "🔮", label: t(locale, TK::NavZwds), close_drawer: move || show_mobile_drawer.set(false) }
                            MobileLink { to: Route::IChingTab {}, icon: "☯️", label: t(locale, TK::NavIChing), close_drawer: move || show_mobile_drawer.set(false) }
                            MobileLink { to: Route::WesternTab {}, icon: "🪐", label: t(locale, TK::NavWestern), close_drawer: move || show_mobile_drawer.set(false) }
                            MobileLink { to: Route::HumanDesignTab {}, icon: "🧬", label: t(locale, TK::NavHumanDesign), close_drawer: move || show_mobile_drawer.set(false) }
                            MobileLink { to: Route::HdPentaTab {}, icon: "🌀", label: "Penta", close_drawer: move || show_mobile_drawer.set(false) }
                            MobileLink { to: Route::QimenTab {}, icon: "🧭", label: t(locale, TK::NavQimen), close_drawer: move || show_mobile_drawer.set(false) }
                            MobileLink { to: Route::TimelineTab {}, icon: "📅", label: t(locale, TK::NavTimeline), close_drawer: move || show_mobile_drawer.set(false) }
                        }
                        // Mobile Language Switcher
                        div { class: "pt-3 border-t border-white/5",
                            p { class: "text-[10px] text-slate-500 uppercase tracking-widest mb-2 font-semibold", "Language / 언어 설정" }
                            div { class: "grid grid-cols-4 gap-2",
                                {Locale::all().iter().map(|&loc| {
                                    let is_active = locale == loc;
                                    let active_cls = if is_active {
                                        "bg-violet-600/20 border-violet-500/40 text-violet-300 font-bold"
                                    } else {
                                        "bg-white/5 border-white/5 text-slate-400 hover:bg-white/10"
                                    };
                                    rsx! {
                                        button {
                                            key: "{loc.code()}",
                                            class: "py-2 rounded-xl border text-xs flex items-center justify-center gap-1.5 cursor-pointer {active_cls}",
                                            onclick: move |_| {
                                                state.locale.set(loc);
                                                let _ = web_sys_set_locale(loc.code());
                                            },
                                            span { "{loc.flag()}" }
                                            span { "{loc.code().to_uppercase()}" }
                                        }
                                    }
                                })}
                            }
                        }
                    }
                }
            }

            // Main Content Area
            main { class: "flex-1 overflow-auto bg-transparent relative flex flex-col z-10",
                div { class: "p-3.5 sm:p-6 pb-24 md:pb-6 w-full max-w-6xl mx-auto space-y-6 flex-1",
                    Outlet::<Route> {}
                }
            }

            // Mobile Bottom Navigation Bar
            div { class: "md:hidden fixed bottom-0 left-0 right-0 bg-[#0d0f22]/95 border-t border-white/10 backdrop-blur-2xl px-2 py-1.5 flex justify-around items-center z-30 shadow-2xl",
                BottomNavLink { to: Route::SajuTab {}, icon: "📝", label: t(locale, TK::NavSaju) }
                BottomNavLink { to: Route::VedicTab {}, icon: "✨", label: t(locale, TK::NavVedic) }
                BottomNavLink { to: Route::TierTab {}, icon: "🏆", label: t(locale, TK::NavTier) }
                BottomNavLink { to: Route::ZwdsTab {}, icon: "🔮", label: t(locale, TK::NavZwds) }
                button {
                    class: "flex flex-col items-center py-1 px-3 rounded-xl text-slate-400 hover:text-slate-200 transition-colors cursor-pointer",
                    onclick: move |_| show_mobile_drawer.set(true),
                    span { class: "text-lg leading-tight", "☰" }
                    span { class: "text-[10px] font-medium mt-0.5", "더보기" }
                }
            }

            if *state.show_export_modal.read() {
                crate::components::shared::export_markdown::ExportModal {}
            }
        }
    }
}

/// 사주와 베딕 데이터가 둘 다 성공 상태일 때 통합 운명 티어 분석을 연쇄 수행합니다.
fn check_and_run_tier(
    mut state: AnalysisState,
    saju_res_opt: Option<eon_service::dto::SajuAnalysisOutput>,
    vedic_res_opt: Option<eon_service::dto::VedicAnalysisOutput>,
) {
    let saju_res = saju_res_opt.or_else(|| {
        if matches!(state.saju.read().status, TaskStatus::Success) {
            state.saju.read().data.clone()
        } else {
            None
        }
    });

    let vedic_res = vedic_res_opt.or_else(|| {
        if matches!(state.vedic.read().status, TaskStatus::Success) {
            state.vedic.read().data.clone()
        } else {
            None
        }
    });

    if let (Some(s), Some(v)) = (saju_res, vedic_res) {
        state.tier.write().status = TaskStatus::Loading;
        match facade::analyze_destiny_tier(s, v, None) {
            Ok(res) => {
                state.tier.write().data = Some(res);
                state.tier.write().status = TaskStatus::Success;
            }
            Err(e) => {
                state.tier.write().error = Some(e.to_string());
                state.tier.write().status = TaskStatus::Error(e.to_string());
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
                SidebarLink { to: Route::HdPentaTab {}, icon: "🌀", label: "Penta" }
                SidebarLink { to: Route::QimenTab {}, icon: "🧭", label: t(locale, TK::NavQimen) }
                SidebarLink { to: Route::TimelineTab {}, icon: "📅", label: t(locale, TK::NavTimeline) }
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

#[component]
fn MobileLink(
    to: Route,
    icon: &'static str,
    label: &'static str,
    close_drawer: EventHandler<()>,
) -> Element {
    let route: Route = use_route();
    let is_active = route == to;

    let active_class = if is_active {
        "bg-violet-600/20 text-violet-300 border-violet-500/40 font-bold"
    } else {
        "bg-white/5 text-slate-300 hover:bg-white/10 border-white/5"
    };

    rsx! {
        Link {
            to: to,
            class: "flex items-center gap-2.5 px-3.5 py-2.5 rounded-xl border text-xs transition-all cursor-pointer {active_class}",
            onclick: move |_| close_drawer.call(()),
            span { class: "text-base", "{icon}" }
            span { class: "truncate", "{label}" }
        }
    }
}

#[component]
fn BottomNavLink(to: Route, icon: &'static str, label: &'static str) -> Element {
    let route: Route = use_route();
    let is_active = route == to;

    let active_class = if is_active {
        "text-violet-300 font-bold"
    } else {
        "text-slate-400 hover:text-slate-200"
    };

    rsx! {
        Link {
            to: to,
            class: "flex flex-col items-center py-1 px-3 rounded-xl transition-colors cursor-pointer {active_class}",
            span { class: "text-lg leading-tight", "{icon}" }
            span { class: "text-[10px] font-medium mt-0.5", "{label}" }
        }
    }
}
