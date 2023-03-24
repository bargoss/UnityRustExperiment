use crate::game_core::components::FixedPoint;
use crate::game_core::view_components::Id;

#[derive(Component, Clone, Copy)]
pub struct CharacterMovement{

}

#[derive(Component, Clone, Copy)]
pub struct PlayerControl{
    controlling_player_id: Id,
}

#[derive(Component, Clone, Copy)]
pub struct Health{
    pub health: FixedPoint,
    pub max_health: FixedPoint,
    pub health_regen_per_second: FixedPoint,
}



