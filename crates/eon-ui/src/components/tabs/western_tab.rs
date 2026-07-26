// crates/eon-ui/src/components/tabs/western_tab.rs
use crate::components::shared::birth_form::BirthForm;
use crate::i18n::{t, Locale, TK};
use crate::store::{AnalysisState, TaskStatus};
use dioxus::prelude::*;
use eon_service::dto::WesternAnalysisInput;
use eon_service::facade;

pub fn get_planet_emoji_and_name<'a>(
    name: &'a str,
    locale: Locale,
) -> (&'static str, std::borrow::Cow<'a, str>) {
    use std::borrow::Cow;
    match name {
        "Sun" => ("☀️", Cow::Borrowed(t(locale, TK::WestPlanetSun))),
        "Moon" => ("🌙", Cow::Borrowed(t(locale, TK::WestPlanetMoon))),
        "Mercury" => ("☿", Cow::Borrowed(t(locale, TK::WestPlanetMercury))),
        "Venus" => ("♀", Cow::Borrowed(t(locale, TK::WestPlanetVenus))),
        "Mars" => ("♂", Cow::Borrowed(t(locale, TK::WestPlanetMars))),
        "Jupiter" => ("♃", Cow::Borrowed(t(locale, TK::WestPlanetJupiter))),
        "Saturn" => ("♄", Cow::Borrowed(t(locale, TK::WestPlanetSaturn))),
        "Uranus" => ("♅", Cow::Borrowed(t(locale, TK::WestPlanetUranus))),
        "Neptune" => ("♆", Cow::Borrowed(t(locale, TK::WestPlanetNeptune))),
        "Pluto" => ("♇", Cow::Borrowed(t(locale, TK::WestPlanetPluto))),
        "Chiron" => ("🔑", Cow::Borrowed(t(locale, TK::WestPlanetChiron))),
        "True Node" => ("☊", Cow::Borrowed(t(locale, TK::WestNodeNorth))),
        "South Node" => ("☋", Cow::Borrowed(t(locale, TK::WestNodeSouth))),
        "NorthNode" => ("☊", Cow::Borrowed(t(locale, TK::WestNodeNorth))),
        "SouthNode" => ("☋", Cow::Borrowed(t(locale, TK::WestNodeSouth))),
        "Lilith" => ("⚸", Cow::Borrowed("Lilith (Dark Moon)")),
        "Ceres" => ("🌾", Cow::Borrowed("Ceres")),
        "Pallas" => ("🗡️", Cow::Borrowed("Pallas")),
        "Juno" => ("👑", Cow::Borrowed("Juno")),
        "Vesta" => ("🔥", Cow::Borrowed("Vesta")),
        _ => ("🪐", Cow::Borrowed(name)),
    }
}

pub fn get_sign_emoji_and_name(idx: usize, locale: Locale) -> (&'static str, &'static str) {
    match idx {
        0 => ("♈", t(locale, TK::WestSignAries)),
        1 => ("♉", t(locale, TK::WestSignTaurus)),
        2 => ("♊", t(locale, TK::WestSignGemini)),
        3 => ("♋", t(locale, TK::WestSignCancer)),
        4 => ("♌", t(locale, TK::WestSignLeo)),
        5 => ("♍", t(locale, TK::WestSignVirgo)),
        6 => ("♎", t(locale, TK::WestSignLibra)),
        7 => ("♏", t(locale, TK::WestSignScorpio)),
        8 => ("♐", t(locale, TK::WestSignSagittarius)),
        9 => ("♑", t(locale, TK::WestSignCapricorn)),
        10 => ("♒", t(locale, TK::WestSignAquarius)),
        11 => ("♓", t(locale, TK::WestSignPisces)),
        _ => ("❓", "Unknown"),
    }
}

pub fn get_aspect_emoji_and_name(
    aspect: eon_western::AspectType,
    locale: Locale,
) -> (&'static str, &'static str) {
    match aspect {
        eon_western::AspectType::Conjunction => ("☌", t(locale, TK::WestAspectConjunction)),
        eon_western::AspectType::Sextile => ("⚹", t(locale, TK::WestAspectSextile)),
        eon_western::AspectType::Square => ("□", t(locale, TK::WestAspectSquare)),
        eon_western::AspectType::Trine => ("△", t(locale, TK::WestAspectTrine)),
        eon_western::AspectType::Opposition => ("☍", t(locale, TK::WestAspectOpposition)),
        eon_western::AspectType::Semisextile => ("⚺", "Semisextile (30°)"),
        eon_western::AspectType::Quincunx => ("⚻", "Quincunx (150°)"),
        eon_western::AspectType::Semisquare => ("∠", "Semisquare (45°)"),
        eon_western::AspectType::Sesquiquadrate => ("⚼", "Sesquiquadrate (135°)"),
        eon_western::AspectType::Quintile => ("Q", "Quintile (72°)"),
        eon_western::AspectType::Biquintile => ("bQ", "Biquintile (144°)"),
    }
}

