use crate::game_core::common::id::Id;

use bevy_ecs::prelude::*;
use crate::game_core::math::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Beam {
    pub a: Id,
    pub b: Id,
    pub length: FixedPoint,
}