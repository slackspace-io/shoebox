pub mod app;
pub mod pages;
pub mod components;
mod lib_models;
#[cfg(feature = "ssr")]
pub mod database;
#[cfg(feature = "ssr")]
mod models;
#[cfg(feature = "ssr")]
pub mod filesystem;

#[cfg(feature = "ssr")]
pub mod schema;



#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
