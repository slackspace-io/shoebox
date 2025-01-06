pub mod app;
pub mod components;
#[cfg(feature = "ssr")]
pub mod database;
#[cfg(feature = "ssr")]
pub mod filesystem;
mod lib_models;
#[cfg(feature = "ssr")]
mod models;
pub mod pages;

#[cfg(feature = "ssr")]
pub mod schema;

pub mod settings;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
