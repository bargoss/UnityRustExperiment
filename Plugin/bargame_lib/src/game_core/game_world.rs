use std::collections::HashMap;
use bevy_ecs::prelude::*;
use bevy_ecs::system::Resource;
use bevy_ecs::world::World;
use crate::game_core::math::FixedPoint;
use crate::game_core::resources::time::Time;
use crate::game_core::view_components::Id;
use crate::game_core::view_resources::view_time::ViewTime;
use crate::rollback_controller::input::Input;

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
    sample_view_snapshots_schedule: Schedule,
}

impl<TInput> GameWorld<TInput> where TInput: Input + 'static
{
    pub fn new(fixed_delta_time: FixedPoint) -> GameWorld<TInput>{
        let mut world = World::new();
        let mut advance_tick_schedule = Schedule::default();
        let mut register_keyframes_schedule = Schedule::default();
        let mut sample_view_snapshots_schedule = Schedule::default();

        let player_id_to_input_map = HashMap::<Id,TInput>::new();
        world.insert_resource(player_id_to_input_map);

        let id_entity_map = HashMap::<Id, Entity>::new();
        world.insert_resource(id_entity_map);

        let time = Time{ tick: 0, fixed_delta_time };
        world.insert_resource(time);

        let view_time = ViewTime{ time: 0.0 };
        world.insert_resource(view_time);

        GameWorld{
            world,
            advance_tick_schedule,
            register_keyframes_schedule,
            sample_view_snapshots_schedule,
            player_id_to_input_map : HashMap::new(),
        }
    }
    pub fn add_stage_to_advance_tick_schedule(&mut self, label: &'static str, stage: SystemStage){
        self.advance_tick_schedule.add_stage(label, stage);
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
    pub fn sample_view_snapshots(&mut self, viewing_time: f64)
    {
        todo!();
        self.world.get_resource_mut::<ViewTime>().unwrap().time = viewing_time;
        self.sample_view_snapshots_schedule.run(&mut self.world);
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