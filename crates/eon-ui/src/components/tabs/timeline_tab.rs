use crate::i18n::{format_age, t, Locale, TK};
use crate::store::AnalysisState;
use dioxus::prelude::*;
use eon_saju::engine::emulator::YearlyScore;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;

#[derive(Clone, Copy)]
struct TimelineLabels {
    title: &'static str,
    year: &'static str,
    score: &'static str,
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
    let has_timeline = !timeline.is_empty();

    let state_for_chart = state.clone();
    use_effect(move || {
        let locale = *state_for_chart.locale.read();
        let timeline = state_for_chart
            .saju
            .read()
            .data
            .as_ref()
            .map(|output| output.report.timeline.clone())
            .unwrap_or_default();

        if timeline.is_empty() {
            return;
        }

        let Some(window) = web_sys::window() else {
            log::warn!("timeline chart skipped: browser window is unavailable");
            return;
        };
        let Some(document) = window.document() else {
            log::warn!("timeline chart skipped: document is unavailable");
            return;
        };
        let Some(canvas_element) = document.get_element_by_id("timeline-canvas") else {
            log::warn!("timeline chart skipped: canvas element is unavailable");
            return;
        };

        use wasm_bindgen::JsCast;
        let Ok(canvas) = canvas_element.dyn_into::<HtmlCanvasElement>() else {
            log::error!("timeline chart skipped: element is not a canvas");
            return;
        };

        if let Err(error) = draw_timeline(&canvas, &timeline, locale) {
            log::error!("failed to draw timeline chart: {error}");
        }
    });

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

