//! Camera following the player.

use bevy::prelude::*;

use crate::components::Player;

const CAM_LERP: f32 = 5.0;

/// Lerp the camera to follow the player horizontally.
pub fn camera_follow(
    player_q: Query<&Transform, With<Player>>,
    mut cam_q: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
) {
    let Ok(pt) = player_q.get_single() else {
        return;
    };
    let Ok(mut ct) = cam_q.get_single_mut() else {
        return;
    };
    ct.translation.x += (pt.translation.x - ct.translation.x) * CAM_LERP * time.delta_secs();
}
