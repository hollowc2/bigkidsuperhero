//! Menu screen — title, new game, load, exit.

use bevy::prelude::*;

use crate::components::{
    GameState, HighScore, LevelsBeaten, MenuEntity, MenuItem, MenuSelection, SaveData,
};
use crate::systems::persistence::{load_save, write_save};

/// Spawn the main menu screen.
pub fn spawn_menu_screen(mut commands: Commands, mut menu_selection: ResMut<MenuSelection>) {
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

/// Handle keyboard input on the menu screen.
pub fn menu_input(
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

/// Despawn menu entities.
pub fn cleanup_menu_screen(mut commands: Commands, q: Query<Entity, With<MenuEntity>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}
