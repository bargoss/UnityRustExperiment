use std::collections::HashMap;
use bevy_ecs::prelude::*;
use bevy_ecs::world::World;
use crate::game_core::math::FixedPoint;
use crate::game_core::resources::id_entity_map::IdEntityMap;
use crate::game_core::resources::time::Time;
use crate::game_core::systems::id_entity_map_sync_system::id_entity_map_sync_system;
use crate::game_core::systems::physics_system::*;
use crate::game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld;
use crate::game_core::common::*;
use crate::game_core::view_resources::view_snapshot::ViewSnapshot;
use crate::game_core::view_resources::view_snapshot_interpolator::{BufferedViewSnapshotInterpolator};
use crate::game_core::view_resources::view_snapshots::line_snapshot::LineSnapshot;
use crate::game_core::view_resources::view_snapshots::sphere_snapshot::SphereSnapshot;
use crate::game_core::view_systems::line_view_system::line_view_system;
use crate::game_core::view_systems::sphere_view_system::sphere_view_system;
use bevy_ecs::schedule::Schedule;
use crate::game_core::input::Input;
use crate::game_core::resources::NetIdCounter;


#[derive(Resource, Default)]
pub struct PlayerInputMap<TInput> where TInput: Input
{
    map: HashMap<Id, TInput>,
}
impl<TInput> PlayerInputMap<TInput> where TInput: Input
{
    pub fn get(&self, id: &Id) -> Option<TInput> { self.map.get(id).copied() }
    pub fn insert(&mut self, id: Id, input: TInput) { self.map.insert(id, input); }
    pub fn remove(&mut self, id: &Id) { self.map.remove(id); }
    pub fn iter(&self) -> std::collections::hash_map::Iter<Id, TInput> { self.map.iter() }
}

pub fn core_systems_executed(){

}


pub struct GameWorld<TInput> where TInput: Input
{
    pub world: World,
    advance_tick_schedule: Schedule,
    register_keyframes_schedule: Schedule,
    #[allow(dead_code)] player_id_to_input_map: HashMap<Id, TInput>,
}

impl<TInput> GameWorld<TInput> where TInput: Input + 'static
{
    pub fn new<M>(fixed_delta_time: FixedPoint, systems: impl IntoSystemConfigs<M>) -> GameWorld<TInput>{
        let mut world = World::new();
        let mut advance_tick_schedule = Schedule::default();
        let mut register_keyframes_schedule = Schedule::default();

        world.insert_resource(PlayerInputMap::<TInput>::default());
        world.insert_resource(IdEntityMap::default());
        world.insert_resource(VerletPhysicsWorld::new());
        world.insert_resource(Time{ tick: 0, fixed_delta_time });
        world.insert_resource(BufferedViewSnapshotInterpolator::<SphereSnapshot>::default());
        world.insert_resource(BufferedViewSnapshotInterpolator::<LineSnapshot>::default());
        world.insert_resource(NetIdCounter::new());

        let internal_systems = (
            id_entity_map_sync_system,
            process_impulses,
            push_all_bodies,
            run_physics_step,
            pull_bodies,
            core_systems_executed
        );
        advance_tick_schedule.add_systems(internal_systems.chain());
        advance_tick_schedule.add_systems(systems.chain().after(core_systems_executed));

        let register_keyframes_systems_internal = (
            sphere_view_system,
            line_view_system,
        );

        register_keyframes_schedule.add_systems(register_keyframes_systems_internal);


        let game_world = GameWorld{
            world,
            advance_tick_schedule,
            register_keyframes_schedule,
            player_id_to_input_map : HashMap::new(),
        };

        game_world
    }

    pub fn get_tick(&self) -> u32 { self.world.get_resource::<Time>().unwrap().tick }
    pub fn get_fixed_delta_time(&self) -> FixedPoint { self.world.get_resource::<Time>().unwrap().fixed_delta_time }

