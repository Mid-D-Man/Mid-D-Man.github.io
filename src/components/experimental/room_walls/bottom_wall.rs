
// =============================================================================
// src/components/room_walls/bottom_wall.rs
use leptos::prelude::*;

#[derive(Clone)]
pub enum FloorType {
    Wood,
    Concrete,
    Carpet,
    Tech,
}

#[component]
pub fn BottomWall(
    #[prop(default = FloorType::Wood)] floor_type: FloorType,
    #[prop(default = String::from("#8b4513"))] color: String,
) -> impl IntoView {
    let wall_class = match floor_type {
        FloorType::Wood => "bottom-wall wood-floor",
        FloorType::Concrete => "bottom-wall concrete-floor",
        FloorType::Carpet => "bottom-wall carpet-floor", 
        FloorType::Tech => "bottom-wall tech-floor",
    };

    view! {
        <div class=wall_class style=format!("background-color: {}", color)>
            <div class="floor-pattern"></div>
            <div class="floor-shadow"></div>
        </div>
    }
}

