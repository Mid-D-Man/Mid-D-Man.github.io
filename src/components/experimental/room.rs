// =============================================================================
// src/components/room.rs
use leptos::prelude::*;
use crate::components::room_walls::*;

#[derive(Clone)]
pub struct RoomConfig {
    pub top_wall_type: TopWallType,
    pub floor_type: FloorType,
    pub window_config: WindowConfig,
    pub wall_color: String,
    pub floor_color: String,
    pub has_side_windows: bool,
}

impl Default for RoomConfig {
    fn default() -> Self {
        Self {
            top_wall_type: TopWallType::Ceiling,
            floor_type: FloorType::Wood,
            window_config: WindowConfig::Large,
            wall_color: "#2c3e50".to_string(),
            floor_color: "#8b4513".to_string(),
            has_side_windows: false,
        }
    }
}

#[component]
pub fn Room(
    #[prop(default = RoomConfig::default())] config: RoomConfig,
    #[prop(default = String::new())] room_id: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="room-container" id=room_id>
            <div class="room-3d">
                <TopWall wall_type=config.top_wall_type.clone() color=config.wall_color.clone() />
                <LeftWall color=config.wall_color.clone() has_window=config.has_side_windows />
                <RightWall color=config.wall_color.clone() has_window=config.has_side_windows />
                <FrontWall window=config.window_config.clone() color=config.wall_color.clone()>
                    {children()}
                </FrontWall>
                <BottomWall floor_type=config.floor_type.clone() color=config.floor_color.clone() />
            </div>
        </div>
    }
}
