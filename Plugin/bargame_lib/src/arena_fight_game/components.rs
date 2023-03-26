use crate::game_core::components::FixedPoint;
use crate::game_core::common::*;
use serde::{Serialize, Deserialize};
use bevy_ecs::component::Component;
use crate::game_core::math::FixedPointV2;

#[derive(Component, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Character {
    pub movement_direction: FixedPointV2,
}

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerCharacterControl {
    pub controlling_player_id: Id,
}

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Health{
    pub health: FixedPoint,
    pub max_health: FixedPoint,
    pub health_regen_per_second: FixedPoint,
}


