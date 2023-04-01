use crate::arena_fight_game::components::*;
use crate::game_core::components::*;
use bevy_ecs::prelude::*;
use crate::game_core::view_components::*;

#[derive(Bundle)]
pub struct UnitBundle{
    pub net_id: NetId,
    pub unit: Unit,
    pub character_movement: CharacterMovement,
    pub position: Position,
    pub rigidbody: Rigidbody,
    pub impulse: Impulse,
    pub collider: CircleCollider,
    pub health: Health,
    pub unit_view: UnitView,

    pub belongs_to_faction: BelongsToFaction,
}

#[derive(Bundle)]
pub struct UnitSpawnerNodeBundle{
    pub net_id: NetId,
    pub position: Position,
    //pub rigidbody: Rigidbody,
    pub collider: CircleCollider,
    pub unit_view: UnitView,

    pub belongs_to_faction: BelongsToFaction,
    pub node: Node,
    pub unit_spawner: UnitSpawner,
}

#[derive(Bundle)]
pub struct ShieldNodeBundle{
    pub net_id: NetId,
    pub position: Position,
    pub rigidbody: Rigidbody,
    pub collider: CircleCollider,
    pub health: Health,
    pub unit_view: UnitView,

    pub belongs_to_faction: BelongsToFaction,
    pub node: Node,
    pub shield: Shield,
}