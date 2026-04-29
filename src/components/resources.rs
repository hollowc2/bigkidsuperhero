//! Resource definitions — singleton game state data.

use bevy::prelude::*;

// ── Persistence ────────────────────────────────────────────────────────────────

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub high_score: u32,
    #[serde(default)]
    pub levels_beaten: u32,
}

// ── State ───────────────────────────────────────────────────────────────────────

/// Game state machine states.
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MenuScreen,
    CharacterSelect,
    MapScreen,
    Playing,
    Won,
}

// ── Resources ─────────────────────────────────────────────────────────────────

/// Current score (collectibles collected).
#[derive(Resource, Default)]
pub struct Score(pub u32);

/// Highest score ever achieved.
#[derive(Resource, Default)]
pub struct HighScore(pub u32);

/// How long the current level has been played.
#[derive(Resource, Default)]
pub struct LevelTimer(pub f32);

/// How many levels have been beaten in order (0 = none, 1 = L1, 2 = L1+L2, etc.).
#[derive(Resource, Default)]
pub struct LevelsBeaten(pub u32);

/// Which level is currently being played (1, 2, 3, etc.).
#[derive(Resource, Default)]
pub struct CurrentLevel(pub usize);

/// Which level node is highlighted on the map screen.
#[derive(Resource, Default)]
pub struct MapSelection(pub usize);

/// Which item is highlighted on the main menu (0=New, 1=Load, 2=Exit).
#[derive(Resource, Default)]
pub struct MenuSelection(pub usize);

/// Timer for the celebration animation after reaching the goal.
#[derive(Resource, Default)]
pub struct CelebrateTimer(pub Option<Timer>);

/// Which character is highlighted on the character select screen.
#[derive(Resource, Default)]
pub struct CharacterSelectIndex(pub usize);

/// The character the player has chosen.
#[derive(Resource, Default, Clone, Copy, PartialEq)]
pub enum SelectedCharacter {
    #[default]
    Bridget,
    Calvin,
}

/// Whether background music is muted.
#[derive(Resource, Default)]
pub struct MusicMuted(pub bool);

/// Loaded audio assets.
#[derive(Resource)]
pub struct SoundAssets {
    pub jump: Handle<AudioSource>,
    pub collect: Handle<AudioSource>,
    pub win: Handle<AudioSource>,
    pub respawn: Handle<AudioSource>,
}
