use super::super::data_types::*;
use bevy_ecs::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct TerrainCollider {
    pub collider_id: u32,
    pub pos: Vector2Int,
    pub scale: Vector2Int,
}
