// =============================================================================
// src/components/background.rs
use leptos::prelude::*;

#[component]
pub fn Background() -> impl IntoView {
    view! {
        <div class="parallax-background">
            // Space gradient (furthest layer)
            <div class="space-gradient"></div>
            // Moon and stars
            <div class="celestial-objects">
                <div class="moon"></div>
                <div class="stars">
                    <div class="star star-1"></div>
                    <div class="star star-2"></div>
                    <div class="star star-3"></div>
                    <div class="star star-4"></div>
                    <div class="star star-5"></div>
                </div>
            </div>
        </div>
    }
}
