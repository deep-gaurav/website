pub mod about;
pub mod app;
pub mod contact;
pub mod footer;
pub mod header;
pub mod home;
pub mod icons;
pub mod project;
pub mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
