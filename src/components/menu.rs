//! Menu and UI navigation components.

use bevy::prelude::*;

/// Tags menu screen entities.
#[derive(Component)]
pub struct MenuEntity;

/// Identifies a menu item (0=New Game, 1=Load, 2=Exit).
#[derive(Component)]
pub struct MenuItem(pub usize);

/// Tags character-select screen entities.
#[derive(Component)]
pub struct CharacterSelectEntity;

/// Identifies a character option (0=Bridget, 1=Calvin).
#[derive(Component)]
pub struct CharacterSelectItem(pub usize);

/// Marks the large character preview image in the character select UI.
#[derive(Component)]
pub struct CharacterPreviewImage;
