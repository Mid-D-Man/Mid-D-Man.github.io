// =============================================================================
// src/components/room_separator.rs
use leptos::prelude::*;

#[component]
pub fn RoomSeparator(
    #[prop(default = String::from("#1a252f"))] color: String,
    #[prop(default = false)] has_decorations: bool,
) -> impl IntoView {
    view! {
        <div class="room-separator" style=format!("background-color: {}", color)>
            <div class="separator-content">
                {if has_decorations {
                    view! {
                        <div class="separator-decorations">
                            <div class="decoration-line"></div>
                            <div class="decoration-dots">
                                <span class="dot"></span>
                                <span class="dot"></span>
                                <span class="dot"></span>
                            </div>
                            <div class="decoration-line"></div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>
        </div>
    }
}
