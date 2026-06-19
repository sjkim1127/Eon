use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use eon_service::dto::{SajuAnalysisInput, AnalysisInput};
use eon_service::facade;
use eon_saju::analysis::strength::StrengthType;
use crate::components::shared::birth_form::BirthForm;

#[component]
pub fn SajuTab() -> Element {
    let mut state = use_context::<AnalysisState>();

    let run_analysis = move |_| {
        spawn(async move {
            state.saju.write().status = TaskStatus::Loading;
            let form = state.form.read().clone();
            let input = SajuAnalysisInput::new(
                AnalysisInput {
                    year: form.year, month: form.month, day: form.day,
                    hour: form.hour, minute: form.minute,
                    is_lunar: form.is_lunar, is_leap_month: form.is_leap_month,
                    lat: form.lat, lon: form.lon,
                    timezone: "Asia/Seoul".to_string(),
                },
                form.is_male, false, Some(false),
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
    };

    rsx! {
        div { class: "space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-700",
            BirthForm {}

            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-amber-200 to-orange-400 bg-clip-text text-transparent",
                    "사주 명식 (四柱 命式)"
                }
                button {
                    class: "px-5 py-2.5 bg-gradient-to-r from-amber-600 to-orange-600 hover:from-amber-500 hover:to-orange-500 rounded-xl font-semibold text-white shadow-lg shadow-amber-900/30 transition-all duration-200 active:scale-95",
                    onclick: run_analysis,
                    "🔮 분석 실행"
                }
            }

            match &state.saju.read().status {
                TaskStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500",
                        span { class: "text-5xl", "🌌" }
                        p { class: "text-lg font-medium", "출생 정보를 입력하고 분석을 실행하세요." }
                    }
                },
                TaskStatus::Loading => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3",
                        div { class: "w-12 h-12 rounded-full border-4 border-amber-500/30 border-t-amber-400 animate-spin" }
                        p { class: "text-amber-400 font-medium animate-pulse", "사주 계산 중..." }
                    }
                },
                TaskStatus::Error(e) => rsx! {
                    div { class: "p-4 rounded-xl bg-red-900/20 border border-red-800/50 text-red-400", "오류: {e}" }
                },
                TaskStatus::Success => {
                    if let Some(data) = &state.saju.read().data {
                        rsx! {
                            // ── 1. 사주 원국 ──────────────────────────────────
                            div { class: "grid grid-cols-4 gap-3",
                                PillarCard { title: "시주 (Hour)", hanja: data.report.pillars.hour.hanja(), hangul: data.report.pillars.hour.hangul(), color: "from-blue-900/60 to-slate-900/60 border-blue-700/50" }
                                PillarCard { title: "일주 (Day)",  hanja: data.report.pillars.day.hanja(),  hangul: data.report.pillars.day.hangul(),  color: "from-violet-900/60 to-slate-900/60 border-violet-700/50" }
                                PillarCard { title: "월주 (Month)",hanja: data.report.pillars.month.hanja(),hangul: data.report.pillars.month.hangul(),color: "from-emerald-900/60 to-slate-900/60 border-emerald-700/50" }
                                PillarCard { title: "연주 (Year)", hanja: data.report.pillars.year.hanja(), hangul: data.report.pillars.year.hangul(), color: "from-amber-900/60 to-slate-900/60 border-amber-700/50" }
                            }

                            // ── 2. 신강/신약 + 메타 ────────────────────────────
                            div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                                // 신강신약 배지
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 flex flex-col gap-3",
                                    h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "신강/신약" }
                                    div { class: "flex items-center gap-3",
                                        {
                                            let st = data.report.strength.strength_type;
                                            let (badge_color, icon) = match st {
                                                StrengthType::Strong =>
                                                    ("bg-red-500/20 text-red-300 border-red-500/50", "🔥"),
                                                StrengthType::Weak =>
                                                    ("bg-blue-500/20 text-blue-300 border-blue-500/50", "💧"),
                                                StrengthType::Balanced =>
                                                    ("bg-emerald-500/20 text-emerald-300 border-emerald-500/50", "⚖️"),
                                            };
                                            rsx! {
                                                span { class: "text-3xl", "{icon}" }
                                                div {
                                                    div { class: "inline-block px-3 py-1 rounded-full border text-sm font-bold {badge_color}",
                                                        "{st.hangul()} ({st.hanja()})"
                                                    }
                                                    p { class: "text-xs text-slate-500 mt-1",
                                                        "조건 {data.report.strength.acquired_count}/4 충족 | 점수 {data.report.strength.strength_score:.1}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    // 득령/득지/득시/득세
                                    div { class: "grid grid-cols-2 gap-1.5 text-xs",
                                        DeukBadge { label: "득령", acquired: data.report.strength.deuk_ryeong.acquired }
                                        DeukBadge { label: "득지", acquired: data.report.strength.deuk_ji.acquired }
                                        DeukBadge { label: "득시", acquired: data.report.strength.deuk_si.acquired }
                                        DeukBadge { label: "득세", acquired: data.report.strength.deuk_se.acquired }
                                    }
                                }

                                // 용신
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 flex flex-col gap-3",
                                    h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "용신 (用神)" }
                                    {
                                        let yn = &data.report.yongshin;
                                        let primary_el = yn.primary;
                                        let assist_el = yn.assistant;
                                        let (p_color, p_icon) = element_style(primary_el.hangul());
                                        let (a_color, _) = element_style(assist_el.hangul());
                                        rsx! {
                                            div { class: "space-y-2",
                                                div { class: "flex items-center gap-2",
                                                    span { class: "text-lg", "{p_icon}" }
                                                    div { class: "flex-1",
                                                        p { class: "text-xs text-slate-400", "제1용신" }
                                                        p { class: "font-bold {p_color}", "{primary_el.hangul()} ({primary_el.hanja()})" }
                                                    }
                                                }
                                                div { class: "flex items-center gap-2",
                                                    div { class: "flex-1",
                                                        p { class: "text-xs text-slate-400", "희신" }
                                                        p { class: "font-semibold {a_color}", "{assist_el.hangul()} ({assist_el.hanja()})" }
                                                    }
                                                }
                                                if !yn.recommendations.is_empty() {
                                                    p { class: "text-xs text-slate-500 mt-1 leading-relaxed",
                                                        "{yn.recommendations[0].summary}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // 분석 메타
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 flex flex-col gap-3",
                                    h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "분석 정보" }
                                    div { class: "space-y-2 text-sm",
                                        MetaRow { label: "입력 시각", value: data.meta.input_time.clone() }
                                        MetaRow { label: "교정 시각", value: data.meta.corrected_time.clone() }
                                        MetaRow { label: "타임존", value: data.meta.analysis_timezone.clone() }
                                        MetaRow { label: "DST", value: if data.meta.is_dst { "적용됨".to_string() } else { "해당없음".to_string() } }
                                    }
                                }
                            }

                            // ── 3. 오행 분포 ──────────────────────────────────
                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5",
                                h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest mb-4", "오행 분포 (Five Elements)" }
                                div { class: "space-y-3",
                                    {
                                        let se = &data.report.strength.deuk_se;
                                        let total = (se.bijie_count + se.yinxing_count + se.shishang_count + se.caisheng_count + se.guanxing_count).max(1) as f32;
                                        let dm_el = data.report.strength.day_master.element();
                                        // 비겁(일간 오행) / 인성 / 식상 / 재성 / 관성
                                        let items = vec![
                                            (dm_el.hangul(), dm_el.hanja(), se.bijie_count, "bg-violet-500"),
                                            (dm_el.generated_by().hangul(), dm_el.generated_by().hanja(), se.yinxing_count, "bg-blue-500"),
                                            (dm_el.generates().hangul(), dm_el.generates().hanja(), se.shishang_count, "bg-emerald-500"),
                                            (dm_el.controls().hangul(), dm_el.controls().hanja(), se.caisheng_count, "bg-amber-500"),
                                            (dm_el.controlled_by().hangul(), dm_el.controlled_by().hanja(), se.guanxing_count, "bg-red-500"),
                                        ];
                                        rsx! {
                                            {items.iter().map(|(name, hanja, count, color)| {
                                                let pct = (*count as f32 / total * 100.0) as u32;
                                                rsx! {
                                                    div { class: "flex items-center gap-3",
                                                        div { class: "w-20 text-sm font-medium text-slate-300 text-right shrink-0",
                                                            "{name}({hanja})"
                                                        }
                                                        div { class: "flex-1 h-3 bg-slate-800 rounded-full overflow-hidden",
                                                            div {
                                                                class: "h-full {color} rounded-full transition-all duration-1000",
                                                                style: "width: {pct}%"
                                                            }
                                                        }
                                                        div { class: "w-12 text-xs font-mono text-slate-500 text-right shrink-0", "{count}개 ({pct}%)" }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }
                            }

                            // ── 4. 대운 타임라인 ──────────────────────────────
                            if let Some(ml) = &data.report.major_luck {
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
                                    div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3 flex items-center justify-between",
                                        h3 { class: "font-semibold text-slate-200", "대운 (大運) — {ml.direction}" }
                                        span { class: "text-xs text-slate-400", "만 {ml.start_age}세 교운" }
                                    }
                                    div { class: "p-4 overflow-x-auto",
                                        div { class: "flex gap-2 min-w-max",
                                            {ml.cycles.iter().map(|cycle| rsx! {
                                                div { class: "flex flex-col items-center gap-1 p-3 rounded-xl bg-slate-800/50 border border-slate-700/50 hover:border-amber-700/60 transition-colors min-w-[80px]",
                                                    span { class: "text-xs text-slate-400 font-mono", "만 {cycle.start_age}세~" }
                                                    span { class: "text-2xl font-serif text-amber-300 font-bold", "{cycle.ganzi.hanja()}" }
                                                    span { class: "text-sm text-slate-300", "{cycle.ganzi.hangul()}" }
                                                    div { class: "flex gap-1 flex-wrap justify-center",
                                                        span { class: "text-xs px-1.5 py-0.5 rounded bg-slate-700 text-amber-400", "{cycle.stem_god.hangul()}" }
                                                        span { class: "text-xs px-1.5 py-0.5 rounded bg-slate-700 text-blue-400", "{cycle.branch_god.hangul()}" }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }
                            }

                            // ── 5. 신살 뱃지 ──────────────────────────────────
                            {
                                let sm = &data.report.spirit_markers;
                                // aux_shinsals: Vec<(기둥명, 기준명, 신살명)>
                                let shinsals: Vec<String> = sm.aux_shinsals.iter()
                                    .map(|s| s.2.clone())
                                    .collect();
                                if !shinsals.is_empty() {
                                    rsx! {
                                        div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5",
                                            h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest mb-3", "신살 (神煞)" }
                                            div { class: "flex flex-wrap gap-2",
                                                {shinsals.iter().map(|s| rsx! {
                                                    span { class: "px-3 py-1 rounded-full bg-indigo-900/40 border border-indigo-700/50 text-indigo-300 text-sm font-medium",
                                                        "✦ {s}"
                                                    }
                                                })}
                                            }
                                        }
                                    }
                                } else {
                                    rsx! { div {} }
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

fn element_style(hangul: &str) -> (&'static str, &'static str) {
    match hangul {
        "목" => ("text-emerald-400", "🌿"),
        "화" => ("text-red-400", "🔥"),
        "토" => ("text-yellow-400", "⛰️"),
        "금" => ("text-slate-300", "⚙️"),
        "수" => ("text-blue-400", "💧"),
        _ => ("text-slate-400", "◆"),
    }
}

#[component]
fn PillarCard(title: &'static str, hanja: String, hangul: String, color: String) -> Element {
    rsx! {
        div { class: "p-5 rounded-2xl bg-gradient-to-b {color} border flex flex-col items-center justify-center gap-2 hover:scale-[1.02] transition-transform duration-200",
            span { class: "text-xs text-slate-400 font-medium tracking-wide", "{title}" }
            span { class: "text-5xl font-serif text-slate-100 leading-none", "{hanja}" }
            span { class: "text-lg text-slate-300 font-medium", "{hangul}" }
        }
    }
}

#[component]
fn DeukBadge(label: &'static str, acquired: bool) -> Element {
    let (bg, text) = if acquired {
        ("bg-emerald-900/40 border-emerald-700/50 text-emerald-300", "○")
    } else {
        ("bg-slate-800/40 border-slate-700/50 text-slate-500", "✗")
    };
    rsx! {
        div { class: "flex items-center gap-1.5 px-2 py-1 rounded-lg border {bg}",
            span { class: "font-mono text-xs", "{text}" }
            span { class: "text-xs font-medium", "{label}" }
        }
    }
}

#[component]
fn MetaRow(label: &'static str, value: String) -> Element {
    rsx! {
        div { class: "flex justify-between items-start gap-2",
            span { class: "text-slate-500 shrink-0", "{label}" }
            span { class: "text-slate-300 text-right font-mono text-xs", "{value}" }
        }
    }
}
