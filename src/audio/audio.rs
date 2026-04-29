//! Audio — background music, sound effects, mute button.

use bevy::prelude::*;

use crate::components::{
    BackgroundMusic, MuteButton, MuteButtonLabel, MusicMuted, SoundAssets,
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
                Text::new("♪"),
                TextFont { font_size: 26.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
}

/// Toggle mute on key press or button click.
#[allow(dead_code)]
pub fn mute_system(
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
