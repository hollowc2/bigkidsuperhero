//! Core component definitions for the game.
//! Each module contains related component structs.

use bevy::prelude::Component;

pub mod player;
pub mod level;
pub mod animation;
pub mod particles;
pub mod map;
pub mod menu;
pub mod ui;
pub mod resources;

// Re-export everything for convenient wildcard imports
pub use player::*;
pub use level::*;
pub use animation::*;
pub use particles::*;
pub use map::*;
pub use menu::*;
pub use ui::*;
pub use resources::*;

/// Marks the background music entity so the mute system can find it.
#[derive(Component)]
pub struct BackgroundMusic;

// ── Helpers ─────────────────────────────────────────────────────────────────

/// AABB overlap check — re-exported from resources since SaveData lives there.
pub fn aabb_overlap(a_pos: bevy::prelude::Vec2, a_half: bevy::prelude::Vec2, b_pos: bevy::prelude::Vec2, b_half: bevy::prelude::Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() < a_half.x + b_half.x
        && (a_pos.y - b_pos.y).abs() < a_half.y + b_half.y
}
