//! Level/entity components — platforms, collectibles, hazards, enemies.

use bevy::prelude::*;

/// Tags every level entity so they can all be despawned on restart.
#[derive(Component)]
pub struct GameEntity;

/// A static or moving platform the player can stand on.
#[derive(Component)]
pub struct Platform;

/// A collectible item (coin, fruit, etc.).
#[derive(Component)]
pub struct Collectible;

/// The goal flag at the end of a level.
#[derive(Component)]
pub struct GoalFlag;

/// Marks a hazard tile (spikes, fire, saw, etc.). Touching one respawns the player.
#[derive(Component)]
pub struct Hazard;

/// A platform that oscillates back and forth.
#[derive(Component)]
pub struct MovingPlatform {
    pub start_x: f32,
    pub start_y: f32,
    pub amplitude: f32,
    pub speed: f32,
    pub horizontal: bool,
    pub elapsed: f32,
    /// How far the platform moved this frame (used to carry the player).
    pub delta: Vec2,
}

/// Patrolling enemy that bounces between start_x and end_x.
#[derive(Component)]
pub struct Monster {
    pub start_x: f32,
    pub end_x: f32,
    pub speed: f32,
    pub dir: f32,
}

/// A hazard that oscillates horizontally (e.g. spinning saw).
#[derive(Component)]
pub struct MovingHazard {
    pub start_x: f32,
    pub amplitude: f32,
    pub speed: f32,
    pub elapsed: f32,
}

/// A platform that shakes then falls when the player lands on it.
#[derive(Component)]
pub struct FallingPlatform {
    pub original_x: f32,
    pub warned: bool,
    pub timer: f32,
    pub falling: bool,
    pub fall_velocity: f32,
}
