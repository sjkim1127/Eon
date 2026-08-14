use crate::store::{AnalysisState, TaskStatus};
use dioxus::prelude::*;
use eon_service::facade;

#[component]
pub fn NumerologyTab() -> Element {
    let mut state = use_context::<AnalysisState>();
    let status = state.numerology.read().status.clone();
    let form = state.form.read().clone();

    use_effect(move || {
        let form_data = form.clone();
        spawn(async move {
            state.numerology.write().status = TaskStatus::Loading;

            let base_input = form_data.to_analysis_input();
            let num_input = eon_service::dto::NumerologyAnalysisInput::new(base_input, None, None);

            match facade::analyze_numerology(num_input) {
                Ok(res) => {
                    state.numerology.write().data = Some(res);
                    state.numerology.write().status = TaskStatus::Success;
                }
                Err(e) => {
                    state.numerology.write().error = Some(e.to_string());
                    state.numerology.write().status = TaskStatus::Error(e.to_string());
                }
            }
        });
    });

    rsx! {
        div { class: "p-6 space-y-6 max-w-7xl mx-auto",
            h2 { class: "text-2xl font-bold text-slate-100 flex items-center gap-2",
                span { "🔢" }
                "피타고라스 수비학 분석 (Pythagorean Numerology)"
            }

            match status {
                TaskStatus::Idle | TaskStatus::Loading => rsx! {
                    div { class: "p-12 text-center text-slate-400 animate-pulse",
                        "수비학 수비 진동 및 5대 코어 지표 연산 중..."
                    }
                },
                TaskStatus::Error(err) => rsx! {
                    div { class: "p-6 bg-red-900/30 border border-red-500/50 rounded-xl text-red-300",
                        "수비학 연산 오류: {err}"
                    }
                },
                TaskStatus::Success => {
                    let data = state.numerology.read().data.clone();
                    if let Some(output) = data {
                        let res = output.result;
                        rsx! {
                            // 5 Core Numbers Cards
                            div { class: "grid grid-cols-1 md:grid-cols-5 gap-4",
                                div { class: "p-5 bg-gradient-to-br from-amber-900/40 to-slate-900 border border-amber-500/30 rounded-xl text-center space-y-2",
                                    div { class: "text-xs text-amber-400 font-semibold tracking-wider uppercase", "생애여정수 (Life Path)" }
                                    div { class: "text-4xl font-extrabold text-amber-200", "{res.core.life_path}" }
                                    div { class: "text-xs text-slate-400", "인생의 핵심 성향 및 본질" }
                                }
                                div { class: "p-5 bg-gradient-to-br from-indigo-900/40 to-slate-900 border border-indigo-500/30 rounded-xl text-center space-y-2",
                                    div { class: "text-xs text-indigo-400 font-semibold tracking-wider uppercase", "표현수 (Expression)" }
                                    div { class: "text-4xl font-extrabold text-indigo-200", "{res.core.expression}" }
                                    div { class: "text-xs text-slate-400", "타고난 사명 및 잠재 재능" }
                                }
                                div { class: "p-5 bg-gradient-to-br from-emerald-900/40 to-slate-900 border border-emerald-500/30 rounded-xl text-center space-y-2",
                                    div { class: "text-xs text-emerald-400 font-semibold tracking-wider uppercase", "영혼수 (Soul Urge)" }
                                    div { class: "text-4xl font-extrabold text-emerald-200", "{res.core.soul_urge}" }
                                    div { class: "text-xs text-slate-400", "내면의 순수한 열망" }
                                }
                                div { class: "p-5 bg-gradient-to-br from-purple-900/40 to-slate-900 border border-purple-500/30 rounded-xl text-center space-y-2",
                                    div { class: "text-xs text-purple-400 font-semibold tracking-wider uppercase", "인격수 (Personality)" }
                                    div { class: "text-4xl font-extrabold text-purple-200", "{res.core.personality}" }
                                    div { class: "text-xs text-slate-400", "타인에게 비치는 외적 인상" }
                                }
                                div { class: "p-5 bg-gradient-to-br from-rose-900/40 to-slate-900 border border-rose-500/30 rounded-xl text-center space-y-2",
                                    div { class: "text-xs text-rose-400 font-semibold tracking-wider uppercase", "생일수 (Birthday)" }
                                    div { class: "text-4xl font-extrabold text-rose-200", "{res.core.birthday}" }
                                    div { class: "text-xs text-slate-400", "특화된 기술 및 강점" }
                                }
                            }

                            // Personal Year & Summary
                            div { class: "p-6 bg-slate-900/80 border border-slate-700/50 rounded-xl space-y-3",
                                div { class: "flex items-center justify-between",
                                    span { class: "text-base font-semibold text-slate-200", "개인 연운수 (Personal Year Number)" }
                                    span { class: "px-3 py-1 bg-amber-500/20 text-amber-300 font-bold rounded-full text-sm", "{res.personal_year}년 주기 진동" }
                                }
                                p { class: "text-sm text-slate-300 leading-relaxed", "{res.summary_ko}" }
                            }

                            // 4 Pinnacles Timeline
                            div { class: "space-y-3",
                                h3 { class: "text-lg font-bold text-slate-200 flex items-center gap-2",
                                    span { "🏔️" }
                                    "4대 정점 및 과제 주기 (4 Pinnacles & Challenges)"
                                }
                                div { class: "grid grid-cols-1 md:grid-cols-4 gap-4",
                                    for p in res.pinnacles {
                                        div { class: "p-4 bg-slate-900/60 border border-slate-700/40 rounded-xl space-y-2",
                                            div { class: "flex items-center justify-between text-xs text-slate-400",
                                                span { "제{p.stage}정점 ({p.start_age}~{p.end_age}세)" }
                                            }
                                            div { class: "flex items-center gap-3",
                                                span { class: "text-2xl font-bold text-cyan-300", "정점 #{p.pinnacle_number}" }
                                                span { class: "text-xs text-slate-400", "과제 #{p.challenge_number}" }
                                            }
                                            div { class: "text-xs text-slate-300", "{p.description_ko}" }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! { div { "데이터 없음" } }
                    }
                }
            }
        }
    }
}
