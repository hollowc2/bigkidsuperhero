//! Map screen — level selection with animated nodes.

use bevy::prelude::*;

use crate::components::{
    MapCursor, MapEntity, MapNode, MapSelection, OrbGlow, LevelsBeaten, PulseOrb,
};

// Map screen node world-positions (4 levels for now, can extend to 7)
const NODE_X: [f32; 4] = [-450.0, -150.0, 150.0, 450.0];
const NODE_Y: f32 = 30.0;

/// Spawn the map selection screen.
pub fn spawn_map_screen(
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

    // Background stars
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

    // Dotted path connecting all nodes
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

    // Cursor halo
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
            (Color::srgb(1.0, 0.78, 0.0), false)
        } else if level_num == lb + 1 {
            (Color::srgb(0.18, 0.92, 0.28), false)
        } else {
            (Color::srgb(0.20, 0.20, 0.24), true)
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

        // Caption
        commands.spawn((
            MapEntity,
            Text2d::new(format!("Level {}", level_num)),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
            Transform::from_xyz(x, NODE_Y - 85.0, 1.0),
        ));
    }
}

/// Handle map screen input.
pub fn map_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    levels_beaten: Res<LevelsBeaten>,
    mut map_selection: ResMut<MapSelection>,
    mut next_state: ResMut<NextState<crate::components::GameState>>,
    mut current_level: ResMut<crate::components::CurrentLevel>,
) {
    use crate::components::GameState;

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

/// Animate map orbs pulsing.
pub fn animate_map_orbs(time: Res<Time>, mut q: Query<(&mut Transform, &mut PulseOrb)>) {
    for (mut t, mut orb) in &mut q {
        orb.phase += time.delta_secs() * orb.speed;
        let s = 1.0 + orb.phase.sin() * orb.pulse_amount;
        t.scale = Vec3::splat(s.max(0.01));
    }
}

/// Animate the cursor lerping to the selected node.
pub fn animate_map_cursor(
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

/// Despawn map screen entities.
pub fn cleanup_map_screen(mut commands: Commands, q: Query<Entity, With<MapEntity>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}
