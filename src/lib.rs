pub mod app;
#[cfg(feature = "ssr")]
pub mod db;

#[cfg(feature = "ssr")]
mod models;
#[cfg(feature = "ssr")]
mod schema;
#[cfg(feature = "ssr")]
pub mod filesystem;
mod lib_models;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
