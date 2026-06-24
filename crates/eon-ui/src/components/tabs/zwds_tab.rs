//! 자미두수 탭 컴포넌트
//!
//! 12궁 성반 격자 시각화(4x4 Grid 테두리 배치 + 중앙 정보 영역),
//! 대한(대운) 목록 및 상세 정보, 마크다운 내보내기 버튼을 지원합니다.

use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use crate::i18n::{t, TK, Locale, translate_zwds_palace, translate_zwds_star, translate_five_elements, format_age_range};
use eon_service::dto::{ZwdsAnalysisInput, AnalysisInput};
use eon_service::facade;
use crate::components::shared::birth_form::BirthForm;
use eon_zwds::types::{PalaceData, ZwdsStar, SiHuaType};
use crate::components::shared::export_markdown::export_zwds_to_markdown;

#[component]
pub fn ZwdsTab() -> Element {
    let mut state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    let run_analysis = move |_| {
        spawn(async move {
            state.zwds.write().status = TaskStatus::Loading;
            let form = state.form.read().clone();
            let input = ZwdsAnalysisInput::new(
                AnalysisInput {
                    year: form.year,
                    month: form.month,
                    day: form.day,
                    hour: form.hour,
                    minute: form.minute,
                    is_lunar: form.is_lunar,
                    is_leap_month: form.is_leap_month,
                    lat: form.lat,
                    lon: form.lon,
                    timezone: "Asia/Seoul".to_string(),
                },
                form.is_male,
                None, // 기본 현재 연도
            );
            match facade::analyze_zwds(input) {
                Ok(res) => {
                    state.zwds.write().data = Some(res);
                    state.zwds.write().status = TaskStatus::Success;
                }
                Err(e) => {
                    state.zwds.write().error = Some(e.to_string());
                    state.zwds.write().status = TaskStatus::Error(e.to_string());
                }
            }
        });
    };

    let zwds_status = state.zwds.read().status.clone();
    let zwds_data = state.zwds.read().data.clone();

    rsx! {
        div { class: "space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-700",
            // 1. 공통 출생 정보 입력 폼
            BirthForm {}

            // 2. 타이틀 및 분석 버튼
            div { class: "flex justify-between items-center border-b border-slate-800/40 pb-4",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-violet-300 to-indigo-400 bg-clip-text text-transparent",
                    "{t(locale, TK::ZwdsReportTitle)}"
                }
                div { class: "flex items-center gap-3",
                    button {
                        class: "px-5 py-2.5 bg-gradient-to-r from-violet-600 to-indigo-600 hover:from-violet-500 hover:to-indigo-500 rounded-xl font-semibold text-white shadow-lg shadow-violet-900/30 transition-all duration-200 active:scale-95 cursor-pointer",
                        onclick: run_analysis,
                        "{t(locale, TK::ZwdsAnalyzeBtn)}"
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
                        if let Some(res) = zwds_data {
                            rsx! {
                                div { class: "space-y-6",
                                    // 4x4 성반 격자 시각화
                                    ZwdsGrid { data: res.clone() }

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
        }
    }
}

/// 자미두수 4x4 격자 성반 렌더링 컴포넌트
#[component]
fn ZwdsGrid(data: eon_service::dto::ZwdsAnalysisOutput) -> Element {
    // 4x4 Grid의 16칸 셀 배치 (외곽 12칸 궁 매핑, 중앙 4칸 비움)
    // ZWDS 지지 인덱스: 0=寅, 1=卯, 2=辰, 3=巳, 4=午, 5=未, 6=申, 7=酉, 8=戌, 9=亥, 10=子, 11=丑
    let grid_cells = vec![
        // Row 1: 巳(3) 午(4) 未(5) 申(6)
        Some(3), Some(4), Some(5), Some(6),
        // Row 2: 辰(2) [중앙] [중앙] 酉(7)
        Some(2), None, None, Some(7),
        // Row 3: 卯(1) [중앙] [중앙] 戌(8)
        Some(1), None, None, Some(8),
        // Row 4: 寅(0) 丑(11) 子(10) 亥(9)
        Some(0), Some(11), Some(10), Some(9),
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
        if palace.index == ln.liu_lu { annual_stars.push(ZwdsStar::LuCun); }
        if palace.index == ln.liu_yang { annual_stars.push(ZwdsStar::QingYang); }
        if palace.index == ln.liu_tuo { annual_stars.push(ZwdsStar::TuoLuo); }
        if palace.index == ln.liu_chang { annual_stars.push(ZwdsStar::WenChang); }
        if palace.index == ln.liu_qu { annual_stars.push(ZwdsStar::WenQu); }
    }

    rsx! {
        div { class: "h-44 p-3 rounded-2xl border flex flex-col justify-between transition-all duration-200 {border_cls}",
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
        format!("{}{}{} ({}-{}세)", data.current_daxian.stem_hanja, data.current_daxian.branch_hanja, t(locale, TK::ZwdsDaxianSuffix), data.current_daxian.age_start, data.current_daxian.age_end)
    } else if locale == Locale::Zh {
        format!("{}{}{} ({}-{}岁)", data.current_daxian.stem_hanja, data.current_daxian.branch_hanja, t(locale, TK::ZwdsDaxianSuffix), data.current_daxian.age_start, data.current_daxian.age_end)
    } else if locale == Locale::Ru {
        format!("{}{}{} ({}-{} лет)", data.current_daxian.stem_hanja, data.current_daxian.branch_hanja, t(locale, TK::ZwdsDaxianSuffix), data.current_daxian.age_start, data.current_daxian.age_end)
    } else {
        format!("{}{}{} (Age {}-{})", data.current_daxian.stem_hanja, data.current_daxian.branch_hanja, t(locale, TK::ZwdsDaxianSuffix), data.current_daxian.age_start, data.current_daxian.age_end)
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