            if has_timeline {
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
                    div { class: "w-full h-[520px] md:h-[620px] overflow-hidden rounded-xl border border-zinc-800 bg-zinc-800",
                        canvas {
                            id: "timeline-canvas",
                            width: "1400",
                            height: "680",
                            class: "w-full h-full"
                        }
                    }
                    p { class: "mt-3 text-xs leading-relaxed text-zinc-500", "{labels.disclaimer}" }
                }
            } else {
                div { class: "min-h-[420px] flex flex-col items-center justify-center gap-3 rounded-2xl border border-dashed border-zinc-700 bg-zinc-900/40 px-6 text-center",
                    span { class: "text-5xl", "📈" }
                    h3 { class: "text-lg font-semibold text-zinc-200", "{labels.empty_title}" }
                    p { class: "max-w-xl text-sm leading-relaxed text-zinc-500", "{labels.empty_body}" }
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

fn draw_timeline(
    canvas: &HtmlCanvasElement,
    timeline: &[YearlyScore],
    locale: Locale,
) -> Result<(), String> {
    let Some(first) = timeline.first() else {
        return Ok(());
    };
    let Some(last) = timeline.last() else {
        return Ok(());
    };

    let labels = timeline_labels(locale);
    let x_start = first.year;
    let x_end = last.year.saturating_add(1).max(x_start.saturating_add(1));
    let (y_start, y_end) = score_bounds(timeline);

    let backend = CanvasBackend::with_canvas_object(canvas.clone())
        .ok_or_else(|| "cannot attach Plotters backend to canvas".to_string())?;
    let root = backend.into_drawing_area();
    root.fill(&RGBColor(39, 39, 42))
        .map_err(|error| format!("background draw failed: {error:?}"))?;

    let mut chart = ChartBuilder::on(&root)
        .caption(labels.title, ("sans-serif", 28).into_font().color(&WHITE))
        .margin(24)
        .x_label_area_size(48)
        .y_label_area_size(58)
        .build_cartesian_2d(x_start..x_end, y_start..y_end)
        .map_err(|error| format!("chart construction failed: {error:?}"))?;

    chart
        .configure_mesh()
        .x_desc(labels.year)
        .y_desc(labels.score)
        .x_labels(11)
        .y_labels(8)
        .light_line_style(RGBColor(63, 63, 70).mix(0.35))
        .bold_line_style(RGBColor(82, 82, 91).mix(0.55))
        .axis_style(RGBColor(113, 113, 122))
        .label_style(
            ("sans-serif", 14)
                .into_font()
                .color(&RGBColor(212, 212, 216)),
        )
        .axis_desc_style(
            ("sans-serif", 15)
                .into_font()
                .color(&RGBColor(161, 161, 170)),
        )
        .draw()
        .map_err(|error| format!("mesh draw failed: {error:?}"))?;

    chart
        .draw_series(
            timeline
                .iter()
                .filter(|point| point.is_transition_period)
                .map(|point| {
                    Rectangle::new(
                        [(point.year, y_start), (point.year.saturating_add(1), y_end)],
                        RGBColor(244, 63, 94).mix(0.12).filled(),
                    )
                }),
        )
        .map_err(|error| format!("transition band draw failed: {error:?}"))?;

    chart
        .draw_series(LineSeries::new(
            timeline
                .iter()
                .filter(|point| point.total_score.is_finite())
                .map(|point| (point.year, point.total_score)),
            RGBColor(52, 211, 153).stroke_width(3),
        ))
        .map_err(|error| format!("overall score draw failed: {error:?}"))?
        .label(labels.overall)
        .legend(|(x, y)| {
            PathElement::new(
                vec![(x, y), (x + 22, y)],
                RGBColor(52, 211, 153).stroke_width(3),
            )
        });

    chart
        .draw_series(LineSeries::new(
            timeline.iter().filter_map(|point| {
                point
                    .trend_ma_5yr
                    .filter(|score| score.is_finite())
                    .map(|score| (point.year, score))
            }),
            RGBColor(56, 189, 248).stroke_width(2),
        ))
        .map_err(|error| format!("moving average draw failed: {error:?}"))?
        .label(labels.trend)
        .legend(|(x, y)| {
            PathElement::new(
                vec![(x, y), (x + 22, y)],
                RGBColor(56, 189, 248).stroke_width(2),
            )
        });

    chart
        .draw_series(LineSeries::new(
            timeline
                .iter()
                .filter(|point| point.volatility_index.is_finite())
                .map(|point| (point.year, point.volatility_index)),
            RGBColor(251, 191, 36).stroke_width(2),
        ))
        .map_err(|error| format!("volatility draw failed: {error:?}"))?
        .label(labels.volatility)
        .legend(|(x, y)| {
            PathElement::new(
                vec![(x, y), (x + 22, y)],
                RGBColor(251, 191, 36).stroke_width(2),
            )
        });

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .background_style(RGBColor(24, 24, 27).mix(0.88))
        .border_style(RGBColor(82, 82, 91))
        .label_font(("sans-serif", 14).into_font().color(&WHITE))
        .draw()
        .map_err(|error| format!("legend draw failed: {error:?}"))?;

    root.present()
        .map_err(|error| format!("canvas presentation failed: {error:?}"))
}

fn score_bounds(timeline: &[YearlyScore]) -> (f64, f64) {
    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;

    for point in timeline {
        for value in [point.total_score, point.volatility_index] {
            if value.is_finite() {
                minimum = minimum.min(value);
                maximum = maximum.max(value);
            }
        }

        if let Some(value) = point.trend_ma_5yr.filter(|value| value.is_finite()) {
            minimum = minimum.min(value);
            maximum = maximum.max(value);
        }
    }

    if !minimum.is_finite() || !maximum.is_finite() {
        return (0.0, 100.0);
    }

    let padding = ((maximum - minimum) * 0.1).max(5.0);
    let lower = (minimum - padding).floor().min(0.0);
    let upper = (maximum + padding).ceil().max(100.0);

    if upper <= lower {
        (lower, lower + 1.0)
    } else {
        (lower, upper)
    }
}

fn summarize_timeline(timeline: &[YearlyScore]) -> Option<TimelineSummary> {
    let valid: Vec<&YearlyScore> = timeline
        .iter()
        .filter(|point| point.total_score.is_finite())
        .collect();
    let peak = valid.iter().copied().max_by(|left, right| {
        left.total_score
            .partial_cmp(&right.total_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    })?;
    let valley = valid.iter().copied().min_by(|left, right| {
        left.total_score
            .partial_cmp(&right.total_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    })?;
    let average_score =
        valid.iter().map(|point| point.total_score).sum::<f64>() / valid.len() as f64;

    Some(TimelineSummary {
        peak_year: peak.year,
        peak_age: peak.age,
        peak_score: peak.total_score,
        valley_year: valley.year,
        valley_age: valley.age,
        valley_score: valley.total_score,
        average_score,
        transition_years: timeline
            .iter()
            .filter(|point| point.is_transition_period)
            .count(),
    })
}

fn timeline_labels(locale: Locale) -> TimelineLabels {
    match locale {
        Locale::Ko => TimelineLabels {
            title: "100년 운명 타임라인",
            year: "연도",
            score: "점수",
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
            title: "100-Year Destiny Timeline",
            year: "Year",
            score: "Score",
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
            title: "百年命运时间线",
            year: "年份",
            score: "分数",
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
            title: "Столетняя линия судьбы",
            year: "Год",
            score: "Оценка",
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
