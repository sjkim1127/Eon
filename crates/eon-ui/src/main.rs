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
    let mut rayon_ready = use_signal(|| false);

    use_future(move || async move {
        #[cfg(target_arch = "wasm32")]
        {
            let num_threads = web_sys::window()
                .map(|w| w.navigator().hardware_concurrency() as usize)
                .unwrap_or(4);

            let _ = wasm_bindgen_futures::JsFuture::from(wasm_bindgen_rayon::init_thread_pool(
                num_threads,
            ))
            .await;
        }
        rayon_ready.write().clone_from(&true);
    });

    // Initialize global state here using Dioxus signals Context
    let mut state = use_context_provider(store::AnalysisState::new);

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
        if *rayon_ready.read() {
            Router::<Route> {}
        } else {
            div {
                class: "w-full h-screen flex flex-col items-center justify-center bg-zinc-900 text-white font-sans",
                div { class: "animate-spin rounded-full h-12 w-12 border-4 border-emerald-500 border-t-transparent mb-4" }
                h1 { class: "text-xl font-bold tracking-widest", "INITIALIZING ENGINE..." }
                p { class: "text-zinc-500 text-sm mt-2", "Booting multithreaded workers" }
            }
        }
    }
}
