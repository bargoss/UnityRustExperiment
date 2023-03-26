use bevy_ecs::prelude::Component;
use crate::game_core::common::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct BubbleTank {
    pub controlling_player_id: Id,
}
