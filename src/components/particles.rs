//! Particle and visual effect components.

use bevy::prelude::*;

/// A short-lived particle with a lifetime counter.
#[derive(Component)]
pub struct Particle {
    pub lifetime: f32,
    pub max_lifetime: f32,
}

/// A parallax background layer.
#[derive(Component)]
pub struct ParallaxLayer {
    pub factor: f32,
}
