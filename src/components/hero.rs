use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    let scroll_to_section = move |section_id: &str| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(element) = document.get_element_by_id(section_id) {
                    let mut options = web_sys::ScrollIntoViewOptions::new();
                    options.set_behavior(web_sys::ScrollBehavior::Smooth);
                    element.scroll_into_view_with_scroll_into_view_options(&options);
                }
            }
        }
    };

    view! {
        <section id="home" class="hero">
            <div class="hero-content">
                <div class="hero-text">
                    <h1 class="hero-title">
                        <span class="title-line">"Welcome to"</span>
                        <span class="title-main">"MidManStudio"</span>
                        <span class="title-sub">"Bridging the Gap Between Vision and Reality"</span>
                    </h1>
                    <p class="hero-description">
                        "We specialize in game development, creating immersive experiences with Unity. 
                        From interactive games to web applications and digital art, we transform your wildest ideas into reality."
                    </p>
                    <div class="hero-buttons">
                        <button 
                            class="btn btn-primary"
                            on:click=move |_| scroll_to_section("projects")
                        >
                            "View Our Games"
                        </button>
                        <button 
                            class="btn btn-secondary"
                            on:click=move |_| scroll_to_section("contact")
                        >
                            "Start a Project"
                        </button>
                    </div>
                </div>
                <div class="hero-visual">
                    <div class="floating-icon game-controller"></div>
                    <div class="floating-icon code-bracket"></div>
                    <div class="floating-icon design-brush"></div>
                </div>
            </div>
            <div class="scroll-indicator" on:click=move |_| scroll_to_section("services")>
                <div class="scroll-arrow"></div>
            </div>
        </section>
    }
        }
