//! Big Kid Superhero! — A modular Bevy platformer.
//!
//! Module structure:
//!   - `components/` — all ECS components and resources
//!   - `screens/`    — menu, character select, map, win screen
//!   - `gameplay/`   — player control, physics, collision, enemies
//!   - `levels/`     — level data definitions and spawn logic
//!   - `audio/`      — sound effects and background music
//!   - `systems/`    — animation, particles, UI, persistence

mod audio;
mod components;
mod gameplay;
mod levels;
mod screens;
mod systems;

use audio::audio::setup_audio;
use bevy::prelude::*;
use components::{
    BackgroundMusic, CelebrateTimer, CharacterSelectIndex, Collider, CurrentLevel, GameEntity,
    GameState, Grounded, Hazard, HighScore, LevelTimer, LevelsBeaten, MapSelection, MenuSelection,
    MovingPlatform, MusicMuted, MuteButton, MuteButtonSlash, Platform, Score, ScoreText,
    SelectedCharacter, SoundAssets, Velocity, aabb_overlap,
};
use gameplay::camera::camera_follow;
use gameplay::collectibles::collectible_collision;
use gameplay::hazards::{update_monsters, update_moving_hazards};
use gameplay::moving_platforms::{update_falling_platforms, update_moving_platforms};
use gameplay::player::{animate_celebration, apply_gravity, apply_velocity, player_input};
use gameplay::win_condition::{celebrate_to_won, goal_detection, keep_in_bounds};
use levels::LEVELS;
use screens::character_select::{
    animate_character_preview, character_select_input, cleanup_character_select,
    spawn_character_select,
};
use screens::map_screen::{
    animate_map_cursor, animate_map_orbs, animate_map_scroll, cleanup_map_screen, map_input,
    spawn_map_screen,
};
use screens::menu::{cleanup_menu_screen, menu_input, spawn_menu_screen};
use screens::win_screen::{cleanup_win_screen, restart_input, show_win_screen};
use systems::animation::animate_sprites;
use systems::particles::{parallax_scroll, update_particles};
use systems::persistence::load_save;
use systems::ui::{hide_score_ui, show_score_ui, tick_level_timer, update_score_ui};

// ── Constants ────────────────────────────────────────────────────────────────

// Allow dead code for constants defined for future use
#[allow(dead_code)]
const GRAVITY: f32 = -900.0;
#[allow(dead_code)]
const JUMP_VELOCITY: f32 = 420.0;
#[allow(dead_code)]
const MOVE_SPEED: f32 = 220.0;
#[allow(dead_code)]
const SPRINT_SPEED: f32 = 370.0;
#[allow(dead_code)]
const PLAYER_HALF_W: f32 = 14.0;
#[allow(dead_code)]
const PLAYER_HALF_H: f32 = 40.0;
#[allow(dead_code)]
const PLATFORM_H: f32 = 20.0;
#[allow(dead_code)]
const CAM_LERP: f32 = 5.0;
#[allow(dead_code)]
const SAVE_PATH: &str = "save.json";

const TOTAL_COINS: u32 = 8;
#[allow(dead_code)]
const COIN_VALUE: u32 = 150;
#[allow(dead_code)]
const TIME_PAR: f32 = 90.0;
#[allow(dead_code)]
const TIME_BONUS_PER_SEC: u32 = 10;

const PARALLAX_TILE_W: f32 = 954.0;
const PARALLAX_TILE_H: f32 = 720.0;

// ── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Big Kid Superhero!".into(),
                        resolution: (1280.0, 720.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb(0.4, 0.7, 1.0)))
        .init_state::<GameState>()
        .insert_resource(Score::default())
        .insert_resource(HighScore::default())
        .insert_resource(LevelTimer::default())
        .insert_resource(LevelsBeaten::default())
        .insert_resource(CurrentLevel::default())
        .insert_resource(MapSelection(1))
        .insert_resource(MenuSelection::default())
        .insert_resource(CharacterSelectIndex::default())
        .insert_resource(SelectedCharacter::default())
        .insert_resource(MusicMuted::default())
        .insert_resource(CelebrateTimer::default())
        // Persistent startup — load save, setup audio, camera, parallax, score UI
        .add_systems(
            Startup,
            (load_persistent, setup_parallax_background, setup_score_ui).chain(),
        )
        // Menu screen
        .add_systems(
            OnEnter(GameState::MenuScreen),
            (reset_camera, spawn_menu_screen).chain(),
        )
        .add_systems(Update, menu_input.run_if(in_state(GameState::MenuScreen)))
        .add_systems(OnExit(GameState::MenuScreen), cleanup_menu_screen)
        // Character select screen
        .add_systems(OnEnter(GameState::CharacterSelect), spawn_character_select)
        .add_systems(
            Update,
            (character_select_input, animate_character_preview)
                .run_if(in_state(GameState::CharacterSelect)),
        )
        .add_systems(OnExit(GameState::CharacterSelect), cleanup_character_select)
        // Map screen
        .add_systems(
            OnEnter(GameState::MapScreen),
            (reset_camera, cleanup_game_entities, spawn_map_screen).chain(),
        )
        .add_systems(
            Update,
            (
                map_input,
                animate_map_scroll,
                animate_map_orbs,
                animate_map_cursor,
            )
                .run_if(in_state(GameState::MapScreen)),
        )
        .add_systems(OnExit(GameState::MapScreen), cleanup_map_screen)
        // Playing state
        .add_systems(
            OnEnter(GameState::Playing),
            (set_level_theme, show_score_ui, spawn_level).chain(),
        )
        .add_systems(
            Update,
            (
                player_input,
                apply_gravity,
                apply_velocity,
                update_moving_platforms,
                update_monsters,
                update_moving_hazards,
                platform_collision,
                keep_in_bounds,
                collectible_collision,
                hazard_collision,
                update_falling_platforms,
                goal_detection,
                animate_celebration,
                celebrate_to_won,
                camera_follow,
                animate_sprites,
                tick_level_timer,
                update_score_ui,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(Update, escape_to_map.run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), hide_score_ui)
        // Win screen
        .add_systems(OnEnter(GameState::Won), show_win_screen)
        .add_systems(Update, restart_input.run_if(in_state(GameState::Won)))
        .add_systems(OnExit(GameState::Won), cleanup_win_screen)
        // Particles and parallax run unconditionally
        .add_systems(Update, (update_particles, parallax_scroll))
        // Mute button always active
        .add_systems(Update, mute_system)
        .run();
}

// ── Persistent Setup ─────────────────────────────────────────────────────────

fn load_persistent(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut high_score: ResMut<HighScore>,
    mut levels_beaten: ResMut<LevelsBeaten>,
) {
    let save = load_save();
    high_score.0 = save.high_score;
    levels_beaten.0 = save.levels_beaten;

    // Camera — spawned once at startup, lives forever
    commands.spawn(Camera2d::default());

    // Setup audio (sounds and background music) — spawned once at startup, lives forever
    setup_audio(commands, asset_server);
}

fn setup_parallax_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let layers: &[(&str, f32, f32, f32)] = &[
        (
            "parallax backgound pack/_11_background.png",
            -10.0,
            1.00,
            0.0,
        ),
        (
            "parallax backgound pack/_10_distant_clouds.png",
            -9.0,
            0.97,
            40.0,
        ),
        (
            "parallax backgound pack/_09_distant_clouds1.png",
            -8.0,
            0.93,
            20.0,
        ),
        (
            "parallax backgound pack/_07_huge_clouds.png",
            -7.0,
            0.89,
            50.0,
        ),
        ("parallax backgound pack/_08_clouds.png", -6.0, 0.84, 10.0),
        ("parallax backgound pack/_06_hill2.png", -5.0, 0.72, -30.0),
        ("parallax backgound pack/_05_hill1.png", -4.0, 0.60, -40.0),
        (
            "parallax backgound pack/_03_distant_trees.png",
            -3.0,
            0.50,
            -50.0,
        ),
        ("parallax backgound pack/_04_bushes.png", -2.0, 0.40, -60.0),
        (
            "parallax backgound pack/_02_trees and bushes.png",
            -1.0,
            0.30,
            -70.0,
        ),
    ];

    for &(path, z, factor, y_offset) in layers {
        let tex = asset_server.load(path);
        commands
            .spawn((
                components::ParallaxLayer { factor },
                Transform::from_xyz(0.0, y_offset, z),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
                GlobalTransform::default(),
            ))
            .with_children(|parent| {
                for i in -2_i32..=2 {
                    parent.spawn((
                        Sprite {
                            image: tex.clone(),
                            custom_size: Some(Vec2::new(PARALLAX_TILE_W, PARALLAX_TILE_H)),
                            ..default()
                        },
                        Transform::from_xyz(i as f32 * PARALLAX_TILE_W, 0.0, 0.0),
                    ));
                }
            });
    }
}

fn setup_score_ui(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    high_score: Res<HighScore>,
) {
    // Score UI — hidden until Playing state
    commands.spawn((
        ScoreText,
        Text::new(format!(
            "Coins: 0/{}   Time: 0:00   Best: {}",
            TOTAL_COINS, high_score.0
        )),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        Visibility::Hidden,
    ));
}

// ── Level Spawn ──────────────────────────────────────────────────────────────

fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut score: ResMut<Score>,
    mut level_timer: ResMut<LevelTimer>,
    mut celebrate_timer: ResMut<CelebrateTimer>,
    existing: Query<Entity, With<GameEntity>>,
    mut cam_q: Query<&mut Transform, With<Camera2d>>,
    current_level: Res<CurrentLevel>,
    selected_character: Res<SelectedCharacter>,
) {
    for e in &existing {
        commands.entity(e).despawn();
    }
    score.0 = 0;
    level_timer.0 = 0.0;
    celebrate_timer.0 = None;

    if let Ok(mut cam_t) = cam_q.get_single_mut() {
        cam_t.translation.x = 0.0;
    }

    // Spawn player
    gameplay::player::spawn_player(
        &mut commands,
        &asset_server,
        &mut layouts,
        *selected_character,
    );

    // Use the new level registry
    let layout = &LEVELS[current_level.0.saturating_sub(1).min(LEVELS.len() - 1)];
    levels::spawn_level_layout(&mut commands, layout, &asset_server, &mut layouts);
}

// ── Level Theme ──────────────────────────────────────────────────────────────

fn set_level_theme(current_level: Res<CurrentLevel>, mut clear_color: ResMut<ClearColor>) {
    let layout = &LEVELS[current_level.0.saturating_sub(1).min(LEVELS.len() - 1)];
    clear_color.0 = layout.theme.clear_color();
}

// ── Platform Collision ───────────────────────────────────────────────────────

fn platform_collision(
    time: Res<Time>,
    mut player_q: Query<
        (&mut Transform, &mut Velocity, &mut Grounded, &Collider),
        With<components::Player>,
    >,
    platform_q: Query<
        (&Transform, &Collider, Option<&MovingPlatform>),
        (With<Platform>, Without<components::Player>),
    >,
) {
    let Ok((mut pt, mut vel, mut grounded, pc)) = player_q.get_single_mut() else {
        return;
    };

    grounded.0 = false;
    let dt = time.delta_secs();

    for (plat_t, plat_c, plat_mp) in &platform_q {
        let dx = (pt.translation.x - plat_t.translation.x).abs();
        if dx >= pc.half_w + plat_c.half_w {
            continue;
        }
        if vel.0.y > 0.0 {
            continue;
        }

        let player_bottom = pt.translation.y - pc.half_h;
        let plat_top = plat_t.translation.y + plat_c.half_h;
        let plat_bottom = plat_t.translation.y - plat_c.half_h;
        let prev_bottom = player_bottom - vel.0.y * dt;

        let swept = prev_bottom >= plat_top && player_bottom <= plat_top;
        let inside = player_bottom >= plat_bottom && player_bottom < plat_top;

        if swept || inside {
            pt.translation.y = plat_top + pc.half_h;
            vel.0.y = 0.0;
            grounded.0 = true;
            if let Some(mp) = plat_mp {
                pt.translation.x += mp.delta.x;
            }
        }
    }
}

// ── Hazard Collision ─────────────────────────────────────────────────────────

fn hazard_collision(
    sounds: Res<SoundAssets>,
    mut commands: Commands,
    mut player_q: Query<
        (&mut Transform, &mut Velocity, &mut Grounded, &Collider),
        With<components::Player>,
    >,
    hazard_q: Query<(&Transform, &Collider), (With<Hazard>, Without<components::Player>)>,
) {
    let Ok((mut pt, mut vel, mut grounded, pc)) = player_q.get_single_mut() else {
        return;
    };
    let p_pos = pt.translation.truncate();
    let p_half = Vec2::new(pc.half_w, pc.half_h);

    for (ht, hc) in &hazard_q {
        if aabb_overlap(
            p_pos,
            p_half,
            ht.translation.truncate(),
            Vec2::new(hc.half_w, hc.half_h),
        ) {
            pt.translation = Vec3::new(-500.0, -260.0, 1.0);
            vel.0 = Vec2::ZERO;
            grounded.0 = false;
            commands.spawn((
                AudioPlayer::<AudioSource>(sounds.respawn.clone()),
                PlaybackSettings::DESPAWN,
            ));
            return;
        }
    }
}

// ── Utility ─────────────────────────────────────────────────────────────────

fn reset_camera(mut cam_q: Query<&mut Transform, With<Camera2d>>) {
    if let Ok(mut ct) = cam_q.get_single_mut() {
        ct.translation.x = 0.0;
        ct.translation.y = 0.0;
    }
}

fn cleanup_game_entities(mut commands: Commands, q: Query<Entity, With<GameEntity>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

fn escape_to_map(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MapScreen);
    }
}

fn mute_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<MuteButton>)>,
    mut slash_q: Query<&mut Visibility, With<MuteButtonSlash>>,
    mut muted: ResMut<MusicMuted>,
    music_q: Query<&AudioSink, With<BackgroundMusic>>,
) {
    let clicked = interaction_q.iter().any(|i| *i == Interaction::Pressed);
    let key_pressed = keyboard.just_pressed(KeyCode::KeyM);

    if clicked || key_pressed {
        muted.0 = !muted.0;
        for sink in &music_q {
            sink.set_volume(if muted.0 { 0.0 } else { 0.4 });
        }
        for mut visibility in &mut slash_q {
            *visibility = if muted.0 {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}
