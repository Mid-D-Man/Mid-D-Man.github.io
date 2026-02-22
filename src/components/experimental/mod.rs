// =============================================================================
// src/components/experimental/mod.rs
// These components are archived/in-progress — not exposed in the main portfolio.
// To use these, move the following files here from src/components/:
//   background.rs, cityscape.rs, room.rs, room_separator.rs, contact_footer.rs
//   and the room_walls/ subfolder
// =============================================================================

pub mod background;
pub mod cityscape;
pub mod room;
pub mod room_separator;
pub mod contact_footer;

pub mod room_walls {
    pub mod top_wall;
    pub mod bottom_wall;
    pub mod left_wall;
    pub mod right_wall;
    pub mod front_wall;

    pub use top_wall::*;
    pub use bottom_wall::*;
    pub use left_wall::*;
    pub use right_wall::*;
    pub use front_wall::*;
}

pub use background::*;
pub use cityscape::*;
pub use room::*;
pub use room_separator::*;
pub use contact_footer::*;
pub use room_walls::*;
