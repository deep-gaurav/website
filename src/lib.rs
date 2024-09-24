pub mod about;
pub mod app;
pub mod contact;
pub mod footer;
pub mod header;
pub mod home;
pub mod icons;
pub mod picture;
pub mod project;
pub mod projects;
pub mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    // use tracing_subscriber::fmt;
    // use tracing_subscriber_wasm::MakeConsoleWriter;

    // fmt()
    //     .with_writer(
    //         // To avoide trace events in the browser from showing their
    //         // JS backtrace, which is very annoying, in my opinion
    //         MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG),
    //     )
    //     // For some reason, if we don't do this in the browser, we get
    //     // a runtime error.
    //     .without_time()
    //     .init();
    leptos::mount::hydrate_islands();
}
