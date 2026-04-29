//! Hazard and enemy systems.

use bevy::prelude::*;

use crate::components::{Monster, MovingHazard};
use bevy::prelude::Sprite;

/// Update moving hazards (e.g. spinning saws).
pub fn update_moving_hazards(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut MovingHazard)>,
) {
    for (mut t, mut mh) in &mut query {
        mh.elapsed += time.delta_secs();
        t.translation.x = mh.start_x + (mh.elapsed * mh.speed).sin() * mh.amplitude;
    }
}

/// Update patrolling monsters.
pub fn update_monsters(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Monster, &mut Sprite)>,
) {
    for (mut t, mut monster, mut sprite) in &mut query {
        t.translation.x += monster.dir * monster.speed * time.delta_secs();
        if t.translation.x >= monster.end_x {
            t.translation.x = monster.end_x;
            monster.dir = -1.0;
            sprite.flip_x = true;
        } else if t.translation.x <= monster.start_x {
            t.translation.x = monster.start_x;
            monster.dir = 1.0;
            sprite.flip_x = false;
        }
    }
}
