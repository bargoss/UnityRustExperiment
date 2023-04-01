
use bevy_ecs::prelude::*;
use crate::game_core::common::*;
use crate::game_core::math::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SphereView {
    pub radius: FixedPoint,
    pub view_custom_id: Id
}