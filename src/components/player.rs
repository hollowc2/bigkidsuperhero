//! Player-related components.

use bevy::prelude::*;

/// Player state and ability trackers.
#[derive(Component)]
pub struct Player {
    pub sprint_timer: f32,
    pub sprint_cooldown: f32,
    pub sparkle_timer: f32,
}

/// Horizontal/vertical velocity.
#[derive(Component)]
pub struct Velocity(pub Vec2);

/// Whether the player is currently standing on a surface.
#[derive(Component)]
pub struct Grounded(pub bool);

/// Axis-aligned bounding box for collision.
#[derive(Component)]
pub struct Collider {
    pub half_w: f32,
    pub half_h: f32,
}

/// Tracks how many frames the player has been ungrounded.
/// Used to add a short buffer before switching to Fall animation.
#[derive(Component)]
pub struct CoyoteFrames(pub u8);

/// Dash ability state: active duration remaining and cooldown.
#[derive(Component)]
pub struct DashState {
    /// Seconds remaining in an active dash (0 = not dashing).
    pub active: f32,
    /// Seconds until the dash can be used again.
    pub cooldown: f32,
    /// Direction of the current dash (-1 left, +1 right).
    pub dir: f32,
}

impl Default for DashState {
    fn default() -> Self {
        Self {
            active: 0.0,
            cooldown: 0.0,
            dir: 1.0,
        }
    }
}
