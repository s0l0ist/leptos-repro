#![forbid(unsafe_code)]
#![recursion_limit = "512"]

use app::App;
use leptos::mount::hydrate_body;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();
    hydrate_body(App);
}
