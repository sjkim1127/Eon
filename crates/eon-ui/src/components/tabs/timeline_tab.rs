use crate::i18n::{format_age, t, Locale, TK};
use crate::store::AnalysisState;
use dioxus::prelude::*;
use eon_saju::engine::emulator::YearlyScore;

#[derive(Clone, Copy)]
struct TimelineLabels {
    overall: &'static str,
    trend: &'static str,
    volatility: &'static str,
    transition: &'static str,
    peak: &'static str,
    valley: &'static str,
    average: &'static str,
    empty_title: &'static str,
    empty_body: &'static str,
    disclaimer: &'static str,
}

#[derive(Clone, Copy)]
struct TimelineSummary {
    peak_year: i32,
    peak_age: u32,
    peak_score: f64,
    valley_year: i32,
    valley_age: u32,
    valley_score: f64,
    average_score: f64,
    transition_years: usize,
}

#[component]
pub fn TimelineTab() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();
    let labels = timeline_labels(locale);
    let timeline = state
        .saju
        .read()
        .data
        .as_ref()
        .map(|output| output.report.timeline.clone())
        .unwrap_or_default();
    let summary = summarize_timeline(&timeline);

    rsx! {
        div {
            class: "w-full h-full flex flex-col p-4 overflow-y-auto overflow-x-hidden gap-4",
            h2 { class: "text-2xl font-bold text-white tracking-wider", "{t(locale, TK::NavTimeline)}" }

            if let Some(summary) = summary {
                div { class: "grid grid-cols-2 xl:grid-cols-4 gap-3",
                    SummaryCard {
                        label: labels.peak,
                        value: format!("{} · {:.1}", summary.peak_year, summary.peak_score),
                        detail: format_age(locale, summary.peak_age as i32),
                    }
                    SummaryCard {
                        label: labels.valley,
                        value: format!("{} · {:.1}", summary.valley_year, summary.valley_score),
                        detail: format_age(locale, summary.valley_age as i32),
                    }
                    SummaryCard {
                        label: labels.average,
                        value: format!("{:.1}", summary.average_score),
                        detail: labels.overall.to_string(),
                    }
                    SummaryCard {
                        label: labels.transition,
                        value: summary.transition_years.to_string(),
                        detail: labels.volatility.to_string(),
                    }
                }
            }

            if timeline.is_empty() {
                div { class: "min-h-[420px] flex flex-col items-center justify-center gap-3 rounded-2xl border border-dashed border-zinc-700 bg-zinc-900/40 px-6 text-center",
                    span { class: "text-5xl", "📈" }
                    h3 { class: "text-lg font-semibold text-zinc-200", "{labels.empty_title}" }
                    p { class: "max-w-xl text-sm leading-relaxed text-zinc-500", "{labels.empty_body}" }
                }
            } else {
                div { class: "bg-zinc-900/70 border border-zinc-800 rounded-2xl p-4 shadow-xl",
                    div { class: "flex flex-wrap items-center gap-x-5 gap-y-2 mb-3 text-xs text-zinc-400",
                        LegendItem { class_name: "bg-emerald-400", label: labels.overall }
                        LegendItem { class_name: "bg-sky-400", label: labels.trend }
                        LegendItem { class_name: "bg-amber-400", label: labels.volatility }
                        span { class: "flex items-center gap-2",
                            span { class: "w-4 h-3 rounded-sm bg-rose-500/25 border border-rose-400/30" }
                            "{labels.transition}"
                        }
                    }
                    div { class: "w-full overflow-x-auto rounded-xl border border-zinc-800 bg-zinc-950/70 p-2",
                        {timeline_chart(&timeline, locale)}
                    }
                    p { class: "mt-3 text-xs leading-relaxed text-zinc-500", "{labels.disclaimer}" }
                }
            }
        }
    }
}

#[component]
fn SummaryCard(label: &'static str, value: String, detail: String) -> Element {
    rsx! {
        div { class: "rounded-xl border border-zinc-800 bg-zinc-900/70 px-4 py-3",
            p { class: "text-[11px] font-semibold uppercase tracking-wider text-zinc-500", "{label}" }
            p { class: "mt-1 text-lg font-bold font-mono text-zinc-100", "{value}" }
            p { class: "mt-0.5 text-xs text-zinc-500", "{detail}" }
        }
    }
}

#[component]
fn LegendItem(class_name: &'static str, label: &'static str) -> Element {
    rsx! {
        span { class: "flex items-center gap-2",
            span { class: "w-4 h-0.5 rounded-full {class_name}" }
            "{label}"
        }
    }
}

