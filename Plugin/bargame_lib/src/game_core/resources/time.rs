use bevy_ecs::prelude::Resource;
use crate::game_core::math::FixedPoint;

#[derive(Resource, Debug, Default)]
pub struct Time{
    pub fixed_delta_time : FixedPoint,
    pub tick: u32
}