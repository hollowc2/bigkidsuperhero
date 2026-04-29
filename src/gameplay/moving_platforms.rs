//! Moving platform systems.

use bevy::prelude::*;

use crate::components::{
    FallingPlatform, Grounded, MovingPlatform, Platform, Collider, Player,
};
use bevy::prelude::Transform;

/// Update moving platforms — oscillate them and track delta for player carry.
pub fn update_moving_platforms(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut MovingPlatform), With<Platform>>,
) {
    for (mut t, mut mp) in &mut query {
        let old_x = t.translation.x;
        let old_y = t.translation.y;

        mp.elapsed += time.delta_secs();
        let offset = (mp.elapsed * mp.speed).sin() * mp.amplitude;
        if mp.horizontal {
            t.translation.x = mp.start_x + offset;
        } else {
            t.translation.y = mp.start_y + offset;
        }

        mp.delta = Vec2::new(t.translation.x - old_x, t.translation.y - old_y);
    }
}

/// Update falling platforms — shake then drop when player lands.
pub fn update_falling_platforms(
    time: Res<Time>,
    mut commands: Commands,
    player_q: Query<(&Transform, &Collider, &Grounded), With<Player>>,
    mut fp_q: Query<
        (Entity, &mut Transform, &mut FallingPlatform, &Collider),
        Without<Player>,
    >,
) {
    let Ok((pt, pc, grounded)) = player_q.get_single() else { return };
    let dt = time.delta_secs();

    for (entity, mut plat_t, mut fp, plat_c) in &mut fp_q {
        if fp.falling {
            fp.fall_velocity -= 900.0 * dt;
            plat_t.translation.y += fp.fall_velocity * dt;
            if plat_t.translation.y < -1000.0 {
                commands.entity(entity).despawn();
            }
            continue;
        }

        let dx = (pt.translation.x - plat_t.translation.x).abs();
        let player_bottom = pt.translation.y - pc.half_h;
        let plat_top = plat_t.translation.y + plat_c.half_h;
        let on_platform = grounded.0
            && dx < pc.half_w + plat_c.half_w
            && (player_bottom - plat_top).abs() < 8.0;

        if on_platform && !fp.warned {
            fp.warned = true;
        }

        if fp.warned {
            fp.timer -= dt;
            let elapsed_warn = 0.6 - fp.timer;
            let shake = (elapsed_warn * 40.0).sin() * 4.0;
            plat_t.translation.x = fp.original_x + shake;

            if fp.timer <= 0.0 {
                fp.falling = true;
                commands.entity(entity).remove::<Platform>();
            }
        }
    }
}
