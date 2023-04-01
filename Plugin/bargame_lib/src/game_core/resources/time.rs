use bevy_ecs::prelude::Resource;
use crate::game_core::math::FP;

#[derive(Resource, Debug, Default)]
pub struct Time{
    pub fixed_delta_time : FP,
    pub tick: u32
}