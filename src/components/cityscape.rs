
// =============================================================================
// src/components/cityscape.rs
use leptos::prelude::*;

#[component] 
pub fn Cityscape() -> impl IntoView {
    view! {
        <div class="cityscape-layer">
            <div class="buildings">
                <div class="building building-1"></div>
                <div class="building building-2"></div>
                <div class="building building-3"></div>
                <div class="building building-4"></div>
                <div class="building building-5"></div>
                <div class="building building-6"></div>
                <div class="building building-7"></div>
                <div class="building building-8"></div>
                <div class="building building-9"></div>
                <div class="building building-10"></div>
            </div>
        </div>
    }
}
