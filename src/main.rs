use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

// ── Constants ────────────────────────────────────────────────────────────────

const GRAVITY: f32 = -900.0;
const JUMP_VELOCITY: f32 = 420.0;
const MOVE_SPEED: f32 = 220.0;
const SPRINT_SPEED: f32 = 370.0;
const PLAYER_HALF_W: f32 = 14.0;
const PLAYER_HALF_H: f32 = 40.0;
const PLATFORM_H: f32 = 20.0;
const CAM_LERP: f32 = 5.0;
const SAVE_PATH: &str = "save.json";

const TOTAL_COINS: u32 = 8;
const COIN_VALUE: u32 = 150;
const TIME_PAR: f32 = 90.0;
const TIME_BONUS_PER_SEC: u32 = 10;

const PARALLAX_TILE_W: f32 = 954.0;
const PARALLAX_TILE_H: f32 = 720.0;

const DASH_SPEED: f32 = 560.0;
const DASH_DURATION: f32 = 0.18;
const DASH_COOLDOWN: f32 = 0.9;

// Map screen node world-positions (4 levels)
const NODE_X: [f32; 4] = [-450.0, -150.0, 150.0, 450.0];
const NODE_Y: f32 = 30.0;

// ── Components ───────────────────────────────────────────────────────────────

/// Tags every level entity so they can all be despawned on restart.
#[derive(Component)]
struct GameEntity;

/// Tags the win-screen UI so it can be despawned on restart.
#[derive(Component)]
struct WinScreen;

/// Tags menu screen entities.
#[derive(Component)]
struct MenuEntity;

/// Tags map screen entities.
#[derive(Component)]
struct MapEntity;

/// The sliding selection cursor on the map screen.
#[derive(Component)]
struct MapCursor;

/// Marks a level node sprite on the map screen.
#[derive(Component)]
struct MapNode;

/// Drives the breathing pulse animation on map orbs and the cursor halo.
#[derive(Component)]
struct PulseOrb {
    phase: f32,
    speed: f32,
    pulse_amount: f32,
}

/// Marks the outer glow ring behind a level node orb.
#[derive(Component)]
struct OrbGlow;

/// Identifies a menu item (0=New Game, 1=Load, 2=Exit).
#[derive(Component)]
struct MenuItem(usize);

/// Tags character-select screen entities.
#[derive(Component)]
struct CharacterSelectEntity;

/// Identifies a character option (0=Bridget, 1=Calvin).
#[derive(Component)]
struct CharacterSelectItem(usize);

/// Marks the large character preview image in the character select UI.
#[derive(Component)]
struct CharacterPreviewImage;

/// Drives idle animation on the character-select preview sprite.
#[derive(Component)]
struct CharacterPreviewAnim {
    frame: usize,
    timer: Timer,
    frame_w: f32,
    frame_h: f32,
}

#[derive(Component)]
struct Player {
    is_sprinting: bool,
}
#[derive(Component)]
struct Platform;
#[derive(Component)]
struct Collectible;
#[derive(Component)]
struct GoalFlag;
#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Grounded(bool);

#[derive(Component)]
struct Collider {
    half_w: f32,
    half_h: f32,
}

#[derive(Component, Clone)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct PlayerAnimations {
    idle_first: usize,
    idle_last: usize,
    idle_secs: f32,
    walk_first: usize,
    walk_last: usize,
    walk_secs: f32,
    run_first: usize,
    run_last: usize,
    run_secs: f32,
    jump_first: usize,
    jump_last: usize,
    fall_first: usize,
    fall_last: usize,
    celebrate_first: usize,
    celebrate_last: usize,
    celebrate_secs: f32,
}

#[derive(Component, PartialEq)]
enum AnimState {
    Idle,
    Walk,
    Run,
    Jump,
    Fall,
    Celebrate,
}

#[derive(Component)]
struct Particle {
    lifetime: f32,
    max_lifetime: f32,
}

#[derive(Component)]
struct ParallaxLayer {
    factor: f32,
}

#[derive(Component)]
struct MovingPlatform {
    start_x: f32,
    start_y: f32,
    amplitude: f32,
    speed: f32,
    horizontal: bool,
    elapsed: f32,
    /// How far the platform moved this frame (used to carry the player).
    delta: Vec2,
}

/// Counts frames the player has been ungrounded; resets to 0 when grounded.
/// Used to add a short buffer before switching to Fall animation (prevents
/// one-frame flicker when a rising platform briefly separates from the player).
#[derive(Component)]
struct CoyoteFrames(u8);

/// Marks a hazard tile (spikes). Touching one respawns the player.
#[derive(Component)]
struct Hazard;

/// Patrolling enemy that bounces between start_x and end_x.
#[derive(Component)]
struct Monster {
    start_x: f32,
    end_x: f32,
    speed: f32,
    dir: f32,
}

/// A hazard that oscillates horizontally (e.g. spinning saw).
#[derive(Component)]
struct MovingHazard {
    start_x: f32,
    amplitude: f32,
    speed: f32,
    elapsed: f32,
}

/// A platform that shakes then falls when the player lands on it.
#[derive(Component)]
struct FallingPlatform {
    original_x: f32,
    warned: bool,
    timer: f32,
    falling: bool,
    fall_velocity: f32,
}

/// Tracks Bridget's dash state: how long the dash has left and cooldown remaining.
#[derive(Component)]
struct DashState {
    /// Seconds remaining in an active dash (0 = not dashing).
    active: f32,
    /// Seconds until the dash can be used again.
    cooldown: f32,
    /// Direction of the current dash (-1 left, +1 right).
    dir: f32,
}

impl Default for DashState {
    fn default() -> Self {
        Self { active: 0.0, cooldown: 0.0, dir: 1.0 }
    }
}

#[derive(Component)]
struct BackgroundMusic;

#[derive(Component)]
struct MuteButton;

#[derive(Component)]
struct MuteButtonLabel;

// ── Resources ────────────────────────────────────────────────────────────────

#[derive(Resource, Default)]
struct Score(u32);

#[derive(Resource, Default)]
struct HighScore(u32);

#[derive(Resource, Default)]
struct LevelTimer(f32);

/// How many levels have been beaten in order (0 = none, 1 = L1, 2 = L1+L2, 3 = all).
#[derive(Resource, Default)]
struct LevelsBeaten(u32);

/// Which level is currently being played (1, 2, or 3).
#[derive(Resource, Default)]
struct CurrentLevel(usize);

/// Which level node is highlighted on the map screen.
#[derive(Resource, Default)]
struct MapSelection(usize);

/// Which item is highlighted on the main menu (0=New, 1=Load, 2=Exit).
#[derive(Resource, Default)]
struct MenuSelection(usize);

/// Which character is highlighted on the character select screen (0=Bridget, 1=Calvin).
#[derive(Resource, Default)]
struct CharacterSelectIndex(usize);

/// The character the player has chosen.
#[derive(Resource, Default, Clone, Copy, PartialEq)]
enum SelectedCharacter {
    #[default]
    Bridget,
    Calvin,
}

#[derive(Resource, Default)]
struct MusicMuted(bool);

#[derive(Resource)]
struct SoundAssets {
    jump: Handle<AudioSource>,
    collect: Handle<AudioSource>,
    win: Handle<AudioSource>,
    respawn: Handle<AudioSource>,
}

// ── Persistence ──────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
struct SaveData {
    high_score: u32,
    #[serde(default)]
    levels_beaten: u32,
}

// ── State ─────────────────────────────────────────────────────────────────────

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    MenuScreen,
    CharacterSelect,
    MapScreen,
    Playing,
    Won,
}

// ── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Big Kid Superhero!".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
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
        // Camera, score UI, sounds, background music — created once
        .add_systems(Startup, (setup_persistent, setup_background).chain())
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
            (map_input, animate_map_orbs, animate_map_cursor)
                .run_if(in_state(GameState::MapScreen)),
        )
        .add_systems(OnExit(GameState::MapScreen), cleanup_map_screen)
        // Playing
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
                camera_follow,
                animate_sprites,
                tick_level_timer,
                update_score_ui,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            escape_to_map.run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), hide_score_ui)
        // Won
        .add_systems(OnEnter(GameState::Won), show_win_screen)
        .add_systems(Update, restart_input.run_if(in_state(GameState::Won)))
        .add_systems(OnExit(GameState::Won), cleanup_win_screen)
        // Particles and parallax run unconditionally
        .add_systems(Update, (update_particles, parallax_scroll))
        // Mute button always active
        .add_systems(Update, mute_system)
        .run();
}

// ── Persistent Setup (Startup) ────────────────────────────────────────────────

fn setup_persistent(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut high_score: ResMut<HighScore>,
    mut levels_beaten: ResMut<LevelsBeaten>,
) {
    let save = load_save();
    high_score.0 = save.high_score;
    levels_beaten.0 = save.levels_beaten;

    commands.insert_resource(SoundAssets {
        jump: asset_server.load("UI Soundpack/UI Soundpack/OGG/Retro4.ogg"),
        collect: asset_server.load("brackeys_platformer_assets/sounds/coin.ogg"),
        win: asset_server.load("UI Soundpack/UI Soundpack/OGG/African1.ogg"),
        respawn: asset_server.load("UI Soundpack/UI Soundpack/OGG/Retro8.ogg"),
    });

    commands.spawn((
        BackgroundMusic,
        AudioPlayer::<AudioSource>(
            asset_server.load("brackeys_platformer_assets/music/time_for_adventure.mp3"),
        ),
        PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::new(0.4)),
    ));

    commands.spawn(Camera2d::default());

    // Mute button — top right, always visible across all states
    commands
        .spawn((
            MuteButton,
            Button,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                width: Val::Px(48.0),
                height: Val::Px(48.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_children(|p| {
            p.spawn((
                MuteButtonLabel,
                Text::new("♪"),
                TextFont { font_size: 26.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

    // Score UI — hidden until Playing state
    commands.spawn((
        ScoreText,
        Text::new(format!(
            "Coins: 0/{}   Time: 0:00   Best: {}",
            TOTAL_COINS, save.high_score
        )),
        TextFont { font_size: 32.0, ..default() },
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

fn show_score_ui(mut q: Query<&mut Visibility, With<ScoreText>>) {
    for mut v in &mut q {
        *v = Visibility::Visible;
    }
}

fn hide_score_ui(mut q: Query<&mut Visibility, With<ScoreText>>) {
    for mut v in &mut q {
        *v = Visibility::Hidden;
    }
}

fn reset_camera(mut cam_q: Query<&mut Transform, With<Camera2d>>) {
    if let Ok(mut ct) = cam_q.get_single_mut() {
        ct.translation.x = 0.0;
        ct.translation.y = 0.0;
    }
}

// ── Parallax Background (Startup) ─────────────────────────────────────────────

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let layers: &[(&str, f32, f32, f32)] = &[
        ("parallax backgound pack/_11_background.png", -10.0, 1.00, 0.0),
        ("parallax backgound pack/_10_distant_clouds.png", -9.0, 0.97, 40.0),
        ("parallax backgound pack/_09_distant_clouds1.png", -8.0, 0.93, 20.0),
        ("parallax backgound pack/_07_huge_clouds.png", -7.0, 0.89, 50.0),
        ("parallax backgound pack/_08_clouds.png", -6.0, 0.84, 10.0),
        ("parallax backgound pack/_06_hill2.png", -5.0, 0.72, -30.0),
        ("parallax backgound pack/_05_hill1.png", -4.0, 0.60, -40.0),
        ("parallax backgound pack/_03_distant_trees.png", -3.0, 0.50, -50.0),
        ("parallax backgound pack/_04_bushes.png", -2.0, 0.40, -60.0),
        ("parallax backgound pack/_02_trees and bushes.png", -1.0, 0.30, -70.0),
    ];

    for &(path, z, factor, y_offset) in layers {
        let tex = asset_server.load(path);
        commands
            .spawn((
                ParallaxLayer { factor },
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

fn parallax_scroll(
    cam_q: Query<&Transform, With<Camera2d>>,
    mut layer_q: Query<(&ParallaxLayer, &mut Transform), Without<Camera2d>>,
) {
    let Ok(cam_t) = cam_q.get_single() else {
        return;
    };
    let cam_x = cam_t.translation.x;
    for (layer, mut t) in &mut layer_q {
        t.translation.x = cam_x * layer.factor;
    }
}

// ── Menu Screen ───────────────────────────────────────────────────────────────

fn spawn_menu_screen(mut commands: Commands, mut menu_selection: ResMut<MenuSelection>) {
    menu_selection.0 = 0;

    commands
        .spawn((
            MenuEntity,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.04, 0.14, 0.82)),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("BIG KID SUPERHERO!"),
                TextFont { font_size: 72.0, ..default() },
                TextColor(Color::srgb(1.0, 0.9, 0.1)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            let items = ["Start New Game", "Load Game", "Exit"];
            for (i, label) in items.iter().enumerate() {
                let color = if i == 0 {
                    Color::srgb(1.0, 1.0, 0.0)
                } else {
                    Color::srgb(0.6, 0.6, 0.6)
                };
                p.spawn((
                    MenuItem(i),
                    Text::new(*label),
                    TextFont { font_size: 48.0, ..default() },
                    TextColor(color),
                ));
            }

            p.spawn((
                Text::new("Up / Down to choose   Enter to select"),
                TextFont { font_size: 22.0, ..default() },
                TextColor(Color::srgb(0.4, 0.4, 0.4)),
                Node {
                    margin: UiRect::top(Val::Px(60.0)),
                    ..default()
                },
            ));
        });
}

fn menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut menu_selection: ResMut<MenuSelection>,
    mut item_q: Query<(&MenuItem, &mut TextColor)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut levels_beaten: ResMut<LevelsBeaten>,
    mut high_score: ResMut<HighScore>,
    mut app_exit: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_exit.send(AppExit::Success);
        return;
    }

    let up = keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW);
    let down = keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS);
    let confirm =
        keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space);

    if up && menu_selection.0 > 0 {
        menu_selection.0 -= 1;
    }
    if down && menu_selection.0 < 2 {
        menu_selection.0 += 1;
    }

    for (item, mut color) in &mut item_q {
        color.0 = if item.0 == menu_selection.0 {
            Color::srgb(1.0, 1.0, 0.0)
        } else {
            Color::srgb(0.6, 0.6, 0.6)
        };
    }

    if confirm {
        match menu_selection.0 {
            0 => {
                // Start New Game — reset progress, then pick character
                levels_beaten.0 = 0;
                write_save(&SaveData {
                    high_score: high_score.0,
                    levels_beaten: 0,
                });
                next_state.set(GameState::CharacterSelect);
            }
            1 => {
                // Load Game — read from disk
                let save = load_save();
                levels_beaten.0 = save.levels_beaten;
                high_score.0 = save.high_score;
                next_state.set(GameState::MapScreen);
            }
            2 => {
                app_exit.send(AppExit::Success);
            }
            _ => {}
        }
    }
}

fn cleanup_menu_screen(mut commands: Commands, q: Query<Entity, With<MenuEntity>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

// ── Character Select Screen ───────────────────────────────────────────────────

fn spawn_character_select(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut char_index: ResMut<CharacterSelectIndex>,
) {
    char_index.0 = 0;

    let preview_tex: Handle<Image> = asset_server.load("bridget/bridget_sprite.png");

    commands
        .spawn((
            CharacterSelectEntity,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(60.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.04, 0.14, 0.82)),
        ))
        .with_children(|p| {
            // Left panel: title + name list + help text
            p.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(16.0),
                    ..default()
                },
            ))
            .with_children(|col| {
                col.spawn((
                    Text::new("Choose Your Hero!"),
                    TextFont { font_size: 64.0, ..default() },
                    TextColor(Color::srgb(1.0, 0.9, 0.1)),
                    Node {
                        margin: UiRect::bottom(Val::Px(40.0)),
                        ..default()
                    },
                ));

                let characters = ["Bridget", "Calvin"];
                for (i, name) in characters.iter().enumerate() {
                    let color = if i == 0 {
                        Color::srgb(1.0, 1.0, 0.0)
                    } else {
                        Color::srgb(0.6, 0.6, 0.6)
                    };
                    col.spawn((
                        CharacterSelectItem(i),
                        Text::new(*name),
                        TextFont { font_size: 48.0, ..default() },
                        TextColor(color),
                    ));
                }

                col.spawn((
                    Text::new("Up / Down to choose   Enter to select"),
                    TextFont { font_size: 22.0, ..default() },
                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                    Node {
                        margin: UiRect::top(Val::Px(60.0)),
                        ..default()
                    },
                ));
            });

            // Right panel: large idle-frame preview of the selected character.
            // Uses rect-based clipping on the sprite sheet — frame 0 is the top-left cell.
            p.spawn((
                CharacterPreviewImage,
                CharacterPreviewAnim {
                    frame: 0,
                    timer: Timer::from_seconds(0.18, TimerMode::Repeating),
                    frame_w: 128.0,
                    frame_h: 128.0,
                },
                Node {
                    width: Val::Px(240.0),
                    height: Val::Px(240.0),
                    ..default()
                },
                ImageNode {
                    image: preview_tex,
                    rect: Some(Rect::new(0.0, 0.0, 128.0, 128.0)),
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn character_select_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut char_index: ResMut<CharacterSelectIndex>,
    mut selected: ResMut<SelectedCharacter>,
    mut item_q: Query<(&CharacterSelectItem, &mut TextColor)>,
    mut preview_q: Query<(&mut ImageNode, &mut CharacterPreviewAnim), With<CharacterPreviewImage>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let up = keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW);
    let down = keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS);
    let confirm =
        keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space);
    let back = keyboard.just_pressed(KeyCode::Escape);

    let old_index = char_index.0;

    if up && char_index.0 > 0 {
        char_index.0 -= 1;
    }
    if down && char_index.0 < 1 {
        char_index.0 += 1;
    }

    let changed = char_index.0 != old_index;

    if changed {
        for (item, mut color) in &mut item_q {
            color.0 = if item.0 == char_index.0 {
                Color::srgb(1.0, 1.0, 0.0)
            } else {
                Color::srgb(0.6, 0.6, 0.6)
            };
        }

        let preview_path = match char_index.0 {
            1 => "calvin/calvin_sprite.png",
            _ => "bridget/bridget_sprite.png",
        };
        if let Ok((mut img, mut anim)) = preview_q.get_single_mut() {
            img.image = asset_server.load(preview_path);
            anim.frame = 0;
            anim.timer.reset();
            anim.frame_w = 128.0;
            anim.frame_h = 128.0;
            img.rect = Some(Rect::new(0.0, 0.0, 128.0, 128.0));
        }
    }

    if confirm {
        *selected = match char_index.0 {
            1 => SelectedCharacter::Calvin,
            _ => SelectedCharacter::Bridget,
        };
        next_state.set(GameState::MapScreen);
    }

    if back {
        next_state.set(GameState::MenuScreen);
    }
}

fn cleanup_character_select(mut commands: Commands, q: Query<Entity, With<CharacterSelectEntity>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

fn animate_character_preview(
    time: Res<Time>,
    mut q: Query<(&mut ImageNode, &mut CharacterPreviewAnim), With<CharacterPreviewImage>>,
) {
    let Ok((mut img, mut anim)) = q.get_single_mut() else { return };
    anim.timer.tick(time.delta());
    if anim.timer.just_finished() {
        anim.frame = (anim.frame + 1) % 5;
        let x = anim.frame as f32 * anim.frame_w;
        img.rect = Some(Rect::new(x, 0.0, x + anim.frame_w, anim.frame_h));
    }
}

// ── Map Screen ────────────────────────────────────────────────────────────────

fn spawn_map_screen(
    mut commands: Commands,
    levels_beaten: Res<LevelsBeaten>,
    mut map_selection: ResMut<MapSelection>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let lb = levels_beaten.0 as usize;
    let initial = (lb + 1).min(4);
    map_selection.0 = initial;

    // Deep night-sky background
    commands.spawn((
        MapEntity,
        Mesh2d(meshes.add(Rectangle::new(1280.0, 720.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.03, 0.03, 0.14)))),
        Transform::from_xyz(0.0, 0.0, -0.5),
    ));

    // Background stars at fixed positions
    const STARS: [(f32, f32); 26] = [
        (-580.0, 280.0), (-420.0, 310.0), (-320.0, 260.0), (-200.0, 295.0),
        (-100.0, 275.0), (50.0, 305.0),  (180.0, 265.0),  (310.0, 285.0),
        (450.0, 270.0),  (550.0, 295.0), (-560.0, 140.0), (-380.0, 170.0),
        (-480.0,-150.0), (-280.0,-200.0),(-100.0,-180.0), (80.0, -165.0),
        (300.0,-195.0),  (500.0,-170.0), (560.0,  130.0), (400.0,  150.0),
        (-150.0, 190.0), (220.0, 175.0), (-50.0, -290.0), (430.0, -280.0),
        (-240.0,-300.0), (0.0,   315.0),
    ];
    for (i, (sx, sy)) in STARS.iter().enumerate() {
        let bright = if i % 3 == 0 { 0.95 } else { 0.55 };
        let r = if i % 5 == 0 { 3.0_f32 } else { 2.0 };
        commands.spawn((
            MapEntity,
            Mesh2d(meshes.add(Circle::new(r))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(1.0, 1.0, 0.9, bright)))),
            Transform::from_xyz(*sx, *sy, 0.05),
        ));
    }

    // Dotted path connecting all four nodes
    let dot_step = 28.0_f32;
    let mut dot_x = NODE_X[0] + dot_step;
    while dot_x < NODE_X[3] {
        commands.spawn((
            MapEntity,
            Mesh2d(meshes.add(Circle::new(4.5))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(0.55, 0.45, 0.25, 0.75)))),
            Transform::from_xyz(dot_x, NODE_Y, 0.1),
        ));
        dot_x += dot_step;
    }

    // Title
    commands.spawn((
        MapEntity,
        Text2d::new("Choose a Level!"),
        TextFont { font_size: 52.0, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 230.0, 1.0),
    ));

    // Hint
    commands.spawn((
        MapEntity,
        Text2d::new("\u{2190} \u{2192} to choose   Enter to play"),
        TextFont { font_size: 24.0, ..default() },
        TextColor(Color::srgb(0.5, 0.5, 0.5)),
        Transform::from_xyz(0.0, -230.0, 1.0),
    ));

    // Cursor halo (soft white glow circle that lerps to the selected node)
    commands.spawn((
        MapEntity,
        MapCursor,
        Mesh2d(meshes.add(Circle::new(54.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(1.0, 1.0, 1.0, 0.28)))),
        Transform::from_xyz(NODE_X[initial - 1], NODE_Y, 0.2),
        PulseOrb { phase: 0.0, speed: 2.2, pulse_amount: 0.06 },
    ));

    // Level nodes
    for i in 0..4usize {
        let level_num = i + 1;
        let x = NODE_X[i];

        let (node_color, locked) = if level_num <= lb {
            (Color::srgb(1.0, 0.78, 0.0), false)       // gold — beaten
        } else if level_num == lb + 1 {
            (Color::srgb(0.18, 0.92, 0.28), false)      // bright green — available
        } else {
            (Color::srgb(0.20, 0.20, 0.24), true)       // dim — locked
        };

        // Outer glow ring for non-locked nodes
        if !locked {
            let glow = if level_num <= lb {
                Color::srgba(1.0, 0.78, 0.0, 0.16)
            } else {
                Color::srgba(0.18, 0.92, 0.28, 0.16)
            };
            commands.spawn((
                MapEntity,
                OrbGlow,
                Mesh2d(meshes.add(Circle::new(58.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(glow))),
                Transform::from_xyz(x, NODE_Y, 0.15),
                PulseOrb { phase: i as f32 * 1.3, speed: 1.6, pulse_amount: 0.14 },
            ));
        }

        // Node orb
        let mut ec = commands.spawn((
            MapEntity,
            MapNode,
            Mesh2d(meshes.add(Circle::new(40.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(node_color))),
            Transform::from_xyz(x, NODE_Y, 0.5),
        ));
        if !locked {
            ec.insert(PulseOrb { phase: i as f32 * 0.9, speed: 2.0, pulse_amount: 0.05 });
        }

        // Level number
        commands.spawn((
            MapEntity,
            Text2d::new(format!("{}", level_num)),
            TextFont { font_size: 38.0, ..default() },
            TextColor(if locked { Color::srgb(0.38, 0.38, 0.38) } else { Color::WHITE }),
            Transform::from_xyz(x, NODE_Y, 1.5),
        ));

        // Status label
        let (status, status_color) = if level_num <= lb {
            ("DONE!", Color::srgb(1.0, 0.9, 0.2))
        } else if level_num == lb + 1 {
            ("GO!", Color::srgb(0.35, 1.0, 0.45))
        } else {
            ("LOCKED", Color::srgb(0.35, 0.35, 0.35))
        };
        commands.spawn((
            MapEntity,
            Text2d::new(status),
            TextFont { font_size: 22.0, ..default() },
            TextColor(status_color),
            Transform::from_xyz(x, NODE_Y - 62.0, 1.0),
        ));

        // "Level X" caption
        commands.spawn((
            MapEntity,
            Text2d::new(format!("Level {}", level_num)),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
            Transform::from_xyz(x, NODE_Y - 85.0, 1.0),
        ));
    }
}

fn map_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    levels_beaten: Res<LevelsBeaten>,
    mut map_selection: ResMut<MapSelection>,
    mut next_state: ResMut<NextState<GameState>>,
    mut current_level: ResMut<CurrentLevel>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MenuScreen);
        return;
    }

    let lb = levels_beaten.0 as usize;
    let max_sel = (lb + 1).min(4);

    let left =
        keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA);
    let right =
        keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD);
    let confirm =
        keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space);

    if left && map_selection.0 > 1 {
        map_selection.0 -= 1;
    }
    if right && map_selection.0 < max_sel {
        map_selection.0 += 1;
    }

    if confirm {
        current_level.0 = map_selection.0;
        next_state.set(GameState::Playing);
    }
}

