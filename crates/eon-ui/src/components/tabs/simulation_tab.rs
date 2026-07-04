use crate::components::shared::birth_form::BirthForm;
use crate::i18n::{format_age, format_age_range, t, Locale, TK};
use crate::store::{AnalysisState, TaskStatus};
use dioxus::prelude::*;
use eon_saju::engine::emulator::YearlyScore;
use eon_service::dto::SajuAnalysisInput;
use eon_service::facade;

#[component]
pub fn SimulationTab() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

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
                state.saju.write().status = TaskStatus::Loading;
                let input = SajuAnalysisInput::new(
                    form.to_analysis_input(),
                    form.is_male,
                    form.use_night_rat_hour,
                    Some(false),
                );
                match facade::analyze_saju(input) {
                    Ok(res) => {
                        state.saju.write().data = Some(res);
                        state.saju.write().status = TaskStatus::Success;
                    }
                    Err(e) => {
                        state.saju.write().error = Some(e.to_string());
                        state.saju.write().status = TaskStatus::Error(e.to_string());
                    }
                }
            });
        }
    });

    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            BirthForm {}

            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-purple-200 to-pink-400 bg-clip-text text-transparent",
                    "{t(locale, TK::SectionSimulation)}"
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

            match &state.saju.read().status {
                TaskStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500",
                        span { class: "text-5xl", "🔮" }
                        p { class: "text-lg font-medium", "{t(locale, TK::StatusIdleHint)}" }
                    }
                },
                TaskStatus::Loading => rsx! {
                    div { class: "flex flex-col items-center justify-center py-16 gap-3",
                        div { class: "w-12 h-12 rounded-full border-4 border-purple-500/30 border-t-purple-400 animate-spin" }
                        p { class: "text-purple-400 font-medium animate-pulse", "{t(locale, TK::StatusLoading)}" }
                    }
                },
                TaskStatus::Error(e) => rsx! {
                    div { class: "p-4 rounded-xl bg-red-900/20 border border-red-800/50 text-red-400", "{t(locale, TK::StatusError)}: {e}" }
                },
                TaskStatus::Success => {
                    if let Some(saju) = &state.saju.read().data {
                        let c = saju.complexity.as_ref();
                        let bottleneck_lbl = saju.qi_topology.bottleneck.map(|e| {
                            match locale {
                                Locale::Ko => format!("병목: {}({})", e.hangul(), e.hanja()),
                                Locale::En => format!("Bottleneck: {}", e.hangul()),
                                Locale::Zh => format!("瓶颈: {}", e.hangul()),
                                Locale::Ru => format!("Узкое место: {}", e.hangul()),
                            }
                        }).unwrap_or_else(|| t(locale, TK::LabelNone).to_string());

                        let cc_sub = if let Some(comp) = c {
                            match locale {
                                Locale::Ko => format!("안정도: {}", comp.stability_grade),
                                Locale::En => format!("Stability: {}", comp.stability_grade),
                                Locale::Zh => format!("稳定性: {}", comp.stability_grade),
                                Locale::Ru => format!("Стабильность: {}", comp.stability_grade),
                            }
                        } else {
                            "".to_string()
                        };

                        let cc_card = if let Some(comp) = c {
                            rsx! {
                                StatCard { icon: "🔬", label: t(locale, TK::SimComplexity), value: format!("{}", comp.cyclomatic_complexity), sub: cc_sub, color: "from-blue-900/30 to-slate-900 border-blue-700/40" }
                            }
                        } else {
                            rsx! { div {} }
                        };

                        rsx! {
                            // ── 요약 스탯 카드 ─────────────────────────────────
                            div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                                StatCard { icon: "⚠️", label: t(locale, TK::SimVulnerability), value: format!("{}", saju.crash_count), sub: t(locale, TK::SimVmCrashes).to_string(), color: "from-red-900/30 to-slate-900 border-red-700/40" }
                                StatCard { icon: "🌀", label: t(locale, TK::SimEntropy), value: format!("{:.2}", saju.entropy.score), sub: saju.entropy.description.clone(), color: "from-purple-900/30 to-slate-900 border-purple-700/40" }
                                StatCard { icon: "⚡", label: t(locale, TK::SimQiEfficiency), value: format!("{:.0}%", saju.qi_topology.throughput * 100.0),
                                    sub: bottleneck_lbl,
                                    color: "from-amber-900/30 to-slate-900 border-amber-700/40"
                                }
                                {cc_card}
                            }

                            // ── 인생 점수 SVG 라인차트 ────────────────────────
                            if !saju.report.timeline.is_empty() {
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5",
                                    div { class: "flex items-center justify-between mb-4",
                                        h3 { class: "font-semibold text-slate-200", "{t(locale, TK::SimEnergyFlowTitle)}" }
                                        div { class: "flex items-center gap-4 text-xs text-slate-500",
                                            span { class: "flex items-center gap-1.5",
                                                span { class: "w-4 h-0.5 bg-purple-400 inline-block rounded" } "{t(locale, TK::SimScoreOverall)}"
                                            }
                                            span { class: "flex items-center gap-1.5",
                                                span { class: "w-4 h-0.5 bg-amber-500 inline-block rounded" } "{t(locale, TK::SimScoreAverage)}"
                                            }
                                        }
                                    }
                                    { LifelineChart(&saju.report.timeline) }
                                }
                            }

                            // ── 인생 골든 타임 (Golden Time) ───────────────────
                            if let Some(gt) = &saju.report.golden_time {
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 bg-gradient-to-r from-amber-950/20 to-slate-900 shadow-xl border-amber-900/30 flex items-center justify-between flex-wrap gap-4",
                                    div { class: "flex items-center gap-4",
                                        div { class: "text-4xl", "👑" }
                                        div { class: "space-y-1",
                                            h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "{t(locale, TK::SimGoldenTimeTitle)}" }
                                            p { class: "text-lg font-bold bg-gradient-to-r from-amber-200 to-yellow-400 bg-clip-text text-transparent",
                                                "{format_age_range(locale, gt.start_age as i32, gt.end_age as i32)}"
                                            }
                                            p { class: "text-xs text-slate-400 leading-relaxed", "{gt.description}" }
                                        }
                                    }
                                    div { class: "text-right bg-slate-900/50 border border-slate-800 rounded-xl px-4 py-2.5",
                                        p { class: "text-[10px] text-slate-500 font-semibold tracking-wider uppercase", "{t(locale, TK::SimGoldenTimeAvg)}" }
                                        p { class: "text-2xl font-mono font-extrabold text-amber-400 mt-0.5", "{gt.average_score:.1}" }
                                    }
                                }
                            }

                            // ── VM 시뮬레이션 프레임 ──────────────────────────
                            if !saju.report.simulation_frames.is_empty() {
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
                                    div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3",
                                        h3 { class: "font-semibold text-slate-200", "{t(locale, TK::SimLifeFramesTitle)}" }
                                    }
                                    div { class: "overflow-x-auto",
                                        table { class: "w-full text-xs",
                                            thead {
                                                tr { class: "bg-slate-800/30 text-slate-400",
                                                    th { class: "px-4 py-3 text-left font-medium", "{t(locale, TK::SimColAge)}" }
                                                    th { class: "px-4 py-3 text-left font-medium", "{t(locale, TK::SimColMajorLuck)}" }
                                                    th { class: "px-4 py-3 text-left font-medium", "{t(locale, TK::SimColAnnualLuck)}" }
                                                    th { class: "px-4 py-3 text-left font-medium", "{t(locale, TK::SimColScore)}" }
                                                    th { class: "px-4 py-3 text-left font-medium", "{t(locale, TK::SimColTrend)}" }
                                                }
                                            }
                                            tbody { class: "divide-y divide-slate-800",
                                                {saju.report.simulation_frames.iter().map(|frame| {
                                                    let score_color = if frame.score > 60.0 {
                                                        "text-emerald-400"
                                                    } else if frame.score < 30.0 {
                                                        "text-red-400"
                                                    } else {
                                                        "text-slate-300"
                                                    };
                                                    rsx! {
                                                        tr { class: "hover:bg-slate-800/20 transition-colors",
                                                            td { class: "px-4 py-2 font-mono text-slate-300", "{format_age(locale, frame.age as i32)}" }
                                                            td { class: "px-4 py-2 font-serif text-amber-300 font-bold", "{frame.major_ganzi.hanja()} ({frame.major_ganzi.hangul()})" }
                                                            td { class: "px-4 py-2 font-serif text-slate-400", "{frame.ganzi.hanja()}" }
                                                            td { class: "px-4 py-2 font-mono {score_color}", "{frame.score:.1}" }
                                                            td { class: "px-4 py-2",
                                                                EnergyBar { value: frame.score as f64 }
                                                            }
                                                        }
                                                    }
                                                })}
                                            }
                                        }
                                    }
                                }
                            }

                            // ── 카르마 부하 진단 ──────────────────────────────
                            if !saju.load_diagnostics.is_empty() {
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5",
                                    h3 { class: "font-semibold text-slate-200 mb-3", "{t(locale, TK::SimKarmaDiagnostics)}" }
                                    div { class: "space-y-2",
                                        {saju.load_diagnostics.iter().map(|d| {
                                            let strategy_lbl = format!("{}: ", t(locale, TK::SimStrategy));
                                            rsx! {
                                                div { class: "flex items-start gap-2 p-3 rounded-lg bg-slate-800/50 text-sm",
                                                    span { class: "text-amber-400 mt-0.5 shrink-0", "▸" }
                                                    div {
                                                        p { class: "text-slate-300", "{format_age(locale, d.age as i32)}: {d.reason}" }
                                                        p { class: "text-xs text-slate-500 mt-0.5", "{strategy_lbl}{d.strategy}" }
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

/// 생애 인생 점수 SVG 라인차트
fn LifelineChart(timeline: &[YearlyScore]) -> Element {
    if timeline.is_empty() {
        return rsx! { div {} };
    }

    let width = 800.0f64;
    let height = 200.0f64;
    let pad_left = 40.0f64;
    let pad_right = 20.0f64;
    let pad_top = 16.0f64;
    let pad_bottom = 24.0f64;

    let chart_w = width - pad_left - pad_right;
    let chart_h = height - pad_top - pad_bottom;

    let min_score = timeline
        .iter()
        .map(|t| t.total_score)
        .fold(f64::INFINITY, f64::min);
    let max_score = timeline
        .iter()
        .map(|t| t.total_score)
        .fold(f64::NEG_INFINITY, f64::max);
    let avg_score = timeline.iter().map(|t| t.total_score).sum::<f64>() / timeline.len() as f64;
    let score_range = (max_score - min_score).max(1.0);

    let n = timeline.len() as f64;
    let points: Vec<(f64, f64)> = timeline
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let x = pad_left + (i as f64 / (n - 1.0).max(1.0)) * chart_w;
            let y = pad_top + (1.0 - (t.total_score - min_score) / score_range) * chart_h;
            (x, y)
        })
        .collect();

    let polyline_pts = points
        .iter()
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect::<Vec<_>>()
        .join(" ");

    let avg_y = pad_top + (1.0 - (avg_score - min_score) / score_range) * chart_h;

    let threshold = min_score + score_range * 0.8;

    let last_pt = points
        .last()
        .cloned()
        .unwrap_or((pad_left, pad_top + chart_h));
    let first_pt = points
        .first()
        .cloned()
        .unwrap_or((pad_left, pad_top + chart_h));
    let area_path = format!(
        "M{:.1},{:.1} {} L{:.1},{:.1} L{:.1},{:.1} Z",
        first_pt.0,
        pad_top + chart_h,
        polyline_pts,
        last_pt.0,
        pad_top + chart_h,
        first_pt.0,
        pad_top + chart_h,
    );

    let max_lbl = format!("{max_score:.0}");
    let min_lbl = format!("{min_score:.0}");
    let avg_y_str = format!("{avg_y:.1}");
    let area_d = area_path.clone();
    let line_pts = polyline_pts.clone();

    rsx! {
        svg {
            view_box: "0 0 {width} {height}",
            class: "w-full h-auto",
            xmlns: "http://www.w3.org/2000/svg",

            defs {
                linearGradient { id: "area-grad", x1: "0", y1: "0", x2: "0", y2: "1",
                    stop { offset: "0%", style: "stop-color:#a855f7;stop-opacity:0.4" }
                    stop { offset: "100%", style: "stop-color:#a855f7;stop-opacity:0.02" }
                }
            }

            {(0..5u32).map(|i| {
                let y = pad_top + (i as f64 / 4.0) * chart_h;
                rsx! {
                    line { x1: "{pad_left}", y1: "{y:.1}", x2: "{width - pad_right}", y2: "{y:.1}", stroke: "#1e293b", stroke_width: "1" }
                }
            })}

            line {
                x1: "{pad_left}", y1: "{avg_y_str}", x2: "{width - pad_right}", y2: "{avg_y_str}",
                stroke: "#d97706", stroke_width: "1", stroke_dasharray: "4 3", opacity: "0.5"
            }

            path { d: "{area_d}", fill: "url(#area-grad)" }

            polyline {
                points: "{line_pts}",
                fill: "none",
                stroke: "#a855f7",
                stroke_width: "2",
                stroke_linejoin: "round",
                stroke_linecap: "round",
            }

            {points.iter().zip(timeline.iter()).filter_map(|((x, y), t)| {
                if t.total_score >= threshold {
                    let cx = format!("{x:.1}");
                    let cy = format!("{y:.1}");
                    Some(rsx! {
                        circle { cx: "{cx}", cy: "{cy}", r: "4", fill: "#fbbf24", opacity: "0.8" }
                    })
                } else {
                    None
                }
            })}

            {
                let locale = *use_context::<AnalysisState>().locale.read();
                rsx! {
                    {(0..=10u32).map(|i| {
                        let idx = ((i as usize) * (timeline.len() - 1)) / 10;
                        let idx = idx.min(timeline.len() - 1);
                        let x = pad_left + (idx as f64 / (n - 1.0).max(1.0)) * chart_w;
                        let year_lbl = timeline[idx].age;
                        let age_str = format_age(locale, year_lbl as i32);
                        rsx! {
                            text {
                                x: "{x:.1}", y: "{height - 6.0}",
                                text_anchor: "middle", font_size: "9",
                                fill: "#64748b",
                                "{age_str}"
                            }
                        }
                    })}
                }
            }

            text { x: "{pad_left - 5.0}", y: "{pad_top + 4.0}", text_anchor: "end", font_size: "9", fill: "#64748b", "{max_lbl}" }
            text { x: "{pad_left - 5.0}", y: "{pad_top + chart_h}", text_anchor: "end", font_size: "9", fill: "#64748b", "{min_lbl}" }
        }
    }
}

#[component]
fn EnergyBar(value: f64) -> Element {
    let pct = value.clamp(0.0, 100.0) as u32;
    let color = if value > 60.0 {
        "bg-emerald-500"
    } else if value < 30.0 {
        "bg-red-500"
    } else {
        "bg-amber-500"
    };
    rsx! {
        div { class: "w-20 h-2 bg-slate-700 rounded-full overflow-hidden",
            div { class: "h-full {color} rounded-full", style: "width: {pct}%" }
        }
    }
}

#[component]
fn StatCard(
    icon: &'static str,
    label: &'static str,
    value: String,
    sub: String,
    color: String,
) -> Element {
    rsx! {
        div { class: "p-4 rounded-2xl bg-gradient-to-b {color} border flex flex-col gap-1",
            div { class: "flex items-center gap-2",
                span { class: "text-2xl", "{icon}" }
                span { class: "text-xs text-slate-400 font-medium", "{label}" }
            }
            p { class: "text-2xl font-bold text-slate-100 mt-1", "{value}" }
            p { class: "text-xs text-slate-400", "{sub}" }
        }
    }
}
