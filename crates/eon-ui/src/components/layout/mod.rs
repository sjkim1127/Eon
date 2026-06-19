use dioxus::prelude::*;
use crate::router::Route;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "flex h-screen w-full bg-slate-950 text-slate-100",
            Sidebar {}
            main { class: "flex-1 overflow-auto bg-slate-900/50 backdrop-blur-xl relative flex flex-col",
                div { class: "p-6 w-full max-w-6xl mx-auto space-y-6 flex-1",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[component]
fn Sidebar() -> Element {
    rsx! {
        aside { class: "w-64 bg-slate-950/80 border-r border-slate-800/50 backdrop-blur-md flex flex-col hidden md:flex",
            div { class: "h-16 flex items-center px-6 border-b border-slate-800/50",
                h1 { class: "text-2xl font-bold bg-gradient-to-r from-violet-400 to-indigo-400 bg-clip-text text-transparent",
                    "EON"
                }
            }
            nav { class: "flex-1 p-4 space-y-2 overflow-y-auto",
                SidebarLink { to: Route::SajuTab {}, icon: "📝", label: "사주 명식" }
                SidebarLink { to: Route::VedicTab {}, icon: "✨", label: "베딕 차트" }
                SidebarLink { to: Route::StrengthTab {}, icon: "💪", label: "세력 분석" }
                SidebarLink { to: Route::TransitTab {}, icon: "⏳", label: "운세 흐름" }
                SidebarLink { to: Route::SimulationTab {}, icon: "🧪", label: "생애 시뮬레이터" }
                SidebarLink { to: Route::TierTab {}, icon: "🏆", label: "데스티니 티어" }
                SidebarLink { to: Route::AiTab {}, icon: "🤖", label: "AI 분석관" }
            }
        }
    }
}

#[component]
fn SidebarLink(to: Route, icon: &'static str, label: &'static str) -> Element {
    let route: Route = use_route();
    let is_active = route == to;
    
    let active_class = if is_active {
        "bg-violet-500/10 text-violet-300 border-violet-500/50"
    } else {
        "text-slate-400 hover:bg-slate-800/50 hover:text-slate-200 border-transparent"
    };

    rsx! {
        Link {
            to: to,
            class: "flex items-center gap-3 px-4 py-3 rounded-xl border transition-all duration-200 {active_class}",
            span { class: "text-xl", "{icon}" }
            span { class: "font-medium", "{label}" }
        }
    }
}
