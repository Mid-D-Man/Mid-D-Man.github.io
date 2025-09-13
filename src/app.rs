use leptos::prelude::*;
use crate::components::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app">
            // Background Effects
            <div class="bg-effects">
                <div class="floating-particles"></div>
                <div class="grid-overlay"></div>
            </div>

            <Header/>
            <Hero/>
            <Services/>
            <Projects/>
            <About/>
            <Contact/>
            <Footer/>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    let scroll_to_section = move |section_id: &str| {
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

    view! {
        <footer class="footer">
            <div class="container">
                <div class="footer-content">
                    <div class="footer-brand">
                        <h3>"MidManStudio"</h3>
                        <p>"\"Every great game starts with a simple idea and the courage to make it real.\""</p>
                    </div>
                    <div class="footer-links">
                        <div class="footer-section">
                            <h4>"Services"</h4>
                            <a href="#services" on:click=move |_| scroll_to_section("services")>"Game Development"</a>
                            <a href="#services" on:click=move |_| scroll_to_section("services")>"Web Development"</a>
                            <a href="#services" on:click=move |_| scroll_to_section("services")>"Software Development"</a>
                            <a href="#services" on:click=move |_| scroll_to_section("services")>"Digital Art"</a>
                        </div>
                        <div class="footer-section">
                            <h4>"Projects"</h4>
                            <a href="https://mid-d-man.github.io/AirCode/" target="_blank" rel="noopener">"AirCode Platform"</a>
                            <a href="https://github.com/mid-d-man" target="_blank" rel="noopener">"Game Portfolio"</a>
                        </div>
                    </div>
                </div>
                <div class="footer-bottom">
                    <p>"© 2025 MidManStudio. All rights reserved. Bridging the gap between vision and reality."</p>
                </div>
            </div>
        </footer>
    }
    }
