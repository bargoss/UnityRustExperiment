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

#[derive(Component, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Faction{
    #[default] Neutral,
    Blue,
    Red,
    Green,
    Yellow,
}

#[derive(Component, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Unit {
    pub following_entity : Option<NetId>,
}

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct OwnedByPlayer {
    pub player_id: Id,
}
#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct BelongsToFaction {
    pub faction: Faction,
}

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct BelongsToBuilding {
    pub building_id: NetId,
}
#[derive(Component, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Node {
    pub capture_progress: FixedPoint,
    pub capture_progress_faction: Faction,
}
#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct UnitSpawner {
    pub last_spawn_time: FixedPoint,
    pub spawn_interval: FixedPoint,
}
#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Shield {
    pub last_shield_up_time: FixedPoint,
    pub shield_up_interval: FixedPoint,
    pub shield_capacity: FixedPoint,
    pub shield_radius: FixedPoint,
}
