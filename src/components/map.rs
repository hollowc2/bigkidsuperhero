//! Map screen components.

use bevy::prelude::*;

/// Tags map screen entities.
#[derive(Component)]
pub struct MapEntity;

/// The sliding selection cursor on the map screen.
#[derive(Component)]
pub struct MapCursor;

/// Marks a level node sprite on the map screen.
#[derive(Component)]
pub struct MapNode;

/// Stores the unscrolled map position for entities that move with the level path.
#[derive(Component)]
pub struct MapScrollItem {
    pub world_x: f32,
    pub world_y: f32,
}

/// Drives the breathing pulse animation on map orbs and the cursor halo.
#[derive(Component)]
pub struct PulseOrb {
    pub phase: f32,
    pub speed: f32,
    pub pulse_amount: f32,
}

/// Marks the outer glow ring behind a level node orb.
#[derive(Component)]
pub struct OrbGlow;