    pub fn advance_tick(&mut self, input_map: HashMap<Id, TInput>){
        let mut input_map_res = self.world.get_resource_mut::<PlayerInputMap<TInput>>().unwrap();
        input_map_res.map.clear();
        input_map_res.map.extend(input_map);
        self.advance_tick_schedule.run(&mut self.world);
        self.world.get_resource_mut::<Time>().unwrap().tick += 1;
    }
    pub fn register_keyframes(&mut self){
        self.register_keyframes_schedule.run(&mut self.world);
    }
    // where T: ViewSnapshot
    pub fn sample_view_snapshots<T>(&mut self, viewing_time: FixedPoint, buffer: &mut Vec<T>) where T: ViewSnapshot + 'static
    {
        buffer.clear();

        let res = self.world.get_resource::<BufferedViewSnapshotInterpolator<T>>().unwrap();
        res.interpolated_keyframes(viewing_time).for_each(|snapshot| {
            buffer.push(snapshot.1);
        });
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::system::Query;
    use crate::game_core::components::position::Position;
    use crate::game_core::components::rigidbody::Rigidbody;
    use super::*;
    use bevy_ecs::bundle::Bundle;

    // derive bundle
    #[derive(Bundle)]
    pub struct ParticleBundle {
        pub position: Position,
        pub velocity: Rigidbody,
    }

    fn simple_rigidbody_system(mut query: Query<(&mut Position, &Rigidbody)>) {
        for (mut position, velocity) in query.iter_mut() {
            position.value += velocity.velocity;
        }
    }

    #[test]
    fn spawn_and_mutate_experiment() {
        let _world = World::new();
        let _init_schedule = Schedule::default();

        let mut update_schedule = Schedule::default();
        update_schedule.add_system(simple_rigidbody_system);
    }
}

#[cfg(test)]
mod view_interpolation_tests {
    use crate::game_core::components::circle_collider::CircleCollider;
    use crate::game_core::components::net_id::NetId;
    use super::*;
    use crate::game_core::components::position::Position;
    use crate::game_core::components::rigidbody::Rigidbody;
    use crate::game_core::math::FixedPointV2;
    use crate::game_core::view_components::sphere_view::SphereView;
    use crate::game_core::view_resources::view_snapshots::sphere_snapshot::SphereSnapshot;

    #[derive(Default, Copy, Clone)]
    pub struct DummyInput;

    // impl Send and Sync for DummyInput
    unsafe impl Send for DummyInput {}
    unsafe impl Sync for DummyInput {}

    impl Input for DummyInput {}

    fn dummy_system() {
        println!("dummy system");
    }

    #[test]
    fn test_view_interpolation_logic() {
        // Initialize the GameWorld
        let mut game_world = GameWorld::<DummyInput>::new(FixedPoint::new(1.000), (
            dummy_system,
        ).chain());

        // Spawn some entities with Position and Rigidbody components
        for i in 0..10 {
            game_world.world.spawn_empty().insert(Position {
                value: FixedPointV2::from_num(i as f64, 0.0),
            }).insert(Rigidbody {
                velocity: FixedPointV2::from_num(1.0, 0.0),
                mass: FixedPoint::new(1.0),
            }).insert(SphereView{
                radius: FixedPoint::new(0.5),
                view_custom_id: Id::new(i),
            }).insert(CircleCollider{
                radius: FixedPoint::new(0.5),
            }).insert(NetId{
                value: Id::new(i),
            });
        }

        // Advance the tick
        game_world.register_keyframes();
        game_world.advance_tick(HashMap::new());
        game_world.register_keyframes();

        // Test the view interpolation logic by sampling view snapshots
        let mut buffer: Vec<SphereSnapshot> = Vec::new();
        game_world.sample_view_snapshots(FixedPoint::new(0.5), &mut buffer);

        // sort the buffer
        // compare position.x
        buffer.sort_by(|a, b| a.position.x().partial_cmp(&b.position.x()).unwrap());

        // print the buffer
        for (i, snapshot) in buffer.iter().enumerate() {
            println!("snapshot: {:?}", snapshot);
            assert_eq!(snapshot.position.x(), FixedPoint::new(i as f64 + 0.5));
        }
    }
}

#[cfg(test)]
mod system_adding_tests
{
    use bevy_ecs::prelude::*;

    pub fn system0() {
        println!("system0");
    }
    pub fn system1() {
        println!("system1");
    }
    pub fn system2() {
        println!("system2");
    }
    pub fn system3() {
        println!("system3");
    }
    pub fn system4() {
        println!("system4");
    }
    pub fn system5() {
        println!("system5");
    }
    pub fn system6() {
        println!("system6");
    }
    pub fn system7() {
        println!("system7");
    }
    pub fn system8() {
        println!("system8");
    }
    pub fn system9() {
        println!("system9");
    }



    #[test]
    fn test_system_adding() {
        let mut schedule = Schedule::default();
        schedule.add_system(system0);
        schedule.add_system(system1.after(system0));
        schedule.add_system(system2.after(system1));
        schedule.add_system(system3.after(system2));
        schedule.add_system(system4.after(system3));
        schedule.run(&mut World::new());
    }

    #[test]
    fn test_system_adding2() {
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (system0, system1, system2, system3, system4).chain()
        );
        schedule.add_systems(
            (system5, system6.after(system5), system7.after(system6), system8.after(system7), system9.after(system8)).after(system4)
        );


        schedule.run(&mut World::new());
    }

}