//! 자미두수 탭 컴포넌트
//!
//! 12궁 성반 격자 시각화(4x4 Grid 테두리 배치 + 중앙 정보 영역),
//! 대한(대운) 목록 및 상세 정보, 마크다운 내보내기 버튼을 지원합니다.

use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use crate::i18n::{t, TK, Locale};
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
                    "자미두수 성반 (紫微斗數 星盤)"
                }
                div { class: "flex items-center gap-3",
                    button {
                        class: "px-5 py-2.5 bg-gradient-to-r from-violet-600 to-indigo-600 hover:from-violet-500 hover:to-indigo-500 rounded-xl font-semibold text-white shadow-lg shadow-violet-900/30 transition-all duration-200 active:scale-95 cursor-pointer",
                        onclick: run_analysis,
                        "🔮 자미두수 분석 실행"
                    }
                }
            }

            // 3. 분석 결과 영역
            {
                match &zwds_status {
                    TaskStatus::Idle => rsx! {
                        div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500 bg-slate-900/20 border border-slate-800/40 rounded-3xl",
                            span { class: "text-5xl animate-bounce", "🔮" }
                            p { class: "text-lg font-medium", "출생 정보를 입력하고 [자미두수 분석 실행]을 누르세요." }
                        }
                    },
                    TaskStatus::Loading => rsx! {
                        div { class: "flex flex-col items-center justify-center py-20 gap-3 bg-slate-900/20 border border-slate-800/40 rounded-3xl",
                            div { class: "w-12 h-12 rounded-full border-4 border-violet-500/30 border-t-violet-400 animate-spin" }
                            p { class: "text-violet-400 font-medium animate-pulse", "자미성계 및 천부성계 성반 포국 중..." }
                        }
                    },
                    TaskStatus::Error(e) => rsx! {
                        div { class: "p-4 rounded-xl bg-red-900/20 border border-red-800/50 text-red-400",
                            "분석 오류: {e}"
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
                            rsx! { div { "데이터 로드 실패" } }
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
fn PalaceCard(palace: PalaceData, is_soul: bool, is_body: bool) -> Element {
    // 테두리 및 배경 디자인
    let border_cls = if palace.is_current_liu_nian {
        "border-amber-500/80 shadow-lg shadow-amber-900/10 bg-amber-950/10"
    } else if is_soul {
        "border-violet-500/70 shadow-lg shadow-violet-900/10 bg-violet-950/10"
    } else {
        "border-slate-800/70 bg-slate-900/40 hover:border-slate-700/60"
    };

    rsx! {
        div { class: "h-44 p-3 rounded-2xl border flex flex-col justify-between transition-all duration-200 {border_cls}",
            // 궁 헤더: 천간/지지 & 궁명
            div { class: "flex justify-between items-start",
                div { class: "flex flex-col",
                    span { class: "text-[10px] text-slate-500 font-bold",
                        "{palace.heavenly_stem}{palace.earthly_branch}"
                    }
                    span { class: "text-[10px] text-slate-400 font-bold",
                        "{palace.earthly_branch}궁"
                    }
                }
                div { class: "flex items-center gap-1",
                    if palace.is_current_liu_nian {
                        span { class: "px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-300 text-[8px] font-black tracking-wider border border-amber-500/30 animate-pulse",
                            "유년"
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
                        "{palace.name.korean()}"
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

                        rsx! {
                            div { key: "{star_in_p.star.korean()}", class: "flex items-center justify-between",
                                span { class: "{star_color}",
                                    "{star_in_p.star.korean()}"
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
                                            span { class: "px-1 rounded text-[8px] font-black border {bg}",
                                                "{sihua.emoji()}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    })
                }
            }

            // 대한 연령대 범위 표시
            div { class: "text-[9px] text-slate-500 text-right border-t border-slate-800/30 pt-1 font-mono",
                {
                    palace.daxian_range
                        .map(|r| format!("{} - {}세", r.0, r.1))
                        .unwrap_or_else(|| "—".to_string())
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

    rsx! {
        div { class: "col-span-2 row-span-2 bg-gradient-to-br from-slate-900 to-slate-950 border border-slate-800/80 rounded-3xl p-6 flex flex-col justify-between shadow-2xl relative overflow-hidden",
            // 은은한 그라데이션 장식 배경
            div { class: "absolute -right-10 -top-10 w-40 h-40 bg-violet-600/10 rounded-full blur-3xl" }
            div { class: "absolute -left-10 -bottom-10 w-40 h-40 bg-indigo-600/10 rounded-full blur-3xl" }

            div { class: "space-y-4 relative z-10",
                div { class: "border-b border-slate-800 pb-3",
                    h3 { class: "text-lg font-bold text-slate-100 flex items-center gap-2",
                        span { "🌌" }
                        "운명 분석 명반 정보"
                    }
                }

                // 세부 속성 리스트
                div { class: "grid grid-cols-2 gap-4 text-xs",
                    div { class: "space-y-2.5",
                        div { class: "flex flex-col gap-0.5",
                            span { class: "text-slate-500 font-medium", "명주 (命主)" }
                            span { class: "text-slate-200 font-bold text-sm", "{chart.soul_master.korean()}" }
                        }
                        div { class: "flex flex-col gap-0.5",
                            span { class: "text-slate-500 font-medium", "신주 (身主)" }
                            span { class: "text-slate-200 font-bold text-sm", "{chart.body_master.korean()}" }
                        }
                    }
                    div { class: "space-y-2.5",
                        div { class: "flex flex-col gap-0.5",
                            span { class: "text-slate-500 font-medium", "오행국 (五行局)" }
                            span { class: "text-violet-300 font-bold text-sm", "{chart.five_elements.korean()}" }
                        }
                        div { class: "flex flex-col gap-0.5",
                            span { class: "text-slate-500 font-medium", "현재 분석 대한" }
                            span { class: "text-indigo-300 font-bold text-sm",
                                "{data.current_daxian.stem_hanja}{data.current_daxian.branch_hanja} 대운 ({data.current_daxian.age_start}-{data.current_daxian.age_end}세)"
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
                        span { class: "text-emerald-400 font-bold animate-pulse", "✓ 성공적으로 복사됨!" }
                    } else {
                        span { "📋" }
                        span { "자미두수 보고서 복사 (Markdown)" }
                    }
                }
            }
        }
    }
}

/// 10년 대한 주기 일람 컴포넌트
#[component]
fn DaXianSection(data: eon_service::dto::ZwdsAnalysisOutput) -> Element {
    let chart = &data.chart;

    rsx! {
        div { class: "bg-slate-900/30 border border-slate-800/40 rounded-3xl p-6 space-y-4 shadow-xl",
            h3 { class: "text-lg font-bold text-slate-200 flex items-center gap-2",
                span { "⏳" }
                "10년 대한 대운 주기"
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

                        rsx! {
                            div {
                                key: "{dx.index}",
                                class: "p-4 rounded-xl border flex flex-col justify-between gap-1.5 transition-all duration-200 {card_cls}",
                                div { class: "flex justify-between items-center",
                                    span { class: "text-[10px] text-slate-500 font-bold uppercase tracking-wider",
                                        "{dx.index + 1} 대운"
                                    }
                                    if is_current {
                                        span { class: "px-1.5 py-0.5 rounded bg-violet-500/20 text-violet-300 text-[8px] font-black border border-violet-500/30",
                                            "현재 진행"
                                        }
                                    }
                                }
                                span { class: "text-sm font-black text-slate-200",
                                    "{dx.age_start} ~ {dx.age_end}세"
                                }
                                div { class: "flex justify-between items-center text-xs border-t border-slate-800/40 pt-1.5 mt-1",
                                    span { class: "text-slate-400 font-semibold",
                                        "{dx.stem_hanja}{dx.branch_hanja} 대운"
                                    }
                                    span { class: "text-slate-500 font-medium text-[10px]",
                                        "궁위 {dx.palace_idx}"
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
