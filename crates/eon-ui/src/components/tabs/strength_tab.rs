use dioxus::prelude::*;
use crate::components::shared::birth_form::BirthForm;

#[component]
pub fn StrengthTab() -> Element {
    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            BirthForm {}
            
            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-emerald-200 to-green-400 bg-clip-text text-transparent",
                    "세력 분석 (Strength Analysis)"
                }
            }

            div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-6 shadow-xl",
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                    // 사주 오행 세력
                    div { class: "space-y-4",
                        h3 { class: "text-lg font-semibold text-slate-300 border-b border-slate-800 pb-2", "사주 오행 (Five Elements)" }
                        
                        ElementBar { name: "목 (Wood)", color: "bg-green-500", percentage: 30 }
                        ElementBar { name: "화 (Fire)", color: "bg-red-500", percentage: 20 }
                        ElementBar { name: "토 (Earth)", color: "bg-yellow-600", percentage: 10 }
                        ElementBar { name: "금 (Metal)", color: "bg-slate-300", percentage: 25 }
                        ElementBar { name: "수 (Water)", color: "bg-blue-500", percentage: 15 }
                    }

                    // 베딕 행성 세력 (Shadbala)
                    div { class: "space-y-4",
                        h3 { class: "text-lg font-semibold text-slate-300 border-b border-slate-800 pb-2", "베딕 샤드발라 (Shadbala)" }
                        
                        ElementBar { name: "태양 (Sun)", color: "bg-orange-500", percentage: 80 }
                        ElementBar { name: "달 (Moon)", color: "bg-slate-100", percentage: 40 }
                        ElementBar { name: "화성 (Mars)", color: "bg-red-600", percentage: 65 }
                        ElementBar { name: "수성 (Mercury)", color: "bg-emerald-400", percentage: 55 }
                        ElementBar { name: "목성 (Jupiter)", color: "bg-yellow-400", percentage: 90 }
                        ElementBar { name: "금성 (Venus)", color: "bg-pink-400", percentage: 70 }
                        ElementBar { name: "토성 (Saturn)", color: "bg-indigo-600", percentage: 35 }
                    }
                }
            }
        }
    }
}

#[component]
fn ElementBar(name: String, color: String, percentage: u8) -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            div { class: "w-24 text-sm font-medium text-slate-400 text-right shrink-0", "{name}" }
            div { class: "flex-1 h-3 bg-slate-800 rounded-full overflow-hidden",
                div { 
                    class: "h-full {color} rounded-full transition-all duration-1000",
                    style: "width: {percentage}%"
                }
            }
            div { class: "w-10 text-sm font-mono text-slate-500 text-right", "{percentage}%" }
        }
    }
}
