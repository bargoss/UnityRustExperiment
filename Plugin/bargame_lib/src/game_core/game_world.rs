use bevy_ecs::prelude::Schedule;
use bevy_ecs::world::World;

pub struct GameWorld{
    world: World,
    init_schedule: Schedule,
    advance_tick_schedule: Schedule,
    register_keyframes_schedule: Schedule,



}

impl GameWorld{
    pub fn new() -> GameWorld{
        let mut world = World::new();
        let mut init_schedule = Schedule::default();
        let mut advance_tick_schedule = Schedule::default();
        let mut register_keyframes_schedule = Schedule::default();

        GameWorld{
            world,
            init_schedule,
            advance_tick_schedule,
            register_keyframes_schedule,
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::system::Query;
    use crate::game_core::components::position::Position;
    use crate::game_core::components::velocity::Velocity;
    use super::*;
    use bevy_ecs::bundle::Bundle;
    use bevy_ecs::schedule::SystemStage;


    // derive bundle
    #[derive(Bundle)]
    pub struct ParticleBundle {
        pub position: Position,
        pub velocity: Velocity,
    }


    fn simple_rigidbody_system(mut query: Query<(&mut Position, &Velocity)>) {
        for (mut position, velocity) in query.iter_mut() {
            position.value += velocity.value;
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