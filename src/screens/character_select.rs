//! Character selection screen.

use bevy::prelude::*;

use crate::components::{
    CharacterPreviewAnim, CharacterPreviewImage, CharacterSelectEntity, CharacterSelectIndex,
    CharacterSelectItem, GameState, SelectedCharacter,
};

/// Spawn the character select screen.
pub fn spawn_character_select(
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

            // Right panel: large idle-frame preview
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

/// Handle character selection input.
pub fn character_select_input(
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

/// Animate the character preview sprite.
pub fn animate_character_preview(
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

/// Despawn character select entities.
pub fn cleanup_character_select(mut commands: Commands, q: Query<Entity, With<CharacterSelectEntity>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}
