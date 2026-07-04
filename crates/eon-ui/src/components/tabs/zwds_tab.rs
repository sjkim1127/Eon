//! 자미두수 탭 컴포넌트
//!
//! 12궁 성반 격자 시각화(4x4 Grid 테두리 배치 + 중앙 정보 영역),
//! 대한(대운) 목록 및 상세 정보, 마크다운 내보내기 버튼을 지원합니다.

use crate::components::shared::birth_form::BirthForm;
use crate::components::shared::export_markdown::export_zwds_to_markdown;
use crate::i18n::{
    format_age_range, t, translate_five_elements, translate_zwds_palace, translate_zwds_star,
    Locale, TK,
};
use crate::store::{AnalysisState, TaskStatus};
use dioxus::prelude::*;
use eon_service::dto::ZwdsAnalysisInput;
use eon_service::facade;
use eon_zwds::types::{PalaceData, SiHuaType, ZwdsStar};

#[component]
pub fn ZwdsTab() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();
    let target_year = use_signal(|| 2026i32);
    let selected_palace_idx = use_signal(|| None::<usize>);
    let hovered_palace_idx = use_signal(|| None::<usize>);

    // Copy signals to avoid borrowing state mutably in closures
    let zwds_signal = state.zwds;
    let form_signal = state.form;
    let target_year_sig = target_year;

    let run_analysis_with_year = move |year: i32| {
        spawn(async move {
            let mut zwds_signal = zwds_signal;
            let form_signal = form_signal;
            zwds_signal.write().status = TaskStatus::Loading;
            let form = form_signal.read().clone();
            let input = ZwdsAnalysisInput::new(form.to_analysis_input(), form.is_male, Some(year));
            match facade::analyze_zwds(input) {
                Ok(res) => {
                    zwds_signal.write().data = Some(res);
                    zwds_signal.write().status = TaskStatus::Success;
                }
                Err(e) => {
                    zwds_signal.write().error = Some(e.to_string());
                    zwds_signal.write().status = TaskStatus::Error(e.to_string());
                }
            }
        });
    };

    let update_year = move |new_year: i32| {
        let mut target_year_sig = target_year_sig;
        target_year_sig.set(new_year);
        let is_success = matches!(zwds_signal.read().status, TaskStatus::Success);
        if is_success {
            run_analysis_with_year(new_year);
        }
    };

    // Auto-run analysis when form or target_year changes
    use_effect(move || {
        let form = state.form.read().clone();
        let year = *target_year.read();
        if form.year > 0 {
            run_analysis_with_year(year);
        }
    });

    let zwds_status = state.zwds.read().status.clone();
    let zwds_data = state.zwds.read().data.clone();

    rsx! {
        div { class: "space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-700",
            // 1. 공통 출생 정보 입력 폼
            BirthForm {}

            // 2. 타이틀 및 분석 버튼
            div { class: "flex flex-col md:flex-row md:items-center justify-between gap-4 p-6 bg-slate-900/40 border border-slate-800/60 rounded-3xl backdrop-blur-md",
                div { class: "space-y-1.5",
                    h2 { class: "text-2xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-violet-400 to-indigo-300",
                        "{t(locale, TK::ZwdsReportTitle)}"
                    }
                    p { class: "text-sm text-slate-400 font-medium",
                        match locale {
                            Locale::Ko => "자미두수 성반을 분석하고 대한 및 유년 운세를 도출합니다.",
                            Locale::Zh => "分析紫微斗数命盘，推算大限及流年运势。",
                            Locale::En => "Analyze ZWDS chart, deduce Da-Xian and Liu-Nian fortunes.",
                            Locale::Ru => "Анализ карты Цзы Вэй Доу Шу, расчет периодов Да-Сянь и Лю-Нянь.",
                        }
                    }
                }

                // 컨트롤바: 대상 연도 조절 및 새로고침 버튼
                div { class: "flex items-center gap-3",
                    div { class: "flex items-center bg-slate-950/80 border border-slate-800/80 px-3 py-1.5 rounded-xl gap-2",
                        span { class: "text-xs font-bold text-slate-400 uppercase tracking-wider",
                            match locale {
                                Locale::Ko => "대상 연도",
                                Locale::Zh => "目标年份",
                                Locale::En => "Target Year",
                                Locale::Ru => "Целевой год",
                            }
                        }
                        button {
                            class: "text-slate-400 hover:text-white transition-colors cursor-pointer text-xs font-bold px-1.5 py-0.5 hover:bg-slate-800 rounded-md",
                            onclick: move |_| update_year(*target_year.read() - 1),
                            "◀"
                        }
                        span { class: "text-sm font-bold text-violet-400 font-mono min-w-[3.5rem] text-center",
                            "{target_year}년"
                        }
                        button {
                            class: "text-slate-400 hover:text-white transition-colors cursor-pointer text-xs font-bold px-1.5 py-0.5 hover:bg-slate-800 rounded-md",
                            onclick: move |_| update_year(*target_year.read() + 1),
                            "▶"
                        }
                    }

                    button {
                        class: "p-2.5 bg-slate-800 hover:bg-slate-700 active:bg-slate-600 border border-slate-700/50 rounded-xl text-slate-300 hover:text-white transition-all cursor-pointer flex items-center justify-center active:scale-95",
                        onclick: move |_| run_analysis_with_year(*target_year.read()),
                        title: "{t(locale, TK::ZwdsAnalyzeBtn)}",
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

            // 3. 분석 결과 영역
            {
                match &zwds_status {
                    TaskStatus::Idle => rsx! {
                        div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500 bg-slate-900/20 border border-slate-800/40 rounded-3xl",
                            span { class: "text-5xl animate-bounce", "🔮" }
                            p { class: "text-lg font-medium", "{t(locale, TK::ZwdsIdleHint)}" }
                        }
                    },
                    TaskStatus::Loading => rsx! {
                        div { class: "flex flex-col items-center justify-center py-20 gap-3 bg-slate-900/20 border border-slate-800/40 rounded-3xl",
                            div { class: "w-12 h-12 rounded-full border-4 border-violet-500/30 border-t-violet-400 animate-spin" }
                            p { class: "text-violet-400 font-medium animate-pulse", "{t(locale, TK::ZwdsLoadingHint)}" }
                        }
                    },
                    TaskStatus::Error(e) => rsx! {
                        div { class: "p-4 rounded-xl bg-red-900/20 border border-red-800/50 text-red-400",
                            "{t(locale, TK::StatusError)}: {e}"
                        }
                    },
                    TaskStatus::Success => {
                        if let Some(res) = &zwds_data {
                            rsx! {
                                div { class: "space-y-6",
                                    // 4x4 성반 격자 시각화
                                    ZwdsGrid { data: res.clone(), selected_palace_idx, hovered_palace_idx }

                                    // 10년 대한 주기 목록
                                    DaXianSection { data: res.clone() }
                                }
                            }
                        } else {
                            rsx! { div { "{t(locale, TK::ZwdsLoadFailed)}" } }
                        }
                    }
                }
            }

            // 4. 궁위 클릭 상세 모달
            {
                if let Some(p_idx) = *selected_palace_idx.read() {
                    if let Some(ref res) = zwds_data {
                        rsx! {
                            PalaceDetailModal {
                                palace_idx: p_idx,
                                data: res.clone(),
                                selected_palace_idx,
                            }
                        }
                    } else {
                        rsx! {}
                    }
                } else {
                    rsx! {}
                }
            }
        }
    }
}

/// 자미두수 4x4 격자 성반 렌더링 컴포넌트
#[component]
fn ZwdsGrid(
    data: eon_service::dto::ZwdsAnalysisOutput,
    selected_palace_idx: Signal<Option<usize>>,
    hovered_palace_idx: Signal<Option<usize>>,
) -> Element {
    // 4x4 Grid의 16칸 셀 배치 (외곽 12칸 궁 매핑, 중앙 4칸 비움)
    // ZWDS 지지 인덱스: 0=寅, 1=卯, 2=辰, 3=巳, 4=午, 5=未, 6=申, 7=酉, 8=戌, 9=亥, 10=子, 11=丑
    let grid_cells = vec![
        // Row 1: 巳(3) 午(4) 未(5) 申(6)
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        // Row 2: 辰(2) [중앙] [중앙] 酉(7)
        Some(2),
        None,
        None,
        Some(7),
        // Row 3: 卯(1) [중앙] [중앙] 戌(8)
        Some(1),
        None,
        None,
        Some(8),
        // Row 4: 寅(0) 丑(11) 子(10) 亥(9)
        Some(0),
        Some(11),
        Some(10),
        Some(9),
    ];

    let chart = &data.chart;

    rsx! {
        div { class: "grid grid-cols-4 gap-3 bg-slate-950 p-4 rounded-3xl border border-slate-800/50 shadow-2xl relative overflow-hidden",
            {
                grid_cells.into_iter().enumerate().map(|(idx, cell)| {
                    match cell {
                        Some(p_idx) => {
                            let palace = &chart.palaces[p_idx];
                            rsx! {
                                PalaceCard {
                                    key: "{p_idx}",
                                    palace: palace.clone(),
                                    is_soul: p_idx == chart.soul_idx,
                                    is_body: p_idx == chart.body_idx,
                                    current_daxian: Some(data.current_daxian.clone()),
                                    current_liu_nian: Some(data.current_liu_nian.clone()),
                                    onclick: move |_| {
                                        selected_palace_idx.set(Some(p_idx));
                                    },
                                    onmouseenter: move |_| {
                                        hovered_palace_idx.set(Some(p_idx));
                                    },
                                    onmouseleave: move |_| {
                                        hovered_palace_idx.set(None);
                                    }
                                }
                            }
                        },
                        None => {
                            // Row 2, Col 2 위치(인덱스 5)에서만 중앙 카드 렌더링
                            if idx == 5 {
                                rsx! {
                                    CenterCard {
                                        key: "center-card",
                                        data: data.clone(),
                                    }
                                }
                            } else {
                                rsx! {}
                            }
                        }
                    }
                })
            }

            // SVG flying stars layer
            if let Some(from_idx) = *hovered_palace_idx.read() {
                {
                    let chart = &data.chart;
                    let hovered_p_name = chart.palaces[from_idx].name;
                    let state = use_context::<AnalysisState>();
                    let locale = *state.locale.read();

                    let flying_lines = chart.flying_sihua.iter()
                        .filter(|fs| fs.from_palace == hovered_p_name)
                        .filter_map(|fs| {
                            let to_idx = chart.palaces.iter().position(|p| p.name == fs.to_palace)?;
                            Some((fs, to_idx))
                        })
                        .collect::<Vec<_>>();

                    if !flying_lines.is_empty() {
                        rsx! {
                            svg {
                                class: "absolute inset-0 w-full h-full pointer-events-none z-10",
                                view_box: "0 0 100 100",
                                defs {
                                    marker {
                                        id: "arrow-lu",
                                        view_box: "0 0 10 10",
                                        ref_x: "8",
                                        ref_y: "5",
                                        marker_width: "6",
                                        marker_height: "6",
                                        orient: "auto-start-reverse",
                                        path { d: "M 0 2 L 8 5 L 0 8 z", fill: "#10b981" }
                                    }
                                    marker {
                                        id: "arrow-quan",
                                        view_box: "0 0 10 10",
                                        ref_x: "8",
                                        ref_y: "5",
                                        marker_width: "6",
                                        marker_height: "6",
                                        orient: "auto-start-reverse",
                                        path { d: "M 0 2 L 8 5 L 0 8 z", fill: "#3b82f6" }
                                    }
                                    marker {
                                        id: "arrow-ke",
                                        view_box: "0 0 10 10",
                                        ref_x: "8",
                                        ref_y: "5",
                                        marker_width: "6",
                                        marker_height: "6",
                                        orient: "auto-start-reverse",
                                        path { d: "M 0 2 L 8 5 L 0 8 z", fill: "#8b5cf6" }
                                    }
                                    marker {
                                        id: "arrow-ji",
                                        view_box: "0 0 10 10",
                                        ref_x: "8",
                                        ref_y: "5",
                                        marker_width: "6",
                                        marker_height: "6",
                                        orient: "auto-start-reverse",
                                        path { d: "M 0 2 L 8 5 L 0 8 z", fill: "#ef4444" }
                                    }
                                }
                                {
                                    flying_lines.into_iter().map(|(fs, to_idx)| {
                                        let (x1, y1) = get_palace_center(from_idx);
                                        let (x2, y2) = get_palace_center(to_idx);

                                        let (color, marker_id) = match fs.sihua_type {
                                            SiHuaType::HuaLu => ("#10b981", "arrow-lu"),
                                            SiHuaType::HuaQuan => ("#3b82f6", "arrow-quan"),
                                            SiHuaType::HuaKe => ("#8b5cf6", "arrow-ke"),
                                            SiHuaType::HuaJi => ("#ef4444", "arrow-ji"),
                                        };

                                        let star_label = translate_zwds_star(locale, fs.star);
                                        let sihua_emoji = fs.sihua_type.emoji();

                                        if from_idx == to_idx {
                                            let path_d = format!(
                                                "M {} {} C {} {}, {} {}, {} {}",
                                                x1 - 2.0, y1 - 2.0,
                                                x1 - 5.0, y1 - 8.0,
                                                x1 + 5.0, y1 - 8.0,
                                                x1 + 2.0, y1 - 2.0
                                            );
                                            let mx = x1;
                                            let my = y1 - 9.0;
                                            let label_len = star_label.chars().count();
                                            let rect_w = 4.0 + (label_len as f64) * 1.5;
                                            let rect_h = 3.6;

                                            rsx! {
                                                g { key: "self-{fs.star.korean()}-{sihua_emoji}",
                                                    path {
                                                        d: "{path_d}",
                                                        fill: "none",
                                                        stroke: "{color}",
                                                        stroke_width: "1.2",
                                                        marker_end: "url(#{marker_id})"
                                                    }
                                                    rect {
                                                        x: "{mx - rect_w / 2.0}",
                                                        y: "{my - rect_h / 2.0}",
                                                        width: "{rect_w}",
                                                        height: "{rect_h}",
                                                        rx: "1.0",
                                                        fill: "#0b0f19",
                                                        stroke: "{color}",
                                                        stroke_width: "0.4"
                                                    }
                                                    text {
                                                        x: "{mx}",
                                                        y: "{my + 1.0}",
                                                        fill: "#cbd5e1",
                                                        font_size: "2.0",
                                                        font_weight: "bold",
                                                        text_anchor: "middle",
                                                        "{star_label}{sihua_emoji}"
                                                    }
                                                }
                                            }
                                        } else {
                                            let dx = x2 - x1;
                                            let dy = y2 - y1;
                                            let dist = (dx*dx + dy*dy).sqrt();
                                            let (x2_short, y2_short) = if dist > 0.0 {
                                                let offset = 8.5;
                                                if dist > offset {
                                                    (x2 - dx * (offset / dist), y2 - dy * (offset / dist))
                                                } else {
                                                    (x2, y2)
                                                }
                                            } else {
                                                (x2, y2)
                                            };

                                            let mx = (x1 + x2) / 2.0;
                                            let my = (y1 + y2) / 2.0;
                                            let label_len = star_label.chars().count();
                                            let rect_w = 4.0 + (label_len as f64) * 1.5;
                                            let rect_h = 3.6;

                                            rsx! {
                                                g { key: "line-{from_idx}-{to_idx}-{fs.star.korean()}",
                                                    line {
                                                        x1: "{x1}",
                                                        y1: "{y1}",
                                                        x2: "{x2_short}",
                                                        y2: "{y2_short}",
                                                        stroke: "{color}",
                                                        stroke_width: "1.2",
                                                        stroke_dasharray: "3 1.5",
                                                        marker_end: "url(#{marker_id})"
                                                    }
                                                    rect {
                                                        x: "{mx - rect_w / 2.0}",
                                                        y: "{my - rect_h / 2.0}",
                                                        width: "{rect_w}",
                                                        height: "{rect_h}",
                                                        rx: "1.0",
                                                        fill: "#0b0f19",
                                                        stroke: "{color}",
                                                        stroke_width: "0.4"
                                                    }
                                                    text {
                                                        x: "{mx}",
                                                        y: "{my + 1.0}",
                                                        fill: "#cbd5e1",
                                                        font_size: "2.0",
                                                        font_weight: "bold",
                                                        text_anchor: "middle",
                                                        "{star_label}{sihua_emoji}"
                                                    }
                                                }
                                            }
                                        }
                                    })
                                }
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
            }
        }
    }
}

/// 개별 궁위 카드 컴포넌트
#[component]
fn PalaceCard(
    palace: PalaceData,
    is_soul: bool,
    is_body: bool,
    current_daxian: Option<eon_zwds::types::DaXian>,
    current_liu_nian: Option<eon_zwds::types::LiuNian>,
    onclick: EventHandler<MouseEvent>,
    onmouseenter: EventHandler<MouseEvent>,
    onmouseleave: EventHandler<MouseEvent>,
) -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    // 대운 동적 궁위명 구하기
    let daxian_palace_name = if let Some(ref dx) = current_daxian {
        let name = eon_zwds::palace::get_palace_name(dx.palace_idx, palace.index);
        let prefix = match locale {
            Locale::Ko => "대",
            Locale::Zh => "大",
            Locale::En => "D-",
            Locale::Ru => "Д-",
        };
        let abbr = crate::i18n::translate_zwds_palace_abbr(locale, name);
        Some(format!("{}{}", prefix, abbr))
    } else {
        None
    };

    // 유년 동적 궁위명 구하기
    let liunian_palace_name = if let Some(ref ln) = current_liu_nian {
        let name = eon_zwds::palace::get_palace_name(ln.palace_idx, palace.index);
        let prefix = match locale {
            Locale::Ko => "유",
            Locale::Zh => "流",
            Locale::En => "A-",
            Locale::Ru => "Г-",
        };
        let abbr = crate::i18n::translate_zwds_palace_abbr(locale, name);
        Some(format!("{}{}", prefix, abbr))
    } else {
        None
    };

    // 테두리 및 배경 디자인
    let border_cls = if palace.is_current_liu_nian {
        "border-amber-500/80 shadow-lg shadow-amber-900/10 bg-amber-950/10"
    } else if is_soul {
        "border-violet-500/70 shadow-lg shadow-violet-900/10 bg-violet-950/10"
    } else {
        "border-slate-800/70 bg-slate-900/40 hover:border-slate-700/60"
    };

    let mut annual_stars = Vec::new();
    if let Some(ref ln) = current_liu_nian {
        if palace.index == ln.liu_lu {
            annual_stars.push(ZwdsStar::LuCun);
        }
        if palace.index == ln.liu_yang {
            annual_stars.push(ZwdsStar::QingYang);
        }
        if palace.index == ln.liu_tuo {
            annual_stars.push(ZwdsStar::TuoLuo);
        }
        if palace.index == ln.liu_chang {
            annual_stars.push(ZwdsStar::WenChang);
        }
        if palace.index == ln.liu_qu {
            annual_stars.push(ZwdsStar::WenQu);
        }
    }

    rsx! {
        div {
            class: "h-44 p-3 rounded-2xl border flex flex-col justify-between transition-all duration-200 cursor-pointer hover:scale-[1.02] hover:shadow-xl hover:shadow-violet-950/10 active:scale-98 {border_cls}",
            onclick: move |e| onclick.call(e),
            onmouseenter: move |e| onmouseenter.call(e),
            onmouseleave: move |e| onmouseleave.call(e),
            // 궁 헤더: 천간/지지 & 궁명
            div { class: "flex justify-between items-start",
                div { class: "flex flex-col",
                    span { class: "text-[10px] text-slate-500 font-bold",
                        "{palace.heavenly_stem}{palace.earthly_branch}"
                    }
                    span { class: "text-[10px] text-slate-400 font-bold",
                        "{palace.earthly_branch}{t(locale, TK::ZwdsPalaceSuffix)}"
                    }
                }
                div { class: "flex items-center gap-1",
                    if palace.is_current_liu_nian {
                        span { class: "px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-300 text-[8px] font-black tracking-wider border border-amber-500/30 animate-pulse",
                            "{t(locale, TK::ZwdsLiuNianBadge)}"
                        }
                    }
                    if is_soul {
                        span { class: "px-1.5 py-0.5 rounded bg-violet-500/20 text-violet-300 text-[8px] font-black tracking-wider border border-violet-500/30",
                            "命"
                        }
                    }
                    if is_body {
                        span { class: "px-1.5 py-0.5 rounded bg-indigo-500/20 text-indigo-300 text-[8px] font-black tracking-wider border border-indigo-500/30",
                            "身"
                        }
                    }
                    span { class: "text-xs font-black text-slate-200 bg-slate-800 px-2 py-0.5 rounded-lg border border-slate-700/50 shadow-inner",
                        "{translate_zwds_palace(locale, palace.name)}"
                    }
                }
            }

            // 별 리스트 (궁 카드 내부 배치)
            div { class: "flex-1 my-2 overflow-y-auto space-y-1.5 pr-0.5 scrollbar-thin",
                {
                    palace.stars.iter().map(|star_in_p| {
                        let is_main = star_in_p.star.is_main_star();
                        let star_color = if is_main {
                            "text-amber-200 font-black text-xs"
                        } else if matches!(star_in_p.star, ZwdsStar::WenChang | ZwdsStar::WenQu | ZwdsStar::ZuoFu | ZwdsStar::YouBi | ZwdsStar::TianKui | ZwdsStar::TianYue) {
                            "text-violet-300 font-bold text-[11px]"
                        } else {
                            "text-slate-400 text-[10px]"
                        };

                        let annual_sihua = if let Some(ref ln) = current_liu_nian {
                            if star_in_p.star == ln.si_hua[0] {
                                Some(SiHuaType::HuaLu)
                            } else if star_in_p.star == ln.si_hua[1] {
                                Some(SiHuaType::HuaQuan)
                            } else if star_in_p.star == ln.si_hua[2] {
                                Some(SiHuaType::HuaKe)
                            } else if star_in_p.star == ln.si_hua[3] {
                                Some(SiHuaType::HuaJi)
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        let brightness_suffix = if let Some(brightness) = star_in_p.brightness {
                            format!(" ({})", crate::i18n::translate_zwds_brightness(locale, brightness))
                        } else {
                            "".to_string()
                        };

                        rsx! {
                            div { key: "{star_in_p.star.korean()}", class: "flex items-center justify-between",
                                span { class: "{star_color}",
                                    "{translate_zwds_star(locale, star_in_p.star)}{brightness_suffix}"
                                }
                                div { class: "flex items-center gap-1",
                                    if let Some(sihua) = star_in_p.si_hua {
                                        {
                                            let bg = match sihua {
                                                SiHuaType::HuaLu => "bg-emerald-500/20 text-emerald-300 border-emerald-500/30",
                                                SiHuaType::HuaQuan => "bg-blue-500/20 text-blue-300 border-blue-500/30",
                                                SiHuaType::HuaKe => "bg-violet-500/20 text-violet-300 border-violet-500/30",
                                                SiHuaType::HuaJi => "bg-red-500/20 text-red-300 border-red-500/30",
                                            };
                                            rsx! {
                                                span { class: "px-1 rounded text-[8px] font-black border {bg}",
                                                    "{sihua.emoji()}"
                                                }
                                            }
                                        }
                                    }
                                    if let Some(ann_sihua) = annual_sihua {
                                        {
                                            let bg = match ann_sihua {
                                                SiHuaType::HuaLu => "bg-amber-500/20 text-amber-300 border-amber-500/30",
                                                SiHuaType::HuaQuan => "bg-amber-650/20 text-amber-400 border-amber-650/30",
                                                SiHuaType::HuaKe => "bg-yellow-500/20 text-yellow-300 border-yellow-500/30",
                                                SiHuaType::HuaJi => "bg-orange-600/20 text-orange-400 border-orange-600/30",
                                            };
                                            rsx! {
                                                span { class: "px-1 rounded text-[8px] font-black border {bg} animate-pulse",
                                                    "流{ann_sihua.emoji()}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    })
                }
                {
                    annual_stars.into_iter().map(|star| {
                        rsx! {
                            div { key: "{star.korean()}-annual", class: "flex items-center justify-between",
                                span { class: "text-amber-500 font-semibold text-[10px] animate-pulse",
                                    "流 {translate_zwds_star(locale, star)}"
                                }
                            }
                        }
                    })
                }
            }

            // 하단 바: 동적 궁위 & 대한 연령대 범위
            div { class: "flex justify-between items-center text-[9px] border-t border-slate-800/30 pt-1 font-mono",
                // 동적 궁위 배지
                div { class: "flex items-center gap-1.5",
                    if let Some(ref dx_name) = daxian_palace_name {
                        span { class: "px-1 py-0.2 rounded bg-violet-500/10 text-violet-300 font-bold border border-violet-500/10 text-[8px]",
                            "{dx_name}"
                        }
                    }
                    if let Some(ref ln_name) = liunian_palace_name {
                        span { class: "px-1 py-0.2 rounded bg-amber-500/10 text-amber-300 font-bold border border-amber-500/10 text-[8px]",
                            "{ln_name}"
                        }
                    }
                }
                // 연령 범위
                span { class: "text-slate-500",
                    {
                        palace.daxian_range
                            .map(|r| {
                                if locale == Locale::Ko { format!("{} - {}세", r.0, r.1) }
                                else if locale == Locale::Zh { format!("{} - {}岁", r.0, r.1) }
                                else if locale == Locale::Ru { format!("{} - {} лет", r.0, r.1) }
                                else { format!("{} - {} yrs", r.0, r.1) }
                            })
                            .unwrap_or_else(|| "—".to_string())
                    }
                }
            }
        }
    }
}

/// 성반 중앙 카드 컴포넌트 (종합 정보 및 마크다운 내보내기)
#[component]
fn CenterCard(data: eon_service::dto::ZwdsAnalysisOutput) -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();
    let mut copied = use_signal(|| false);

    let chart = &data.chart;
    let data_cloned = data.clone();

    let handle_copy = move |_| {
        let form = state.form.read().clone();
        let md = export_zwds_to_markdown(&data_cloned, &form, locale);
        crate::components::shared::export_markdown::copy_to_clipboard(&md);
        copied.set(true);
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(2000).await;
            copied.set(false);
        });
    };

    let current_daxian_formatted = if locale == Locale::Ko {
        format!(
            "{}{}{} ({}-{}세)",
            data.current_daxian.stem_hanja,
            data.current_daxian.branch_hanja,
            t(locale, TK::ZwdsDaxianSuffix),
            data.current_daxian.age_start,
            data.current_daxian.age_end
        )
    } else if locale == Locale::Zh {
        format!(
            "{}{}{} ({}-{}岁)",
            data.current_daxian.stem_hanja,
            data.current_daxian.branch_hanja,
            t(locale, TK::ZwdsDaxianSuffix),
            data.current_daxian.age_start,
            data.current_daxian.age_end
        )
    } else if locale == Locale::Ru {
        format!(
            "{}{}{} ({}-{} лет)",
            data.current_daxian.stem_hanja,
            data.current_daxian.branch_hanja,
            t(locale, TK::ZwdsDaxianSuffix),
            data.current_daxian.age_start,
            data.current_daxian.age_end
        )
    } else {
        format!(
            "{}{}{} (Age {}-{})",
            data.current_daxian.stem_hanja,
            data.current_daxian.branch_hanja,
            t(locale, TK::ZwdsDaxianSuffix),
            data.current_daxian.age_start,
            data.current_daxian.age_end
        )
    };

    rsx! {
        div { class: "col-span-2 row-span-2 bg-gradient-to-br from-slate-900 to-slate-950 border border-slate-800/80 rounded-3xl p-6 flex flex-col justify-between shadow-2xl relative overflow-hidden",
            // 은은한 그라데이션 장식 배경
            div { class: "absolute -right-10 -top-10 w-40 h-40 bg-violet-600/10 rounded-full blur-3xl" }
            div { class: "absolute -left-10 -bottom-10 w-40 h-40 bg-indigo-600/10 rounded-full blur-3xl" }

            div { class: "space-y-4 relative z-10",
                div { class: "border-b border-slate-800 pb-3",
                    h3 { class: "text-lg font-bold text-slate-100 flex items-center gap-2",
                        span { "🌌" }
                        "{t(locale, TK::ZwdsCenterCardTitle)}"
                    }
                }

                // 세부 속성 리스트
                div { class: "grid grid-cols-2 gap-4 text-xs",
                    div { class: "space-y-2.5",
                        div { class: "flex flex-col gap-0.5",
                            span { class: "text-slate-500 font-medium", "{t(locale, TK::ZwdsMasterSoul)}" }
                            span { class: "text-slate-200 font-bold text-sm", "{translate_zwds_star(locale, chart.soul_master)}" }
                        }
                        div { class: "flex flex-col gap-0.5",
                            span { class: "text-slate-500 font-medium", "{t(locale, TK::ZwdsMasterBody)}" }
                            span { class: "text-slate-200 font-bold text-sm", "{translate_zwds_star(locale, chart.body_master)}" }
                        }
                    }
                    div { class: "space-y-2.5",
                        div { class: "flex flex-col gap-0.5",
                            span { class: "text-slate-500 font-medium", "{t(locale, TK::ZwdsElementsBureau)}" }
                            span { class: "text-violet-300 font-bold text-sm", "{translate_five_elements(locale, chart.five_elements)}" }
                        }
                        div { class: "flex flex-col gap-0.5",
                            span { class: "text-slate-500 font-medium", "{t(locale, TK::ZwdsCurrentDaxian)}" }
                            span { class: "text-indigo-300 font-bold text-sm",
                                "{current_daxian_formatted}"
                            }
                        }
                    }
                }

                // 격국 리스트 (Destiny Patterns Badge List)
                if !chart.destiny_patterns.is_empty() {
                    div { class: "border-t border-slate-800/60 pt-3 space-y-2",
                        span { class: "text-[10px] text-slate-500 font-semibold tracking-wider uppercase block",
                            if locale == Locale::Ko { "감지된 격국" }
                            else if locale == Locale::Zh { "检测到的格局" }
                            else if locale == Locale::Ru { "Обнаруженные структуры" }
                            else { "Detected Patterns" }
                        }
                        div { class: "flex flex-wrap gap-1.5",
                            {
                                chart.destiny_patterns.iter().map(|pat| {
                                    let badge_cls = if pat.is_auspicious {
                                        "bg-emerald-500/10 text-emerald-300 border-emerald-500/20 hover:bg-emerald-500/20"
                                    } else {
                                        "bg-red-500/10 text-red-300 border-red-500/20 hover:bg-red-500/20"
                                    };
                                    let display_name = if locale == Locale::Zh {
                                        pat.name_hanja.clone()
                                    } else {
                                        format!("{} ({})", pat.name_korean, pat.name_hanja)
                                    };
                                    let tooltip_desc = if locale == Locale::Ko {
                                        pat.description_korean.clone()
                                    } else {
                                        pat.description_english.clone()
                                    };
                                    rsx! {
                                        span {
                                            key: "{pat.name_hanja}",
                                            class: "px-2 py-1 rounded-lg border text-[11px] font-black cursor-help transition-colors duration-150 {badge_cls}",
                                            title: "{tooltip_desc}",
                                            "{display_name}"
                                        }
                                    }
                                })
                            }
                        }
                    }
                }
            }

            // 마크다운 복사 버튼
            div { class: "mt-6 relative z-10",
                button {
                    class: "w-full py-2.5 bg-slate-850 hover:bg-slate-800 rounded-xl border border-slate-700/50 text-xs font-semibold text-slate-300 hover:text-white flex items-center justify-center gap-2 transition-all duration-200 cursor-pointer shadow-inner active:scale-95",
                    onclick: handle_copy,
                    if *copied.read() {
                        span { class: "text-emerald-400 font-bold animate-pulse", "{t(locale, TK::ZwdsCopySuccess)}" }
                    } else {
                        span { "📋" }
                        span { "{t(locale, TK::ZwdsCopyBtn)}" }
                    }
                }
            }
        }
    }
}

/// 10년 대한 주기 일람 컴포넌트
#[component]
fn DaXianSection(data: eon_service::dto::ZwdsAnalysisOutput) -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();
    let chart = &data.chart;

    rsx! {
        div { class: "bg-slate-900/30 border border-slate-800/40 rounded-3xl p-6 space-y-4 shadow-xl",
            h3 { class: "text-lg font-bold text-slate-200 flex items-center gap-2",
                span { "⏳" }
                "{t(locale, TK::ZwdsDaxianTitle)}"
            }

            div { class: "grid grid-cols-2 md:grid-cols-4 gap-3",
                {
                    chart.daxian.iter().map(|dx| {
                        let is_current = dx.palace_idx == data.current_daxian.palace_idx;
                        let card_cls = if is_current {
                            "border-violet-500/60 bg-violet-950/20 shadow-lg shadow-violet-950/20"
                        } else {
                            "border-slate-800 bg-slate-900/20 hover:border-slate-700"
                        };

                        let daxian_title = if locale == Locale::Ko {
                            format!("{} 대운", dx.index + 1)
                        } else if locale == Locale::Zh {
                            format!("第 {} 大限", dx.index + 1)
                        } else if locale == Locale::Ru {
                            format!("{} Да-Сянь", dx.index + 1)
                        } else {
                            format!("Cycle {}", dx.index + 1)
                        };

                        let range_str = format_age_range(locale, dx.age_start as i32, dx.age_end as i32);
                        let daxian_label = format!("{}{}{}", dx.stem_hanja, dx.branch_hanja, t(locale, TK::ZwdsDaxianSuffix));

                        let palace_label = if locale == Locale::Ko {
                            format!("{}번 궁", dx.palace_idx)
                        } else if locale == Locale::Zh {
                            format!("{}号宫", dx.palace_idx)
                        } else if locale == Locale::Ru {
                            format!("Дворец {}", dx.palace_idx)
                        } else {
                            format!("Palace {}", dx.palace_idx)
                        };

                        rsx! {
                            div {
                                key: "{dx.index}",
                                class: "p-4 rounded-xl border flex flex-col justify-between gap-1.5 transition-all duration-200 {card_cls}",
                                div { class: "flex justify-between items-center",
                                    span { class: "text-[10px] text-slate-500 font-bold uppercase tracking-wider",
                                        "{daxian_title}"
                                    }
                                    if is_current {
                                        span { class: "px-1.5 py-0.5 rounded bg-violet-500/20 text-violet-300 text-[8px] font-black border border-violet-500/30",
                                            "{t(locale, TK::ZwdsCurrentDaxianBadge)}"
                                        }
                                    }
                                }
                                span { class: "text-[11px] font-black text-slate-200",
                                    "{range_str}"
                                }
                                div { class: "flex justify-between items-center text-xs border-t border-slate-800/40 pt-1.5 mt-1",
                                    span { class: "text-slate-400 font-semibold",
                                        "{daxian_label}"
                                    }
                                    span { class: "text-slate-500 font-medium text-[10px]",
                                        "{palace_label}"
                                    }
                                }
                            }
                        }
                    })
                }
            }
        }
    }
}

/// 궁위 상세 정보 모달 컴포넌트
#[component]
fn PalaceDetailModal(
    palace_idx: usize,
    data: eon_service::dto::ZwdsAnalysisOutput,
    mut selected_palace_idx: Signal<Option<usize>>,
) -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    let chart = &data.chart;
    let palace = &chart.palaces[palace_idx];

    let opposite_idx = (palace_idx + 6) % 12;
    let triad1_idx = (palace_idx + 4) % 12;
    let triad2_idx = (palace_idx + 8) % 12;

    let opposite_palace = &chart.palaces[opposite_idx];
    let triad1_palace = &chart.palaces[triad1_idx];
    let triad2_palace = &chart.palaces[triad2_idx];

    let palace_desc = get_palace_description(locale, palace.name);

    let mini_grid_cells = vec![
        // Row 1: 巳(3) 午(4) 未(5) 申(6)
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        // Row 2: 辰(2) [중앙] [중앙] 酉(7)
        Some(2),
        None,
        None,
        Some(7),
        // Row 3: 卯(1) [중앙] [중앙] 戌(8)
        Some(1),
        None,
        None,
        Some(8),
        // Row 4: 寅(0) 丑(11) 子(10) 亥(9)
        Some(0),
        Some(11),
        Some(10),
        Some(9),
    ];

    rsx! {
        div {
            class: "fixed inset-0 bg-slate-950/80 backdrop-blur-sm z-50 flex items-center justify-center p-4 transition-all duration-300 animate-in fade-in",
            onclick: move |_| selected_palace_idx.set(None),

            div {
                class: "bg-slate-900 border border-slate-800/80 rounded-3xl max-w-3xl w-full max-h-[90vh] overflow-hidden flex flex-col shadow-2xl relative animate-in fade-in zoom-in-95 duration-200",
                onclick: |e| e.stop_propagation(),

                // 닫기 버튼
                button {
                    class: "absolute top-5 right-5 text-slate-400 hover:text-white transition-colors cursor-pointer text-xl w-8 h-8 flex items-center justify-center rounded-full hover:bg-slate-800/60",
                    onclick: move |_| selected_palace_idx.set(None),
                    "✕"
                }

                // 모달 헤더
                div { class: "p-6 border-b border-slate-800/50 flex flex-col gap-1 pr-12",
                    div { class: "flex items-center gap-2.5",
                        span { class: "text-2xl", "🌌" }
                        h3 { class: "text-xl font-black text-slate-100 flex items-baseline gap-2",
                            "{translate_zwds_palace(locale, palace.name)}"
                            span { class: "text-xs text-slate-500 font-mono",
                                "{palace.heavenly_stem}{palace.earthly_branch}궁 ({palace.earthly_branch}{t(locale, TK::ZwdsPalaceSuffix)})"
                            }
                        }
                    }
                    p { class: "text-xs text-slate-400 mt-2.5 leading-relaxed bg-slate-950/30 p-3.5 rounded-xl border border-slate-850",
                        "{palace_desc}"
                    }
                }

                // 모달 바디
                div { class: "p-6 overflow-y-auto space-y-6 flex-1 text-slate-300 text-sm",
                    div { class: "grid grid-cols-1 md:grid-cols-12 gap-6",
                        // 왼쪽 칼럼: 배치 성계 (7/12)
                        div { class: "md:col-span-7 space-y-5",
                            // 1. 궁위 심층 분석 리딩 (Advanced Reading)
                            {
                                let advanced_reading = crate::i18n::zwds_interpret::get_advanced_palace_interpretation(
                                    locale,
                                    palace.name,
                                    &palace.stars,
                                    &chart.destiny_patterns,
                                );
                                rsx! {
                                    div { class: "p-4.5 rounded-2xl bg-gradient-to-r from-violet-950/20 to-indigo-950/20 border border-violet-800/20 space-y-2.5 shadow-md",
                                        h4 { class: "text-xs font-black text-violet-300 uppercase tracking-wider flex items-center gap-2",
                                            span { "📖" }
                                            match locale {
                                                Locale::Ko => "궁위 심층 분석 리딩 (Life Reading)",
                                                Locale::Zh => "宫位深层分析命运导读",
                                                Locale::En => "Palace Deep Destiny Reading",
                                                Locale::Ru => "Глубокое толкование судьбы дворца",
                                            }
                                        }
                                        div { class: "space-y-2 text-slate-300 text-xs leading-relaxed font-sans",
                                            {
                                                advanced_reading.split("\n\n").enumerate().map(|(idx, p)| {
                                                    rsx! {
                                                        p { key: "{idx}",
                                                            "{p}"
                                                        }
                                                    }
                                                })
                                            }
                                        }
                                    }
                                }
                            }

                            // 2. 본궁 성계 목록
                            div { class: "space-y-3",
                                h4 { class: "text-xs font-black text-slate-400 uppercase tracking-wider border-l-2 border-violet-500 pl-2",
                                    match locale {
                                        Locale::Ko => "배치된 성계 (본궁)",
                                        Locale::Zh => "本宫星曜",
                                        Locale::En => "Stars in Palace",
                                        Locale::Ru => "Звезды во дворце",
                                    }
                                }
                                if palace.stars.is_empty() {
                                    div { class: "text-xs text-slate-500 italic p-4 bg-slate-950/20 rounded-xl border border-dashed border-slate-800 text-center",
                                        match locale {
                                            Locale::Ko => "배치된 주요 성계가 없습니다. 대궁(천이궁)의 영향을 강하게 받습니다.",
                                            Locale::Zh => "无主要星曜，受对宫（迁移）强烈影响",
                                            Locale::En => "No major stars. Heavily influenced by the opposite palace.",
                                            Locale::Ru => "Нет главных звезд. Сильно зависит от противоположного дворца.",
                                        }
                                    }
                                } else {
                                    div { class: "space-y-2.5",
                                        {
                                            palace.stars.iter().map(|star_in_p| {
                                                let is_main = star_in_p.star.is_main_star();
                                                let star_name = translate_zwds_star(locale, star_in_p.star);
                                                let brightness_label = star_in_p.brightness.map(|b| {
                                                    format!(" ({})", crate::i18n::translate_zwds_brightness(locale, b))
                                                }).unwrap_or_default();

                                                let star_desc = get_star_description(locale, star_in_p.star);
                                                let badge_cls = if is_main {
                                                    "bg-amber-500/10 text-amber-300 border-amber-500/20"
                                                } else {
                                                    "bg-slate-800/60 text-slate-400 border-slate-700/30"
                                                };

                                                rsx! {
                                                    div {
                                                        key: "{star_in_p.star.korean()}",
                                                        class: "p-3 rounded-xl border bg-slate-950/25 border-slate-850 flex flex-col gap-1.5 hover:border-slate-800/80 transition-colors",
                                                        div { class: "flex items-center justify-between",
                                                            span { class: "px-2 py-0.5 rounded-lg border text-xs font-black {badge_cls}",
                                                                "{star_name}{brightness_label}"
                                                            }
                                                            if let Some(sihua) = star_in_p.si_hua {
                                                                {
                                                                    let bg = match sihua {
                                                                        SiHuaType::HuaLu => "bg-emerald-500/20 text-emerald-300 border-emerald-500/30",
                                                                        SiHuaType::HuaQuan => "bg-blue-500/20 text-blue-300 border-blue-500/30",
                                                                        SiHuaType::HuaKe => "bg-violet-500/20 text-violet-300 border-violet-500/30",
                                                                        SiHuaType::HuaJi => "bg-red-500/20 text-red-300 border-red-500/30",
                                                                    };
                                                                    rsx! {
                                                                        span { class: "px-1.5 py-0.5 rounded text-[9px] font-black border {bg}",
                                                                            "선천 {sihua.emoji()}"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        if !star_desc.is_empty() {
                                                            p { class: "text-xs text-slate-400 leading-relaxed pl-1",
                                                                "{star_desc}"
                                                            }
                                                        }
                                                    }
                                                }
                                            })
                                        }
                                    }
                                }
                            }

                            // 3. 비성사화 분석 (Flying Stars)
                            {
                                let outbound_stars = chart.flying_sihua.iter()
                                    .filter(|fs| fs.from_palace == palace.name)
                                    .collect::<Vec<_>>();
                                let inbound_stars = chart.flying_sihua.iter()
                                    .filter(|fs| fs.to_palace == palace.name)
                                    .collect::<Vec<_>>();

                                rsx! {
                                    div { class: "space-y-3",
                                        h4 { class: "text-xs font-black text-slate-400 uppercase tracking-wider border-l-2 border-emerald-500 pl-2",
                                            match locale {
                                                Locale::Ko => "궁간 비성사화 분석 (Palace Flying Stars)",
                                                Locale::Zh => "宫干飞星四化分析",
                                                Locale::En => "Palace Flying Stars Analysis",
                                                Locale::Ru => "Анализ Летящих Звезд Дворца",
                                            }
                                        }
                                        div { class: "grid grid-cols-1 sm:grid-cols-2 gap-3",
                                            // 송출 (Outbound)
                                            div { class: "bg-slate-950/20 border border-slate-850 rounded-xl p-3.5 space-y-2.5",
                                                span { class: "text-[10px] text-emerald-400 font-bold flex items-center gap-1.5",
                                                    span { class: "w-1.5 h-1.5 rounded-full bg-emerald-500" }
                                                    match locale {
                                                        Locale::Ko => "송출 사화 (Outbound)",
                                                        Locale::Zh => "送出飞星 (向外)",
                                                        Locale::En => "Outbound Sihua",
                                                        Locale::Ru => "Исходящие Сихуа",
                                                    }
                                                }
                                                if outbound_stars.is_empty() {
                                                    p { class: "text-[10px] text-slate-500 italic pl-3", "—" }
                                                } else {
                                                    div { class: "space-y-1.5 pl-3",
                                                        {
                                                            outbound_stars.iter().map(|fs| {
                                                                let color = match fs.sihua_type {
                                                                    SiHuaType::HuaLu => "text-emerald-400",
                                                                    SiHuaType::HuaQuan => "text-blue-400",
                                                                    SiHuaType::HuaKe => "text-violet-400",
                                                                    SiHuaType::HuaJi => "text-red-400",
                                                                };
                                                                let star_label = translate_zwds_star(locale, fs.star);
                                                                let to_label = translate_zwds_palace(locale, fs.to_palace);
                                                                rsx! {
                                                                    div { key: "{fs.star.korean()}-out", class: "text-xs flex items-center gap-1.5 text-slate-300",
                                                                        span { class: "font-semibold {color}", "{fs.sihua_type.emoji()}" }
                                                                        span { class: "text-slate-400", "{star_label}" }
                                                                        span { class: "text-slate-500", "➔" }
                                                                        span { class: "font-bold text-slate-200", "{to_label}" }
                                                                    }
                                                                }
                                                            })
                                                        }
                                                    }
                                                }
                                            }

                                            // 유입 (Inbound)
                                            div { class: "bg-slate-950/20 border border-slate-850 rounded-xl p-3.5 space-y-2.5",
                                                span { class: "text-[10px] text-indigo-400 font-bold flex items-center gap-1.5",
                                                    span { class: "w-1.5 h-1.5 rounded-full bg-indigo-500" }
                                                    match locale {
                                                        Locale::Ko => "유입 사화 (Inbound)",
                                                        Locale::Zh => "引入飞星 (向内)",
                                                        Locale::En => "Inbound Sihua",
                                                        Locale::Ru => "Входящие Сихуа",
                                                    }
                                                }
                                                if inbound_stars.is_empty() {
                                                    p { class: "text-[10px] text-slate-500 italic pl-3", "—" }
                                                } else {
                                                    div { class: "space-y-1.5 pl-3",
                                                        {
                                                            inbound_stars.iter().map(|fs| {
                                                                let color = match fs.sihua_type {
                                                                    SiHuaType::HuaLu => "text-emerald-400",
                                                                    SiHuaType::HuaQuan => "text-blue-400",
                                                                    SiHuaType::HuaKe => "text-violet-400",
                                                                    SiHuaType::HuaJi => "text-red-400",
                                                                };
                                                                let star_label = translate_zwds_star(locale, fs.star);
                                                                let from_label = translate_zwds_palace(locale, fs.from_palace);
                                                                rsx! {
                                                                    div { key: "{fs.star.korean()}-in", class: "text-xs flex items-center gap-1.5 text-slate-300",
                                                                        span { class: "font-semibold {color}", "{fs.sihua_type.emoji()}" }
                                                                        span { class: "text-slate-400", "{star_label}" }
                                                                        span { class: "text-slate-500", "➔" }
                                                                        span { class: "font-bold text-slate-200", "{from_label}" }
                                                                    }
                                                                }
                                                            })
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // 오른쪽 칼럼: 삼방사정 그리드 (5/12)
                        div { class: "md:col-span-5 space-y-6 flex flex-col items-center md:items-stretch",
                            div { class: "w-full space-y-3",
                                h4 { class: "text-xs font-black text-slate-400 uppercase tracking-wider border-l-2 border-indigo-500 pl-2",
                                    match locale {
                                        Locale::Ko => "삼방사정 (三方四正) 맵",
                                        Locale::Zh => "三方四正图",
                                        Locale::En => "Three-Party & Four-Directions",
                                        Locale::Ru => "Схема влияний (Триада)",
                                    }
                                }

                                div { class: "grid grid-cols-4 gap-1.5 p-2 bg-slate-950 rounded-2xl border border-slate-850/60 max-w-[240px] mx-auto w-full aspect-square justify-center items-center shadow-inner",
                                    {
                                        mini_grid_cells.into_iter().enumerate().map(|(idx, cell)| {
                                            match cell {
                                                Some(p_idx) => {
                                                    let p = &chart.palaces[p_idx];
                                                    let abbr = crate::i18n::translate_zwds_palace_abbr(locale, p.name);

                                                    let cell_cls = if p_idx == palace_idx {
                                                        "bg-violet-600 border-violet-400 text-white font-black scale-105 shadow-md shadow-violet-900/20"
                                                    } else if p_idx == opposite_idx {
                                                        "bg-fuchsia-950/30 border-fuchsia-500/80 text-fuchsia-300 font-bold"
                                                    } else if p_idx == triad1_idx || p_idx == triad2_idx {
                                                        "bg-indigo-950/30 border-indigo-500/80 text-indigo-300 font-bold"
                                                    } else {
                                                        "bg-slate-900/60 border-slate-800/40 text-slate-600 hover:border-slate-800"
                                                    };

                                                    rsx! {
                                                        div {
                                                            key: "mini-{p_idx}",
                                                            class: "w-10 h-10 rounded-lg border text-[10px] flex items-center justify-center transition-all duration-150 {cell_cls}",
                                                            title: "{p.heavenly_stem}{p.earthly_branch}",
                                                            "{abbr}"
                                                        }
                                                    }
                                                },
                                                None => {
                                                    if idx == 5 {
                                                        rsx! {
                                                            div {
                                                                class: "col-span-2 row-span-2 text-[8px] text-slate-700 font-mono font-bold flex items-center justify-center text-center leading-none",
                                                                "三方"
                                                                br {}
                                                                "四正"
                                                            }
                                                        }
                                                    } else {
                                                        rsx! {}
                                                    }
                                                }
                                            }
                                        })
                                    }
                                }
                            }

                            div { class: "w-full space-y-3.5 bg-slate-950/30 p-4 rounded-2xl border border-slate-850 shadow-inner",
                                h5 { class: "text-[11px] font-black text-slate-400 tracking-wide uppercase border-b border-slate-850 pb-1.5",
                                    match locale {
                                        Locale::Ko => "삼방사정 영향 성계",
                                        Locale::Zh => "会照星曜",
                                        Locale::En => "Projected Stars",
                                        Locale::Ru => "Влияющие звезды",
                                    }
                                }

                                // 1. 대궁 영향성
                                div { class: "space-y-1",
                                    span { class: "text-[10px] text-fuchsia-400 font-bold flex items-center gap-1.5",
                                        span { class: "w-1.5 h-1.5 rounded-full bg-fuchsia-500" }
                                        "{translate_zwds_palace(locale, opposite_palace.name)} (대궁)"
                                    }
                                    div { class: "flex flex-wrap gap-1 pl-3",
                                        if opposite_palace.stars.is_empty() {
                                            span { class: "text-[10px] text-slate-600 italic", "배치성 없음" }
                                        } else {
                                            {
                                                opposite_palace.stars.iter().map(|s| {
                                                    rsx! {
                                                        span { key: "opp-{s.star.korean()}", class: "text-[10px] px-1.5 py-0.5 rounded bg-slate-900 border border-slate-850 text-slate-400",
                                                            "{translate_zwds_star(locale, s.star)}"
                                                        }
                                                    }
                                                })
                                            }
                                        }
                                    }
                                }

                                // 2. 삼합궁1 영향성
                                div { class: "space-y-1",
                                    span { class: "text-[10px] text-indigo-400 font-bold flex items-center gap-1.5",
                                        span { class: "w-1.5 h-1.5 rounded-full bg-indigo-500" }
                                        "{translate_zwds_palace(locale, triad1_palace.name)} (합궁)"
                                    }
                                    div { class: "flex flex-wrap gap-1 pl-3",
                                        if triad1_palace.stars.is_empty() {
                                            span { class: "text-[10px] text-slate-650", "—" }
                                        } else {
                                            {
                                                triad1_palace.stars.iter().map(|s| {
                                                    rsx! {
                                                        span { key: "tri1-{s.star.korean()}", class: "text-[10px] px-1.5 py-0.5 rounded bg-slate-900 border border-slate-850 text-slate-400",
                                                            "{translate_zwds_star(locale, s.star)}"
                                                        }
                                                    }
                                                })
                                            }
                                        }
                                    }
                                }

                                // 3. 삼합궁2 영향성
                                div { class: "space-y-1",
                                    span { class: "text-[10px] text-indigo-400 font-bold flex items-center gap-1.5",
                                        span { class: "w-1.5 h-1.5 rounded-full bg-indigo-500" }
                                        "{translate_zwds_palace(locale, triad2_palace.name)} (합궁)"
                                    }
                                    div { class: "flex flex-wrap gap-1 pl-3",
                                        if triad2_palace.stars.is_empty() {
                                            span { class: "text-[10px] text-slate-650", "—" }
                                        } else {
                                            {
                                                triad2_palace.stars.iter().map(|s| {
                                                    rsx! {
                                                        span { key: "tri2-{s.star.korean()}", class: "text-[10px] px-1.5 py-0.5 rounded bg-slate-900 border border-slate-850 text-slate-400",
                                                            "{translate_zwds_star(locale, s.star)}"
                                                        }
                                                    }
                                                })
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
    }
}

fn get_palace_description(locale: Locale, name: eon_zwds::types::PalaceName) -> &'static str {
    match locale {
        Locale::Ko => match name {
            eon_zwds::types::PalaceName::Ming => "명궁은 자아, 성격, 평생의 운명적 지향점과 선천적인 복덕을 상징합니다. 인생 전반의 방향성을 좌우하는 가장 중요한 궁입니다.",
            eon_zwds::types::PalaceName::Xiongdi => "형제궁은 형제자매, 동료와의 관계, 협력 관계 및 재정적 동반자를 의미합니다.",
            eon_zwds::types::PalaceName::Fuqi => "부처궁은 배우자와의 관계, 이상적인 배우자 상, 결혼 생활의 길흉을 의미합니다.",
            eon_zwds::types::PalaceName::Zinv => "자녀궁은 자녀와의 유대감, 자손의 번창 여부, 창작 및 투자 기운을 나타냅니다.",
            eon_zwds::types::PalaceName::Caibo => "재백궁은 재물을 버는 방식, 수입의 원천, 재정적 능력 및 소비 성향을 상징합니다.",
            eon_zwds::types::PalaceName::Jie => "질액궁은 선천적인 건강 상태, 체질, 주의해야 할 질병 및 재난을 의미합니다.",
            eon_zwds::types::PalaceName::Qianyi => "천이궁은 사회 활동, 타향/해외 이동, 대인관계에서의 외적인 모습과 여행 운을 나타냅니다.",
            eon_zwds::types::PalaceName::Nupao => "노복궁은 부하 직원, 친구, 지인, 사회적 인맥과의 관계 및 인복을 의미합니다.",
            eon_zwds::types::PalaceName::Guanlu => "관록궁은 직업적인 성취, 학업, 승진, 적합한 직업 분야 및 사회적 위상을 상징합니다.",
            eon_zwds::types::PalaceName::Tianzhai => "전택궁은 부동산, 주거 환경, 가정의 평화 및 자산 축적 형태를 의미합니다.",
            eon_zwds::types::PalaceName::Fude => "복덕궁은 정신세계, 취미, 영적인 성향, 내면의 행복감 및 노후의 편안함을 상징합니다.",
            eon_zwds::types::PalaceName::Fumu => "부모궁은 부모님과의 관계, 윗사람이나 국가 기관의 덕, 학문 및 문서를 상징합니다.",
        },
        Locale::Zh => match name {
            eon_zwds::types::PalaceName::Ming => "命宫代表自我、性格、一生的命运走向和先天福德。是决定人生大方向的最核心宫位。",
            eon_zwds::types::PalaceName::Xiongdi => "兄弟宫代表与兄弟姐妹、亲密同事的关系，以及财务上的合作伙伴。",
            eon_zwds::types::PalaceName::Fuqi => "夫妻宫掌管婚姻、恋爱风格、理想伴侣的特征以及婚姻生活的吉凶。",
            eon_zwds::types::PalaceName::Zinv => "子女宫代表与子女的缘分、子嗣昌盛与否、创造力以及小额投资运气。",
            eon_zwds::types::PalaceName::Caibo => "财帛宫描述赚钱能力、收入来源、财务理财方式和消费倾向。",
            eon_zwds::types::PalaceName::Jie => "疾厄宫代表先天健康状况、体质、容易罹患的疾病和人生灾难。",
            eon_zwds::types::PalaceName::Qianyi => "迁移宫掌管社会活动、出外及海外发展、人际关系中的外在形象和旅行运势。",
            eon_zwds::types::PalaceName::Nupao => "奴仆宫（交友宫）代表与下属、朋友、人脉圈的关系以及社交福气。",
            eon_zwds::types::PalaceName::Guanlu => "官禄宫（事业宫）代表职业成就、学业运势、升迁、适合的行业及社会地位。",
            eon_zwds::types::PalaceName::Tianzhai => "田宅宫代表不动产运势、居住环境、家业继承、家庭和谐度以及资产累积形态。",
            eon_zwds::types::PalaceName::Fude => "福德宫代表精神世界、兴趣爱好、潜意识、内心幸福感以及晚年享福运。",
            eon_zwds::types::PalaceName::Fumu => "父母宫代表与父母的关系、长辈及上司的提携、学业证书和法律契约。",
        },
        _ => match name {
            eon_zwds::types::PalaceName::Ming => "The Life Palace represents the self, personality, lifelong destiny, and innate fortune. It is the most critical palace governing your life.",
            eon_zwds::types::PalaceName::Xiongdi => "The Siblings Palace represents relationships with siblings, close colleagues, and financial partners.",
            eon_zwds::types::PalaceName::Fuqi => "The Spouse Palace governs marriage, relationship style, and the traits of your ideal partner.",
            eon_zwds::types::PalaceName::Zinv => "The Children Palace represents relationships with children, descendant luck, creativity, and minor investments.",
            eon_zwds::types::PalaceName::Caibo => "The Wealth Palace describes your earning capacity, income sources, and financial management style.",
            eon_zwds::types::PalaceName::Jie => "The Health Palace indicates physical constitution, potential diseases, and general physical well-being.",
            eon_zwds::types::PalaceName::Qianyi => "The Travel Palace represents social relations, relocation, travel fortunes, and how you appear to the public.",
            eon_zwds::types::PalaceName::Nupao => "The Friends Palace governs relationships with subordinates, friends, social circles, and general networking luck.",
            eon_zwds::types::PalaceName::Guanlu => "The Career Palace represents professional achievements, academic success, job suitability, and public status.",
            eon_zwds::types::PalaceName::Tianzhai => "The Property Palace governs real estate, home environment, family inheritance, and long-term asset accumulation.",
            eon_zwds::types::PalaceName::Fude => "The Karma Palace represents your inner mind, spiritual affinity, hobbies, happiness, and comfort in late life.",
            eon_zwds::types::PalaceName::Fumu => "The Parents Palace represents relations with parents, support from mentors or government, education, and legal documents.",
        }
    }
}

fn get_star_description(locale: Locale, star: eon_zwds::types::ZwdsStar) -> &'static str {
    match locale {
        Locale::Ko => match star {
            ZwdsStar::ZiWei => "제왕의 별로 권위, 명예, 리더십, 품격을 상징합니다.",
            ZwdsStar::TianJi => "기획과 지혜의 별로 계산, 분석, 총명함과 잦은 변화를 나타냅니다.",
            ZwdsStar::TaiYang => "태양처럼 빛을 퍼뜨리며 명예, 열정, 공익, 부친/남편을 상징합니다.",
            ZwdsStar::WuQu => "실질적인 재물과 결단력을 상징하는 강력한 금전의 재성(財星)입니다.",
            ZwdsStar::TianTong => {
                "안락함과 복덕을 상징하며, 온화하고 친화력이 풍부하지만 다소 나태할 수 있습니다."
            }
            ZwdsStar::LianZhen => {
                "감정과 규율의 별로 강한 주관, 예술성, 도화 기질, 집념을 의미합니다."
            }
            ZwdsStar::TianFu => {
                "안정적인 곳간을 의미하며 자산 보존, 보수적 성향, 포용력을 상징합니다."
            }
            ZwdsStar::TaiYin => "저축, 부동산, 모성애를 상징하며 부드럽고 섬세한 재성(財星)입니다.",
            ZwdsStar::TanLang => {
                "욕망, 사교성, 현실적인 예술 및 신비한 학문(역학)을 상징하는 도화성입니다."
            }
            ZwdsStar::JuMen => {
                "말(言), 상세한 연구, 의심, 구설수를 뜻하며 깊은 탐구력을 나타냅니다."
            }
            ZwdsStar::TianXiang => {
                "도장(인장)과 보필을 상징하며, 타인을 배려하고 품위와 조화를 유지합니다."
            }
            ZwdsStar::TianLiang => {
                "보호와 천수를 상징하며 문제를 해결하고 아랫사람을 챙겨주는 장로의 성향입니다."
            }
            ZwdsStar::QiSha => {
                "장수와 투지를 상징하며 강력한 돌파력, 추진력과 독립심, 고독을 뜻합니다."
            }
            ZwdsStar::PoJun => {
                "개척과 파괴의 별로 기존 질서를 부수고 새로운 변화를 주도하는 혁명적 기운입니다."
            }

            ZwdsStar::WenChang => "학문, 시험, 문서적 성취 및 이론적인 두뇌 능력을 돕습니다.",
            ZwdsStar::WenQu => "예술적 재능, 감수성, 임기응변 및 실무적인 문예 재능을 돕습니다.",
            ZwdsStar::ZuoFu => {
                "주변 조력자와 귀인의 도움을 보조하여 매사 순조로운 흐름을 만듭니다."
            }
            ZwdsStar::YouBi => "보이지 않는 귀인의 조력과 중재를 상징하며 협력을 강하게 만듭니다.",
            ZwdsStar::TianKui => "사회적인 기회와 공개적인 조력자가 등장하여 명예를 돕습니다.",
            ZwdsStar::TianYue => {
                "음덕과 예상치 못한 후원자, 귀인의 도움으로 위기를 돌파하게 합니다."
            }

            ZwdsStar::LuCun => "선천적인 금전운과 녹봉을 나타내며 안정적인 현금 흐름을 보장합니다.",
            ZwdsStar::QingYang => {
                "강력한 경쟁심, 돌파력 및 때로는 부상이나 수술 등의 칼날을 상징합니다."
            }
            ZwdsStar::TuoLuo => {
                "지연, 정체, 끈질김을 나타내며 보이지 않는 암초나 내면의 고민을 뜻합니다."
            }
            ZwdsStar::HuoXing => {
                "빠른 행동력과 폭발성, 때로는 성급함으로 인한 손해나 화재를 뜻합니다."
            }
            ZwdsStar::LingXing => {
                "내면의 열기, 암묵적인 노력, 스트레스와 급작스러운 이탈을 나타냅니다."
            }
            ZwdsStar::DiKong => {
                "정신적인 지향, 공허함, 기존 상식을 벗어난 독창성과 물질적 상실을 상징합니다."
            }
            ZwdsStar::DiJie => {
                "재정적 낭비, 예기치 못한 도난이나 소모, 기발한 발상과 정신적 깨달음을 의미합니다."
            }
            _ => "자미두수의 운명적 영향력을 지닌 잡성(雜星)입니다.",
        },
        Locale::Zh => match star {
            ZwdsStar::ZiWei => "帝王之星，象征权威、名誉、领导力和高贵品质。",
            ZwdsStar::TianJi => "谋划与智慧之星，代表计算、分析、聪明才智与多动变化。",
            ZwdsStar::TaiYang => "如太阳般普照，象征名誉、热情、公益以及父亲/丈夫。",
            ZwdsStar::WuQu => "代表实际财富与决断力，是强有力的金属财星。",
            ZwdsStar::TianTong => "象征安乐与福德，温和且亲和力强，但可能略显被动。",
            ZwdsStar::LianZhen => "情感与纪律之星，代表强烈的主观意识、艺术细胞与执着。",
            ZwdsStar::TianFu => "象征稳定的库藏，掌管资产保存、保守态度与包容力。",
            ZwdsStar::TaiYin => "象征储蓄、房地产及母爱，是温柔细腻的财星。",
            ZwdsStar::TanLang => "代表欲望、社交、现实艺术与玄学，属于第一桃花星。",
            ZwdsStar::JuMen => "代表语言、细致研究、多疑与口舌是非，探究能力强。",
            ZwdsStar::TianXiang => "掌管印信与辅佐，待人随和，注重品位与和谐。",
            ZwdsStar::TianLiang => "象征保护、寿考与荫庇，好为人师，善于解决纠纷。",
            ZwdsStar::QiSha => "象征将军与斗志，冲劲十足，极具开拓力、独立性与孤独感。",
            ZwdsStar::PoJun => "开创与破败之星，打破旧秩序并主导新变化的革命性力量。",

            ZwdsStar::WenChang => "辅助学术、考试、文书成就及理论脑力。",
            ZwdsStar::WenQu => "辅助艺术天分、感性思维、临机应变及实务才华。",
            ZwdsStar::ZuoFu => "辅助周围同辈的助力，令谋事更为顺遂。",
            ZwdsStar::YouBi => "象征暗中贵人相助与调解，加强人际合作关系。",
            ZwdsStar::TianKui => "带来社会机遇与公开的贵人提携，利于名声。",
            ZwdsStar::TianYue => "隐秘的福报与出乎意料的长辈帮助，化解危机。",

            ZwdsStar::LuCun => "掌管先天禄禄与俸禄，保障稳定的现金流与福气。",
            ZwdsStar::QingYang => "象征强烈竞争心、破坏力，有时暗示受伤或手术。",
            ZwdsStar::TuoLuo => "象征拖延、停滞不前、暗礁以及内心深处的纠结。",
            ZwdsStar::HuoXing => "行动迅速且爆发力强，有时因急躁造成损失或火灾。",
            ZwdsStar::LingXing => "内心火热、暗中努力、压力积聚或突发性波动。",
            ZwdsStar::DiKong => "象征精神上的探索、空虚感、突破常规的创意及物质流失。",
            ZwdsStar::DiJie => "财务超支、预料之外的破财、独特巧思与精神顿悟。",
            _ => "紫微斗数中具有特定命运影响力的杂曜。",
        },
        _ => match star {
            ZwdsStar::ZiWei => {
                "The Emperor star, representing authority, prestige, leadership, and nobility."
            }
            ZwdsStar::TianJi => {
                "The Planner star, representing intellect, strategy, and changeability."
            }
            ZwdsStar::TaiYang => {
                "The Sun star, representing recognition, energy, public service, and male figures."
            }
            ZwdsStar::WuQu => {
                "The Soldier star, a powerful wealth star representing execution and decisiveness."
            }
            ZwdsStar::TianTong => {
                "The Harmony star, representing comfort, blessing, and emotional sensitivity."
            }
            ZwdsStar::LianZhen => {
                "The Judge star, representing emotions, discipline, magnetism, and ambition."
            }
            ZwdsStar::TianFu => {
                "The Treasury star, representing asset conservation, stability, and tolerance."
            }
            ZwdsStar::TaiYin => {
                "The Moon star, representing savings, intuition, real estate, and female figures."
            }
            ZwdsStar::TanLang => {
                "The Wolf star, representing desires, social skills, and spiritual pursuits."
            }
            ZwdsStar::JuMen => {
                "The Gate star, representing speaking, deep analysis, and potential disputes."
            }
            ZwdsStar::TianXiang => {
                "The Minister star, representing trust, contract verification, and dignity."
            }
            ZwdsStar::TianLiang => {
                "The Blessing star, representing elderly protection, benevolence, and support."
            }
            ZwdsStar::QiSha => {
                "The General star, representing combativeness, rapid breakthroughs, and loneliness."
            }
            ZwdsStar::PoJun => {
                "The Pioneer star, representing revolution, consumption, and major changes."
            }

            ZwdsStar::WenChang => {
                "Supports academic success, official exams, and theoretical skills."
            }
            ZwdsStar::WenQu => "Supports artistic talent, intuition, and communication skills.",
            ZwdsStar::ZuoFu => "Provides direct assistance and visible peer support.",
            ZwdsStar::YouBi => "Provides indirect assistance, mediation, and relationship harmony.",
            ZwdsStar::TianKui => "Brings formal opportunities and prominent mentors.",
            ZwdsStar::TianYue => {
                "Brings unexpected helpers, hidden fortunes, and crisis resolution."
            }

            ZwdsStar::LuCun => "Governs innate wealth, salary, and steady financial flow.",
            ZwdsStar::QingYang => {
                "Represents intense competition, breakthrough energy, or sharp injuries."
            }
            ZwdsStar::TuoLuo => {
                "Represents delays, hidden obstacles, and persistent inner conflicts."
            }
            ZwdsStar::HuoXing => "Represents rapid action, explosive passion, or sudden anger.",
            ZwdsStar::LingXing => "Represents hidden stress, quiet dedication, and sudden shifts.",
            ZwdsStar::DiKong => {
                "Represents spiritual inclinations, empty space, and unconventional creativity."
            }
            ZwdsStar::DiJie => {
                "Represents financial spendings, unexpected losses, and unique insights."
            }
            _ => "A minor helper star in Zi Wei Dou Shu.",
        },
    }
}

/// ZWDS 12궁위의 성반 4x4 Grid 상의 중심 좌표 (0..100 기준)
fn get_palace_center(p_idx: usize) -> (f64, f64) {
    let (r, c) = match p_idx {
        0 => (3, 0),  // 寅
        1 => (2, 0),  // 卯
        2 => (1, 0),  // 辰
        3 => (0, 0),  // 巳
        4 => (0, 1),  // 午
        5 => (0, 2),  // 未
        6 => (0, 3),  // 申
        7 => (1, 3),  // 酉
        8 => (2, 3),  // 戌
        9 => (3, 3),  // 亥
        10 => (3, 2), // 子
        11 => (3, 1), // 丑
        _ => (0, 0),
    };
    ((c as f64 * 2.0 + 1.0) * 12.5, (r as f64 * 2.0 + 1.0) * 12.5)
}
