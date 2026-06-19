use dioxus::prelude::*;
use crate::components::shared::birth_form::BirthForm;

#[component]
pub fn SimulationTab() -> Element {
    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            BirthForm {}
            
            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-purple-200 to-fuchsia-400 bg-clip-text text-transparent",
                    "생애 시뮬레이터 (Life Simulator)"
                }
            }

            div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-6 shadow-xl",
                div { class: "relative border-l border-slate-700 ml-4 md:ml-6 space-y-8 pb-4",
                    
                    // Timeline Item 1
                    div { class: "relative pl-8",
                        div { class: "absolute w-4 h-4 bg-fuchsia-500 rounded-full -left-[9px] top-1 border-4 border-slate-900" }
                        div { class: "flex flex-col md:flex-row md:items-center gap-2 md:gap-4 mb-2",
                            span { class: "text-sm font-mono text-fuchsia-400 font-bold", "2024년 (35세)" }
                            span { class: "text-xs font-mono px-2 py-1 bg-slate-800 text-slate-300 rounded", "갑진(甲辰)년" }
                        }
                        div { class: "bg-slate-800/50 border border-slate-700 rounded-xl p-4",
                            h4 { class: "font-semibold text-slate-200 mb-2", "새로운 시작과 도약의 시기" }
                            p { class: "text-sm text-slate-400 leading-relaxed", 
                                "사주상 갑목(甲)이 진토(辰)에 뿌리를 내리는 시기로, 문서운과 승진운이 강하게 들어옵니다. 베딕 점성학적으로도 목성(Jupiter)이 10하우스를 통과하며 직업적 성취를 암시합니다." 
                            }
                        }
                    }

                    // Timeline Item 2
                    div { class: "relative pl-8",
                        div { class: "absolute w-4 h-4 bg-slate-600 rounded-full -left-[9px] top-1 border-4 border-slate-900" }
                        div { class: "flex flex-col md:flex-row md:items-center gap-2 md:gap-4 mb-2",
                            span { class: "text-sm font-mono text-slate-400 font-bold", "2025년 (36세)" }
                            span { class: "text-xs font-mono px-2 py-1 bg-slate-800 text-slate-300 rounded", "을사(乙巳)년" }
                        }
                        div { class: "bg-slate-800/50 border border-slate-700 rounded-xl p-4",
                            h4 { class: "font-semibold text-slate-200 mb-2", "인간관계의 확장과 주의" }
                            p { class: "text-sm text-slate-400 leading-relaxed", 
                                "겁재가 강해지는 시기로 경쟁이 치열해질 수 있습니다. 베딕 기준으로는 사데사티(Sade Sati)가 끝나가는 무렵이므로 내면의 평화를 찾는 것이 중요합니다." 
                            }
                        }
                    }

                    // Timeline Item 3
                    div { class: "relative pl-8",
                        div { class: "absolute w-4 h-4 bg-orange-500 rounded-full -left-[9px] top-1 border-4 border-slate-900" }
                        div { class: "flex flex-col md:flex-row md:items-center gap-2 md:gap-4 mb-2",
                            span { class: "text-sm font-mono text-orange-400 font-bold", "2026년 (37세)" }
                            span { class: "text-xs font-mono px-2 py-1 bg-slate-800 text-slate-300 rounded", "병오(丙午)년" }
                        }
                        div { class: "bg-slate-800/50 border border-slate-700 rounded-xl p-4",
                            h4 { class: "font-semibold text-slate-200 mb-2", "재물운의 폭발적 상승" }
                            p { class: "text-sm text-slate-400 leading-relaxed", 
                                "화(火) 기운이 극에 달하며 재물창고가 열리는 시기입니다. 투자나 사업 확장에 유리하며 라후(Rahu) 대운의 절정기로서 사회적 영향력이 커집니다." 
                            }
                        }
                    }

                }
            }
        }
    }
}