fn animate_map_orbs(time: Res<Time>, mut q: Query<(&mut Transform, &mut PulseOrb)>) {
    for (mut t, mut orb) in &mut q {
        orb.phase += time.delta_secs() * orb.speed;
        let s = 1.0 + orb.phase.sin() * orb.pulse_amount;
        t.scale = Vec3::splat(s.max(0.01));
    }
}

fn animate_map_cursor(
    time: Res<Time>,
    map_selection: Res<MapSelection>,
    mut cursor_q: Query<&mut Transform, With<MapCursor>>,
) {
    let target_x = NODE_X[map_selection.0 - 1];
    if let Ok(mut t) = cursor_q.get_single_mut() {
        let dx = target_x - t.translation.x;
        t.translation.x += dx * (time.delta_secs() * 14.0).min(1.0);
    }
}

fn cleanup_map_screen(mut commands: Commands, q: Query<Entity, With<MapEntity>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

/// Despawn all level entities (platforms, player, coins, flag) when leaving gameplay.
fn cleanup_game_entities(mut commands: Commands, q: Query<Entity, With<GameEntity>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

// ── Level Spawn (OnEnter Playing) ─────────────────────────────────────────────

fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut score: ResMut<Score>,
    mut level_timer: ResMut<LevelTimer>,
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

    if let Ok(mut cam_t) = cam_q.get_single_mut() {
        cam_t.translation.x = 0.0;
    }

    // ── Player ────────────────────────────────────────────────────────────────
    let player_tex = match *selected_character {
        SelectedCharacter::Bridget => asset_server.load("bridget/bridget_sprite.png"),
        SelectedCharacter::Calvin  => asset_server.load("calvin/calvin_sprite.png"),
    };
    // Both characters: 128×128 cells, 5 cols × 5 rows — idle(0-4), walk(5-9), jump(10-14), fall(15-19), celebrate(20-24)
    let player_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(128, 128),
        5,
        5,
        None,
        None,
    ));

    commands.spawn((
        GameEntity,
        Player { is_sprinting: false },
        Velocity(Vec2::ZERO),
        Grounded(false),
        CoyoteFrames(0),
        DashState::default(),
        Collider {
            half_w: PLAYER_HALF_W,
            half_h: PLAYER_HALF_H,
        },
        AnimationIndices { first: 0, last: 3 },
        AnimationTimer(Timer::from_seconds(0.6, TimerMode::Repeating)),
        AnimState::Idle,
        PlayerAnimations {
            idle_first: 0,
            idle_last: 4,
            idle_secs: 0.18,
            walk_first: 5,
            walk_last: 9,
            walk_secs: 0.14,
            run_first: 5,
            run_last: 9,
            run_secs: 0.09,
            jump_first: 10,
            jump_last: 14,
            fall_first: 15,
            fall_last: 19,
            celebrate_first: 20,
            celebrate_last: 24,
            celebrate_secs: 0.12,
        },
        Sprite {
            image: player_tex,
            texture_atlas: Some(TextureAtlas {
                layout: player_layout,
                index: 0,
            }),
            custom_size: Some(Vec2::new(120.0, 120.0)),
            color: Color::WHITE,
            ..default()
        },
        Transform::from_xyz(-500.0, -260.0, 1.0),
    ));

    let terrain = asset_server.load("Pixel Adventure 1/Free/Terrain/Terrain (16x16).png");
    let flag_tex =
        asset_server.load("Pixel Adventure 1/Free/Items/Checkpoints/End/End (Idle).png");
    let spike_tex = asset_server.load("Pixel Adventure 1/Free/Traps/Spikes/Idle.png");

    match current_level.0 {
        2 => {
            let cherry_tex = asset_server.load("Pixel Adventure 1/Free/Items/Fruits/Cherries.png");
            let cherry_layout =
                layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(32), 17, 1, None, None));
            spawn_level_2(&mut commands, &terrain, &cherry_tex, &cherry_layout, &flag_tex, &spike_tex);
        }
        3 => {
            let strawberry_tex =
                asset_server.load("Pixel Adventure 1/Free/Items/Fruits/Strawberry.png");
            let strawberry_layout =
                layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(32), 17, 1, None, None));
            spawn_level_3(&mut commands, &terrain, &strawberry_tex, &strawberry_layout, &flag_tex, &spike_tex);
        }
        4 => {
            let apple_tex = asset_server.load("Pixel Adventure 1/Free/Items/Fruits/Apple.png");
            let apple_layout =
                layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(32), 17, 1, None, None));
            let saw_tex =
                asset_server.load("Pixel Adventure 1/Free/Traps/Saw/On (38x38).png");
            let saw_layout =
                layouts.add(TextureAtlasLayout::from_grid(UVec2::new(38, 38), 8, 1, None, None));
            let fire_tex =
                asset_server.load("Pixel Adventure 1/Free/Traps/Fire/On (16x32).png");
            let fire_layout =
                layouts.add(TextureAtlasLayout::from_grid(UVec2::new(16, 32), 3, 1, None, None));
            let monster_tex = asset_server
                .load("Pixel Adventure 1/Free/Main Characters/Mask Dude/Run (32x32).png");
            let monster_layout =
                layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(32), 12, 1, None, None));
            let fall_plat_tex =
                asset_server.load("Pixel Adventure 1/Free/Traps/Falling Platforms/Off.png");
            spawn_level_4(
                &mut commands, &terrain, &apple_tex, &apple_layout,
                &flag_tex, &spike_tex, &saw_tex, &saw_layout,
                &fire_tex, &fire_layout, &monster_tex, &monster_layout,
                &fall_plat_tex,
            );
        }
        _ => {
            let coin_tex = asset_server.load("brackeys_platformer_assets/sprites/coin.png");
            let coin_layout =
                layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(16), 12, 1, None, None));
            spawn_level_1(&mut commands, &terrain, &coin_tex, &coin_layout, &flag_tex, &spike_tex);
        }
    }
}

