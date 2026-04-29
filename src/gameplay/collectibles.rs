//! Collectible pickup logic.

use bevy::prelude::*;

use crate::components::{Collectible, Collider, Score, SoundAssets};

/// Check for player overlap with collectibles and remove them.
pub fn collectible_collision(
    mut commands: Commands,
    sounds: Res<SoundAssets>,
    player_q: Query<(&Transform, &Collider), With<crate::components::Player>>,
    collectible_q: Query<(Entity, &Transform, &Collider), With<Collectible>>,
    mut score: ResMut<Score>,
) {
    let Ok((pt, pc)) = player_q.get_single() else {
        return;
    };
    let p_pos = pt.translation.truncate();
    let p_half = Vec2::new(pc.half_w, pc.half_h);
    for (entity, ct, cc) in &collectible_q {
        if aabb_overlap(
            p_pos,
            p_half,
            ct.translation.truncate(),
            Vec2::new(cc.half_w, cc.half_h),
        ) {
            commands.entity(entity).despawn();
            score.0 += 1;
            commands.spawn((
                AudioPlayer::<AudioSource>(sounds.collect.clone()),
                PlaybackSettings::DESPAWN,
            ));
        }
    }
}

fn aabb_overlap(a_pos: Vec2, a_half: Vec2, b_pos: Vec2, b_half: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() < a_half.x + b_half.x
        && (a_pos.y - b_pos.y).abs() < a_half.y + b_half.y
}
