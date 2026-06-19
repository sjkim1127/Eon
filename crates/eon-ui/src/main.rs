#![allow(non_snake_case)]
use dioxus::prelude::*;

mod components;
mod i18n;
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
    let mut state = use_context_provider(|| store::AnalysisState::new());

    // Restore locale from localStorage on first render
    use_effect(move || {
        use wasm_bindgen::prelude::*;
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = localStorage, js_name = getItem)]
            fn ls_get(key: &str) -> Option<String>;
        }
        if let Some(code) = ls_get("eon_locale") {
            state.locale.set(i18n::Locale::from_code(&code));
        }
    });

    rsx! {
        Router::<Route> {}
    }
}
