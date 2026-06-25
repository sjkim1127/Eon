// crates/eon-ui/src/components/tabs/iching_tab.rs
use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use crate::i18n::{t, TK, Locale};
use crate::i18n::iching_db::{get_hexagram_info, get_yao_name, get_yao_description};
use eon_service::facade;
use eon_service::dto::{AnalysisInput, SajuAnalysisInput};
use crate::components::shared::birth_form::BirthForm;
use eon_saju::core::element::ElementRelation;

#[component]
pub fn IChingTab() -> Element {
    let mut state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    // 로컬 상호작용 상태 (선택된 효와 마우스 호버 중인 효)
    let mut selected_yao = use_signal(|| None::<(bool, u8)>);
    let mut hovered_yao = use_signal(|| None::<(bool, u8)>);
    
    // 유년괘 선택용 나이 상태 (기본값 30세)
    let mut selected_age = use_signal(|| 30u32);

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
                    if let Some(iching_data) = &state.iching.read().data {
                        let yd = iching_data.result.yuan_dang_yao;
                        selected_yao.set(Some((true, yd)));
                        
                        // 현재 만나이 계산하여 초기 나이 설정
                        let form_year = state.form.read().year;
                        let current_age = (2026 - form_year).max(1) as u32;
                        selected_age.set(current_age.min(100));
                    }
                }
                Err(e) => {
                    state.iching.write().error = Some(e.to_string());
                    state.iching.write().status = TaskStatus::Error(e.to_string());
                }
            }
        });
    };

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

                        // 납갑 정보 및 신살 가져오기
                        let active_najia = &res.najia_lines[active_line_idx as usize - 1];
                        let active_shinsal = &res.shinsal_lines[active_line_idx as usize - 1];

                        // 체용 관계 해설 텍스트 생성
                        let ti_yong_rel_text = match res.ti_yong.relationship {
                            ElementRelation::Same => "체(體)와 용(用)이 비화(比和)하여 내외의 역량이 조화를 이루고, 하는 일마다 막힘없이 순조롭게 번창하는 격국입니다. (吉)",
                            ElementRelation::Generates => "내(體)가 환경(用)을 도와주거나 설기(洩氣)하니, 겉으로는 힘을 쓰나 내실이 부족할 수 있어 기력 낭비를 조심해야 합니다. (平)",
                            ElementRelation::GeneratedBy => "환경(用)이 나(體)를 상생(相生)하여 든든히 뒷받침해주니, 생각지 못한 은인과 귀인의 원조를 통해 크게 성취하는 격국입니다. (大吉)",
                            ElementRelation::Controls => "내(體)가 환경(用)을 극(剋)하여 주도권을 잡으니, 장애를 극복하고 스스로의 힘으로 성취를 쟁취하며 재물을 성실히 축적합니다. (吉)",
                            ElementRelation::ControlledBy => "환경(用)이 나(體)를 극(剋)해와 사방의 억압이 심하니, 주위의 고난과 방해가 잦아 인내와 각별한 신중함이 요구되는 격국입니다. (凶)",
                        };

                        // 현재 선택된 나이에 따른 유년괘 추출
                        let current_yh = res.yearly_hexagrams.iter().find(|yh| yh.age == *selected_age.read());

                        rsx! {
                            div { class: "space-y-6",
                                // ── 1. 상단 원기 / 화공 격국 요약 카드 ───────────────────
                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                    // 원기 격국
                                    div { class: "p-5 rounded-2xl bg-gradient-to-br from-slate-900 to-slate-950 border border-slate-800 flex items-center justify-between shadow-lg",
                                        div { class: "space-y-1.5",
                                            span { class: "text-xs font-bold text-violet-400 tracking-wider uppercase", "{t(locale, TK::IChingYuanQi)}" }
                                            h4 { class: "text-xl font-black text-slate-100 flex items-center gap-2",
                                                if res.yuan_qi { 
                                                    span { class: "text-emerald-400", "得元氣 (득원기)" }
                                                } else { 
                                                    span { class: "text-slate-400", "失元氣 (실원기)" }
                                                }
                                            }
                                            p { class: "text-xs text-slate-400 leading-relaxed",
                                                if res.yuan_qi {
                                                    "출생 연도의 납음(納音) 오행과 평생 본명괘의 오행이 상생 또는 비화되어, 하늘의 원천적인 기운과 큰 지지를 얻었습니다."
                                                } else {
                                                    "원천적 상생 기운은 다소 아쉬우나, 후천적 노력을 통해 자신의 고유한 운명을 개척해나갈 수 있습니다."
                                                }
                                            }
                                        }
                                        div { class: "text-3xl p-3 rounded-xl bg-slate-950/60 border border-slate-800",
                                            if res.yuan_qi { "🌟" } else { "⚖️" }
                                        }
                                    }

                                    // 화공 격국
                                    div { class: "p-5 rounded-2xl bg-gradient-to-br from-slate-900 to-slate-950 border border-slate-800 flex items-center justify-between shadow-lg",
                                        div { class: "space-y-1.5",
                                            span { class: "text-xs font-bold text-indigo-400 tracking-wider uppercase", "{t(locale, TK::IChingHuaGong)}" }
                                            h4 { class: "text-xl font-black text-slate-100 flex items-center gap-2",
                                                if res.hua_gong { 
                                                    span { class: "text-emerald-400", "得化工 (득화공)" }
                                                } else { 
                                                    span { class: "text-slate-400", "失化工 (실화공)" }
                                                }
                                            }
                                            p { class: "text-xs text-slate-400 leading-relaxed",
                                                if res.hua_gong {
                                                    "태어난 계절의 기운(월지)에 맞춰 본명괘가 힘과 지지(旺/相)를 얻어, 계절의 조화와 환경의 귀한 기회를 득했습니다."
                                                } else {
                                                    "계절의 조화는 득하지 못했으나, 주변 인덕과 환경의 변화를 슬기롭게 포착하여 발전할 수 있습니다."
                                                }
                                            }
                                        }
                                        div { class: "text-3xl p-3 rounded-xl bg-slate-950/60 border border-slate-800",
                                            if res.hua_gong { "🔥" } else { "💧" }
                                        }
                                    }
                                }

                                // ── 2. 메인 본명괘 분석 섹션 ────────────────────────────
                                div { class: "grid grid-cols-1 lg:grid-cols-12 gap-6 items-start",
                                    // ── 좌측 컬럼: 선천괘 / 후천괘 시각화 (lg:col-span-5) ──────────
                                    div { class: "lg:col-span-5 space-y-6",
                                        // 선천괘 카드
                                        div { class: "p-5 rounded-2xl bg-slate-900 border border-slate-800 space-y-4 shadow-xl",
                                            div { class: "flex justify-between items-center border-b border-slate-800 pb-2",
                                                h3 { class: "font-bold text-violet-400 flex items-center gap-2",
                                                    span { "🧬" }
                                                    "{t(locale, TK::IChingPreNatal)}"
                                                }
                                                span { class: "text-xs bg-violet-500/10 text-violet-300 px-2 py-0.5 rounded-full border border-violet-500/20 font-semibold",
                                                    "본괘 {res.pre_natal_hexagram}"
                                                }
                                            }
                                            
                                            // 선천괘 효 렌더링
                                            div { class: "flex flex-col gap-3 py-2",
                                                {(1..=6).rev().map(|line_num| {
                                                    let is_pre_natal = true;
                                                    let mut line_is_yang = true;
                                                    if let Some(c) = res.lifetime_cycles.iter().find(|c| c.is_pre_natal == is_pre_natal && c.line_index == line_num) {
                                                        line_is_yang = (c.end_age - c.start_age + 1) == 9;
                                                    }

                                                    let is_yd = line_num == res.yuan_dang_yao;
                                                    let is_se = line_num == res.se_yao;
                                                    let is_ying = line_num == res.ying_yao;
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
                                                                    div { class: "h-6 rounded-md bg-gradient-to-r from-violet-600 via-indigo-500 to-violet-700 flex items-center justify-center text-[10px] text-white/50 font-bold" }
                                                                }
                                                            } else {
                                                                rsx! {
                                                                    div { class: "h-6 flex gap-4",
                                                                        div { class: "flex-1 h-full rounded-md bg-gradient-to-r from-slate-800 via-slate-700 to-slate-800" }
                                                                        div { class: "flex-1 h-full rounded-md bg-gradient-to-r from-slate-800 via-slate-700 to-slate-800" }
                                                                    }
                                                                }
                                                            }}
                                                            // 원당효 배지
                                                            if is_yd {
                                                                span { class: "absolute left-3 top-1/2 -translate-y-1/2 text-[9px] font-black text-yellow-450 bg-slate-955/80 px-1.5 py-0.5 rounded border border-yellow-500/30 tracking-wider",
                                                                    "★ {t(locale, TK::IChingYuanDang)}"
                                                                }
                                                            }
                                                            // 세효 / 응효 뱃지
                                                            if is_se {
                                                                span { class: "absolute left-20 top-1/2 -translate-y-1/2 text-[9px] font-black text-emerald-450 bg-slate-955/80 px-1.5 py-0.5 rounded border border-emerald-500/30 tracking-wider",
                                                                    "世"
                                                                }
                                                            } else if is_ying {
                                                                span { class: "absolute left-20 top-1/2 -translate-y-1/2 text-[9px] font-black text-rose-400 bg-slate-955/80 px-1.5 py-0.5 rounded border border-rose-500/30 tracking-wider",
                                                                    "應"
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

                                        // 후천괘 카드
                                        div { class: "p-5 rounded-2xl bg-slate-900 border border-slate-800 space-y-4 shadow-xl",
                                            div { class: "flex justify-between items-center border-b border-slate-800 pb-2",
                                                h3 { class: "font-bold text-indigo-400 flex items-center gap-2",
                                                    span { "🌀" }
                                                    "{t(locale, TK::IChingPostNatal)}"
                                                }
                                                span { class: "text-xs bg-indigo-500/10 text-indigo-300 px-2 py-0.5 rounded-full border border-indigo-500/20 font-semibold",
                                                    "본괘 {res.post_natal_hexagram}"
                                                }
                                            }
                                            
                                            // 후천괘 효 렌더링
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
                                                    span { class: "text-xs font-bold text-slate-450",
                                                        "{active_line_idx} 효 선택됨"
                                                    }
                                                    if is_active_yuan_dang {
                                                        span { class: "px-1.5 py-0.5 text-[9px] font-black text-amber-450 bg-amber-950/50 rounded border border-amber-500/20",
                                                            "원당효"
                                                        }
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

                                            // 체용(體用) 생극제화 카드 추가
                                            div { class: "p-4 rounded-xl bg-slate-950/40 border border-slate-800/80 space-y-2",
                                                h5 { class: "text-xs font-black text-slate-300 tracking-wide flex items-center gap-1.5",
                                                    span { "⚖️" }
                                                    "체용(體用) 분석"
                                                }
                                                div { class: "flex gap-4 text-xs font-bold text-slate-200 py-1 border-b border-slate-800/60",
                                                    span { "체(體): {res.ti_yong.ti_element.hangul()}({res.ti_yong.ti_element.hanja()})" }
                                                    span { "용(用): {res.ti_yong.yong_element.hangul()}({res.ti_yong.yong_element.hanja()})" }
                                                }
                                                p { class: "text-xs text-slate-400 leading-relaxed", "{ti_yong_rel_text}" }
                                            }

                                            // 효사 및 납갑/신살 정보
                                            div { class: "p-5 rounded-2xl bg-slate-950/50 border border-slate-800/80 space-y-4",
                                                div { class: "flex justify-between items-center border-b border-slate-800 pb-2",
                                                    span { class: "font-bold text-slate-200 text-lg flex items-center gap-2",
                                                        span { "✦" }
                                                        "{yao_title}"
                                                    }
                                                    span { class: "text-sm font-semibold bg-violet-500/10 text-violet-400 px-2 py-0.5 rounded border border-violet-500/20",
                                                        "{active_age_range}"
                                                    }
                                                }
                                                
                                                // 납갑 & 신살 정보 바 렌더링
                                                div { class: "flex flex-wrap gap-2 text-[10px] pb-2 border-b border-slate-800/50",
                                                    span { class: "px-2 py-0.5 rounded bg-slate-850 text-slate-300 font-bold",
                                                        "납갑: {active_najia.stem.hanja()}{active_najia.branch.hanja()} ({active_najia.stem.hangul()}{active_najia.branch.hangul()})"
                                                    }
                                                    if active_shinsal.is_noble {
                                                        span { class: "px-2 py-0.5 rounded bg-emerald-950/50 text-emerald-450 border border-emerald-900/60 font-bold",
                                                            "✨ 귀인"
                                                        }
                                                    }
                                                    if active_shinsal.is_void {
                                                        span { class: "px-2 py-0.5 rounded bg-rose-950/50 text-rose-450 border border-rose-900/60 font-bold",
                                                            "🕳️ 공망"
                                                        }
                                                    }
                                                    if active_shinsal.is_rok {
                                                        span { class: "px-2 py-0.5 rounded bg-amber-950/50 text-amber-450 border border-amber-900/60 font-bold",
                                                            "💰 록"
                                                        }
                                                    }
                                                    if active_shinsal.is_horse {
                                                        span { class: "px-2 py-0.5 rounded bg-blue-950/50 text-blue-450 border border-blue-900/60 font-bold",
                                                            "🐎 역마"
                                                        }
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
                                        div { class: "p-6 rounded-3xl bg-slate-900 border border-slate-800 space-y-4 shadow-xl",
                                            h3 { class: "text-lg font-bold text-slate-200 flex items-center gap-2 border-b border-slate-800 pb-3",
                                                span { "📅" }
                                                "{t(locale, TK::IChingTimelineTitle)}"
                                            }
                                            
                                            div { class: "space-y-3 max-h-[300px] overflow-y-auto pr-1",
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
                                                                div { class: "w-20 py-1.5 rounded-lg bg-slate-950/80 border border-slate-800 text-center font-black text-sm text-slate-200",
                                                                    "{cycle.start_age} ~ {cycle.end_age}"
                                                                }
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

                                // ── 3. 하단 인터랙티브 유년괘 / 유월괘 연동 위젯 ─────────────────────────
                                {if let Some(yh) = current_yh {
                                    let yh_hex = get_hexagram_info(yh.hexagram_index);
                                    let yh_name = match locale {
                                        Locale::Ko => yh_hex.name,
                                        Locale::En => yh_hex.name_en,
                                        Locale::Zh => yh_hex.name_zh,
                                        Locale::Ru => yh_hex.name_ru,
                                    };
                                    
                                    let yh_upper_trigram = get_trigram_lines(get_trigram_num_from_hex(yh.hexagram_index, true));
                                    let yh_lower_trigram = get_trigram_lines(get_trigram_num_from_hex(yh.hexagram_index, false));
                                    let mut yh_lines = [false; 6];
                                    yh_lines[0] = yh_lower_trigram[0];
                                    yh_lines[1] = yh_lower_trigram[1];
                                    yh_lines[2] = yh_lower_trigram[2];
                                    yh_lines[3] = yh_upper_trigram[0];
                                    yh_lines[4] = yh_upper_trigram[1];
                                    yh_lines[5] = yh_upper_trigram[2];

                                    rsx! {
                                        div { class: "p-6 rounded-3xl bg-slate-900 border border-slate-800 space-y-6 shadow-xl",
                                            div { class: "border-b border-slate-855 pb-4 space-y-2",
                                                h3 { class: "text-lg font-bold text-slate-200 flex items-center gap-2",
                                                    span { "🌀" }
                                                    "유년괘(流年卦) 세운 조회"
                                                }
                                                p { class: "text-xs text-slate-400 leading-relaxed",
                                                    "원당효부터 경과한 세수에 따라 매년 괘상의 1개 효가 동(動)하여 해당 세운의 유년괘를 형성합니다. 슬라이더를 통해 만나이별 세운의 변화를 조회해보세요."
                                                }
                                            }

                                            // 나이 선택 슬라이더 위젯
                                            div { class: "p-4 rounded-2xl bg-slate-955/40 border border-slate-800/80 flex flex-col md:flex-row items-center gap-4 justify-between",
                                                div { class: "flex items-center gap-2 w-full md:w-auto justify-between md:justify-start",
                                                    span { class: "text-sm text-slate-400 font-medium", "연도 조절" }
                                                    div { class: "flex items-center gap-1.5",
                                                        button {
                                                            class: "w-8 h-8 rounded-lg bg-slate-800 hover:bg-slate-750 border border-slate-700/60 font-bold text-slate-200 cursor-pointer flex items-center justify-center active:scale-90 transition-transform",
                                                            onclick: move |_| {
                                                                let current = *selected_age.read();
                                                                if current > 1 { selected_age.set(current - 1); }
                                                            },
                                                            "−"
                                                        }
                                                        span { class: "px-4 py-1.5 rounded-xl bg-slate-900 border border-slate-850 font-black text-lg text-violet-400 min-w-[70px] text-center",
                                                            "{selected_age} 세"
                                                        }
                                                        button {
                                                            class: "w-8 h-8 rounded-lg bg-slate-800 hover:bg-slate-750 border border-slate-700/60 font-bold text-slate-200 cursor-pointer flex items-center justify-center active:scale-90 transition-transform",
                                                            onclick: move |_| {
                                                                let current = *selected_age.read();
                                                                if current < 100 { selected_age.set(current + 1); }
                                                            },
                                                            "+"
                                                        }
                                                    }
                                                }
                                                
                                                // 슬라이더 바
                                                input {
                                                    type: "range",
                                                    min: "1",
                                                    max: "100",
                                                    value: "{selected_age}",
                                                    class: "w-full md:flex-1 h-1.5 rounded-lg bg-slate-800 appearance-none cursor-pointer accent-violet-500",
                                                    oninput: move |e| {
                                                        if let Ok(v) = e.value().parse::<u32>() {
                                                            selected_age.set(v);
                                                        }
                                                    }
                                                }
                                            }

                                            // 유년괘 / 유월괘 세부 카드
                                            div { class: "grid grid-cols-1 lg:grid-cols-12 gap-6 items-stretch",
                                                // 유년괘 카드 시각화 (lg:col-span-5)
                                                div { class: "lg:col-span-5 p-5 rounded-2xl bg-slate-955/60 border border-slate-850 flex flex-col md:flex-row gap-5 items-center justify-center",
                                                    // 유년괘 괘상 렌더링
                                                    div { class: "flex flex-col gap-1.5 w-32",
                                                        {(1..=6).rev().map(|line_num| {
                                                            let line_is_yang = yh_lines[line_num as usize - 1];
                                                            let is_yearly_change_line = line_num == yh.yearly_line;
                                                            let highlight_cls = if is_yearly_change_line {
                                                                "ring-1 ring-amber-500 shadow-[0_0_8px_rgba(251,191,36,0.2)] animate-pulse"
                                                            } else {
                                                                ""
                                                            };
                                                            rsx! {
                                                                div {
                                                                    key: "yh-{line_num}",
                                                                    class: "relative transition-all duration-300 {highlight_cls}",
                                                                    {if line_is_yang {
                                                                        rsx! {
                                                                            div { class: "h-3.5 rounded bg-gradient-to-r from-indigo-500 via-blue-400 to-indigo-600" }
                                                                        }
                                                                    } else {
                                                                        rsx! {
                                                                            div { class: "h-3.5 flex gap-2.5",
                                                                                div { class: "flex-1 h-full rounded bg-gradient-to-r from-slate-800 to-slate-700" }
                                                                                div { class: "flex-1 h-full rounded bg-gradient-to-r from-slate-800 to-slate-700" }
                                                                            }
                                                                        }
                                                                    }}
                                                                    if is_yearly_change_line {
                                                                        span { class: "absolute left-1/2 -translate-x-1/2 -translate-y-1/2 top-1/2 text-[8px] font-black text-yellow-300 bg-slate-900/90 px-1 py-0.2 rounded border border-yellow-500/20 tracking-wider scale-90",
                                                                            "동(動)"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        })}
                                                    }
                                                    
                                                    // 유년괘 정보 출력
                                                    div { class: "space-y-1.5 text-center md:text-left flex-1",
                                                        div { class: "flex items-center justify-center md:justify-start gap-1 text-[10px] font-bold text-violet-400",
                                                            span { "🎯" }
                                                            "{selected_age}세 유년괘"
                                                        }
                                                        h4 { class: "text-2xl font-black text-slate-100 flex items-baseline justify-center md:justify-start gap-2",
                                                            "{yh_name}"
                                                            span { class: "text-sm text-slate-500 font-medium", "({yh_hex.hanja})" }
                                                        }
                                                        p { class: "text-xs text-slate-400 leading-relaxed", "{yh_hex.desc_ko}" }
                                                        div { class: "text-[10px] text-yellow-450 font-bold bg-yellow-950/20 border border-yellow-900/30 px-2.5 py-1 rounded-lg inline-block",
                                                            "올해는 {yh.yearly_line}효가 동(動)하여 대변동을 주도합니다."
                                                        }
                                                    }
                                                }

                                                // 유월괘 카드 리스트 (lg:col-span-7)
                                                div { class: "lg:col-span-7 p-5 rounded-2xl bg-slate-955/60 border border-slate-850 flex flex-col justify-between gap-4",
                                                    div { class: "flex justify-between items-center border-b border-slate-855 pb-2",
                                                        h4 { class: "text-xs font-bold text-slate-300 tracking-wide flex items-center gap-1.5",
                                                            span { "🌙" }
                                                            "음력 12개월 유월괘(流月卦) 흐름"
                                                        }
                                                        span { class: "text-[10px] text-slate-500", "정월(1월) = 유년괘 변효 {yh.yearly_line}효 기점" }
                                                    }
                                                    
                                                    // 12개월의 유월괘 그리드 렌더링
                                                    div { class: "grid grid-cols-4 sm:grid-cols-6 gap-3",
                                                        {yh.monthly_hexagrams.iter().enumerate().map(|(idx, &m_hex_idx)| {
                                                            let m_hex = get_hexagram_info(m_hex_idx);
                                                            let m_name = match locale {
                                                                Locale::Ko => m_hex.name,
                                                                Locale::En => m_hex.name_en,
                                                                Locale::Zh => m_hex.name_zh,
                                                                Locale::Ru => m_hex.name_ru,
                                                            };
                                                            rsx! {
                                                                div { 
                                                                    key: "m-{idx}",
                                                                    class: "p-2 rounded-lg bg-slate-900 border border-slate-850 text-center hover:border-violet-500/20 hover:bg-slate-850/50 transition-colors flex flex-col items-center justify-center gap-1",
                                                                    span { class: "text-[10px] font-black text-slate-500", "{idx + 1}월" }
                                                                    span { class: "font-black text-slate-200 text-xs tracking-tight", "{m_name}" }
                                                                    span { class: "text-[9px] text-slate-500 font-medium", "{m_hex.hanja}" }
                                                                }
                                                            }
                                                        })}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else { rsx! {} }}
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

// 64괘 인덱스로부터 상괘/하괘 번호(LuoShu 1..9)를 얻는 헬퍼 함수
fn get_trigram_num_from_hex(hexagram_index: u8, is_upper: bool) -> u8 {
    let (upper, lower) = match hexagram_index {
        1 => (6, 6),  // 乾
        2 => (2, 2),  // 坤
        3 => (1, 3),  // 屯
        4 => (8, 1),  // 蒙
        5 => (6, 1),  // 需
        6 => (1, 6),  // 訟
        7 => (2, 1),  // 師
        8 => (1, 2),  // 比
        9 => (4, 6),  // 小畜
        10 => (6, 7), // 履
        11 => (2, 6), // 泰
        12 => (6, 2), // 否
        13 => (6, 9), // 同人
        14 => (9, 6), // 大有
        15 => (2, 8), // 謙
        16 => (3, 2), // 豫
        17 => (7, 3), // 隨
        18 => (8, 4), // 蠱
        19 => (2, 7), // 臨
        20 => (4, 2), // 觀
        21 => (9, 3), // 噬嗑
        22 => (8, 9), // 賁
        23 => (8, 2), // 剝
        24 => (2, 3), // 復
        25 => (6, 3), // 無妄
        26 => (8, 6), // 大畜
        27 => (8, 3), // 頤
        28 => (7, 4), // 大過
        29 => (1, 1), // 坎
        30 => (9, 9), // 離
        31 => (7, 8), // 咸
        32 => (3, 4), // 恆
        33 => (6, 8), // 遯
        34 => (3, 6), // 大壯
        35 => (9, 2), // 晉
        36 => (2, 9), // 明夷
        37 => (4, 9), // 家人
        38 => (9, 7), // 睽
        39 => (1, 8), // 蹇
        40 => (3, 1), // 解
        41 => (8, 7), // 損
        42 => (4, 3), // 益
        43 => (7, 6), // 夬
        44 => (6, 4), // 姤
        45 => (7, 2), // 萃
        46 => (2, 4), // 升
        47 => (7, 1), // 困
        48 => (1, 4), // 井
        49 => (7, 9), // 革
        50 => (9, 4), // 鼎
        51 => (3, 3), // 震
        52 => (8, 8), // 艮
        53 => (4, 8), // 漸
        54 => (3, 7), // 歸妹
        55 => (3, 9), // 豐
        56 => (9, 8), // 旅
        57 => (4, 4), // 巽
        58 => (7, 7), // 兌
        59 => (4, 1), // 渙
        60 => (1, 7), // 節
        61 => (4, 7), // 中孚
        62 => (3, 8), // 小過
        63 => (1, 9), // 既濟
        64 => (9, 1), // 未濟
        _ => (6, 6),
    };
    if is_upper { upper } else { lower }
}

fn get_trigram_lines(num: u8) -> [bool; 3] {
    match num {
        1 => [false, true, false],  // 坎 ☵
        2 => [false, false, false], // 坤 ☷
        3 => [true, false, false],  // 震 ☳
        4 => [false, true, true],   // 巽 ☴
        6 => [true, true, true],    // 乾 ☰
        7 => [true, true, false],   // 兌 ☱
        8 => [false, false, true],  // 艮 ☶
        9 => [true, false, true],   // 離 ☲
        _ => [false, false, false],
    }
}
