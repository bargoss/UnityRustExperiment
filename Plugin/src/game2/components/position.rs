use super::super::data_types::*;
use bevy_ecs::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct WorldPosition {
    pub pos: Vec2FFloat,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct TilePosition {
    pub collider_id: u32,
    pub pos: Vector2Int,
    pub scale: Vector2Int,
}
