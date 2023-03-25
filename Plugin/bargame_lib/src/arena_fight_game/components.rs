use crate::game_core::components::FixedPoint;
use crate::game_core::view_components::Id;
use serde::{Serialize, Deserialize};
use bevy_ecs::component::Component;
#[derive(Component, Clone, Copy)]
pub struct Character { }

#[derive(Component, Clone, Copy)]
pub struct PlayerControl{
    pub controlling_player_id: Id,
}

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Health{
    pub health: FixedPoint,
    pub max_health: FixedPoint,
    pub health_regen_per_second: FixedPoint,
}