fn spawn_level_1(
    commands: &mut Commands,
    terrain: &Handle<Image>,
    coin_tex: &Handle<Image>,
    coin_layout: &Handle<TextureAtlasLayout>,
    flag_tex: &Handle<Image>,
    spike_tex: &Handle<Image>,
) {
    spawn_platform(commands, terrain, 640.0, -320.0, 3200.0);

    for (x, y, w) in [
        (-200.0_f32, -270.0_f32, 200.0_f32),
        (100.0, -250.0, 180.0),
        (380.0, -220.0, 220.0),
        (650.0, -250.0, 160.0),
        (900.0, -220.0, 240.0),
        (1150.0, -250.0, 160.0),
        (1400.0, -275.0, 200.0),
    ] {
        spawn_platform(commands, terrain, x, y, w);
    }

    for (x, y) in [
        (-300.0_f32, -260.0_f32),
        (-200.0, -220.0),
        (100.0, -200.0),
        (380.0, -170.0),
        (650.0, -200.0),
        (900.0, -170.0),
        (1150.0, -200.0),
        (1400.0, -225.0),
    ] {
        spawn_coin(commands, coin_tex, coin_layout, x, y);
    }

    // Spikes on the ground floor — gaps between platforms
    for x in [50.0_f32, 270.0, 540.0, 790.0, 1060.0] {
        spawn_hazard(commands, spike_tex, x, -302.0);
    }

    commands.spawn((
        GameEntity,
        GoalFlag,
        Collider {
            half_w: 30.0,
            half_h: 30.0,
        },
        Sprite {
            image: flag_tex.clone(),
            custom_size: Some(Vec2::splat(64.0)),
            ..default()
        },
        Transform::from_xyz(1800.0, -278.0, 0.5),
    ));
}

