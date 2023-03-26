use bevy_ecs::prelude::Component;
use crate::game_core::common::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct TankBubble{
    pub tank_id: Id,
}
