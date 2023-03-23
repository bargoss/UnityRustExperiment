use std::collections::HashMap;
use bevy_ecs::prelude::*;
use bevy_ecs::world::World;
use crate::game_core::math::FixedPoint;
use crate::game_core::resources::id_entity_map::IdEntityMap;
use crate::game_core::resources::time::Time;
use crate::game_core::systems::id_entity_map_sync_system::id_entity_map_sync_system;
use crate::game_core::systems::physics_system::*;
use crate::game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld;
use crate::game_core::view_components::Id;
use crate::game_core::view_resources::view_snapshot::ViewSnapshot;
use crate::game_core::view_resources::view_snapshot_interpolator::{BufferedViewSnapshotInterpolator};
use crate::game_core::view_resources::view_snapshots::LineSnapshot::LineSnapshot;
use crate::game_core::view_resources::view_snapshots::SphereSnapshot::SphereSnapshot;
use crate::game_core::view_systems::line_view_system::line_view_system;
use crate::game_core::view_systems::sphere_view_system::sphere_view_system;
use crate::rollback_controller::input::Input;
use bevy_ecs::schedule::{BoxedSystemSet, Schedule, SystemSetConfig};


#[derive(Resource, Default)]
pub struct PlayerInputMap<TInput> where TInput: Input
{
    pub map: HashMap<Id, TInput>,
}


pub struct GameWorld<TInput> where TInput: Input
{
    world: World,
    advance_tick_schedule: Schedule,
    register_keyframes_schedule: Schedule,
    player_id_to_input_map: HashMap<Id, TInput>,
}

impl<TInput> GameWorld<TInput> where TInput: Input + 'static
{
    pub fn new<M>(fixed_delta_time: FixedPoint, systems: impl IntoSystemConfigs<M>) -> GameWorld<TInput>{
        let mut world = World::new();
        let mut advance_tick_schedule = Schedule::default();
        let mut register_keyframes_schedule = Schedule::default();

        let player_input_map = PlayerInputMap::<TInput>::default();
        world.insert_resource(player_input_map);

        let id_entity_map = IdEntityMap::default();
        world.insert_resource(id_entity_map);

        let verlet_physics_world = VerletPhysicsWorld::new();
        world.insert_resource(verlet_physics_world);

        let time = Time{ tick: 0, fixed_delta_time };
        world.insert_resource(time);


        let sphere_snapshots = BufferedViewSnapshotInterpolator::<SphereSnapshot>::default();
        world.insert_resource(sphere_snapshots);

        let line_snapshots = BufferedViewSnapshotInterpolator::<LineSnapshot>::default();
        world.insert_resource(line_snapshots);


        let internal_systems = (
            id_entity_map_sync_system,
            push_all_bodies,
            run_physics_step,
            pull_bodies,
        );

        let external_systems = (
            line_view_system,
            sphere_view_system,
        );

        SystemSet::is_base()
        let a = SystemSetConfig::new(BoxedSystemSet::new());
        //let all_systems = (internal_systems, external_systems).chain();

        //let into_configs_a = internal_systems;
        //let into_configs_b = systems;
        //IntoSystemSetConfigs::
        //let chained = (into_configs_a, into_configs_b).chain();

        //let mut chained = internal_systems.chain().after(internal_systems.into());

        //let chained = internal_systems.chain().after(internal_systems.into());
        //let advance_tick_systems = internal_systems.chain().after(systems.chain());
        //advance_tick_schedule.add_systems(advance_tick_systems);
        let view_systems = (
            sphere_view_system,
            line_view_system,
        );

        let register_keyframes_systems = view_systems.chain();
        register_keyframes_schedule.add_systems(register_keyframes_systems);


        let mut game_world = GameWorld{
            world,
            advance_tick_schedule,
            register_keyframes_schedule,
            player_id_to_input_map : HashMap::new(),
        };



        game_world
    }

    pub fn add_systems_to_advance_tick_schedule<M>(&mut self, systems: impl IntoSystemConfigs<M>) {
        self.advance_tick_schedule.add_systems(systems);
    }

    pub fn add_systems_to_register_keyframes_schedule<M>(&mut self, systems: impl IntoSystemConfigs<M>) {
        self.register_keyframes_schedule.add_systems(systems);
    }

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
    pub fn sample_view_snapshots<T>(&mut self, viewing_time: f64, buffer: &mut Vec<T>) where T: ViewSnapshot + 'static
    {
        buffer.clear();
        // get Res<ViewTime> and set viewing time
        //let mut view_time_res = self.world.get_resource_mut::<ViewTime>().unwrap();
        //view_time_res.time = viewing_time;

        let res = self.world.get_resource::<BufferedViewSnapshotInterpolator<T>>().unwrap();
        // iter
        res.interpolated_keyframes(viewing_time as f32).for_each(|snapshot| {
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
        let mut world = World::new();
        let mut init_schedule = Schedule::default();

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
    use crate::game_core::view_resources::view_snapshots::SphereSnapshot::SphereSnapshot;

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
                impulse: FixedPointV2::from_num(0.0, 0.0),
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
        game_world.sample_view_snapshots(0.5, &mut buffer);

        // sort the buffer
        // compare position.x
        buffer.sort_by(|a, b| a.position.x.partial_cmp(&b.position.x).unwrap());

        // print the buffer
        for (i, snapshot) in buffer.iter().enumerate() {
            println!("snapshot: {:?}", snapshot);
        }
    }
}