use bevy_ecs::prelude::Component;
use crate::game_core::view_components::Id;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct TankBubble{
    pub tank_id: Id,
}