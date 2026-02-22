// =============================================================================
// src/components/room_walls/front_wall.rs
use leptos::prelude::*;

#[derive(Clone)]
pub enum WindowConfig {
    None,
    Small,
    Large, 
    Panoramic,
    Multiple,
}

#[component]
pub fn FrontWall(
    #[prop(default = WindowConfig::None)] window: WindowConfig,
    #[prop(default = String::from("#2c3e50"))] color: String,
    children: Children,
) -> impl IntoView {
    let window_class = match window {
        WindowConfig::None => "front-wall no-window",
        WindowConfig::Small => "front-wall small-window",
        WindowConfig::Large => "front-wall large-window",
        WindowConfig::Panoramic => "front-wall panoramic-window", 
        WindowConfig::Multiple => "front-wall multiple-windows",
    };

    view! {
        <div class=window_class style=format!("background-color: {}", color)>
            {match window {
                WindowConfig::None => view! { <div></div> }.into_any(),
                _ => view! {
                    <div class="window-container">
                        <div class="cityscape-view">
                            <div class="distant-buildings"></div>
                            <div class="window-frame-inner"></div>
                        </div>
                    </div>
                }.into_any(),
            }}
            <div class="room-content">
                {children()}
            </div>
        </div>
    }
}

