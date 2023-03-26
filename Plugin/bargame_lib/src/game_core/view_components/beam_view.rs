use crate::game_core::components;
use super::*;
use bevy_ecs::prelude::*;
use crate::game_core::common::*;
use crate::game_core::math::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct LineView {
    pub thickness : FixedPoint,
    pub start : Id,
    pub end : Id,
    pub view_custom_id : Id
}