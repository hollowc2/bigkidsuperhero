//! Player control — input, movement, animation state.

use bevy::prelude::*;
use rand::RngExt;

use crate::components::{
    AnimState, AnimationIndices, AnimationTimer, DashState, Grounded, Player,
    PlayerAnimations, Velocity, CoyoteFrames, SelectedCharacter, SoundAssets,
};
use bevy::prelude::{Sprite, Transform, TextureAtlasLayout, TextureAtlas, UVec2, Timer, TimerMode, AudioPlayer, AudioSource, PlaybackSettings};

const GRAVITY: f32 = -900.0;
const JUMP_VELOCITY: f32 = 420.0;
const MOVE_SPEED: f32 = 220.0;
const SPRINT_SPEED: f32 = 370.0;
const PLAYER_HALF_W: f32 = 14.0;
const PLAYER_HALF_H: f32 = 40.0;
const DASH_SPEED: f32 = 560.0;
const DASH_DURATION: f32 = 0.18;
const DASH_COOLDOWN: f32 = 0.9;
const SPRINT_DURATION: f32 = 1.0;
const SPRINT_COOLDOWN: f32 = 1.5;
const SPARKLE_INTERVAL: f32 = 0.05;

/// Handle player input — movement, jump, sprint, dash.
#[allow(clippy::too_many_arguments)]
pub fn player_input(
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

    if *anim_state == AnimState::Celebrate {
        return;
    }

    let dt = time.delta_secs();

    // Tick dash timers
    dash.active = (dash.active - dt).max(0.0);
    dash.cooldown = (dash.cooldown - dt).max(0.0);

    let left = keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA);
    let right = keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD);

    // Sprint burst: press Shift for a 1-second speed burst
    player.sprint_cooldown = (player.sprint_cooldown - dt).max(0.0);
    if (keyboard.just_pressed(KeyCode::ShiftLeft) || keyboard.just_pressed(KeyCode::ShiftRight))
        && player.sprint_cooldown == 0.0
        && player.sprint_timer == 0.0
    {
        player.sprint_timer = SPRINT_DURATION;
    }
    if player.sprint_timer > 0.0 {
        player.sprint_timer = (player.sprint_timer - dt).max(0.0);
        if player.sprint_timer == 0.0 {
            player.sprint_cooldown = SPRINT_COOLDOWN;
        }
    }
    let is_sprinting = player.sprint_timer > 0.0;

    // Sparkle trail while sprinting
    if is_sprinting {
        player.sparkle_timer -= dt;
        if player.sparkle_timer <= 0.0 {
            player.sparkle_timer = SPARKLE_INTERVAL;
            let mut rng = rand::rng();
            let behind = if sprite.flip_x { 1.0 } else { -1.0 };
            for _ in 0..2 {
                let vx: f32 = rng.random_range(-30.0..30.0);
                let vy: f32 = rng.random_range(30.0..90.0);
                let sz: f32 = rng.random_range(3.0..8.0);
                let lt: f32 = rng.random_range(0.2..0.45);
                let ox: f32 = rng.random_range(0.0..18.0) * behind;
                let oy: f32 = rng.random_range(-20.0..20.0);
                let hue: f32 = rng.random_range(40.0_f32..70.0);
                commands.spawn((
                    crate::components::Particle { lifetime: lt, max_lifetime: lt },
                    Velocity(Vec2::new(vx, vy)),
                    Sprite::from_color(Color::hsla(hue, 1.0, 0.65, 0.9), Vec2::splat(sz)),
                    Transform::from_xyz(
                        transform.translation.x + ox,
                        transform.translation.y + oy,
                        0.85,
                    ),
                ));
            }
        }
    } else {
        player.sparkle_timer = 0.0;
    }

    // Dash (F key)
    if keyboard.just_pressed(KeyCode::KeyF) && dash.cooldown == 0.0 {
        dash.dir = if sprite.flip_x { -1.0 } else { 1.0 };
        dash.active = DASH_DURATION;
        dash.cooldown = DASH_COOLDOWN;

        let origin = transform.translation.truncate();
        for i in 0..8_usize {
            let offset = Vec2::new(i as f32 * -8.0 * dash.dir, (i as f32 - 4.0) * 4.0);
            let lifetime = 0.15 + i as f32 * 0.03;
            commands.spawn((
                crate::components::Particle { lifetime, max_lifetime: lifetime },
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
        let speed = if is_sprinting { SPRINT_SPEED } else { MOVE_SPEED };
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

    // Coyote time: track frames ungrounded
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
        if is_sprinting { AnimState::Run } else { AnimState::Walk }
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

/// Apply gravity to the player.
pub fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut vel in &mut query {
        vel.0.y = (vel.0.y + GRAVITY * time.delta_secs()).max(-800.0);
    }
}

/// Apply velocity to transform.
pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform), With<Player>>,
) {
    for (vel, mut t) in &mut query {
        t.translation.x += vel.0.x * time.delta_secs();
        t.translation.y += vel.0.y * time.delta_secs();
    }
}

/// Spawn the player entity with all its components.
pub fn spawn_player(
    commands: &mut Commands,
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    selected_character: SelectedCharacter,
) {
    let player_tex = match selected_character {
        SelectedCharacter::Bridget => asset_server.load("bridget/bridget_sprite.png"),
        SelectedCharacter::Calvin => asset_server.load("calvin/calvin_sprite.png"),
    };
    let player_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(128, 128),
        5,
        5,
        None,
        None,
    ));

    commands.spawn((
        crate::components::GameEntity,
        Player { sprint_timer: 0.0, sprint_cooldown: 0.0, sparkle_timer: 0.0 },
        Velocity(Vec2::ZERO),
        Grounded(false),
        CoyoteFrames(0),
        DashState::default(),
        crate::components::Collider {
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
}
