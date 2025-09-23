// src/main.rs - FIXED for Leptos 0.8 CSR
use leptos::prelude::*;

mod app;
mod components;

use app::App;

fn main() {
    // Set up error handling
    console_error_panic_hook::set_once();
    
    // Simple console log
    web_sys::console::log_1(&"MidManStudio: Starting Leptos CSR...".into());
    
    // FIXED: Correct way to mount in Leptos 0.8 CSR
    leptos::mount::mount_to_body(App);
}
