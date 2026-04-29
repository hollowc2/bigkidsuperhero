//! Particle system and parallax.

use bevy::prelude::*;

use crate::components::{ParallaxLayer, Particle, Velocity};

const GRAVITY: f32 = -900.0;

/// Update all particles — move, fade, and despawn expired ones.
pub fn update_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Velocity, &mut Sprite, &mut Particle)>,
) {
    let dt = time.delta_secs();
    for (entity, mut transform, mut vel, mut sprite, mut particle) in &mut query {
        particle.lifetime -= dt;
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        vel.0.y += GRAVITY * 0.25 * dt;
        transform.translation.x += vel.0.x * dt;
        transform.translation.y += vel.0.y * dt;
        let alpha = (particle.lifetime / particle.max_lifetime).max(0.0);
        sprite.color = sprite.color.with_alpha(alpha);
    }
}

/// Scroll parallax background layers based on camera position.
pub fn parallax_scroll(
    cam_q: Query<&Transform, With<Camera2d>>,
    mut layer_q: Query<(&ParallaxLayer, &mut Transform), Without<Camera2d>>,
) {
    let Ok(cam_t) = cam_q.get_single() else {
        return;
    };
    let cam_x = cam_t.translation.x;
    for (layer, mut t) in &mut layer_q {
        t.translation.x = cam_x * layer.factor;
    }
}
