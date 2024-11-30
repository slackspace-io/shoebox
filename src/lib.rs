pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
mod filesystem;
#[cfg(feature = "ssr")]
mod db;
#[cfg(feature = "ssr")]
pub mod schema;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}


