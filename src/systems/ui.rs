//! UI systems — score display, timer.

use bevy::prelude::*;

use crate::components::{HighScore, LevelTimer, Score, ScoreText};

const TOTAL_COINS: u32 = 8;

/// Show the score UI.
pub fn show_score_ui(mut q: Query<&mut Visibility, With<ScoreText>>) {
    for mut v in &mut q {
        *v = Visibility::Visible;
    }
}

/// Hide the score UI.
pub fn hide_score_ui(mut q: Query<&mut Visibility, With<ScoreText>>) {
    for mut v in &mut q {
        *v = Visibility::Hidden;
    }
}

/// Update the score display text.
pub fn update_score_ui(
    score: Res<Score>,
    high_score: Res<HighScore>,
    level_timer: Res<LevelTimer>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    let Ok(mut text) = query.get_single_mut() else {
        return;
    };
    let total_secs = level_timer.0 as u32;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    text.0 = format!(
        "Coins: {}/{}   Time: {}:{:02}   Best: {}",
        score.0, TOTAL_COINS, mins, secs, high_score.0,
    );
}

/// Tick the level timer up.
pub fn tick_level_timer(time: Res<Time>, mut level_timer: ResMut<LevelTimer>) {
    level_timer.0 += time.delta_secs();
}
