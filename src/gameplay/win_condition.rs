//! Win condition and respawn logic.

use bevy::prelude::*;

use crate::components::{
    AnimState, CelebrateTimer, Collider, CurrentLevel, GoalFlag, Grounded, HighScore,
    LevelTimer, LevelsBeaten, Score, SoundAssets, Velocity, SaveData,
};
use crate::systems::persistence::write_save;

/// Detect when player reaches the goal flag.
pub fn goal_detection(
    mut player_q: Query<(&Transform, &Collider, &mut AnimState, &mut Velocity), With<crate::components::Player>>,
    flag_q: Query<(&Transform, &Collider), With<GoalFlag>>,
    mut score: ResMut<Score>,
    level_timer: Res<LevelTimer>,
    mut high_score: ResMut<HighScore>,
    mut levels_beaten: ResMut<LevelsBeaten>,
    current_level: Res<CurrentLevel>,
    mut celebrate_timer: ResMut<CelebrateTimer>,
) {
    if celebrate_timer.0.is_some() {
        return;
    }
    let Ok((pt, pc, mut anim_state, mut vel)) = player_q.get_single_mut() else {
        return;
    };
    let Ok((ft, fc)) = flag_q.get_single() else {
        return;
    };
    if aabb_overlap(
        pt.translation.truncate(),
        Vec2::new(pc.half_w, pc.half_h),
        ft.translation.truncate(),
        Vec2::new(fc.half_w, fc.half_h),
    ) {
        *anim_state = AnimState::Celebrate;
        vel.0 = Vec2::ZERO;
        let coins = score.0;
        let time_bonus = ((90.0 - level_timer.0).max(0.0) as u32) * 10;
        let final_score = coins * 150 + time_bonus;
        score.0 = final_score;

        if final_score > high_score.0 {
            high_score.0 = final_score;
        }

        if current_level.0 as u32 > levels_beaten.0 {
            levels_beaten.0 = current_level.0 as u32;
        }

        write_save(&SaveData {
            high_score: high_score.0,
            levels_beaten: levels_beaten.0,
        });

        celebrate_timer.0 = Some(Timer::from_seconds(1.5, TimerMode::Once));
    }
}

/// Transition from celebrate to won state after timer completes.
pub fn celebrate_to_won(
    time: Res<Time>,
    mut celebrate_timer: ResMut<CelebrateTimer>,
    mut next_state: ResMut<NextState<crate::components::GameState>>,
) {
    if let Some(timer) = &mut celebrate_timer.0 {
        timer.tick(time.delta());
        if timer.just_finished() {
            celebrate_timer.0 = None;
            next_state.set(crate::components::GameState::Won);
        }
    }
}

/// Keep player in bounds — respawn if they fall off.
pub fn keep_in_bounds(
    current_level: Res<CurrentLevel>,
    sounds: Res<SoundAssets>,
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Velocity, &mut Grounded), With<crate::components::Player>>,
) {
    let Ok((mut t, mut vel, mut grounded)) = query.get_single_mut() else {
        return;
    };

    let right_bound = match current_level.0 {
        2 => 2350.0,
        3 => 3050.0,
        4 => 4000.0,
        _ => 1950.0,
    };
    t.translation.x = t.translation.x.clamp(-580.0, right_bound);

    if t.translation.y < -700.0 {
        t.translation = Vec3::new(-500.0, -260.0, 1.0);
        vel.0 = Vec2::ZERO;
        grounded.0 = false;
        commands.spawn((
            AudioPlayer::<AudioSource>(sounds.respawn.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}

fn aabb_overlap(a_pos: Vec2, a_half: Vec2, b_pos: Vec2, b_half: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() < a_half.x + b_half.x
        && (a_pos.y - b_pos.y).abs() < a_half.y + b_half.y
}
