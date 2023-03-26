use crate::game_core::common::id::Id;
use super::*;
use bevy_ecs::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct NetId {
    pub value: Id,
}
