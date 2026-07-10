use crate::i18n::{format_weight_score, t, TK};
use crate::store::{AnalysisState, TaskStatus};
use dioxus::prelude::*;
use eon_service::dto::{SajuAnalysisInput, VedicAnalysisInput};
use eon_service::facade;

use crate::components::shared::birth_form::BirthForm;

#[component]
pub fn TierTab() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();
    let mut copied_feedback = use_signal(|| false);
    // Reactive trigger for manual analysis runs
    let mut analysis_trigger = use_signal(|| 0);
    let run_analysis = move |_| {
        let current = *analysis_trigger.peek();
        analysis_trigger.set(current + 1);
    };

    // Auto-run or manually triggered analysis when form or trigger changes
    let state_cloned = state.clone();
    use_effect(move || {
        let form = state_cloned.form.read().clone();
        let _trig = *analysis_trigger.read();

        if form.year > 0 {
            let mut state = state_cloned.clone();
            spawn(async move {
                state.tier.write().status = TaskStatus::Loading;

                let base_input = form.to_analysis_input();

                let saju_input = SajuAnalysisInput::new(
                    base_input.clone(),
                    form.is_male,
                    form.use_night_rat_hour,
                    Some(false),
                );
                let vedic_input = VedicAnalysisInput::new(base_input.clone(), Some(false), None);

                // 병렬이 좋지만 간소화를 위해 순차 실행
                let saju_res = match facade::analyze_saju(saju_input) {
                    Ok(r) => r,
                    Err(e) => {
                        state.tier.write().error = Some(e.to_string());
                        state.tier.write().status = TaskStatus::Error(e.to_string());
                        return;
                    }
                };

                let vedic_res = match facade::analyze_vedic(vedic_input) {
                    Ok(r) => r,
                    Err(e) => {
                        state.tier.write().error = Some(e.to_string());
                        state.tier.write().status = TaskStatus::Error(e.to_string());
                        return;
                    }
                };

                match facade::analyze_destiny_tier(saju_res, vedic_res, None) {
                    Ok(res) => {
                        state.tier.write().data = Some(res);
                        state.tier.write().status = TaskStatus::Success;
                    }
                    Err(e) => {
                        state.tier.write().error = Some(e.to_string());
                        state.tier.write().status = TaskStatus::Error(e.to_string());
                    }
                }
            });
        }
    });

    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            BirthForm {}

            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-yellow-200 to-amber-500 bg-clip-text text-transparent",
                    "{t(locale, TK::TierTitle)}"
                }
                div { class: "flex items-center gap-2",
                    button {
                        class: if *copied_feedback.read() {
                            "p-2.5 bg-emerald-650 hover:bg-emerald-600 text-white border border-emerald-500/50 rounded-xl transition-all cursor-pointer flex items-center justify-center active:scale-95 gap-1"
                        } else {
                            "p-2.5 bg-slate-800 hover:bg-slate-700 active:bg-slate-600 border border-slate-700/50 rounded-xl text-slate-300 hover:text-white transition-all cursor-pointer flex items-center justify-center active:scale-95 gap-1"
                        },
                        title: "Copy Report",
                        onclick: move |_| {
                            if let Some(ref data) = state.tier.read().data.as_ref() {
                                let txt = crate::components::shared::export_markdown::export_tier_to_markdown(data, &state.form.read(), locale);
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
                        class: "p-2.5 bg-slate-800 hover:bg-slate-700 active:bg-slate-600 border border-slate-700/50 rounded-xl text-slate-300 hover:text-white transition-all cursor-pointer flex items-center justify-center active:scale-95",
                        onclick: run_analysis,
                        title: "{t(locale, TK::BtnCalculate)}",
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

            match &state.tier.read().status {
                TaskStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500",
                        span { class: "text-5xl", "⚖️" }
                        p { class: "text-lg font-medium", "{t(locale, TK::StatusIdleHint)}" }
                    }
                },
                TaskStatus::Loading => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3",
                        div { class: "w-12 h-12 rounded-full border-4 border-amber-500/30 border-t-amber-400 animate-spin" }
                        p { class: "text-amber-400 font-medium animate-pulse", "{t(locale, TK::StatusLoadingTier)}" }
                    }
                },
                TaskStatus::Error(e) => rsx! {
                    div { class: "p-4 rounded-xl bg-red-900/20 border border-red-800/50 text-red-400", "{t(locale, TK::StatusError)}: {e}" }
                },
                TaskStatus::Success => {
                    if let Some(data) = &state.tier.read().data {
                        rsx! {
                            // ── 1. 메인 티어 배너 ──────────────────────────────
                            div { class: "relative p-8 rounded-3xl bg-gradient-to-br from-slate-900 to-slate-800 border border-amber-500/30 overflow-hidden shadow-2xl shadow-amber-900/20 flex flex-col items-center text-center",
                                div { class: "absolute -top-20 -right-20 w-64 h-64 bg-amber-500/10 rounded-full blur-3xl pointer-events-none" }
                                div { class: "absolute -bottom-20 -left-20 w-64 h-64 bg-orange-500/10 rounded-full blur-3xl pointer-events-none" }

                                h3 { class: "text-sm font-bold text-amber-400/80 tracking-widest uppercase mb-2", "{t(locale, TK::TierPotentialTitle)}" }
                                div { class: "text-8xl font-black bg-gradient-to-br from-yellow-200 via-amber-400 to-orange-500 text-transparent bg-clip-text drop-shadow-[0_0_40px_rgba(251,191,36,0.3)] mb-4 leading-tight",
                                    "{data.destiny_tier.grade}"
                                }
                                div { class: "text-2xl text-amber-100 font-bold mb-2 tracking-wide", "{data.destiny_tier.label}" }
                                p { class: "text-slate-400 text-sm max-w-lg", "{data.destiny_tier.desc}" }

                                div { class: "w-full max-w-md mt-8",
                                    div { class: "flex justify-between text-xs text-slate-400 font-bold mb-2",
                                        span { "{t(locale, TK::TierDestinyPowerScore)}" }
                                        span { class: "text-amber-400", "{data.destiny_score:.1} / 100" }
                                    }
                                    div { class: "h-3 bg-slate-800 rounded-full overflow-hidden border border-slate-700/50",
                                        div {
                                            class: "h-full bg-gradient-to-r from-orange-500 via-amber-500 to-yellow-400 rounded-full transition-all duration-1000 relative",
                                            style: "width: {data.destiny_score}%",
                                            div { class: "absolute inset-0 bg-white/20 w-full h-full animate-[shimmer_2s_infinite]" }
                                        }
                                    }
                                }
                            }

                            // ── 2. 분야별 티어 ────────────────────────────────
                            div { class: "grid grid-cols-2 lg:grid-cols-4 gap-4",
                                {data.domain_tiers.iter().map(|dt| rsx! {
                                    div { class: "p-4 rounded-2xl bg-slate-900/80 border border-slate-800 flex flex-col items-center justify-center text-center gap-2 transition-transform hover:scale-105",
                                        div { class: "text-3xl font-black text-amber-300 drop-shadow-md", "{dt.tier}" }
                                        div { class: "text-sm text-slate-400 font-medium", "{dt.domain}" }
                                    }
                                })}
                            }

                            // ── 3. 상세 세부 점수 ───────────────────────────────
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                // 사주 기반
                                div { class: "p-5 rounded-2xl bg-slate-900 border border-slate-800",
                                    div { class: "flex justify-between items-center mb-4",
                                        h4 { class: "font-bold text-emerald-400", "{t(locale, TK::TierEasternSajuScore)}" }
                                        span { class: "text-2xl font-black text-slate-200", "{data.saju_result.score:.1}" }
                                    }
                                    ul { class: "space-y-2",
                                        {data.saju_result.highlights.iter().map(|hl| rsx! {
                                            li { class: "flex gap-2 text-sm text-slate-400",
                                                span { class: "text-emerald-500", "✓" }
                                                span { "{hl}" }
                                            }
                                        })}
                                    }
                                }
                                // 베딕 기반
                                div { class: "p-5 rounded-2xl bg-slate-900 border border-slate-800",
                                    div { class: "flex justify-between items-center mb-4",
                                        h4 { class: "font-bold text-blue-400", "{t(locale, TK::TierVedicScore)}" }
                                        span { class: "text-2xl font-black text-slate-200", "{data.vedic_result.score:.1}" }
                                    }
                                    ul { class: "space-y-2",
                                        {data.vedic_result.highlights.iter().map(|hl| rsx! {
                                            li { class: "flex gap-2 text-sm text-slate-400",
                                                span { class: "text-blue-500", "✓" }
                                                span { "{hl}" }
                                            }
                                        })}
                                    }
                                }
                            }

                            // ── 4. 강점 & 약점 요약 ───────────────────────────
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                div { class: "p-5 rounded-2xl bg-emerald-900/10 border border-emerald-900/30",
                                    h4 { class: "font-bold text-emerald-400 mb-3 flex items-center gap-2", span { class: "text-lg", "🌟" } "{t(locale, TK::TierStrengthsInherent)}" }
                                    ul { class: "space-y-2",
                                        {data.strengths.iter().map(|s| rsx! {
                                            li { class: "text-sm text-emerald-200/80 leading-relaxed", "• {s}" }
                                        })}
                                    }
                                }
                                div { class: "p-5 rounded-2xl bg-red-900/10 border border-red-900/30",
                                    h4 { class: "font-bold text-red-400 mb-3 flex items-center gap-2", span { class: "text-lg", "⚠️" } "{t(locale, TK::TierWeaknessesCaution)}" }
                                    ul { class: "space-y-2",
                                        {data.weaknesses.iter().map(|w| rsx! {
                                            li { class: "text-sm text-red-200/80 leading-relaxed", "• {w}" }
                                        })}
                                    }
                                }
                            }

                            // ── 5. 컴포넌트 세부 가중치 ─────────────────────────
                            if !data.detailed_components.is_empty() {
                                div { class: "p-5 rounded-2xl bg-slate-900 border border-slate-800",
                                    h4 { class: "font-bold text-slate-300 mb-4", "{t(locale, TK::TierWeightsTitle)}" }
                                    div { class: "space-y-4",
                                        {data.detailed_components.iter().map(|c| {
                                            let pct = c.score as u32;
                                            let weight_lbl = format_weight_score(locale, (c.weight * 100.0) as f64, pct as f64);
                                            rsx! {
                                                div {
                                                    div { class: "flex justify-between items-end mb-1",
                                                        span { class: "text-sm font-medium text-slate-300", "{c.label}" }
                                                        span { class: "text-xs font-mono text-slate-500", "{weight_lbl}" }
                                                    }
                                                    div { class: "h-2 w-full bg-slate-800 rounded-full overflow-hidden",
                                                        div { class: "h-full bg-slate-500 rounded-full", style: "width: {pct}%" }
                                                    }
                                                    if !c.reasons.is_empty() {
                                                        p { class: "text-xs text-slate-500 mt-1", "{c.reasons.join(\", \")}" }
                                                    }
                                                }
                                            }
                                        })}
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
