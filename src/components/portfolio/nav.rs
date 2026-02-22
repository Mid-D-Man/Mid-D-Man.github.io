// =============================================================================
// src/components/portfolio/nav.rs
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::components::portfolio::logo::LogoMark;

#[component]
pub fn Nav() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);
    let (scrolled, set_scrolled) = signal(false);

    // Scroll listener to add shadow/blur to nav
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            let window_clone = window.clone();
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                let y = window_clone.scroll_y().unwrap_or(0.0);
                set_scrolled.set(y > 60.0);
            }) as Box<dyn Fn()>);
            let _ = window.add_event_listener_with_callback(
                "scroll",
                closure.as_ref().unchecked_ref(),
            );
            closure.forget();
        }
    });

    let close_menu = move || set_menu_open.set(false);

    view! {
        <nav class=move || {
            if scrolled.get() { "portfolio-nav scrolled" } else { "portfolio-nav" }
        }>
            <div class="nav-inner">
                // Logo
                <a href="#hero" class="nav-logo-link" on:click=move |_| close_menu()>
                    <LogoMark />
                    <div class="nav-logo-text">
                        <span class="nav-logo-name">"Mid-D-Man"</span>
                        <span class="nav-logo-studio">"MidManStudio"</span>
                    </div>
                </a>

                // Desktop links
                <ul class="nav-links">
                    <li><a href="#about" class="nav-link">"About"</a></li>
                    <li><a href="#services" class="nav-link">"Services"</a></li>
                    <li><a href="#projects" class="nav-link">"Projects"</a></li>
                    <li>
                        <a href="#contact" class="nav-link nav-cta">"Contact"</a>
                    </li>
                </ul>

                // Hamburger
                <button
                    class=move || {
                        if menu_open.get() { "nav-hamburger open" } else { "nav-hamburger" }
                    }
                    on:click=move |_| set_menu_open.update(|v| *v = !*v)
                    aria-label="Toggle menu"
                >
                    <span class="bar"></span>
                    <span class="bar"></span>
                    <span class="bar"></span>
                </button>
            </div>

            // Mobile drawer
            <div class=move || {
                if menu_open.get() { "nav-drawer open" } else { "nav-drawer" }
            }>
                <ul class="drawer-links">
                    <li><a href="#about" class="drawer-link" on:click=move |_| close_menu()>"About"</a></li>
                    <li><a href="#services" class="drawer-link" on:click=move |_| close_menu()>"Services"</a></li>
                    <li><a href="#projects" class="drawer-link" on:click=move |_| close_menu()>"Projects"</a></li>
                    <li><a href="#contact" class="drawer-link drawer-cta" on:click=move |_| close_menu()>"Contact"</a></li>
                </ul>
            </div>
        </nav>
    }
}