fn spawn_level_2(
    commands: &mut Commands,
    terrain: &Handle<Image>,
    fruit_tex: &Handle<Image>,
    fruit_layout: &Handle<TextureAtlasLayout>,
    flag_tex: &Handle<Image>,
    spike_tex: &Handle<Image>,
) {
    // Cherry Cloud Kingdom — bright sky, moving platforms, wider gaps

    // Ground floor
    spawn_platform(commands, terrain, 900.0, -320.0, 3600.0);

    // 5 static cloud platforms (sky blue tint)
    for (x, y, w) in [
        (-300.0_f32, -255.0_f32, 180.0_f32),
        (60.0, -215.0, 160.0),
        (700.0, -245.0, 140.0),
        (1310.0, -225.0, 160.0),
        (1920.0, -240.0, 160.0),
    ] {
        spawn_platform_tinted(commands, terrain, x, y, w, Color::srgb(0.7, 0.9, 1.0));
    }

    // 3 moving platforms (horizontal, cyan tint)
    spawn_moving_platform(commands, terrain, 380.0, -178.0, 160.0, 80.0, 1.2, true,  Color::srgb(0.4, 0.95, 0.9));
    spawn_moving_platform(commands, terrain, 1000.0, -190.0, 160.0, 100.0, 1.5, true, Color::srgb(0.4, 0.95, 0.9));
    spawn_moving_platform(commands, terrain, 1620.0, -178.0, 160.0, 70.0,  1.0, true, Color::srgb(0.4, 0.95, 0.9));

    // Cherries above each platform
    for (x, y) in [
        (-300.0_f32, -205.0_f32),
        (60.0, -165.0),
        (380.0, -128.0),
        (700.0, -195.0),
        (1000.0, -140.0),
        (1310.0, -175.0),
        (1620.0, -128.0),
        (1920.0, -190.0),
    ] {
        spawn_fruit(commands, fruit_tex, fruit_layout, x, y);
    }

    // Spikes on the ground floor — between cloud platforms
    for x in [-80.0_f32, 230.0, 560.0, 870.0, 1170.0, 1660.0] {
        spawn_hazard(commands, spike_tex, x, -302.0);
    }

    commands.spawn((
        GameEntity,
        GoalFlag,
        Collider { half_w: 30.0, half_h: 30.0 },
        Sprite {
            image: flag_tex.clone(),
            custom_size: Some(Vec2::splat(64.0)),
            ..default()
        },
        Transform::from_xyz(2250.0, -278.0, 0.5),
    ));
}

fn spawn_level_3(
    commands: &mut Commands,
    terrain: &Handle<Image>,
    fruit_tex: &Handle<Image>,
    fruit_layout: &Handle<TextureAtlasLayout>,
    flag_tex: &Handle<Image>,
    spike_tex: &Handle<Image>,
) {
    // Strawberry Cave — dark underground, moving platforms, large gaps

    // Long cave floor
    spawn_platform(commands, terrain, 1250.0, -320.0, 4800.0);

    // 5 static cave platforms (warm stone tint)
    for (x, y, w) in [
        (-250.0_f32, -248.0_f32, 160.0_f32),
        (580.0,  -158.0, 140.0),
        (1380.0, -172.0, 140.0),
        (2200.0, -155.0, 140.0),
        (2640.0, -225.0, 140.0),
    ] {
        spawn_platform_tinted(commands, terrain, x, y, w, Color::srgb(1.0, 0.75, 0.5));
    }

    // 2 horizontal movers (lava-orange tint)
    spawn_moving_platform(commands, terrain, 200.0,  -200.0, 130.0, 70.0, 1.1, true,  Color::srgb(1.0, 0.45, 0.2));
    spawn_moving_platform(commands, terrain, 1800.0, -215.0, 120.0, 90.0, 1.4, true,  Color::srgb(1.0, 0.45, 0.2));

    // 1 vertical mover (glowing yellow-green tint)
    spawn_moving_platform(commands, terrain, 1010.0, -225.0, 120.0, 65.0, 0.9, false, Color::srgb(0.6, 1.0, 0.4));

    // Strawberries above each platform
    for (x, y) in [
        (-250.0_f32, -198.0_f32),
        (200.0,  -150.0),
        (580.0,  -108.0),
        (1010.0, -160.0),
        (1380.0, -122.0),
        (1800.0, -165.0),
        (2200.0, -105.0),
        (2640.0, -175.0),
    ] {
        spawn_fruit(commands, fruit_tex, fruit_layout, x, y);
    }

    // Spikes on the cave floor — guarding the gaps
    for x in [0.0_f32, 420.0, 810.0, 1220.0, 1620.0, 2020.0, 2440.0] {
        spawn_hazard(commands, spike_tex, x, -302.0);
    }

    commands.spawn((
        GameEntity,
        GoalFlag,
        Collider { half_w: 30.0, half_h: 30.0 },
        Sprite {
            image: flag_tex.clone(),
            custom_size: Some(Vec2::splat(64.0)),
            ..default()
        },
        Transform::from_xyz(2950.0, -278.0, 0.5),
    ));
}

