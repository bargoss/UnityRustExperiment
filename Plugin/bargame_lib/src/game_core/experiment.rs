use std::collections::HashMap;
use bevy_ecs;
use bevy_ecs::prelude::Schedule;
use bevy_ecs::world::World;
use super::math::FixedPoint;
use super::math::FixedPointV2;
use bevy_ecs::prelude::Bundle;
use bevy_ecs::prelude::Component;

#[derive(Component, Clone ,Debug, Default)]
pub struct Position {
    pub position: FixedPointV2,
}
#[derive(Component, Clone ,Debug, Default)]
pub struct Velocity {
    pub velocity: FixedPointV2,
    pub impulse: FixedPointV2,
}
#[derive(Bundle, Clone, Default)]
pub struct ParticleBundle {
    pub position: Position,
    pub velocity: Velocity,
}


#[cfg(test)]
mod tests {
    use simba::scalar::{ComplexField, FixedI64};
    use super::*;

    #[test]
    fn create_world(){
        let mut world = World::new();
        let mut init_schedule = Schedule::default();
        let mut update_schedule = Schedule::default();

        // add entity to world
        let new_entity = world.spawn();
        let id = new_entity.id();

        /// fn example_system(mut commands: Commands) {
        ///     // Create a new, empty entity
        ///     let entity = commands.spawn().id();
        ///
        ///     commands.entity(entity)
        ///         // adds a new component bundle to the entity
        ///         .insert_bundle((Strength(1), Agility(2)))
        ///         // adds a single component to the entity
        ///         .insert(Label("hello world"));
        /// }
        /// # bevy_ecs::system::assert_is_system(example_system);
        /// ```

        // get command buffer
        let mut commands = world.get_resource_mut::<bevy_ecs::system::Commands>().unwrap();
        let velocity_component = Velocity {
            velocity: FixedPointV2::new(FixedPoint::from_num(0.5), FixedPoint::from_num(0.5)),
            impulse: FixedPointV2::new(FixedPoint::from_num(0.0), FixedPoint::from_num(0.0)),
        };
        commands.entity(id).insert_bundle((velocity_component));
    }

    #[test]
    fn test_0() {

    }
}


