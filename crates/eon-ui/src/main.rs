#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod components;
mod router;
mod store;
mod types;
mod utils;
mod worker;

use router::Route;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting Eon Dioxus UI...");
    
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize global state here using Dioxus signals Context
    use_context_provider(|| store::AnalysisState::new());

    rsx! {
        Router::<Route> {}
    }
}