fn WesternChartWheel(res: eon_western::WesternResult) -> Element {
    let asc = res.ascendant;

    rsx! {
        div { class: "p-6 bg-slate-900 border border-slate-800 rounded-2xl shadow-xl flex flex-col items-center gap-4",
            div { class: "w-full flex justify-between items-center border-b border-slate-800 pb-3",
                h3 { class: "text-base font-bold text-slate-200 uppercase tracking-widest flex items-center gap-2",
                    span { "🔮" }
                    span { "360° Circular Astrology Chart Wheel" }
                }
                span { class: "text-xs text-indigo-400 font-mono font-semibold bg-indigo-950/40 px-3 py-1 rounded-full border border-indigo-800/40",
                    "ASC: {asc:.2}°"
                }
            }

            svg {
                class: "w-full max-w-[480px] h-auto text-slate-300",
                view_box: "0 0 500 500",
                // Outer ring
                circle { cx: "250", cy: "250", r: "220", fill: "#0f172a", stroke: "#334155", stroke_width: "2" }
                circle { cx: "250", cy: "250", r: "170", fill: "#020617", stroke: "#334155", stroke_width: "1.5" }
                circle { cx: "250", cy: "250", r: "95", fill: "#090d16", stroke: "#1e293b", stroke_width: "1.5" }

                // 12 House Cusp radial lines
                {res.houses.iter().map(|h| {
                    let angle_deg = (180.0 - (h.cusp_longitude - asc)).to_radians();
                    let x1 = 250.0 + 95.0 * angle_deg.cos();
                    let y1 = 250.0 - 95.0 * angle_deg.sin();
                    let x2 = 250.0 + 220.0 * angle_deg.cos();
                    let y2 = 250.0 - 220.0 * angle_deg.sin();
                    let stroke_cls = if h.house_number == 1 || h.house_number == 10 { "#818cf8" } else { "#334155" };
                    let stroke_w = if h.house_number == 1 || h.house_number == 10 { "2.5" } else { "1" };
                    rsx! {
                        line { x1: "{x1}", y1: "{y1}", x2: "{x2}", y2: "{y2}", stroke: "{stroke_cls}", stroke_width: "{stroke_w}" }
                    }
                })}

                // Aspect Chord Lines in Center
                {res.aspects.iter().filter(|a| a.is_major).map(|asp| {
                    let p1 = res.planets.iter().find(|p| p.name == asp.body_a_name);
                    let p2 = res.planets.iter().find(|p| p.name == asp.body_b_name);
                    if let (Some(pa), Some(pb)) = (p1, p2) {
                        let rad1 = (180.0 - (pa.longitude - asc)).to_radians();
                        let rad2 = (180.0 - (pb.longitude - asc)).to_radians();
                        let x1 = 250.0 + 90.0 * rad1.cos();
                        let y1 = 250.0 - 90.0 * rad1.sin();
                        let x2 = 250.0 + 90.0 * rad2.cos();
                        let y2 = 250.0 - 90.0 * rad2.sin();
                        let color = match asp.aspect_type {
                            eon_western::AspectType::Conjunction => "#f59e0b",
                            eon_western::AspectType::Trine | eon_western::AspectType::Sextile => "#3b82f6",
                            eon_western::AspectType::Square | eon_western::AspectType::Opposition => "#ef4444",
                            _ => "#a855f7",
                        };
                        rsx! {
                            line { x1: "{x1}", y1: "{y1}", x2: "{x2}", y2: "{y2}", stroke: "{color}", stroke_width: "1.2", stroke_opacity: "0.65" }
                        }
                    } else {
                        rsx! {}
                    }
                })}

                // Planet Emojis
                {res.planets.iter().map(|p| {
                    let rad = (180.0 - (p.longitude - asc)).to_radians();
                    let px = 250.0 + 135.0 * rad.cos();
                    let py = 250.0 - 135.0 * rad.sin();
                    let dot_x = 250.0 + 170.0 * rad.cos();
                    let dot_y = 250.0 - 170.0 * rad.sin();
                    let (emoji, _) = get_planet_emoji_and_name(&p.name, Locale::Ko);
                    rsx! {
                        g {
                            circle { cx: "{dot_x}", cy: "{dot_y}", r: "3", fill: "#fbbf24" }
                            text { x: "{px}", y: "{py}", fill: "#f1f5f9", font_size: "12", text_anchor: "middle", dominant_baseline: "middle", "{emoji}" }
                        }
                    }
                })}
            }
        }
    }
}

