use crate::game_core::components::*;
use crate::game_core::common::*;
use serde::{Serialize, Deserialize};
use bevy_ecs::component::Component;
use crate::game_core::math::*;

#[derive(Component, Clone, Copy, Serialize, Deserialize, Default)]
pub struct CharacterMovement {
    pub movement_direction: FP2,
}

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerCharacterControl {
    pub controlling_player_id: Id,
}

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Health{
    pub health: FP,
    pub max_health: FP,
    pub health_regen_per_second: FP,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
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
#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Node {
    pub capture_progress: FP,
    pub capture_progress_faction: Faction,
}
#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct UnitSpawner {
    pub last_spawn_time: FP,
    pub spawn_interval: FP,
}
#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Shield {
    pub last_shield_up_time: FP,
    pub shield_up_interval: FP,
    pub shield_capacity: FP,
    pub shield_radius: FP,
}

#[derive(Component, Clone, Copy)]
pub struct UnitView{
    pub view_custom_id: Id,
}

#[derive(Component, Clone, Copy)]
pub struct NodeView{
    pub view_custom_id: Id,
}