fn spawn_platform(
    commands: &mut Commands,
    terrain: &Handle<Image>,
    x: f32,
    y: f32,
    width: f32,
) {
    commands.spawn((
        GameEntity,
        Platform,
        Collider {
            half_w: width / 2.0,
            half_h: PLATFORM_H / 2.0,
        },
        Sprite {
            image: terrain.clone(),
            rect: Some(Rect::new(112.0, 0.0, 128.0, 16.0)),
            custom_size: Some(Vec2::new(width, PLATFORM_H)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0,
            },
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
    ));
}

fn spawn_coin(
    commands: &mut Commands,
    coin_tex: &Handle<Image>,
    coin_layout: &Handle<TextureAtlasLayout>,
    x: f32,
    y: f32,
) {
    commands.spawn((
        GameEntity,
        Collectible,
        Collider {
            half_w: 7.0,
            half_h: 7.0,
        },
        AnimationIndices { first: 0, last: 11 },
        AnimationTimer(Timer::from_seconds(0.07, TimerMode::Repeating)),
        Sprite {
            image: coin_tex.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: coin_layout.clone(),
                index: 0,
            }),
            custom_size: Some(Vec2::splat(32.0)),
            ..default()
        },
        Transform::from_xyz(x, y, 0.5),
    ));
}

/// Spawn a collectible fruit (17-frame 32×32 Pixel Adventure sprite).
fn spawn_fruit(
    commands: &mut Commands,
    fruit_tex: &Handle<Image>,
    fruit_layout: &Handle<TextureAtlasLayout>,
    x: f32,
    y: f32,
) {
    commands.spawn((
        GameEntity,
        Collectible,
        Collider { half_w: 12.0, half_h: 12.0 },
        AnimationIndices { first: 0, last: 16 },
        AnimationTimer(Timer::from_seconds(0.06, TimerMode::Repeating)),
        Sprite {
            image: fruit_tex.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: fruit_layout.clone(),
                index: 0,
            }),
            custom_size: Some(Vec2::splat(36.0)),
            ..default()
        },
        Transform::from_xyz(x, y, 0.5),
    ));
}

