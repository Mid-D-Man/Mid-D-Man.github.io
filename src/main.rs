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
    
    // Mount to body - this is the correct way for Leptos 0.8 CSR
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
