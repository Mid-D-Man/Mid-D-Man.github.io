// =============================================================================
// src/components/room_walls/left_wall.rs
use leptos::prelude::*;

#[component]
pub fn LeftWall(
    #[prop(default = String::from("#34495e"))] color: String,
    #[prop(default = false)] has_window: bool,
) -> impl IntoView {
    view! {
        <div class="left-wall" style=format!("background-color: {}", color)>
            <div class="wall-perspective-left">
                {if has_window {
                    view! {
                        <div class="side-window">
                            <div class="window-light"></div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>
        </div>
    }
}