fn timeline_chart(timeline: &[YearlyScore], locale: Locale) -> Element {
    if timeline.is_empty() {
        return rsx! { div {} };
    }

    let width = 1200.0;
    let height = 540.0;
    let pad_left = 58.0;
    let pad_right = 24.0;
    let pad_top = 24.0;
    let pad_bottom = 44.0;
    let chart_width = width - pad_left - pad_right;
    let chart_height = height - pad_top - pad_bottom;
    let denominator = timeline.len().saturating_sub(1).max(1) as f64;
    let x_for = |index: usize| pad_left + index as f64 / denominator * chart_width;
    let y_for = |score: f64| pad_top + (1.0 - score.clamp(0.0, 100.0) / 100.0) * chart_height;

    let overall_points = timeline
        .iter()
        .enumerate()
        .filter(|(_, point)| point.total_score.is_finite())
        .map(|(index, point)| format!("{:.1},{:.1}", x_for(index), y_for(point.total_score)))
        .collect::<Vec<_>>()
        .join(" ");
    let trend_points = timeline
        .iter()
        .enumerate()
        .filter_map(|(index, point)| {
            point
                .trend_ma_5yr
                .filter(|score| score.is_finite())
                .map(|score| format!("{:.1},{:.1}", x_for(index), y_for(score)))
        })
        .collect::<Vec<_>>()
        .join(" ");
    let volatility_points = timeline
        .iter()
        .enumerate()
        .filter(|(_, point)| point.volatility_index.is_finite())
        .map(|(index, point)| format!("{:.1},{:.1}", x_for(index), y_for(point.volatility_index)))
        .collect::<Vec<_>>()
        .join(" ");

    let band_width = (chart_width / denominator).max(2.0);
    let has_overall_line = overall_points.split_whitespace().nth(1).is_some();
    let area_points = format!(
        "{pad_left:.1},{:.1} {overall_points} {:.1},{:.1}",
        pad_top + chart_height,
        width - pad_right,
        pad_top + chart_height,
    );
    let summary = summarize_timeline(timeline);
    let peak_marker = summary.and_then(|summary| {
        timeline
            .iter()
            .position(|point| point.year == summary.peak_year)
            .map(|index| (x_for(index), y_for(summary.peak_score)))
    });
    let valley_marker = summary.and_then(|summary| {
        timeline
            .iter()
            .position(|point| point.year == summary.valley_year)
            .map(|index| (x_for(index), y_for(summary.valley_score)))
    });

    rsx! {
        svg {
            view_box: "0 0 {width} {height}",
            class: "w-full min-w-[900px] h-auto",
            xmlns: "http://www.w3.org/2000/svg",

            defs {
                linearGradient { id: "timeline-area-gradient", x1: "0", y1: "0", x2: "0", y2: "1",
                    stop { offset: "0%", style: "stop-color:#34d399;stop-opacity:0.20" }
                    stop { offset: "100%", style: "stop-color:#34d399;stop-opacity:0.01" }
                }
            }

            {(0..=4u32).map(|index| {
                let score = 100.0 - index as f64 * 25.0;
                let y = y_for(score);
                rsx! {
                    line {
                        x1: "{pad_left}",
                        y1: "{y:.1}",
                        x2: "{width - pad_right}",
                        y2: "{y:.1}",
                        stroke: "#3f3f46",
                        stroke_width: "1",
                        opacity: "0.55"
                    }
                    text {
                        x: "{pad_left - 10.0}",
                        y: "{y + 4.0:.1}",
                        text_anchor: "end",
                        font_size: "12",
                        fill: "#71717a",
                        "{score:.0}"
                    }
                }
            })}

            {timeline.iter().enumerate().filter(|(_, point)| point.is_transition_period).map(|(index, _)| {
                let x = x_for(index) - band_width / 2.0;
                rsx! {
                    rect {
                        x: "{x:.1}",
                        y: "{pad_top}",
                        width: "{band_width:.1}",
                        height: "{chart_height}",
                        fill: "#f43f5e",
                        opacity: "0.12"
                    }
                }
            })}

            if has_overall_line {
                polygon { points: "{area_points}", fill: "url(#timeline-area-gradient)" }
            }

            polyline {
                points: "{overall_points}",
                fill: "none",
                stroke: "#34d399",
                stroke_width: "3",
                stroke_linejoin: "round",
                stroke_linecap: "round"
            }
            polyline {
                points: "{trend_points}",
                fill: "none",
                stroke: "#38bdf8",
                stroke_width: "2",
                stroke_linejoin: "round",
                stroke_linecap: "round"
            }
            polyline {
                points: "{volatility_points}",
                fill: "none",
                stroke: "#fbbf24",
                stroke_width: "2",
                stroke_linejoin: "round",
                stroke_linecap: "round",
                opacity: "0.9"
            }

            if let Some((peak_x, peak_y)) = peak_marker {
                circle {
                    cx: "{peak_x:.1}",
                    cy: "{peak_y:.1}",
                    r: "5",
                    fill: "#facc15",
                    stroke: "#fef08a",
                    stroke_width: "2"
                }
            }
            if let Some((valley_x, valley_y)) = valley_marker {
                circle {
                    cx: "{valley_x:.1}",
                    cy: "{valley_y:.1}",
                    r: "5",
                    fill: "#fb7185",
                    stroke: "#fecdd3",
                    stroke_width: "2"
                }
            }

            {(0..=10u32).map(|tick| {
                let index = (tick as usize * (timeline.len() - 1)) / 10;
                let point = &timeline[index];
                let x = x_for(index);
                let age_label = format_age(locale, point.age as i32);
                rsx! {
                    line {
                        x1: "{x:.1}",
                        y1: "{pad_top + chart_height}",
                        x2: "{x:.1}",
                        y2: "{pad_top + chart_height + 6.0}",
                        stroke: "#71717a",
                        stroke_width: "1"
                    }
                    text {
                        x: "{x:.1}",
                        y: "{height - 14.0}",
                        text_anchor: "middle",
                        font_size: "11",
                        fill: "#71717a",
                        "{age_label}"
                    }
                    text {
                        x: "{x:.1}",
                        y: "{height - 2.0}",
                        text_anchor: "middle",
                        font_size: "9",
                        fill: "#52525b",
                        "{point.year}"
                    }
                }
            })}
        }
    }
}

