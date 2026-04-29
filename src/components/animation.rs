//! Animation-related components.

use bevy::prelude::*;

/// Frame range for sprite animation.
#[derive(Component, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

/// Drives the ticking of the animation timer.
#[derive(Component)]
pub struct AnimationTimer(pub Timer);

/// Animation frame ranges specific to player characters.
#[derive(Component)]
pub struct PlayerAnimations {
    pub idle_first: usize,
    pub idle_last: usize,
    pub idle_secs: f32,
    pub walk_first: usize,
    pub walk_last: usize,
    pub walk_secs: f32,
    pub run_first: usize,
    pub run_last: usize,
    pub run_secs: f32,
    pub jump_first: usize,
    pub jump_last: usize,
    pub fall_first: usize,
    pub fall_last: usize,
    pub celebrate_first: usize,
    pub celebrate_last: usize,
    pub celebrate_secs: f32,
}

/// The current animation state of a character.
#[derive(Component, PartialEq)]
pub enum AnimState {
    Idle,
    Walk,
    Run,
    Jump,
    Fall,
    Celebrate,
}

/// Drives idle animation on the character-select preview sprite.
#[derive(Component)]
pub struct CharacterPreviewAnim {
    pub frame: usize,
    pub timer: Timer,
    pub frame_w: f32,
    pub frame_h: f32,
}
