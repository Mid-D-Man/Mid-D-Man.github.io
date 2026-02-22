// =============================================================================
// src/components/room_walls/top_wall.rs
use leptos::prelude::*;

#[derive(Clone)]
pub enum TopWallType {
    Ceiling,
    SkyView,
    Window,
}

#[component]
pub fn TopWall(
    #[prop(default = TopWallType::Ceiling)] wall_type: TopWallType,
    #[prop(default = String::from("#2c3e50"))] color: String,
) -> impl IntoView {
    let wall_class = match wall_type {
        TopWallType::Ceiling => "top-wall ceiling",
        TopWallType::SkyView => "top-wall sky-view", 
        TopWallType::Window => "top-wall window",
    };

    view! {
        <div class=wall_class style=format!("background-color: {}", color)>
            <div class="wall-content">
                {match wall_type {
                    TopWallType::SkyView => view! {
                        <div class="sky-gradient"></div>
                    }.into_any(),
                    TopWallType::Window => view! {
                        <div class="window-frame">
                            <div class="window-view"></div>
                        </div>
                    }.into_any(),
                    _ => view! { <div></div> }.into_any(),
                }}
            </div>
        </div>
    }
}
