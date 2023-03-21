use std::collections::HashMap;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::StageLabelId;
use bevy_ecs::system::Resource;
use bevy_ecs::world::World;
use crate::game_core::math::FixedPoint;
use crate::game_core::resources::id_entity_map::IdEntityMap;
use crate::game_core::resources::time::Time;
use crate::game_core::systems::id_entity_map_sync_system::id_entity_map_sync_system;
use crate::game_core::systems::physics_system::*;
use crate::game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld;
use crate::game_core::view_components::Id;
use crate::game_core::view_resources::view_snapshot::ViewSnapshot;
use crate::game_core::view_resources::view_snapshot_buffer::ViewSnapshotBuffer;
use crate::game_core::view_resources::view_snapshot_interpolator::{BufferedViewSnapshotInterpolator};
use crate::game_core::view_resources::view_snapshots::LineSnapshot::LineSnapshot;
use crate::game_core::view_resources::view_snapshots::SphereSnapshot::SphereSnapshot;
use crate::game_core::view_systems::line_view_system::line_view_system;
use crate::game_core::view_systems::sphere_view_system::sphere_view_system;
use crate::rollback_controller::input::Input;

#[derive(Default)]
pub struct PlayerInputMap<TInput> where TInput: Input
{
    pub map: HashMap<Id, TInput>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum OrderStageLabel{
    a0,
    a1,
    a2,
    a3,
    a4,
    a5,
    a6,
    a7,
    a8,
    a9,
    a10,
    a11,
    a12,
    a13,
    a14,
    a15,
    a16,
    a17,
    a18,
    a19,
    a20,
    a21,
    a22,
    a23,
    a24,
    a25,
    a26,
    a27,
    a28,
    a29,
    a30,
    a31,
    a32,
    a33,
    a34,
    a35,
    a36,
    a37,
    a38,
    a39,
    a40,
    a41,
    a42,
    a43,
    a44,
    a45,
    a46,
    a47,
    a48,
    a49,
}

impl OrderStageLabel{
    fn id(&self) -> usize {
        *self as usize
    }
    // use id and from_id
    fn get_next(&self) -> Self {
        OrderStageLabel::from_id(self.id() + 1)
    }

