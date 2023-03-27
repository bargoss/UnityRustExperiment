use crate::game_core::components::*;
use crate::game_core::common::*;
use serde::{Serialize, Deserialize};
use bevy_ecs::component::Component;
use crate::game_core::math::*;

#[derive(Component, Clone, Copy, Serialize, Deserialize, Default)]
pub struct CharacterMovement {
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

pub enum UnitType{
    MeleeFighter{
        attack_range: FixedPoint,
        attack_damage: FixedPoint,
        attack_cooldown: FixedPoint,
    },
    RangedFighter,
    Builder,
    ResourceCarrier,
}


#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub enum Faction{
    Neutral,
    Blue,
    Red,
    Green,
    Yellow,
}

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Unit {
    pub target_movement_position : FixedPointV2,
    pub team: Faction,
    pub belonging_player_id: Option<Id>,
    pub belonging_building_id : Option<Id>,
}