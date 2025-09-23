// src/lib.rs - Main library entry point for Leptos CSR
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

pub mod app;
pub mod components;

use app::App;

// WASM entry point - this is called by the generated JavaScript
#[wasm_bindgen(start)]
pub fn main() {
    // Set up error handling
    console_error_panic_hook::set_once();
    
    // Log startup
    web_sys::console::log_1(&"MidManStudio: Starting Leptos CSR...".into());
    
    // Mount the app to the document body
    leptos::mount::mount_to_body(App);
}
