use std::collections::HashMap;
use bevy_ecs;
use bevy_ecs::prelude::Schedule;
use bevy_ecs::world::World;
use simba::scalar::FixedI40F24;
use nalgebra::Vector2;

// velocity component



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_world(){
        let mut world = World::new();
        let mut init_schedule = Schedule::default();
        let mut update_schedule = Schedule::default();

        // add entity to world
        let entity = world.spawn();
        // add component to entity


    }

    #[test]
    fn test_0() {
        let num0 = FixedI40F24::from_num(0.5);
        let num1 = FixedI40F24::from_num(1.5);

        let vec0 = Vector2::new(num0, num1);

        let normalized_vec0 = vec0.normalize();
        let adsa = normalized_vec0 + normalized_vec0;

        println!("normalized_vec0: {:?}", normalized_vec0);
        println!("adsa: {:?}", adsa);

    }
}


