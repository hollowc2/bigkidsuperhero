//! Win screen — celebration and score display.

use bevy::prelude::*;
use std::f32::consts::TAU;

use crate::components::{
    GameState, GoalFlag, HighScore, LevelTimer, Particle, Score, Velocity, WinScreen,
};

/// Spawn the win screen overlay.
pub fn show_win_screen(
    mut commands: Commands,
    sounds: Res<crate::components::SoundAssets>,
    score: Res<Score>,
    high_score: Res<HighScore>,
    level_timer: Res<LevelTimer>,
    flag_q: Query<&Transform, With<GoalFlag>>,
) {
    commands.spawn((
        AudioPlayer::<AudioSource>(sounds.win.clone()),
        PlaybackSettings::DESPAWN,
    ));

    if let Ok(flag_t) = flag_q.get_single() {
        spawn_fireworks(&mut commands, flag_t.translation.truncate());
    }

    commands
        .spawn((
            WinScreen,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(24.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.72)),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("You Win!"),
                TextFont { font_size: 80.0, ..default() },
                TextColor(Color::srgb(1.0, 0.9, 0.1)),
            ));
            p.spawn((
                Text::new({
                    let total_secs = level_timer.0 as u32;
                    format!(
                        "Score: {}   Time: {}:{:02}   Best: {}",
                        score.0,
                        total_secs / 60,
                        total_secs % 60,
                        high_score.0,
                    )
                }),
                TextFont { font_size: 38.0, ..default() },
                TextColor(Color::WHITE),
            ));
            p.spawn((
                Text::new("Press SPACE to return to map"),
                TextFont { font_size: 30.0, ..default() },
                TextColor(Color::srgb(0.5, 1.0, 0.5)),
            ));
        });
}

/// Handle restart / return to map input.
pub fn restart_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::Enter)
        || keyboard.just_pressed(KeyCode::KeyR)
        || keyboard.just_pressed(KeyCode::Escape)
    {
        next_state.set(GameState::MapScreen);
    }
}

/// Despawn win screen entities.
pub fn cleanup_win_screen(mut commands: Commands, win_q: Query<Entity, With<WinScreen>>) {
    for e in &win_q {
        commands.entity(e).despawn_recursive();
    }
}

/// Spawn celebratory fireworks at a given position.
pub fn spawn_fireworks(commands: &mut Commands, origin: Vec2) {
    let palette = [
        Color::srgb(1.0, 0.2, 0.2),
        Color::srgb(1.0, 0.55, 0.1),
        Color::srgb(1.0, 1.0, 0.1),
        Color::srgb(0.2, 1.0, 0.2),
        Color::srgb(0.2, 0.5, 1.0),
        Color::srgb(0.85, 0.2, 1.0),
        Color::srgb(1.0, 0.4, 0.8),
        Color::WHITE,
    ];

    for i in 0..80_usize {
        let t = i as f32 / 80.0;
        let angle = t * TAU;
        let speed = 90.0 + (i % 3) as f32 * 130.0 + (i % 7) as f32 * 18.0;
        let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed + 55.0);
        let color = palette[i % palette.len()];
        let size = 5.0 + (i % 4) as f32 * 2.0;
        let lifetime = 1.2 + (i % 6) as f32 * 0.2;

        commands.spawn((
            Particle {
                lifetime,
                max_lifetime: lifetime,
            },
            Velocity(vel),
            Sprite::from_color(color, Vec2::splat(size)),
            Transform::from_xyz(origin.x, origin.y, 3.0),
        ));
    }
}