/// Spawn a static platform with a color tint.
fn spawn_platform_tinted(
    commands: &mut Commands,
    terrain: &Handle<Image>,
    x: f32,
    y: f32,
    width: f32,
    color: Color,
) {
    commands.spawn((
        GameEntity,
        Platform,
        Collider {
            half_w: width / 2.0,
            half_h: PLATFORM_H / 2.0,
        },
        Sprite {
            image: terrain.clone(),
            color,
            rect: Some(Rect::new(112.0, 0.0, 128.0, 16.0)),
            custom_size: Some(Vec2::new(width, PLATFORM_H)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0,
            },
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
    ));
}

/// Spawn a platform that oscillates back and forth.
fn spawn_moving_platform(
    commands: &mut Commands,
    terrain: &Handle<Image>,
    x: f32,
    y: f32,
    width: f32,
    amplitude: f32,
    speed: f32,
    horizontal: bool,
    color: Color,
) {
    commands.spawn((
        GameEntity,
        Platform,
        MovingPlatform {
            start_x: x,
            start_y: y,
            amplitude,
            speed,
            horizontal,
            elapsed: 0.0,
            delta: Vec2::ZERO,
        },
        Collider {
            half_w: width / 2.0,
            half_h: PLATFORM_H / 2.0,
        },
        Sprite {
            image: terrain.clone(),
            color,
            rect: Some(Rect::new(112.0, 0.0, 128.0, 16.0)),
            custom_size: Some(Vec2::new(width, PLATFORM_H)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0,
            },
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
    ));
}

fn spawn_hazard(commands: &mut Commands, spike_tex: &Handle<Image>, x: f32, y: f32) {
    commands.spawn((
        GameEntity,
        Hazard,
        Collider { half_w: 8.0, half_h: 8.0 },
        Sprite {
            image: spike_tex.clone(),
            custom_size: Some(Vec2::splat(28.0)),
            ..default()
        },
        Transform::from_xyz(x, y, 0.3),
    ));
}

fn spawn_fire(
    commands: &mut Commands,
    fire_tex: &Handle<Image>,
    fire_layout: &Handle<TextureAtlasLayout>,
    x: f32,
    y: f32,
) {
    commands.spawn((
        GameEntity,
        Hazard,
        Collider { half_w: 8.0, half_h: 20.0 },
        AnimationIndices { first: 0, last: 2 },
        AnimationTimer(Timer::from_seconds(0.12, TimerMode::Repeating)),
        Sprite {
            image: fire_tex.clone(),
            texture_atlas: Some(TextureAtlas { layout: fire_layout.clone(), index: 0 }),
            custom_size: Some(Vec2::new(32.0, 64.0)),
            ..default()
        },
        Transform::from_xyz(x, y, 0.3),
    ));
}

fn spawn_saw(
    commands: &mut Commands,
    saw_tex: &Handle<Image>,
    saw_layout: &Handle<TextureAtlasLayout>,
    x: f32,
    y: f32,
    amplitude: f32,
    speed: f32,
) {
    commands.spawn((
        GameEntity,
        Hazard,
        MovingHazard { start_x: x, amplitude, speed, elapsed: 0.0 },
        Collider { half_w: 18.0, half_h: 18.0 },
        AnimationIndices { first: 0, last: 7 },
        AnimationTimer(Timer::from_seconds(0.06, TimerMode::Repeating)),
        Sprite {
            image: saw_tex.clone(),
            texture_atlas: Some(TextureAtlas { layout: saw_layout.clone(), index: 0 }),
            custom_size: Some(Vec2::splat(48.0)),
            ..default()
        },
        Transform::from_xyz(x, y, 0.4),
    ));
}

fn spawn_monster(
    commands: &mut Commands,
    monster_tex: &Handle<Image>,
    monster_layout: &Handle<TextureAtlasLayout>,
    x: f32,
    y: f32,
    start_x: f32,
    end_x: f32,
    speed: f32,
) {
    commands.spawn((
        GameEntity,
        Monster { start_x, end_x, speed, dir: 1.0 },
        Hazard,
        Collider { half_w: 14.0, half_h: 14.0 },
        AnimationIndices { first: 0, last: 11 },
        AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
        Sprite {
            image: monster_tex.clone(),
            texture_atlas: Some(TextureAtlas { layout: monster_layout.clone(), index: 0 }),
            custom_size: Some(Vec2::splat(48.0)),
            ..default()
        },
        Transform::from_xyz(x, y, 0.6),
    ));
}

fn spawn_falling_platform(commands: &mut Commands, fall_plat_tex: &Handle<Image>, x: f32, y: f32) {
    let width = 96.0;
    commands.spawn((
        GameEntity,
        Platform,
        FallingPlatform { original_x: x, warned: false, timer: 0.6, falling: false, fall_velocity: 0.0 },
        Collider { half_w: width / 2.0, half_h: 10.0 },
        Sprite {
            image: fall_plat_tex.clone(),
            color: Color::srgb(0.95, 0.72, 0.25),
            custom_size: Some(Vec2::new(width, 20.0)),
            image_mode: SpriteImageMode::Tiled { tile_x: true, tile_y: false, stretch_value: 1.0 },
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
    ));
}

#[allow(clippy::too_many_arguments)]
fn spawn_level_4(
    commands: &mut Commands,
    terrain: &Handle<Image>,
    apple_tex: &Handle<Image>,
    apple_layout: &Handle<TextureAtlasLayout>,
    flag_tex: &Handle<Image>,
    spike_tex: &Handle<Image>,
    saw_tex: &Handle<Image>,
    saw_layout: &Handle<TextureAtlasLayout>,
    fire_tex: &Handle<Image>,
    fire_layout: &Handle<TextureAtlasLayout>,
    monster_tex: &Handle<Image>,
    monster_layout: &Handle<TextureAtlasLayout>,
    fall_plat_tex: &Handle<Image>,
) {
    // Long volcanic ground floor
    spawn_platform(commands, terrain, 1900.0, -320.0, 6400.0);

    // Static platforms — dark volcanic stone tint
    let stone = Color::srgb(0.80, 0.38, 0.18);
    for (x, y, w) in [
        (-300.0_f32, -248.0_f32, 200.0_f32),
        (150.0, -215.0, 160.0),
        (650.0, -180.0, 150.0),
        (1100.0, -168.0, 140.0),
        (1630.0, -192.0, 140.0),
        (2060.0, -172.0, 140.0),
        (2560.0, -188.0, 140.0),
        (3110.0, -168.0, 150.0),
        (3620.0, -202.0, 200.0),
    ] {
        spawn_platform_tinted(commands, terrain, x, y, w, stone);
    }

    // Moving platforms — lava-orange tint, faster than previous levels
    let lava = Color::srgb(1.0, 0.48, 0.18);
    spawn_moving_platform(commands, terrain, 420.0,  -198.0, 110.0, 90.0, 2.0, true,  lava);
    spawn_moving_platform(commands, terrain, 870.0,  -202.0, 110.0, 80.0, 2.2, true,  lava);
    spawn_moving_platform(commands, terrain, 1365.0, -188.0, 110.0, 70.0, 1.8, true,  lava);
    spawn_moving_platform(commands, terrain, 1850.0, -172.0, 100.0, 65.0, 2.5, true,  lava);
    // Vertical mover — glowing green
    spawn_moving_platform(commands, terrain, 2315.0, -168.0, 100.0, 55.0, 1.4, false,
        Color::srgb(0.4, 1.0, 0.35));
    spawn_moving_platform(commands, terrain, 2830.0, -182.0, 100.0, 75.0, 2.0, true,  lava);
    spawn_moving_platform(commands, terrain, 3355.0, -178.0, 100.0, 78.0, 2.3, true,  lava);

    // Falling platforms (shake then drop after the player lands)
    spawn_falling_platform(commands, fall_plat_tex, -80.0, -268.0);   // early gap
    spawn_falling_platform(commands, fall_plat_tex,  560.0, -228.0);  // mid-level
    spawn_falling_platform(commands, fall_plat_tex, 1920.0, -228.0);  // late-level

    // Spinning saws (moving hazards)
    spawn_saw(commands, saw_tex, saw_layout,  280.0, -245.0, 60.0, 3.0);
    spawn_saw(commands, saw_tex, saw_layout,  760.0, -228.0, 70.0, 3.5);
    spawn_saw(commands, saw_tex, saw_layout, 1480.0, -225.0, 65.0, 3.2);
    spawn_saw(commands, saw_tex, saw_layout, 2700.0, -222.0, 60.0, 3.8);

    // Fire hazards on the ground floor
    for x in [50.0_f32, 345.0, 720.0, 1200.0, 1755.0, 2450.0, 3010.0] {
        spawn_fire(commands, fire_tex, fire_layout, x, -278.0);
    }

    // Spike clusters in the gaps
    for x in [-100.0_f32, 228.0, 512.0, 988.0, 1448.0, 2112.0, 2688.0, 3258.0] {
        spawn_hazard(commands, spike_tex, x, -302.0);
    }

    // Patrolling monsters on platforms
    // Platform top = platform_center_y + 10. Monster center_y = plat_top + 24 (half of 48px sprite).
    // Platform (650, -180): top = -170. Monster y = -170 + 24 = -146.
    spawn_monster(commands, monster_tex, monster_layout, 625.0, -146.0, 568.0, 732.0, 88.0);
    // Platform (2060, -172): top = -162. Monster y = -138.
    spawn_monster(commands, monster_tex, monster_layout, 2040.0, -138.0, 1968.0, 2142.0, 96.0);
    // Platform (3110, -168): top = -158. Monster y = -134.
    spawn_monster(commands, monster_tex, monster_layout, 3090.0, -134.0, 3018.0, 3192.0, 92.0);

    // Apples — 8 collectibles, some in risky spots
    for (x, y) in [
        (-300.0_f32, -218.0_f32),
        ( 150.0,     -185.0),
        ( 420.0,     -148.0),  // on the first moving platform
        (1100.0,     -138.0),
        (1630.0,     -162.0),
        (2315.0,     -108.0),  // above the vertical mover — skill shot
        (3110.0,     -138.0),  // near the monster
        (3620.0,     -172.0),  // near the flag
    ] {
        spawn_fruit(commands, apple_tex, apple_layout, x, y);
    }

    // Goal flag
    commands.spawn((
        GameEntity,
        GoalFlag,
        Collider { half_w: 30.0, half_h: 30.0 },
        Sprite {
            image: flag_tex.clone(),
            custom_size: Some(Vec2::splat(64.0)),
            ..default()
        },
        Transform::from_xyz(3860.0, -278.0, 0.5),
    ));
}

// ── Level Theme & Moving Platforms ───────────────────────────────────────────

fn set_level_theme(current_level: Res<CurrentLevel>, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = match current_level.0 {
        2 => Color::srgb(0.55, 0.82, 1.0),  // bright cloud-sky blue
        3 => Color::srgb(0.12, 0.08, 0.18), // dark purple cave
        4 => Color::srgb(0.22, 0.06, 0.04), // volcanic red sky
        _ => Color::srgb(0.4, 0.7, 1.0),    // normal sky
    };
}

fn update_moving_platforms(
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

fn update_monsters(
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

fn update_moving_hazards(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut MovingHazard)>,
) {
    for (mut t, mut mh) in &mut query {
        mh.elapsed += time.delta_secs();
        t.translation.x = mh.start_x + (mh.elapsed * mh.speed).sin() * mh.amplitude;
    }
}

fn update_falling_platforms(
    time: Res<Time>,
    mut commands: Commands,
    player_q: Query<(&Transform, &Collider, &Grounded), With<Player>>,
    mut fp_q: Query<
        (Entity, &mut Transform, &mut FallingPlatform, &Collider),
        Without<Player>,
    >,
) {
    let Ok((pt, pc, grounded)) = player_q.get_single() else { return; };
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

// ── Gameplay Systems ──────────────────────────────────────────────────────────

fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    sounds: Res<SoundAssets>,
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (
            &mut Player,
            &mut Velocity,
            &mut Grounded,
            &mut CoyoteFrames,
            &mut DashState,
            &mut AnimState,
            &mut AnimationIndices,
            &mut AnimationTimer,
            &mut Sprite,
            &PlayerAnimations,
            &Transform,
        ),
        With<Player>,
    >,
) {
    let Ok((mut player, mut vel, mut grounded, mut coyote, mut dash, mut anim_state, mut indices, mut timer, mut sprite, anims, transform)) =
        query.get_single_mut()
    else {
        return;
    };

    let dt = time.delta_secs();

    // Tick dash timers
    dash.active = (dash.active - dt).max(0.0);
    dash.cooldown = (dash.cooldown - dt).max(0.0);

    let left = keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA);
    let right = keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD);

    // Sprint toggle
    if keyboard.just_pressed(KeyCode::ShiftLeft) || keyboard.just_pressed(KeyCode::ShiftRight) {
        player.is_sprinting = !player.is_sprinting;
    }

    // Start a new dash (moved to KeyF to avoid conflict with Shift)
    if keyboard.just_pressed(KeyCode::KeyF) && dash.cooldown == 0.0 {
        dash.dir = if sprite.flip_x { -1.0 } else { 1.0 };
        dash.active = DASH_DURATION;
        dash.cooldown = DASH_COOLDOWN;

        // Burst of trail particles
        let origin = transform.translation.truncate();
        for i in 0..8_usize {
            let offset = Vec2::new(i as f32 * -8.0 * dash.dir, (i as f32 - 4.0) * 4.0);
            let lifetime = 0.15 + i as f32 * 0.03;
            commands.spawn((
                Particle { lifetime, max_lifetime: lifetime },
                Velocity(Vec2::new(-dash.dir * 40.0, 0.0)),
                Sprite::from_color(Color::srgba(0.5, 0.8, 1.0, 0.8), Vec2::splat(10.0 - i as f32)),
                Transform::from_xyz(origin.x + offset.x, origin.y + offset.y, 0.9),
            ));
        }
    }

    // While dashing, override horizontal velocity
    if dash.active > 0.0 {
        vel.0.x = dash.dir * DASH_SPEED;
    } else {
        let speed = if player.is_sprinting { SPRINT_SPEED } else { MOVE_SPEED };
        vel.0.x = if left {
            -speed
        } else if right {
            speed
        } else {
            0.0
        };
    }

    if left {
        sprite.flip_x = true;
    }
    if right {
        sprite.flip_x = false;
    }

    let jump_pressed = keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::KeyW);
    if jump_pressed && grounded.0 {
        vel.0.y = JUMP_VELOCITY;
        grounded.0 = false;
        commands.spawn((
            AudioPlayer::<AudioSource>(sounds.jump.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }

    // Track how many frames we've been ungrounded; gives a short buffer
    // before switching to Fall so rising-platform descent doesn't cause a
    // one-frame flicker.
    if grounded.0 {
        coyote.0 = 0;
    } else if coyote.0 < 255 {
        coyote.0 += 1;
    }
    let effectively_grounded = grounded.0 || coyote.0 <= 2;

    let target = if *anim_state == AnimState::Celebrate {
        AnimState::Celebrate
    } else if !effectively_grounded && vel.0.y > 0.0 {
        AnimState::Jump
    } else if !effectively_grounded && vel.0.y <= 0.0 {
        AnimState::Fall
    } else if vel.0.x != 0.0 {
        if player.is_sprinting { AnimState::Run } else { AnimState::Walk }
    } else {
        AnimState::Idle
    };

    if *anim_state != target {
        *anim_state = target;
        let (new_first, new_last, new_secs) = match *anim_state {
            AnimState::Idle => (anims.idle_first, anims.idle_last, anims.idle_secs),
            AnimState::Walk => (anims.walk_first, anims.walk_last, anims.walk_secs),
            AnimState::Run => (anims.run_first, anims.run_last, anims.run_secs),
            AnimState::Jump => (anims.jump_first, anims.jump_last, 0.1),
            AnimState::Fall => (anims.fall_first, anims.fall_last, 0.1),
            AnimState::Celebrate => (anims.celebrate_first, anims.celebrate_last, anims.celebrate_secs),
        };
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = new_first;
        }
        *indices = AnimationIndices {
            first: new_first,
            last: new_last,
        };
        *timer = AnimationTimer(Timer::from_seconds(new_secs, TimerMode::Repeating));
    }
}

fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut vel in &mut query {
        vel.0.y = (vel.0.y + GRAVITY * time.delta_secs()).max(-800.0);
    }
}

fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform), With<Player>>,
) {
    for (vel, mut t) in &mut query {
        t.translation.x += vel.0.x * time.delta_secs();
        t.translation.y += vel.0.y * time.delta_secs();
    }
}

