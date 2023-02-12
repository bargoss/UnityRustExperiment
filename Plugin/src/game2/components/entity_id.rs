use super::super::data_types::*;
use bevy_ecs::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct EntityId {
    pub id: u32,
}