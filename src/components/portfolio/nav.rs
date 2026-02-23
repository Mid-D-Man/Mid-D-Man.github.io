// =============================================================================
// src/components/portfolio/nav.rs
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn Nav() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);
    let (scrolled, set_scrolled) = signal(false);

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
        // Overlay — sits behind drawer, closes menu on tap
        <div
            class=move || if menu_open.get() { "nav-overlay active" } else { "nav-overlay" }
            on:click=move |_| close_menu()
        ></div>

        <nav class=move || {
            if scrolled.get() { "portfolio-nav scrolled" } else { "portfolio-nav" }
        }>
            <div class="nav-inner">
                // Logo
                <a href="#hero" class="nav-logo-link" on:click=move |_| close_menu()>
                    // PNG logo from sharedPublic — swap filename if different
                    <img
                        src="sharedPublic/midmanstudio-logo-nobg.png"
                        alt="MidManStudio Logo"
                        class="nav-logo-img"
                        onerror="this.style.display='none';this.nextElementSibling.style.display='flex'"
                    />
                    // SVG fallback (shown if PNG fails to load)
                    <svg
                        class="nav-logo-svg"
                        style="display:none"
                        width="36" height="36"
                        viewBox="0 0 40 40"
                        xmlns="http://www.w3.org/2000/svg"
                        aria-hidden="true"
                    >
                        <defs>
                            <linearGradient id="lg" x1="0%" y1="0%" x2="100%" y2="100%">
                                <stop offset="0%" stop-color="#4169e1"/>
                                <stop offset="65%" stop-color="#0047ab"/>
                                <stop offset="100%" stop-color="#dc143c"/>
                            </linearGradient>
                        </defs>
                        <polygon points="20,2 36,11 36,29 20,38 4,29 4,11"
                            fill="none" stroke="url(#lg)" stroke-width="1.5" opacity="0.6"/>
                        <path d="M9 29L9 11L20 22L31 11L31 29"
                            fill="none" stroke="url(#lg)" stroke-width="3"
                            stroke-linecap="round" stroke-linejoin="round"/>
                        <circle cx="20" cy="22" r="2.5" fill="#dc143c"/>
                    </svg>
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
                    <li><a href="#contact" class="nav-link nav-cta">"Contact"</a></li>
                </ul>

                // Burger button — BurgerMenu.razor animation ported to Leptos
                <button
                    class=move || {
                        if menu_open.get() { "nav-hamburger open" } else { "nav-hamburger" }
                    }
                    on:click=move |_| set_menu_open.update(|v| *v = !*v)
                    aria-label="Toggle menu"
                    aria-expanded=move || menu_open.get().to_string()
                >
                    <span class="bar bar--top"></span>
                    <span class="bar bar--middle"></span>
                    <span class="bar bar--bottom"></span>
                </button>
            </div>

            // Mobile drawer
            <div class=move || {
                if menu_open.get() { "nav-drawer open" } else { "nav-drawer" }
            }>
                <ul class="drawer-links">
                    <li><a href="#about"   class="drawer-link" on:click=move |_| close_menu()>"About"</a></li>
                    <li><a href="#services" class="drawer-link" on:click=move |_| close_menu()>"Services"</a></li>
                    <li><a href="#projects" class="drawer-link" on:click=move |_| close_menu()>"Projects"</a></li>
                    <li><a href="#contact"  class="drawer-link drawer-cta" on:click=move |_| close_menu()>"Contact"</a></li>
                </ul>
            </div>
        </nav>
    }
}
