use crate::game_core::components::FixedPoint;
use crate::game_core::view_components::Id;
use serde::{Serialize, Deserialize};
use bevy_ecs::component::Component;
use crate::game_core::math::FixedPointV2;

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Character {
    pub movement_direction: FixedPointV2,
}

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerControl{
    pub controlling_player_id: Id,
}

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Health{
    pub health: FixedPoint,
    pub max_health: FixedPoint,
    pub health_regen_per_second: FixedPoint,
}

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Stunned{
    pub stun_duration_left: FixedPoint,
}


