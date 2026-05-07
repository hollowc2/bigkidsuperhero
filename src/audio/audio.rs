//! Audio — background music, sound effects, mute button.

use bevy::prelude::*;

use crate::components::{
    BackgroundMusic, MuteButton, MuteButtonLabel, MuteButtonSlash, MusicMuted, SoundAssets,
};

/// Setup audio: load sounds, start background music, create mute button.
#[allow(dead_code)]
pub fn setup_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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

    // Mute button — top right corner, always visible
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
                Node {
                    position_type: PositionType::Relative,
                    width: Val::Px(30.0),
                    height: Val::Px(26.0),
                    ..default()
                },
            ))
            .with_children(|icon| {
                icon.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(4.0),
                        top: Val::Px(10.0),
                        width: Val::Px(7.0),
                        height: Val::Px(8.0),
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                ));
                icon.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(11.0),
                        top: Val::Px(7.0),
                        width: Val::Px(7.0),
                        height: Val::Px(14.0),
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                ));
                icon.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(22.0),
                        top: Val::Px(8.0),
                        width: Val::Px(3.0),
                        height: Val::Px(12.0),
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                    BorderRadius::all(Val::Px(2.0)),
                ));
                icon.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(27.0),
                        top: Val::Px(5.0),
                        width: Val::Px(3.0),
                        height: Val::Px(18.0),
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                    BorderRadius::all(Val::Px(2.0)),
                ));
                icon.spawn((
                    MuteButtonSlash,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(10.0),
                        top: Val::Px(-10.0),
                        ..default()
                    },
                    Text::new("/"),
                    TextFont { font_size: 38.0, ..default() },
                    TextColor(Color::srgb(1.0, 0.25, 0.2)),
                    Visibility::Hidden,
                ));
            });
        });
}

/// Toggle mute on key press or button click.
#[allow(dead_code)]
pub fn mute_system(
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
