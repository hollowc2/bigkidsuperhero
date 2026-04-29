//! UI components.

use bevy::prelude::*;

/// Tags the win-screen UI so it can be despawned on restart.
#[derive(Component)]
pub struct WinScreen;

/// Tags the score display text.
#[derive(Component)]
pub struct ScoreText;

/// Mute button button entity.
#[derive(Component)]
pub struct MuteButton;

/// Mute button label text entity.
#[derive(Component)]
pub struct MuteButtonLabel;
