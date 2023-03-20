use bevy_ecs::prelude::Component;
use crate::game_core::view_components::Id;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct BubbleTank {
    pub controlling_player_id: Id,
}
