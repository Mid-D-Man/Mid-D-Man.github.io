use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn Header() -> impl IntoView {
    let (nav_open, set_nav_open) = signal(false);
    let (is_scrolled, set_is_scrolled) = signal(false);
    
    // Handle smooth scrolling to sections
    let scroll_to_section = move |section_id: &str| {
        // Close mobile menu
        set_nav_open.set(false);
        
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(element) = document.get_element_by_id(section_id) {
                    let options = web_sys::ScrollIntoViewOptions::new();
                    options.set_behavior(web_sys::ScrollBehavior::Smooth);
                    element.scroll_into_view_with_scroll_into_view_options(&options);
                }
            }
        }
    };

    // Handle scroll effect for navbar
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            let window_clone = window.clone();
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                let scroll_y = window_clone.page_y_offset().unwrap_or(0.0);
                set_is_scrolled.set(scroll_y > 100.0);
            }) as Box<dyn Fn()>);
            
            let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });
    
    view! {
        <nav class="navbar" class:scrolled=is_scrolled>
            <div class="nav-container">
                <div class="nav-logo" on:click=move |_| scroll_to_section("home")>
                    <span class="logo-text">"MidManStudio"</span>
                    <span class="logo-tagline">"Vision → Reality"</span>
                </div>
                <div class="nav-menu" class:active=nav_open>
                    <a href="#home" class="nav-link" on:click=move |_| scroll_to_section("home")>"Home"</a>
                    <a href="#services" class="nav-link" on:click=move |_| scroll_to_section("services")>"Services"</a>
                    <a href="#projects" class="nav-link" on:click=move |_| scroll_to_section("projects")>"Projects"</a>
                    <a href="#about" class="nav-link" on:click=move |_| scroll_to_section("about")>"About"</a>
                    <a href="#contact" class="nav-link" on:click=move |_| scroll_to_section("contact")>"Contact"</a>
                </div>
                <div 
                    class="nav-toggle" 
                    class:active=nav_open
                    on:click=move |_| set_nav_open.update(|open| *open = !*open)
                >
                    <span class="bar"></span>
                    <span class="bar"></span>
                    <span class="bar"></span>
                </div>
            </div>
        </nav>
    }
}
