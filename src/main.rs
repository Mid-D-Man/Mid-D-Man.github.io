// src/main.rs - FIXED for GitHub Pages deployment
use leptos::prelude::*;

mod app;
mod components;

use app::App;

fn main() {
    // CRITICAL: Enhanced error handling for GitHub Pages
    console_error_panic_hook::set_once();
    
    // Add detailed console logging for debugging
    console_log!("MidManStudio: Starting Leptos initialization...");
    
    // CRITICAL FIX: Use mount_to_body with error handling
    match std::panic::catch_unwind(|| {
        console_log!("MidManStudio: Mounting to body...");
        mount_to_body(|| {
            view! {
                <App/>
            }
        })
    }) {
        Ok(_) => {
            console_log!("MidManStudio: Successfully mounted to DOM");
        },
        Err(e) => {
            console_error!("MidManStudio: Failed to mount - {:?}", e);
            
            // Fallback: Try to show error message in DOM
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        let _ = body.set_inner_html(
                            r#"
                            <div style="
                                display: flex; 
                                justify-content: center; 
                                align-items: center; 
                                min-height: 100vh; 
                                background: linear-gradient(135deg, #4169e1, #0047ab);
                                color: white; 
                                font-family: Arial, sans-serif;
                                text-align: center;
                                padding: 20px;
                            ">
                                <div>
                                    <h1>🎮 MidManStudio</h1>
                                    <p>Portfolio initialization failed. Please refresh the page.</p>
                                    <p style="opacity: 0.7;">If this issue persists, please check the browser console.</p>
                                </div>
                            </div>
                            "#
                        );
                    }
                }
            }
        }
    }
    
    console_log!("MidManStudio: Main function completed");
}

// Add console logging macros for debugging
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}

#[macro_export]
macro_rules! console_error {
    ($($t:tt)*) => (web_sys::console::error_1(&format_args!($($t)*).to_string().into()))
}
