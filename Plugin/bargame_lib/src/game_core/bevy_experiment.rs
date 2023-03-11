use bevy_ecs::prelude::Schedule;
use bevy_ecs::world::World;
use super::components::*;

// bundle

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_world() {
        let mut world = World::new();
        let mut init_schedule = Schedule::default();
        let mut update_schedule = Schedule::default();

        // add entity
        let entity = world.spawn();
        // add component

    }
}