fn platform_collision(
    time: Res<Time>,
    mut player_q: Query<
        (&mut Transform, &mut Velocity, &mut Grounded, &Collider),
        With<Player>,
    >,
    platform_q: Query<(&Transform, &Collider, Option<&MovingPlatform>), (With<Platform>, Without<Player>)>,
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

fn keep_in_bounds(
    current_level: Res<CurrentLevel>,
    sounds: Res<SoundAssets>,
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Velocity, &mut Grounded), With<Player>>,
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

fn collectible_collision(
    mut commands: Commands,
    sounds: Res<SoundAssets>,
    player_q: Query<(&Transform, &Collider), With<Player>>,
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

fn hazard_collision(
    sounds: Res<SoundAssets>,
    mut commands: Commands,
    mut player_q: Query<(&mut Transform, &mut Velocity, &mut Grounded, &Collider), With<Player>>,
    hazard_q: Query<(&Transform, &Collider), (With<Hazard>, Without<Player>)>,
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

fn goal_detection(
    mut player_q: Query<(&Transform, &Collider, &mut AnimState, &mut Velocity), With<Player>>,
    flag_q: Query<(&Transform, &Collider), With<GoalFlag>>,
    mut score: ResMut<Score>,
    level_timer: Res<LevelTimer>,
    mut high_score: ResMut<HighScore>,
    mut levels_beaten: ResMut<LevelsBeaten>,
    current_level: Res<CurrentLevel>,
    mut next_state: ResMut<NextState<GameState>>,
) {
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
        let time_bonus =
            ((TIME_PAR - level_timer.0).max(0.0) as u32) * TIME_BONUS_PER_SEC;
        let final_score = coins * COIN_VALUE + time_bonus;
        score.0 = final_score;

        if final_score > high_score.0 {
            high_score.0 = final_score;
        }

        // Advance progress if this is a new level beaten
        if current_level.0 as u32 > levels_beaten.0 {
            levels_beaten.0 = current_level.0 as u32;
        }

        write_save(&SaveData {
            high_score: high_score.0,
            levels_beaten: levels_beaten.0,
        });

        next_state.set(GameState::Won);
    }
}

fn camera_follow(
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

fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index >= indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

fn tick_level_timer(time: Res<Time>, mut level_timer: ResMut<LevelTimer>) {
    level_timer.0 += time.delta_secs();
}

fn update_score_ui(
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

// ── Win Screen ────────────────────────────────────────────────────────────────

fn show_win_screen(
    mut commands: Commands,
    sounds: Res<SoundAssets>,
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

fn spawn_fireworks(commands: &mut Commands, origin: Vec2) {
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

fn update_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Velocity, &mut Sprite, &mut Particle)>,
) {
    let dt = time.delta_secs();
    for (entity, mut transform, mut vel, mut sprite, mut particle) in &mut query {
        particle.lifetime -= dt;
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        vel.0.y += GRAVITY * 0.25 * dt;
        transform.translation.x += vel.0.x * dt;
        transform.translation.y += vel.0.y * dt;
        let alpha = (particle.lifetime / particle.max_lifetime).max(0.0);
        sprite.color = sprite.color.with_alpha(alpha);
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

fn restart_input(
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

fn cleanup_win_screen(mut commands: Commands, win_q: Query<Entity, With<WinScreen>>) {
    for e in &win_q {
        commands.entity(e).despawn_recursive();
    }
}

// ── Mute System ───────────────────────────────────────────────────────────────

fn mute_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<MuteButton>)>,
    mut label_q: Query<&mut Text, With<MuteButtonLabel>>,
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
        for mut text in &mut label_q {
            text.0 = if muted.0 { "✕".to_string() } else { "♪".to_string() };
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn aabb_overlap(a_pos: Vec2, a_half: Vec2, b_pos: Vec2, b_half: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() < a_half.x + b_half.x
        && (a_pos.y - b_pos.y).abs() < a_half.y + b_half.y
}

fn load_save() -> SaveData {
    let Ok(s) = std::fs::read_to_string(SAVE_PATH) else {
        return SaveData {
            high_score: 0,
            levels_beaten: 0,
        };
    };
    serde_json::from_str::<SaveData>(&s).unwrap_or(SaveData {
        high_score: 0,
        levels_beaten: 0,
    })
}

fn write_save(data: &SaveData) {
    if let Ok(json) = serde_json::to_string_pretty(data) {
        let _ = std::fs::write(SAVE_PATH, json);
    }
}
