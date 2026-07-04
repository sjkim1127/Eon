use dioxus::prelude::*;

use crate::components::layout::AppLayout;
use crate::components::tabs::{
    human_design_tab::HumanDesignTab, iching_tab::IChingTab, saju_tab::SajuTab,
    simulation_tab::SimulationTab, strength_tab::StrengthTab, tier_tab::TierTab,
    transit_tab::TransitTab, vedic_tab::VedicTab, western_tab::WesternTab, zwds_tab::ZwdsTab,
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
