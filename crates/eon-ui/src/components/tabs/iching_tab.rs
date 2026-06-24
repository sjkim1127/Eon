// crates/eon-ui/src/components/tabs/iching_tab.rs
use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use crate::i18n::{t, TK, Locale};
use crate::i18n::iching_db::{get_hexagram_info, get_yao_name, get_yao_description};
use eon_service::facade;
use eon_service::dto::{AnalysisInput, SajuAnalysisInput};
use crate::components::shared::birth_form::BirthForm;

#[component]
pub fn IChingTab() -> Element {
    let mut state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    // 로컬 상호작용 상태 (선택된 효와 마우스 호버 중인 효)
    // (is_pre_natal, line_idx 1..6)
    let mut selected_yao = use_signal(|| None::<(bool, u8)>);
    let mut hovered_yao = use_signal(|| None::<(bool, u8)>);

    let run_analysis = move |_| {
        spawn(async move {
            state.iching.write().status = TaskStatus::Loading;
            let form = state.form.read().clone();

            let base_input = AnalysisInput {
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
            };

            let saju_input = SajuAnalysisInput::new(
                base_input,
                form.is_male,
                form.use_night_rat_hour,
                Some(false),
            );

            match facade::analyze_iching(saju_input) {
                Ok(res) => {
                    state.iching.write().data = Some(res);
                    state.iching.write().status = TaskStatus::Success;
                    // 성공 시 기본적으로 선천괘의 원당효를 선택된 상태로 둠
                    if let Some(iching_data) = &state.iching.read().data {
                        let yd = iching_data.result.yuan_dang_yao;
                        selected_yao.set(Some((true, yd)));
                    }
                }
                Err(e) => {
                    state.iching.write().error = Some(e.to_string());
                    state.iching.write().status = TaskStatus::Error(e.to_string());
                }
            }
        });
    };

    // 만나이 계산 (분석 기준 연도 또는 현재 연도 2026)
    let form_year = state.form.read().year;
    let current_age = (2026 - form_year).max(0) as u32;

    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            // ── 출생 정보 입력 폼 ──────────────────────────────────────────
            BirthForm {}

            // ── 타이틀 및 분석 버튼 ─────────────────────────────────────────
            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-violet-400 to-indigo-400 bg-clip-text text-transparent",
                    "{t(locale, TK::IChingTitle)}"
                }
                button {
                    class: "px-5 py-2.5 bg-gradient-to-r from-violet-600 to-indigo-600 hover:from-violet-500 hover:to-indigo-500 rounded-xl font-semibold text-white shadow-lg shadow-indigo-900/30 transition-all duration-200 active:scale-95 cursor-pointer",
                    onclick: run_analysis,
                    "🔮 {t(locale, TK::FormAnalyzeBtn)}"
                }
            }

            // ── 상태별 화면 분기 ───────────────────────────────────────────
            match &state.iching.read().status {
                TaskStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500",
                        span { class: "text-5xl", "☯️" }
                        p { class: "text-lg font-medium", "{t(locale, TK::StatusIdleHint)}" }
                    }
                },
                TaskStatus::Loading => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3",
                        div { class: "w-12 h-12 rounded-full border-4 border-violet-500/30 border-t-violet-400 animate-spin" }
                        p { class: "text-violet-400 font-medium animate-pulse", "{t(locale, TK::StatusLoadingIChing)}" }
                    }
                },
                TaskStatus::Error(e) => rsx! {
                    div { class: "p-4 rounded-xl bg-red-900/20 border border-red-800/50 text-red-400", "{t(locale, TK::StatusError)}: {e}" }
                },
                TaskStatus::Success => {
                    if let Some(iching_output) = &state.iching.read().data {
                        let res = &iching_output.result;

                        // 현재 활성화된(호버 또는 클릭된) 효 정보 가져오기
                        let active_target = hovered_yao.read().or(*selected_yao.read()).unwrap_or((true, res.yuan_dang_yao));
                        let (is_pre_natal, active_line_idx) = active_target;

                        // 괘 정보 구하기
                        let hexagram_idx = if is_pre_natal { res.pre_natal_hexagram } else { res.post_natal_hexagram };
                        let hex_info = get_hexagram_info(hexagram_idx);

                        // 괘 이름 다국어 매핑
                        let hex_name = match locale {
                            Locale::Ko => hex_info.name,
                            Locale::En => hex_info.name_en,
                            Locale::Zh => hex_info.name_zh,
                            Locale::Ru => hex_info.name_ru,
                        };

                        let hex_desc = match locale {
                            Locale::Ko => hex_info.desc_ko,
                            Locale::En => hex_info.desc_en,
                            Locale::Zh => hex_info.desc_zh,
                            Locale::Ru => hex_info.desc_ru,
                        };

                        // 실제 음양 찾기
                        let mut final_is_yang = true;
                        let mut active_age_range = "".to_string();
                        if let Some(cycle) = res.lifetime_cycles.iter().find(|c| c.is_pre_natal == is_pre_natal && c.line_index == active_line_idx) {
                            final_is_yang = (cycle.end_age - cycle.start_age + 1) == 9;
                            active_age_range = format!("{} ~ {}{}", cycle.start_age, cycle.end_age, t(locale, TK::IChingAgeSuffix));
                        }

                        let is_active_yuan_dang = is_pre_natal && active_line_idx == res.yuan_dang_yao;
                        let yao_title = get_yao_name(active_line_idx, final_is_yang, locale);
                        let yao_desc = get_yao_description(active_line_idx, final_is_yang, is_active_yuan_dang, locale);

                        rsx! {
                            div { class: "grid grid-cols-1 lg:grid-cols-12 gap-6 items-start",
                                // ── 좌측 컬럼: 선천괘 / 후천괘 시각화 (lg:col-span-5) ──────────
                                div { class: "lg:col-span-5 space-y-6",
                                    // 1. 선천괘 카드
                                    div { class: "p-5 rounded-2xl bg-slate-900 border border-slate-800 space-y-4",
                                        div { class: "flex justify-between items-center border-b border-slate-800 pb-2",
                                            h3 { class: "font-bold text-violet-400 flex items-center gap-2",
                                                span { "🧬" }
                                                "{t(locale, TK::IChingPreNatal)}"
                                            }
                                            span { class: "text-xs bg-violet-500/10 text-violet-300 px-2 py-0.5 rounded-full border border-violet-500/20 font-semibold",
                                                "본괘 {res.pre_natal_hexagram}"
                                            }
                                        }
                                        
                                        // 선천괘 효 렌더링 (상효 6부터 초효 1까지 하향식으로 그림)
                                        div { class: "flex flex-col gap-3 py-2",
                                            {(1..=6).rev().map(|line_num| {
                                                let is_pre_natal = true;
                                                let mut line_is_yang = true;
                                                if let Some(c) = res.lifetime_cycles.iter().find(|c| c.is_pre_natal == is_pre_natal && c.line_index == line_num) {
                                                    line_is_yang = (c.end_age - c.start_age + 1) == 9;
                                                }

                                                let is_yd = line_num == res.yuan_dang_yao;
                                                let is_sel = *selected_yao.read() == Some((is_pre_natal, line_num));
                                                let is_hov = *hovered_yao.read() == Some((is_pre_natal, line_num));
                                                
                                                let state_class = if is_sel || is_hov {
                                                    "scale-[1.03] ring-2 ring-violet-500 shadow-lg shadow-violet-900/30"
                                                } else if is_yd {
                                                    "ring-1 ring-amber-500/50 shadow-[0_0_8px_rgba(251,191,36,0.3)]"
                                                } else {
                                                    "opacity-85 hover:opacity-100 hover:scale-[1.01]"
                                                };

                                                rsx! {
                                                    div { 
                                                        key: "pre-{line_num}",
                                                        class: "relative transition-all duration-300 cursor-pointer {state_class}",
                                                        onclick: move |_| selected_yao.set(Some((is_pre_natal, line_num))),
                                                        onmouseenter: move |_| hovered_yao.set(Some((is_pre_natal, line_num))),
                                                        onmouseleave: move |_| hovered_yao.set(None),
                                                        
                                                        {if line_is_yang {
                                                            rsx! {
                                                                // 양효: 속이 찬 꽉 찬 그라데이션 바
                                                                div { class: "h-6 rounded-md bg-gradient-to-r from-violet-600 via-indigo-500 to-violet-700 flex items-center justify-center text-[10px] text-white/50 font-bold" }
                                                            }
                                                        } else {
                                                            rsx! {
                                                                // 음효: 가운데가 나뉜 두 개의 그라데이션 바
                                                                div { class: "h-6 flex gap-4",
                                                                    div { class: "flex-1 h-full rounded-md bg-gradient-to-r from-slate-800 via-slate-700 to-slate-800" }
                                                                    div { class: "flex-1 h-full rounded-md bg-gradient-to-r from-slate-800 via-slate-700 to-slate-800" }
                                                                }
                                                            }
                                                        }}
                                                        // 원당효 뱃지 표시
                                                        if is_yd {
                                                            span { class: "absolute left-3 top-1/2 -translate-y-1/2 text-[9px] font-black text-yellow-400 bg-slate-950/80 px-1.5 py-0.5 rounded border border-yellow-500/30 tracking-wider",
                                                                "★ {t(locale, TK::IChingYuanDang)}"
                                                            }
                                                        }
                                                        span { class: "absolute right-3 top-1/2 -translate-y-1/2 text-[9px] font-bold text-slate-400",
                                                            "{line_num} 효"
                                                        }
                                                    }
                                                }
                                            })}
                                        }
                                    }

                                    // 2. 후천괘 카드
                                    div { class: "p-5 rounded-2xl bg-slate-900 border border-slate-800 space-y-4",
                                        div { class: "flex justify-between items-center border-b border-slate-800 pb-2",
                                            h3 { class: "font-bold text-indigo-400 flex items-center gap-2",
                                                span { "🌀" }
                                                "{t(locale, TK::IChingPostNatal)}"
                                            }
                                            span { class: "text-xs bg-indigo-500/10 text-indigo-300 px-2 py-0.5 rounded-full border border-indigo-500/20 font-semibold",
                                                "본괘 {res.post_natal_hexagram}"
                                            }
                                        }
                                        
                                        // 후천괘 효 렌더링 (상효 6부터 초효 1까지 하향식으로 그림)
                                        div { class: "flex flex-col gap-3 py-2",
                                            {(1..=6).rev().map(|line_num| {
                                                let is_pre_natal = false;
                                                let mut line_is_yang = true;
                                                if let Some(c) = res.lifetime_cycles.iter().find(|c| c.is_pre_natal == is_pre_natal && c.line_index == line_num) {
                                                    line_is_yang = (c.end_age - c.start_age + 1) == 9;
                                                }

                                                let is_sel = *selected_yao.read() == Some((is_pre_natal, line_num));
                                                let is_hov = *hovered_yao.read() == Some((is_pre_natal, line_num));
                                                
                                                let state_class = if is_sel || is_hov {
                                                    "scale-[1.03] ring-2 ring-indigo-500 shadow-lg shadow-indigo-900/30"
                                                } else {
                                                    "opacity-85 hover:opacity-100 hover:scale-[1.01]"
                                                };

                                                rsx! {
                                                    div { 
                                                        key: "post-{line_num}",
                                                        class: "relative transition-all duration-300 cursor-pointer {state_class}",
                                                        onclick: move |_| selected_yao.set(Some((is_pre_natal, line_num))),
                                                        onmouseenter: move |_| hovered_yao.set(Some((is_pre_natal, line_num))),
                                                        onmouseleave: move |_| hovered_yao.set(None),
                                                        
                                                        {if line_is_yang {
                                                            rsx! {
                                                                div { class: "h-6 rounded-md bg-gradient-to-r from-indigo-600 via-blue-500 to-indigo-700 flex items-center justify-center text-[10px] text-white/50 font-bold" }
                                                            }
                                                        } else {
                                                            rsx! {
                                                                div { class: "h-6 flex gap-4",
                                                                    div { class: "flex-1 h-full rounded-md bg-gradient-to-r from-slate-800 via-slate-700 to-slate-800" }
                                                                    div { class: "flex-1 h-full rounded-md bg-gradient-to-r from-slate-800 via-slate-700 to-slate-800" }
                                                                }
                                                            }
                                                        }}
                                                        span { class: "absolute right-3 top-1/2 -translate-y-1/2 text-[9px] font-bold text-slate-400",
                                                            "{line_num} 효"
                                                        }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }

                                // ── 우측 컬럼: 효 상세 해석 & 평생 대운 타임라인 (lg:col-span-7) ────────
                                div { class: "lg:col-span-7 space-y-6",
                                    // 1. 선택된 괘 & 효 상세 해석 카드
                                    div { class: "p-6 rounded-3xl bg-gradient-to-br from-slate-900 to-slate-850 border border-slate-800 shadow-2xl relative overflow-hidden space-y-6",
                                        div { class: "absolute -top-24 -right-24 w-48 h-48 bg-violet-600/10 rounded-full blur-3xl pointer-events-none" }
                                        
                                        // 괘 타이틀 정보
                                        div { class: "space-y-1",
                                            div { class: "flex items-center gap-2",
                                                span { class: "text-xs font-bold text-violet-400 uppercase tracking-wider",
                                                    if is_pre_natal { "先天卦 / Innate" } else { "後天卦 / Acquired" }
                                                }
                                                span { class: "text-slate-600", "|" }
                                                span { class: "text-xs font-bold text-slate-400",
                                                    "{active_line_idx} 효 선택됨"
                                                }
                                            }
                                            h4 { class: "text-3xl font-black text-slate-100 flex items-baseline gap-3",
                                                "{hex_name}"
                                                span { class: "text-lg font-bold text-violet-300/80", "({hex_info.hanja})" }
                                            }
                                            p { class: "text-slate-400 text-sm leading-relaxed mt-1",
                                                "{hex_desc}"
                                            }
                                        }

                                        // 효사 정보
                                        div { class: "p-5 rounded-2xl bg-slate-950/50 border border-slate-800/80 space-y-4",
                                            div { class: "flex justify-between items-center border-b border-slate-800 pb-2",
                                                span { class: "font-bold text-slate-200 text-lg flex items-center gap-2",
                                                    span { class: "text-violet-400", "✦" }
                                                    "{yao_title}"
                                                }
                                                span { class: "text-sm font-semibold bg-violet-500/10 text-violet-400 px-2 py-0.5 rounded border border-violet-500/20",
                                                    "{active_age_range}"
                                                }
                                            }
                                            p { class: "text-slate-300 text-sm leading-relaxed whitespace-pre-line",
                                                "{yao_desc}"
                                            }
                                        }
                                        
                                        p { class: "text-[11px] text-slate-500 text-center italic",
                                            "{t(locale, TK::IChingSelectYaoHint)}"
                                        }
                                    }

                                    // 2. 평생 대운 타임라인 리스트
                                    div { class: "p-6 rounded-3xl bg-slate-900 border border-slate-800 space-y-4",
                                        h3 { class: "text-lg font-bold text-slate-200 flex items-center gap-2 border-b border-slate-800 pb-3",
                                            span { "📅" }
                                            "{t(locale, TK::IChingTimelineTitle)}"
                                        }
                                        
                                        div { class: "space-y-3 max-h-[360px] overflow-y-auto pr-1",
                                            {res.lifetime_cycles.iter().map(|cycle| {
                                                let is_active_cycle = current_age >= cycle.start_age && current_age <= cycle.end_age;
                                                let c_hex = get_hexagram_info(cycle.hexagram_index);
                                                let c_name = match locale {
                                                    Locale::Ko => c_hex.name,
                                                    Locale::En => c_hex.name_en,
                                                    Locale::Zh => c_hex.name_zh,
                                                    Locale::Ru => c_hex.name_ru,
                                                };
                                                
                                                let cycle_is_yang = (cycle.end_age - cycle.start_age + 1) == 9;
                                                let line_name = get_yao_name(cycle.line_index, cycle_is_yang, locale);
                                                let cycle_is_pre = cycle.is_pre_natal;
                                                let cycle_line = cycle.line_index;

                                                let active_cls = if is_active_cycle {
                                                    "border-violet-500/50 bg-violet-950/20 shadow-[0_0_15px_rgba(139,92,246,0.15)] ring-1 ring-violet-500/30 scale-[1.01]"
                                                } else {
                                                    "border-slate-800 hover:border-slate-700/60 bg-slate-950/30 hover:bg-slate-950/50"
                                                };

                                                rsx! {
                                                    div { 
                                                        key: "{cycle.start_age}-{cycle.is_pre_natal}",
                                                        class: "p-3.5 rounded-xl border flex items-center justify-between gap-4 transition-all duration-200 cursor-pointer {active_cls}",
                                                        onclick: move |_| {
                                                            selected_yao.set(Some((cycle_is_pre, cycle_line)));
                                                        },
                                                        div { class: "flex items-center gap-3",
                                                            // 나이 대 뱃지
                                                            div { class: "w-20 py-1.5 rounded-lg bg-slate-950/80 border border-slate-800 text-center font-black text-sm text-slate-200",
                                                                "{cycle.start_age} ~ {cycle.end_age}"
                                                            }
                                                            // 대성괘 이름 및 효 정보
                                                            div { class: "space-y-0.5",
                                                                div { class: "flex items-center gap-1.5",
                                                                    span { class: "font-bold text-slate-100 text-sm", "{c_name}" }
                                                                    span { class: "text-[10px] text-slate-500", "({c_hex.hanja})" }
                                                                }
                                                                div { class: "flex items-center gap-2",
                                                                    span { class: "text-xs text-slate-400", "{line_name}" }
                                                                    span { class: "text-[10px] px-1 py-0.2 bg-slate-800 text-slate-500 rounded font-medium",
                                                                        if cycle.is_pre_natal { "선천" } else { "후천" }
                                                                    }
                                                                }
                                                            }
                                                        },

                                                        // 현재 활동 유무
                                                        div { class: "flex items-center gap-2",
                                                            if is_active_cycle {
                                                                span { class: "text-[10px] font-black bg-gradient-to-r from-violet-500 to-indigo-500 text-white px-2 py-0.5 rounded-full border border-violet-400/20 animate-pulse tracking-wide",
                                                                    "현재 주기"
                                                                }
                                                            }
                                                            span { class: "text-slate-600 text-sm", "›" }
                                                        }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {
                            div { class: "p-4 rounded-xl bg-red-900/20 border border-red-800/50 text-red-400",
                                "데이터가 비어 있습니다."
                            }
                        }
                    }
                }
            }
        }
    }
}
