use std::collections::HashMap;
use crate::game_core::game_world::*;
use crate::arena_fight_game::components::*;
use crate::game_core::components::*;
use crate::game_core::view_resources::*;
use bevy_ecs::prelude::*;
use crate::arena_fight_game::bundles::*;
use crate::arena_fight_game::systems::*;
use crate::arena_fight_game::view_systems::*;
use crate::bubble_tanks_game::dummy_system;
use crate::game_core::view_components::sphere_view::SphereView;
use crate::game_core::common::*;
use crate::game_core::input::Input;
use crate::game_core::math::*;
use crate::game_core::resources::*;
use crate::game_core::systems::*;
use crate::game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld;
use crate::game_core::view_resources::*;
use crate::game_core::view_systems::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct SelectAndSetDestinationInput {
    pub position: FP2,
    pub radius: FP,
    pub target_node_net_id: NetId,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ArenaInput {
    pub select_and_set_destination: Option<SelectAndSetDestinationInput>,
}
unsafe impl Sync for ArenaInput {}
unsafe impl Send for ArenaInput {}
impl Input for ArenaInput {}

pub struct ArenaFightGame {
    pub world: World,
    advance_tick_schedule: Schedule,
    register_views_schedule: Schedule,
    render_schedule: Schedule,
}
impl Default for ArenaFightGame {
    fn default() -> Self {
        let mut world = World::new();

        world.insert_resource(PlayerInputMap::<ArenaInput>::default());
        world.insert_resource(IdEntityMap::default());
        world.insert_resource(VerletPhysicsWorld::new());
        world.insert_resource(Time{ tick: 0, fixed_delta_time: FP::new(0.02) });
        world.insert_resource(BufferedViewSnapshotInterpolator::<SphereSnapshot>::default());
        world.insert_resource(BufferedViewSnapshotInterpolator::<LineSnapshot>::default());

        let mut advance_tick_schedule = Schedule::default();
        advance_tick_schedule.add_systems((
            id_entity_map_sync_system,
            process_impulses,
            push_all_bodies,
            run_physics_step,
            pull_bodies,
            //core_systems_executed,
            player_control_system,
            unit_movement_system,
            character_movement_system,
            unit_capture_node_system,
            unit_spawner_system,
            unit_attack_system,
            health_system,
        ).chain());

        let mut register_views_schedule = Schedule::default();
        register_views_schedule.add_systems((
            unit_view_system,
            line_view_system,
            sphere_view_system,
        ).chain());

        let mut render_schedule = Schedule::default();
        render_schedule.add_systems((
            crate::bubble_tanks_game::dummy_system,
        ).chain());

        let mut game = ArenaFightGame {
            world,
            advance_tick_schedule,
            register_views_schedule,
            render_schedule,
        };

        game.add_spawner_node(FP2::from_num(0.0, 6.0), Faction::Blue, NetId::from_u32(0));
        game.add_spawner_node(FP2::from_num(0.0, -6.0), Faction::Red, NetId::from_u32(1));
        game.add_unit(FP2::from_num(0.0, 1.0), Faction::Blue, NetId::from_u32(2));
        game.add_unit(FP2::from_num(0.0, 2.0), Faction::Blue, NetId::from_u32(3));
        game.add_unit(FP2::from_num(0.0, 3.0), Faction::Blue, NetId::from_u32(4));
        game.add_unit(FP2::from_num(0.0, -1.0), Faction::Red, NetId::from_u32(5));
        game.add_unit(FP2::from_num(0.0, -2.0), Faction::Red, NetId::from_u32(6));
        game.add_unit(FP2::from_num(0.0, -3.0), Faction::Red, NetId::from_u32(7));

        game
    }
}

impl ArenaFightGame {
    pub fn add_spawner_node(&mut self, position: FP2, faction: Faction, net_id: NetId){
        let next_id = net_id.value;
        println!("node net_id is: {}", next_id);
        self.world.spawn(UnitSpawnerNodeBundle{
            node: Node {
                capture_progress_faction: faction,
                capture_progress: FP::one(),
            },
            position: Position{ value: position, },
            unit_spawner: UnitSpawner{
                spawn_interval: FP::new(0.5),
                last_spawn_time: FP::new(0.0),
            },
            net_id : NetId{value:next_id},
            collider: CircleCollider { radius: FP::new(1.5), },
            //rigidbody: Rigidbody{velocity: FP2::zero(),mass: FP::new(1000.0),},
            unit_view: UnitView{view_custom_id: next_id},
            belongs_to_faction: BelongsToFaction{faction: faction},
        });
    }
    pub fn add_unit(&mut self, position: FP2, faction: Faction, net_id: NetId){
        let next_id = net_id.value;
        self.world.spawn(UnitBundle {
            net_id: NetId{ value: next_id, }, //todo use proper logic to generate net id
            position: Position{ value: position, },
            rigidbody: Rigidbody::default(),
            impulse: Impulse::default(),
            collider: CircleCollider { radius: FP::new(0.5), },
            health: Health{health: FP::new(5.0), max_health: FP::new(5.0), health_regen_per_second: FP::new(0.0),},
            character_movement: CharacterMovement::default(),
            unit: Unit::default(),
            belongs_to_faction: BelongsToFaction{faction: faction},
            unit_view: UnitView{view_custom_id: next_id},
        });
    }


    pub fn get_tick(&self) -> u32 { self.world.get_resource::<Time>().unwrap().tick }
    pub fn get_fixed_delta_time(&self) -> FP { self.world.get_resource::<Time>().unwrap().fixed_delta_time }

    pub fn advance_tick(&mut self, input_map: HashMap<Id, ArenaInput>){
        let mut input_map_res = self.world.get_resource_mut::<PlayerInputMap<ArenaInput>>().unwrap();
        input_map_res.clear();
        input_map_res.extend(input_map);
        self.advance_tick_schedule.run(&mut self.world);
        self.world.get_resource_mut::<Time>().unwrap().tick += 1;
    }
    pub fn register_views(&mut self){
        self.register_views_schedule.run(&mut self.world);
    }
    pub fn render(
        &self,
        viewing_time: FP,
        on_sphere_view_updated: impl Fn(SphereSnapshot),
        on_line_view_updated: impl Fn(LineSnapshot),
    ) {
        self.world.get_resource::<BufferedViewSnapshotInterpolator<SphereSnapshot>>().unwrap()
            .interpolated_keyframes(viewing_time)
            .for_each(|snapshot| on_sphere_view_updated(snapshot.1));

        self.world.get_resource::<BufferedViewSnapshotInterpolator<LineSnapshot>>().unwrap()
            .interpolated_keyframes(viewing_time)
            .for_each(|snapshot| on_line_view_updated(snapshot.1));
    }

}

#[cfg(test)]
mod tests {
    use crate::game_core::view_resources::SphereSnapshot;
    use super::*;

    #[test]
    fn game_test_0() {
        let mut arena_game = ArenaFightGame::default();
        arena_game.register_views();
        arena_game.advance_tick(HashMap::new());
        arena_game.register_views();

        arena_game.render(FP::new(0.05), |snapshot| {
            println!("snapshot: {:?}", snapshot);
        }, |snapshot| {
            println!("snapshot: {:?}", snapshot);
        });
    }
}