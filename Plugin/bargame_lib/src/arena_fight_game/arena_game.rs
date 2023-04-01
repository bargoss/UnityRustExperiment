use std::collections::HashMap;
use crate::game_core::game_world::GameWorld;
use crate::rollback_controller::input::Input;
//use bevy_ecs::schedule::IntoSystemConfigs;
use crate::arena_fight_game::components::*;
use crate::game_core::components::*;
use crate::game_core::view_resources::view_snapshot::ViewSnapshot;
use bevy_ecs::prelude::*;
use crate::arena_fight_game::bundles::{UnitBundle, UnitSpawnerNodeBundle};
use crate::arena_fight_game::systems::character_movement_system::character_movement_system;
use crate::arena_fight_game::systems::health_system::health_system;
use crate::arena_fight_game::systems::player_input_system::player_input_system;
use crate::arena_fight_game::systems::unit_attack_system::unit_attack_system;
use crate::arena_fight_game::systems::unit_capture_node_system::unit_capture_node_system;
use crate::arena_fight_game::systems::unit_movement_system::unit_movement_system;
use crate::arena_fight_game::systems::unit_spawner_system::{unit_spawner_system};
use crate::bubble_tanks_game::dummy_system;
//use crate::game_core::common::Id;
use crate::game_core::components::*;
use crate::game_core::math::FixedPointV2;
use crate::game_core::view_components::sphere_view::SphereView;
use crate::game_core::common::*;
use crate::game_core::math::*;
use crate::game_core::resources::NetIdCounter;

#[derive(Copy, Clone, Debug, Default)]
pub struct ArenaInput {
    pub movement_direction: FixedPointV2,
}
unsafe impl Sync for ArenaInput {}
unsafe impl Send for ArenaInput {}
impl Input for ArenaInput {}

#[derive(Bundle)]
pub struct PlayerCharacterBundle {
    pub net_id: NetId,
    pub position: Position,
    pub rigidbody: Rigidbody,
    pub impulse: Impulse,
    pub collider: CircleCollider,
    pub health: Health,
    pub player_control: PlayerCharacterControl,
    pub character: CharacterMovement,
    pub sphere_view: SphereView,
}

pub struct ArenaFightGame {
    pub game_world: GameWorld<ArenaInput>,
}
impl Default for ArenaFightGame {
    fn default() -> Self {
        let mut arena = ArenaFightGame {
            game_world: GameWorld::new(
                FixedPoint::new(0.02) ,
                (
                    player_input_system,
                    unit_movement_system,
                    character_movement_system,
                    unit_capture_node_system,
                    unit_spawner_system,
                    unit_attack_system,
                    health_system,
                ).chain()
            ),
        };

        arena.add_unit(FixedPointV2::from_num(0.0, 1.0), Faction::Blue);
        arena.add_unit(FixedPointV2::from_num(0.0, 2.0), Faction::Blue);
        arena.add_unit(FixedPointV2::from_num(0.0, 3.0), Faction::Blue);
        arena.add_unit(FixedPointV2::from_num(0.0, -1.0), Faction::Red);
        arena.add_unit(FixedPointV2::from_num(0.0, -2.0), Faction::Red);
        arena.add_unit(FixedPointV2::from_num(0.0, -3.0), Faction::Red);


        arena.add_spawner_node(FixedPointV2::from_num(0.0, 6.0), Faction::Blue);
        arena.add_spawner_node(FixedPointV2::from_num(0.0, -6.0), Faction::Red);

        arena
    }
}

impl ArenaFightGame {
    /*
    pub fn add_player_character(&mut self, id: Id, position: FixedPointV2) {
        // default bundle
        self.game_world.world.spawn(PlayerCharacterBundle {
            net_id: NetId{ value: id, }, //todo use proper logic to generate net id
            position: Position{ value: position, },
            rigidbody: Rigidbody::default(),
            impulse: Impulse::default(),
            collider: CircleCollider { radius: FixedPoint::new(0.5), },
            health: Health{health: FixedPoint::new(100.0), max_health: FixedPoint::new(100.0), health_regen_per_second: FixedPoint::new(10.0),},
            player_control: PlayerCharacterControl{controlling_player_id: id},
            character: CharacterMovement::default(),
            sphere_view: SphereView{view_custom_id: Id::new(0), radius: FixedPoint::new(0.5),},
        });
    }
    */

    pub fn add_spawner_node(&mut self, position: FixedPointV2, faction: Faction){
        let next_id = self.game_world.world.get_resource_mut::<NetIdCounter>().unwrap().next();
        self.game_world.world.spawn(UnitSpawnerNodeBundle{
            node: Default::default(),
            position: Position{ value: position, },
            unit_spawner: Default::default(),
            net_id : NetId{value:next_id},
            collider: CircleCollider { radius: FixedPoint::new(1.5), },
            rigidbody: Rigidbody::default(),
            sphere_view: SphereView{view_custom_id: next_id, radius: FixedPoint::new(1.5),},
            belongs_to_faction: BelongsToFaction{faction: faction},
        });
    }
    pub fn add_unit(&mut self, position: FixedPointV2, faction: Faction){
        let next_id = self.game_world.world.get_resource_mut::<NetIdCounter>().unwrap().next();
        self.game_world.world.spawn(UnitBundle {
            net_id: NetId{ value: next_id, }, //todo use proper logic to generate net id
            position: Position{ value: position, },
            rigidbody: Rigidbody::default(),
            impulse: Impulse::default(),
            collider: CircleCollider { radius: FixedPoint::new(0.5), },
            health: Health{health: FixedPoint::new(100.0), max_health: FixedPoint::new(100.0), health_regen_per_second: FixedPoint::new(10.0),},
            character_movement: CharacterMovement::default(),
            unit: Unit::default(),
            belongs_to_faction: BelongsToFaction{faction: faction},

            sphere_view: SphereView{view_custom_id: next_id, radius: FixedPoint::new(0.5),},
        });
    }


    pub fn get_tick(&self) -> u32 { self.game_world.get_tick() }
    pub fn get_fixed_delta_time(&self) -> FixedPoint { self.game_world.get_fixed_delta_time() }
    pub fn advance_tick(&mut self, input_map: HashMap<Id, ArenaInput>){ self.game_world.advance_tick(input_map); }
    pub fn register_keyframes(&mut self){ self.game_world.register_keyframes(); }
    pub fn sample_view_snapshots<T>(&mut self, viewing_time: FixedPoint, buffer: &mut Vec<T>) where T: ViewSnapshot + 'static { self.game_world.sample_view_snapshots(viewing_time, buffer); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_test_0() {
        let mut arena_game = ArenaFightGame::default();
        arena_game.advance_tick(HashMap::new());
    }
}