use dioxus::prelude::*;
use crate::components::shared::birth_form::BirthForm;

#[component]
pub fn TransitTab() -> Element {
    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            BirthForm {}
            
            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-sky-200 to-blue-400 bg-clip-text text-transparent",
                    "운세 흐름 (Transit & Dasha)"
                }
            }

            div { class: "grid grid-cols-1 xl:grid-cols-2 gap-6",
                // 사주 대운 (Major Luck)
                div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden shadow-xl",
                    div { class: "bg-slate-800/50 border-b border-slate-800 p-4",
                        h3 { class: "font-semibold text-slate-200", "사주 대운 (Daewun)" }
                    }
                    div { class: "p-4 overflow-x-auto",
                        table { class: "w-full text-sm text-left",
                            thead { class: "text-xs text-slate-400 uppercase bg-slate-800/30",
                                tr {
                                    th { class: "px-4 py-3 font-medium", "나이" }
                                    th { class: "px-4 py-3 font-medium", "연도" }
                                    th { class: "px-4 py-3 font-medium", "간지" }
                                    th { class: "px-4 py-3 font-medium", "십성" }
                                }
                            }
                            tbody { class: "divide-y divide-slate-800",
                                for i in 0..10 {
                                    tr { class: "hover:bg-slate-800/20 transition-colors",
                                        td { class: "px-4 py-3 font-mono text-slate-300", "{i * 10 + 2}세" }
                                        td { class: "px-4 py-3 font-mono text-slate-400", "{1992 + (i * 10)}" }
                                        td { class: "px-4 py-3 font-bold text-amber-400", "갑자(甲子)" }
                                        td { class: "px-4 py-3 text-slate-400", "비견 / 정인" }
                                    }
                                }
                            }
                        }
                    }
                }

                // 베딕 다샤 (Vimshottari Dasha)
                div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden shadow-xl",
                    div { class: "bg-slate-800/50 border-b border-slate-800 p-4",
                        h3 { class: "font-semibold text-slate-200", "베딕 마하다샤 (Mahadasha)" }
                    }
                    div { class: "p-4 overflow-x-auto",
                        table { class: "w-full text-sm text-left",
                            thead { class: "text-xs text-slate-400 uppercase bg-slate-800/30",
                                tr {
                                    th { class: "px-4 py-3 font-medium", "행성 (Planet)" }
                                    th { class: "px-4 py-3 font-medium", "시작" }
                                    th { class: "px-4 py-3 font-medium", "종료" }
                                    th { class: "px-4 py-3 font-medium", "기간" }
                                }
                            }
                            tbody { class: "divide-y divide-slate-800",
                                tr { class: "hover:bg-slate-800/20 transition-colors",
                                    td { class: "px-4 py-3 font-bold text-orange-400", "Sun (태양)" }
                                    td { class: "px-4 py-3 font-mono text-slate-300", "1990-05" }
                                    td { class: "px-4 py-3 font-mono text-slate-400", "1996-05" }
                                    td { class: "px-4 py-3 text-slate-400", "6년" }
                                }
                                tr { class: "hover:bg-slate-800/20 transition-colors",
                                    td { class: "px-4 py-3 font-bold text-slate-200", "Moon (달)" }
                                    td { class: "px-4 py-3 font-mono text-slate-300", "1996-05" }
                                    td { class: "px-4 py-3 font-mono text-slate-400", "2006-05" }
                                    td { class: "px-4 py-3 text-slate-400", "10년" }
                                }
                                tr { class: "hover:bg-slate-800/20 transition-colors",
                                    td { class: "px-4 py-3 font-bold text-red-500", "Mars (화성)" }
                                    td { class: "px-4 py-3 font-mono text-slate-300", "2006-05" }
                                    td { class: "px-4 py-3 font-mono text-slate-400", "2013-05" }
                                    td { class: "px-4 py-3 text-slate-400", "7년" }
                                }
                                tr { class: "hover:bg-slate-800/20 transition-colors bg-violet-900/20",
                                    td { class: "px-4 py-3 font-bold text-slate-400", "Rahu (라후)" }
                                    td { class: "px-4 py-3 font-mono text-slate-300", "2013-05" }
                                    td { class: "px-4 py-3 font-mono text-slate-400", "2031-05" }
                                    td { class: "px-4 py-3 text-violet-400", "18년 (현재)" }
                                }
                                tr { class: "hover:bg-slate-800/20 transition-colors",
                                    td { class: "px-4 py-3 font-bold text-yellow-400", "Jupiter (목성)" }
                                    td { class: "px-4 py-3 font-mono text-slate-300", "2031-05" }
                                    td { class: "px-4 py-3 font-mono text-slate-400", "2047-05" }
                                    td { class: "px-4 py-3 text-slate-400", "16년" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