fn summarize_timeline(timeline: &[YearlyScore]) -> Option<TimelineSummary> {
    let mut points = timeline
        .iter()
        .filter(|point| point.total_score.is_finite());
    let first = points.next()?;
    let mut peak = first;
    let mut valley = first;
    let mut total = first.total_score;
    let mut count = 1usize;

    for point in points {
        if point.total_score > peak.total_score {
            peak = point;
        }
        if point.total_score < valley.total_score {
            valley = point;
        }
        total += point.total_score;
        count += 1;
    }

    Some(TimelineSummary {
        peak_year: peak.year,
        peak_age: peak.age,
        peak_score: peak.total_score,
        valley_year: valley.year,
        valley_age: valley.age,
        valley_score: valley.total_score,
        average_score: total / count as f64,
        transition_years: timeline
            .iter()
            .filter(|point| point.is_transition_period)
            .count(),
    })
}

fn timeline_labels(locale: Locale) -> TimelineLabels {
    match locale {
        Locale::Ko => TimelineLabels {
            overall: "종합 점수",
            trend: "5년 이동평균",
            volatility: "변동성",
            transition: "교운기",
            peak: "최고점",
            valley: "최저점",
            average: "평균 점수",
            empty_title: "표시할 타임라인이 없습니다",
            empty_body: "먼저 사주 분석을 실행하세요. 생애 시뮬레이션이 성공하면 100년 점수와 교운기 변동을 이 화면에서 확인할 수 있습니다.",
            disclaimer: "이 차트는 Eon의 규칙 기반 시뮬레이션 결과를 시각화한 것으로, 과학적으로 검증된 예측이나 결정론적 진단이 아닙니다.",
        },
        Locale::En => TimelineLabels {
            overall: "Overall score",
            trend: "5-year moving average",
            volatility: "Volatility",
            transition: "Transition period",
            peak: "Peak",
            valley: "Valley",
            average: "Average score",
            empty_title: "No timeline data yet",
            empty_body: "Run the Saju analysis first. When the life-path simulation succeeds, the 100-year score and transition-period volatility will appear here.",
            disclaimer: "This chart visualizes Eon's rule-based simulation output. It is not a scientifically validated prediction or deterministic diagnosis.",
        },
        Locale::Zh => TimelineLabels {
            overall: "综合分数",
            trend: "5年移动平均",
            volatility: "波动率",
            transition: "交运期",
            peak: "最高点",
            valley: "最低点",
            average: "平均分",
            empty_title: "暂无时间线数据",
            empty_body: "请先运行四柱分析。生涯模拟成功后，此处会显示百年分数与交运期波动。",
            disclaimer: "此图仅可视化 Eon 的规则模拟结果，不属于经过科学验证的预测或决定性诊断。",
        },
        Locale::Ru => TimelineLabels {
            overall: "Общая оценка",
            trend: "Скользящее среднее за 5 лет",
            volatility: "Волатильность",
            transition: "Переходный период",
            peak: "Пик",
            valley: "Минимум",
            average: "Средняя оценка",
            empty_title: "Данных для графика пока нет",
            empty_body: "Сначала запустите анализ Саджу. После успешной симуляции здесь появятся столетние оценки и переходные периоды.",
            disclaimer: "График визуализирует результат правиловой симуляции Eon и не является научно подтвержденным прогнозом или детерминированной диагностикой.",
        },
    }
}