    fn from_id(id: usize) -> Self {
        match id {
            0 => OrderStageLabel::a0,
            1 => OrderStageLabel::a1,
            2 => OrderStageLabel::a2,
            3 => OrderStageLabel::a3,
            4 => OrderStageLabel::a4,
            5 => OrderStageLabel::a5,
            6 => OrderStageLabel::a6,
            7 => OrderStageLabel::a7,
            8 => OrderStageLabel::a8,
            9 => OrderStageLabel::a9,
            10 => OrderStageLabel::a10,
            11 => OrderStageLabel::a11,
            12 => OrderStageLabel::a12,
            13 => OrderStageLabel::a13,
            14 => OrderStageLabel::a14,
            15 => OrderStageLabel::a15,
            16 => OrderStageLabel::a16,
            17 => OrderStageLabel::a17,
            18 => OrderStageLabel::a18,
            19 => OrderStageLabel::a19,
            20 => OrderStageLabel::a20,
            21 => OrderStageLabel::a21,
            22 => OrderStageLabel::a22,
            23 => OrderStageLabel::a23,
            24 => OrderStageLabel::a24,
            25 => OrderStageLabel::a25,
            26 => OrderStageLabel::a26,
            27 => OrderStageLabel::a27,
            28 => OrderStageLabel::a28,
            29 => OrderStageLabel::a29,
            30 => OrderStageLabel::a30,
            31 => OrderStageLabel::a31,
            32 => OrderStageLabel::a32,
            33 => OrderStageLabel::a33,
            34 => OrderStageLabel::a34,
            35 => OrderStageLabel::a35,
            36 => OrderStageLabel::a36,
            37 => OrderStageLabel::a37,
            38 => OrderStageLabel::a38,
            39 => OrderStageLabel::a39,
            40 => OrderStageLabel::a40,
            41 => OrderStageLabel::a41,
            42 => OrderStageLabel::a42,
            43 => OrderStageLabel::a43,
            44 => OrderStageLabel::a44,
            45 => OrderStageLabel::a45,
            46 => OrderStageLabel::a46,
            47 => OrderStageLabel::a47,
            48 => OrderStageLabel::a48,
            49 => OrderStageLabel::a49,
            _ => panic!("Invalid id"),
}

pub struct GameWorld<TInput> where TInput: Input
{
    world: World,
    advance_tick_schedule: Schedule,
    register_keyframes_schedule: Schedule,
    player_id_to_input_map: HashMap<Id, TInput>,
    system_counter: OrderStageLabel,
}

impl<TInput> GameWorld<TInput> where TInput: Input + 'static
{
    pub fn new(fixed_delta_time: FixedPoint) -> GameWorld<TInput>{
        let mut world = World::new();
        let advance_tick_schedule = Schedule::default();
        let register_keyframes_schedule = Schedule::default();

        let player_input_map = PlayerInputMap::<TInput>::default();
        world.insert_resource(player_input_map);

        let id_entity_map = IdEntityMap::new();
        world.insert_resource(id_entity_map);

        let verlet_physics_world = VerletPhysicsWorld::new();
        world.insert_resource(verlet_physics_world);

        let time = Time{ tick: 0, fixed_delta_time };
        world.insert_resource(time);


        let sphere_snapshots = BufferedViewSnapshotInterpolator::<SphereSnapshot>::default();
        world.insert_resource(sphere_snapshots);

        let line_snapshots = BufferedViewSnapshotInterpolator::<LineSnapshot>::default();
        world.insert_resource(line_snapshots);

        let mut game_world = GameWorld{
            world,
            advance_tick_schedule,
            register_keyframes_schedule,
            player_id_to_input_map : HashMap::new(),
            system_counter: OrderStageLabel::a0,
        };

        game_world.add_stage_to_advance_tick_schedule("sync_net_ids", SystemStage::single_threaded()
            .with_system(id_entity_map_sync_system)
        );
        game_world.add_stage_to_advance_tick_schedule("physics", SystemStage::single_threaded()
            .with_system(push_all_bodies)
            .with_system(run_physics_step)
            .with_system(pull_bodies)
        );

        game_world.add_stage_to_register_keyframes_schedule("register_keyframes", SystemStage::single_threaded()
            .with_system(sphere_view_system)
            .with_system(line_view_system)
        );

        game_world
    }
    pub fn add_stage_to_advance_tick_schedule(&mut self, label: &'static str, stage: SystemStage){
        self.advance_tick_schedule.add_stage(label, stage);
    }
    pub fn add_stage_to_advance_tick_schedule2(&mut self, stage: SystemStage){
        let last_label = self.system_counter;
        let label = last_label.get_next();
        self.advance_tick_schedule.add_stage(label, stage);
    }
    /*
    pub fn add_ordered_system<F>(&mut self, system: F)
        where
            F: System<In = (), Out = ()> + Send + Sync + 'static,
    {
        let label = format!("order_{}", self.counter);
        self.app.add_system(system.label(label.clone()));
        if self.system_counter > 0 {
            let prev_label = format!("order_{}", self.system_counter - 1);
            self.app.add_system_to_stage(
                CoreStage::Update,
                bevy::ecs::schedule::SystemStage::parallel()
                    .with_system(
                        bevy::ecs::schedule::SystemSet::new()
                            .label(label)
                            .after(prev_label),
                    ),
            );
        }
        self.system_counter += 1;
    }

     */


    pub fn add_stage_to_register_keyframes_schedule(&mut self, label: &'static str, stage: SystemStage){
        self.register_keyframes_schedule.add_stage(label, stage);
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
    use bevy_ecs::schedule::SystemStage;

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
        update_schedule
            .add_stage(
                "update",
                    SystemStage::parallel()
                    .with_system(simple_rigidbody_system)
        );


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


    #[test]
    fn test_view_interpolation_logic() {
        // Initialize the GameWorld
        let mut game_world = GameWorld::<DummyInput>::new(FixedPoint::new(1.000));

        // Spawn some entities with Position and Rigidbody components
        for i in 0..10 {
            game_world.world.spawn().insert(Position {
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