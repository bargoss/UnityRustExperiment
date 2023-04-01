
use bevy_ecs::prelude::*;
use crate::game_core::math::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Position {
    pub value: FP2
}