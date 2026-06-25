use dioxus::prelude::*;

use crate::components::layout::AppLayout;
use crate::components::tabs::{
    saju_tab::SajuTab,
    vedic_tab::VedicTab,
    strength_tab::StrengthTab,
    transit_tab::TransitTab,
    simulation_tab::SimulationTab,
    tier_tab::TierTab,
    ai_tab::AiTab,
    zwds_tab::ZwdsTab,
    iching_tab::IChingTab,
    western_tab::WesternTab,
    human_design_tab::HumanDesignTab,
};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        SajuTab {},
        
        #[route("/vedic_charts")]
        VedicTab {},
        
        #[route("/strength")]
        StrengthTab {},
        
        #[route("/transit")]
        TransitTab {},
        
        #[route("/simulation")]
        SimulationTab {},
        
        #[route("/destiny_tier")]
        TierTab {},
        
        #[route("/ai_audit")]
        AiTab {},

        #[route("/zwds")]
        ZwdsTab {},

        #[route("/iching")]
        IChingTab {},

        #[route("/western")]
        WesternTab {},

        #[route("/human_design")]
        HumanDesignTab {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center h-full w-full",
            h1 { class: "text-2xl font-bold", "404 - Page Not Found" }
        }
    }
}