#[component]
pub fn WesternTab() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    let mut selected_house_sys = use_signal(|| "P".to_string());
    let mut copied_feedback = use_signal(|| false);
    let mut analysis_trigger = use_signal(|| 0);

    let state_cloned = state.clone();
    use_effect(move || {
        let form = state_cloned.form.read().clone();
        let house_sys = selected_house_sys.read().clone();
        let _trig = *analysis_trigger.read();

        if form.year > 0 {
            let mut state = state_cloned.clone();
            spawn(async move {
                state.western.write().status = TaskStatus::Loading;
                let base_input = form.to_analysis_input();
                let west_input = WesternAnalysisInput::new(base_input, house_sys);

                match facade::analyze_western(west_input) {
                    Ok(res) => {
                        state.western.write().data = Some(res);
                        state.western.write().status = TaskStatus::Success;
                    }
                    Err(e) => {
                        state.western.write().error = Some(e.to_string());
                        state.western.write().status = TaskStatus::Error(e.to_string());
                    }
                }
            });
        }
    });

    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            BirthForm {}

            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-pink-400 via-purple-400 to-indigo-400 bg-clip-text text-transparent",
                    "{t(locale, TK::WestTitle)}"
                }
                div { class: "flex items-center gap-3",
                    div { class: "flex items-center bg-slate-900/80 border border-slate-700/50 rounded-xl px-3 py-1.5 gap-2 shadow-inner",
                        span { class: "text-xs font-semibold text-slate-400", "{t(locale, TK::WestHouseSystem)}:" }
                        select {
                            class: "bg-transparent text-slate-200 border-none outline-none text-xs font-bold cursor-pointer",
                            value: "{selected_house_sys.read()}",
                            onchange: move |evt| {
                                let val = evt.value();
                                selected_house_sys.set(val.clone());
                                if let TaskStatus::Success = &state.western.read().status {
                                    let current = *analysis_trigger.peek();
                                    analysis_trigger.set(current + 1);
                                }
                            },
                            option { value: "P", "{t(locale, TK::WestPlacidus)}" }
                            option { value: "K", "{t(locale, TK::WestKoch)}" }
                            option { value: "W", "{t(locale, TK::WestWholeSign)}" }
                            option { value: "E", "{t(locale, TK::WestEqual)}" }
                        }
                    }

                    if let TaskStatus::Success = &state.western.read().status {
                        if let Some(output) = &state.western.read().data {
                            {
                                let data_cloned = output.clone();
                                let form_cloned = state.form.read().clone();
                                let btn_text = if *copied_feedback.read() {
                                    t(locale, TK::WestReportCopiedBtn)
                                } else {
                                    t(locale, TK::WestReportCopyBtn)
                                };
                                let btn_cls = if *copied_feedback.read() {
                                    "px-5 py-2.5 bg-emerald-600 text-white rounded-xl font-semibold shadow-lg transition-all duration-200 active:scale-95 cursor-pointer text-sm"
                                } else {
                                    "px-5 py-2.5 bg-slate-800 hover:bg-slate-700/80 text-slate-200 rounded-xl font-semibold border border-slate-700/60 shadow-lg transition-all duration-200 active:scale-95 cursor-pointer text-sm"
                                };
                                rsx! {
                                    button {
                                        class: "{btn_cls}",
                                        onclick: move |_| {
                                            let txt = crate::components::shared::export_markdown::export_western_to_markdown(&data_cloned, &form_cloned, locale);
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

            match &state.western.read().status {
                TaskStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500 bg-slate-900/20 border border-slate-800/40 rounded-2xl backdrop-blur-sm",
                        span { class: "text-5xl animate-bounce", "🪐" }
                        p { class: "text-lg font-medium", "{t(locale, TK::StatusIdleHint)}" }
                    }
                },
                TaskStatus::Loading => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3",
                        div { class: "w-12 h-12 rounded-full border-4 border-purple-500/30 border-t-purple-400 animate-spin" }
                        p { class: "text-purple-400 font-medium animate-pulse", "{t(locale, TK::StatusLoading)}" }
                    }
                },
                TaskStatus::Error(err) => rsx! {
                    div { class: "p-6 bg-rose-500/10 border border-rose-500/30 rounded-2xl text-rose-200 text-center space-y-2",
                        h4 { class: "font-bold text-lg", "{t(locale, TK::StatusError)}" }
                        p { class: "text-sm", "{err}" }
                    }
                },
                TaskStatus::Success => {
                    if let Some(out) = &state.western.read().data {
                        let res = &out.result;
                        let asc_sign_idx = (res.ascendant / 30.0).floor() as usize;
                        let (asc_emoji, asc_name) = get_sign_emoji_and_name(asc_sign_idx, locale);
                        let asc_deg = res.ascendant % 30.0;
                        let asc_deg_str = format!("{:.0}° {:.0}'", asc_deg.floor(), (asc_deg.fract() * 60.0).round());

                        let (_, cr_korean_name) = get_planet_emoji_and_name(&res.chart_ruler, locale);

                        let (el_emoji, el_name) = match res.dominant_element.as_str() {
                            "Fire" => ("🔥", t(locale, TK::WestElementFire)),
                            "Earth" => ("⛰️", t(locale, TK::WestElementEarth)),
                            "Air" => ("💨", t(locale, TK::WestElementAir)),
                            "Water" => ("💧", t(locale, TK::WestElementWater)),
                            _ => ("❓", res.dominant_element.as_str()),
                        };

                        let (mo_emoji, mo_name) = match res.dominant_modality.as_str() {
                            "Cardinal" => ("⚡", t(locale, TK::WestModalityCardinal)),
                            "Fixed" => ("🔒", t(locale, TK::WestModalityFixed)),
                            "Mutable" => ("🌀", t(locale, TK::WestModalityMutable)),
                            _ => ("❓", res.dominant_modality.as_str()),
                        };

                        rsx! {
                            div { class: "grid grid-cols-1 md:grid-cols-4 gap-4",
                                div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md relative overflow-hidden group shadow-lg",
                                    div { class: "absolute -right-4 -bottom-4 text-slate-800/30 text-7xl font-bold transition-all duration-300 group-hover:scale-110", "ASC" }
                                    p { class: "text-xs font-semibold text-slate-400 uppercase tracking-widest", "Ascendant" }
                                    h3 { class: "text-lg font-bold text-violet-300 mt-2 flex items-center gap-1.5",
                                        span { "{asc_emoji}" }
                                        span { "{asc_name}" }
                                    }
                                    p { class: "text-sm font-semibold text-slate-300 mt-0.5", "{asc_deg_str}" }
                                }
                                div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md relative overflow-hidden group shadow-lg",
                                    div { class: "absolute -right-4 -bottom-4 text-slate-800/30 text-7xl font-bold transition-all duration-300 group-hover:scale-110", "RUL" }
                                    p { class: "text-xs font-semibold text-slate-400 uppercase tracking-widest", "{t(locale, TK::WestChartRuler)}" }
                                    h3 { class: "text-lg font-bold text-pink-300 mt-2 flex items-center gap-1.5",
                                        span { "{cr_korean_name}" }
                                    }
                                    p { class: "text-[11px] text-slate-500 mt-0.5", "Ascendant Ruler" }
                                }
                                div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md relative overflow-hidden group shadow-lg",
                                    div { class: "absolute -right-4 -bottom-4 text-slate-800/30 text-7xl font-bold transition-all duration-300 group-hover:scale-110", "ELE" }
                                    p { class: "text-xs font-semibold text-slate-400 uppercase tracking-widest", "Dominant Element" }
                                    h3 { class: "text-lg font-bold text-amber-300 mt-2 flex items-center gap-1.5",
                                        span { "{el_emoji}" }
                                        span { "{el_name}" }
                                    }
                                    p { class: "text-[11px] text-slate-500 mt-0.5", "Elemental Temperament" }
                                }
                                div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md relative overflow-hidden group shadow-lg",
                                    div { class: "absolute -right-4 -bottom-4 text-slate-800/30 text-7xl font-bold transition-all duration-300 group-hover:scale-110", "MOD" }
                                    p { class: "text-xs font-semibold text-slate-400 uppercase tracking-widest", "Dominant Modality" }
                                    h3 { class: "text-lg font-bold text-emerald-300 mt-2 flex items-center gap-1.5",
                                        span { "{mo_emoji}" }
                                        span { "{mo_name}" }
                                    }
                                    p { class: "text-[11px] text-slate-500 mt-0.5", "Behavioral Modality" }
                                }
                            }

                            // ── Interactive SVG 360° Chart Wheel ──
                            {WesternChartWheel(res.clone())}

                            // ── Aspect Patterns (Geometries) ──
                            if !res.aspect_patterns.is_empty() {
                                div { class: "p-6 bg-slate-900 border border-slate-800 rounded-2xl shadow-xl space-y-3",
                                    h3 { class: "text-base font-bold text-slate-200 uppercase tracking-widest flex items-center gap-2 border-b border-slate-800 pb-2",
                                        span { "📐" }
                                        span { "Aspect Geometry Patterns (기하학적 아스펙트 패턴)" }
                                    }
                                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-3",
                                        {res.aspect_patterns.iter().map(|pat| rsx! {
                                            div { class: "p-4 rounded-xl bg-slate-950/60 border border-purple-800/30 space-y-1.5",
                                                div { class: "flex justify-between items-center",
                                                    span { class: "font-bold text-sm text-purple-300", "{pat.name_kr}" }
                                                    span { class: "text-xs font-mono text-purple-400 bg-purple-950/50 px-2 py-0.5 rounded border border-purple-800/40",
                                                        "{pat.planets.join(\" - \")}"
                                                    }
                                                }
                                                p { class: "text-xs text-slate-300 leading-relaxed", "{pat.interpretation}" }
                                            }
                                        })}
                                    }
                                }
                            }

                            // ── Arabian Parts (Lots) ──
                            if !res.arabian_parts.is_empty() {
                                div { class: "p-6 bg-slate-900 border border-slate-800 rounded-2xl shadow-xl space-y-3",
                                    h3 { class: "text-base font-bold text-slate-200 uppercase tracking-widest flex items-center gap-2 border-b border-slate-800 pb-2",
                                        span { "🎯" }
                                        span { "Arabian Parts / Sensitivity Lots (아라비안 파트 민감점)" }
                                    }
                                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-3",
                                        {res.arabian_parts.iter().map(|ap| {
                                            let (s_emoji, s_name) = get_sign_emoji_and_name(ap.sign_index, locale);
                                            rsx! {
                                                div { class: "p-4 rounded-xl bg-slate-950/60 border border-cyan-800/30 space-y-1.5 flex flex-col justify-between",
                                                    div { class: "flex justify-between items-center",
                                                        span { class: "font-bold text-xs text-cyan-300", "{ap.name_kr}" }
                                                        span { class: "text-xs font-semibold text-slate-200 flex items-center gap-1",
                                                            span { "{s_emoji}" }
                                                            span { "{s_name}" }
                                                        }
                                                    }
                                                    div { class: "flex justify-between items-center text-xs font-mono text-slate-400 pt-1 border-t border-slate-800/40",
                                                        span { "하우스: {ap.house_number}H" }
                                                        span { "{ap.degree_in_sign:.1}°" }
                                                    }
                                                    span { class: "text-[10px] text-slate-500 font-mono", "공식: {ap.formula}" }
                                                }
                                            }
                                        })}
                                    }
                                }
                            }

                            // ── Essential Dignities & House Rulerships ──
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                                // Essential Dignities
                                div { class: "p-6 bg-slate-900 border border-slate-800 rounded-2xl shadow-xl space-y-3",
                                    h3 { class: "text-base font-bold text-slate-200 uppercase tracking-widest border-b border-slate-800 pb-2",
                                        "🏛️ Essential Dignities (고전 행성 위계)"
                                    }
                                    div { class: "space-y-2 max-h-[350px] overflow-y-auto pr-1",
                                        {res.dignities.iter().map(|d| {
                                            let (p_emoji, p_name) = get_planet_emoji_and_name(&d.planet_name, locale);
                                            let score_cls = if d.score > 0 { "text-emerald-400" } else if d.score < 0 { "text-rose-400" } else { "text-slate-400" };
                                            rsx! {
                                                div { class: "p-2.5 rounded-xl bg-slate-950/60 border border-slate-800 flex justify-between items-center text-xs",
                                                    div { class: "flex items-center gap-2 font-bold text-slate-200",
                                                        span { "{p_emoji}" }
                                                        span { "{p_name}" }
                                                    }
                                                    span { class: "font-mono font-bold {score_cls}", "{d.status_summary}" }
                                                }
                                            }
                                        })}
                                    }
                                }

                                // House Rulership Network
                                div { class: "p-6 bg-slate-900 border border-slate-800 rounded-2xl shadow-xl space-y-3",
                                    h3 { class: "text-base font-bold text-slate-200 uppercase tracking-widest border-b border-slate-800 pb-2",
                                        "🔗 House Rulership Network (하우스 통치 네트워크)"
                                    }
                                    div { class: "space-y-2 max-h-[350px] overflow-y-auto pr-1",
                                        {res.house_rulerships.iter().map(|hr| rsx! {
                                            div { class: "p-2.5 rounded-xl bg-slate-950/60 border border-slate-800 flex flex-col gap-1 text-xs",
                                                div { class: "flex justify-between items-center font-bold",
                                                    span { class: "text-indigo-300", "House {hr.house_number} Ruler: {hr.ruler_planet}" }
                                                    span { class: "text-slate-400 font-mono", "➞ House {hr.ruler_in_house}" }
                                                }
                                                p { class: "text-[11px] text-slate-400 leading-snug", "{hr.interpretation}" }
                                            }
                                        })}
                                    }
                                }
                            }

                            // ── Main Body: 2 Column Layout (Planet & House & Aspect Tables) ──
                            div { class: "grid grid-cols-1 lg:grid-cols-12 gap-6",
                                div { class: "lg:col-span-7 space-y-6",
                                    div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md shadow-lg",
                                        h3 { class: "text-base font-bold text-slate-300 mb-4 border-b border-slate-800/60 pb-2", "Planet Positions" }
                                        div { class: "overflow-x-auto",
                                            table { class: "w-full text-sm text-left border-collapse",
                                                thead { class: "text-slate-500 text-xs font-semibold border-b border-slate-800",
                                                    tr {
                                                        th { class: "py-2 px-3", "{t(locale, TK::WestPlanet)}" }
                                                        th { class: "py-2 px-3", "{t(locale, TK::WestSign)}" }
                                                        th { class: "py-2 px-3", "{t(locale, TK::WestDegree)}" }
                                                        th { class: "py-2 px-3 text-center", "{t(locale, TK::WestHouse)}" }
                                                    }
                                                }
                                                tbody { class: "divide-y divide-slate-800/50",
                                                    {res.planets.iter().map(|p| {
                                                        let (p_emoji, p_name) = get_planet_emoji_and_name(&p.name, locale);
                                                        let (s_emoji, s_name) = get_sign_emoji_and_name(p.sign_index, locale);
                                                        let p_deg = p.degree_in_sign;
                                                        let p_deg_str = format!("{:.0}° {:.0}'", p_deg.floor(), (p_deg.fract() * 60.0).round());

                                                        rsx! {
                                                            tr { key: "{p.id}", class: "hover:bg-slate-800/20 transition-colors",
                                                                td { class: "py-3 px-3 font-medium text-slate-200 flex items-center gap-2",
                                                                    span { class: "text-lg text-slate-400", "{p_emoji}" }
                                                                    span { "{p_name}" }
                                                                    if p.is_retrograde {
                                                                        span { class: "text-[10px] font-bold text-rose-400 bg-rose-500/10 border border-rose-500/20 px-1 rounded", "℞" }
                                                                    }
                                                                }
                                                                td { class: "py-3 px-3 text-slate-300 font-semibold",
                                                                    span { class: "mr-1 text-slate-400", "{s_emoji}" }
                                                                    span { "{s_name}" }
                                                                }
                                                                td { class: "py-3 px-3 text-slate-400 font-mono text-xs", "{p_deg_str}" }
                                                                td { class: "py-3 px-3 text-center",
                                                                    span { class: "inline-block bg-indigo-500/10 text-indigo-300 border border-indigo-500/20 text-xs px-2.5 py-0.5 rounded-full font-bold",
                                                                        "{p.house_number}"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    })}
                                                }
                                            }
                                        }
                                    }

                                    div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md shadow-lg",
                                        h3 { class: "text-base font-bold text-slate-300 mb-4 border-b border-slate-800/60 pb-2", "House Cusps" }
                                        div { class: "grid grid-cols-2 md:grid-cols-3 gap-3",
                                            {res.houses.iter().map(|h| {
                                                let (s_emoji, s_name) = get_sign_emoji_and_name(h.sign_index, locale);
                                                let h_deg = h.degree_in_sign;
                                                let h_deg_str = format!("{:.0}° {:.0}'", h_deg.floor(), (h_deg.fract() * 60.0).round());

                                                rsx! {
                                                    div { key: "{h.house_number}", class: "flex flex-col bg-slate-950/40 border border-slate-800/60 p-2.5 rounded-xl text-center hover:border-violet-500/40 transition-colors",
                                                        span { class: "text-[10px] text-slate-500 font-bold uppercase tracking-wider", "House {h.house_number}" }
                                                        span { class: "text-xs font-semibold text-slate-200 mt-1 flex items-center justify-center gap-1",
                                                            span { "{s_emoji}" }
                                                            span { "{s_name}" }
                                                        }
                                                        span { class: "text-[10px] text-slate-400 font-mono mt-0.5", "{h_deg_str}" }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }

                                div { class: "lg:col-span-5 space-y-6",
                                    div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md shadow-lg space-y-5",
                                        div { class: "space-y-3",
                                            h4 { class: "text-sm font-bold text-slate-400 uppercase tracking-widest", "{t(locale, TK::WestElementsTitle)}" }
                                            div { class: "space-y-2.5",
                                                div {
                                                    div { class: "flex justify-between text-xs font-bold text-slate-300",
                                                        span { "🔥 {t(locale, TK::WestElementFire)}" }
                                                        span { "{res.elements.fire:.1}%" }
                                                    }
                                                    div { class: "h-2 w-full bg-slate-800 rounded-full overflow-hidden mt-1",
                                                        div { class: "h-full bg-gradient-to-r from-red-500 to-amber-500 rounded-full", style: "width: {res.elements.fire}%" }
                                                    }
                                                }
                                                div {
                                                    div { class: "flex justify-between text-xs font-bold text-slate-300",
                                                        span { "⛰️ {t(locale, TK::WestElementEarth)}" }
                                                        span { "{res.elements.earth:.1}%" }
                                                    }
                                                    div { class: "h-2 w-full bg-slate-800 rounded-full overflow-hidden mt-1",
                                                        div { class: "h-full bg-gradient-to-r from-emerald-500 to-teal-500 rounded-full", style: "width: {res.elements.earth}%" }
                                                    }
                                                }
                                                div {
                                                    div { class: "flex justify-between text-xs font-bold text-slate-300",
                                                        span { "💨 {t(locale, TK::WestElementAir)}" }
                                                        span { "{res.elements.air:.1}%" }
                                                    }
                                                    div { class: "h-2 w-full bg-slate-800 rounded-full overflow-hidden mt-1",
                                                        div { class: "h-full bg-gradient-to-r from-cyan-400 to-sky-400 rounded-full", style: "width: {res.elements.air}%" }
                                                    }
                                                }
                                                div {
                                                    div { class: "flex justify-between text-xs font-bold text-slate-300",
                                                        span { "💧 {t(locale, TK::WestElementWater)}" }
                                                        span { "{res.elements.water:.1}%" }
                                                    }
                                                    div { class: "h-2 w-full bg-slate-800 rounded-full overflow-hidden mt-1",
                                                        div { class: "h-full bg-gradient-to-r from-blue-500 to-indigo-500 rounded-full", style: "width: {res.elements.water}%" }
                                                    }
                                                }
                                            }
                                        }

                                        div { class: "space-y-3 pt-3 border-t border-slate-800/60",
                                            h4 { class: "text-sm font-bold text-slate-400 uppercase tracking-widest", "{t(locale, TK::WestModalitiesTitle)}" }
                                            div { class: "space-y-2.5",
                                                div {
                                                    div { class: "flex justify-between text-xs font-bold text-slate-300",
                                                        span { "⚡ {t(locale, TK::WestModalityCardinal)}" }
                                                        span { "{res.modalities.cardinal:.1}%" }
                                                    }
                                                    div { class: "h-2 w-full bg-slate-800 rounded-full overflow-hidden mt-1",
                                                        div { class: "h-full bg-gradient-to-r from-purple-500 to-pink-500 rounded-full", style: "width: {res.modalities.cardinal}%" }
                                                    }
                                                }
                                                div {
                                                    div { class: "flex justify-between text-xs font-bold text-slate-300",
                                                        span { "🔒 {t(locale, TK::WestModalityFixed)}" }
                                                        span { "{res.modalities.fixed:.1}%" }
                                                    }
                                                    div { class: "h-2 w-full bg-slate-800 rounded-full overflow-hidden mt-1",
                                                        div { class: "h-full bg-gradient-to-r from-indigo-500 to-blue-500 rounded-full", style: "width: {res.modalities.fixed}%" }
                                                    }
                                                }
                                                div {
                                                    div { class: "flex justify-between text-xs font-bold text-slate-300",
                                                        span { "🌀 {t(locale, TK::WestModalityMutable)}" }
                                                        span { "{res.modalities.mutable:.1}%" }
                                                    }
                                                    div { class: "h-2 w-full bg-slate-800 rounded-full overflow-hidden mt-1",
                                                        div { class: "h-full bg-gradient-to-r from-teal-500 to-emerald-400 rounded-full", style: "width: {res.modalities.mutable}%" }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Aspects List Card
                                    div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md shadow-lg",
                                        h3 { class: "text-base font-bold text-slate-300 mb-4 border-b border-slate-800/60 pb-2", "{t(locale, TK::WestAspectsTitle)}" }
                                        div { class: "max-h-[480px] overflow-y-auto space-y-2 pr-1",
                                            if res.aspects.is_empty() {
                                                p { class: "text-sm text-slate-500 text-center py-4", "No aspects found." }
                                            } else {
                                                {res.aspects.iter().map(|asp| {
                                                    let (b_a_emoji, b_a_name) = get_planet_emoji_and_name(&asp.body_a_name, locale);
                                                    let (b_b_emoji, b_b_name) = get_planet_emoji_and_name(&asp.body_b_name, locale);
                                                    let (asp_emoji, asp_name) = get_aspect_emoji_and_name(asp.aspect_type, locale);

                                                    let border_color = match asp.aspect_type {
                                                        eon_western::AspectType::Conjunction => "border-violet-500/20 hover:border-violet-500/40 bg-violet-500/5",
                                                        eon_western::AspectType::Sextile => "border-sky-500/20 hover:border-sky-500/40 bg-sky-500/5",
                                                        eon_western::AspectType::Square => "border-red-500/20 hover:border-red-500/40 bg-red-500/5",
                                                        eon_western::AspectType::Trine => "border-emerald-500/20 hover:border-emerald-500/40 bg-emerald-500/5",
                                                        eon_western::AspectType::Opposition => "border-amber-500/20 hover:border-amber-500/40 bg-amber-500/5",
                                                        _ => "border-purple-500/20 hover:border-purple-500/40 bg-purple-500/5",
                                                    };

                                                    let dynamics_badge = match asp.dynamics {
                                                        eon_western::AspectDynamics::Applying => ("text-emerald-400 bg-emerald-950/40 border-emerald-800/40", "Applying ➔"),
                                                        eon_western::AspectDynamics::Separating => ("text-slate-400 bg-slate-950/40 border-slate-800/40", "Separating ⬅"),
                                                        eon_western::AspectDynamics::Exact => ("text-amber-400 bg-amber-950/40 border-amber-800/40", "Exact 🎯"),
                                                    };

                                                    rsx! {
                                                        div {
                                                            key: "{asp.body_a_name}-{asp.body_b_name}-{asp.aspect_type.angle()}",
                                                            class: "flex items-center justify-between p-3 rounded-xl border transition-all duration-200 {border_color}",

                                                            div { class: "flex items-center gap-2",
                                                                div { class: "flex items-center gap-1.5 text-slate-200 text-xs font-semibold",
                                                                    span { "{b_a_emoji}" }
                                                                    span { "{b_a_name}" }
                                                                }
                                                                span { class: "text-slate-400 font-bold", "—" }
                                                                div { class: "flex items-center gap-1.5 text-slate-200 text-xs font-semibold",
                                                                    span { "{b_b_emoji}" }
                                                                    span { "{b_b_name}" }
                                                                }
                                                            }

                                                            div { class: "text-right flex flex-col justify-center items-end gap-0.5",
                                                                span { class: "text-xs font-bold text-slate-300", "{asp_emoji} {asp_name}" }
                                                                div { class: "flex items-center gap-1.5",
                                                                    span { class: "text-[9px] font-semibold px-1.5 py-0.2 rounded border {dynamics_badge.0}", "{dynamics_badge.1}" }
                                                                    span { class: "text-[10px] text-slate-500 font-mono", "Orb: {asp.orb:.2}°" }
                                                                }
                                                            }
                                                        }
                                                    }
                                                })}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {
                            div { class: "text-slate-500 text-center py-10", "No analysis data." }
                        }
                    }
                }
            }
        }
    }
}
