use bevy_ecs::prelude::Schedule;
use bevy_ecs::world::World;
use crate::game_core::components::circle_collider::CircleCollider;
use super::components::*;
use crate::game_core::components::position::Position;
use crate::game_core::components::velocity::Velocity;
use bevy_ecs::bundle::Bundle;

// "particle" bundle that has components Position, Velocity, Collider
#[derive(Bundle)]
pub struct ParticleBundle {
    pub position: Position,
    pub velocity: Velocity,
    pub collider: CircleCollider,
}


#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn spawn_and_mutate_experiment() {
        let mut world = World::new();
        let mut init_schedule = Schedule::default();
        let mut update_schedule = Schedule::default();

        // spawn bundle
        let mut new_spawn = world.spawn();
        new_spawn.insert_bundle(ParticleBundle{
            position: Position{value: FixedPointV2::from_num(0.0, 0.0)},
            velocity: Default::default(),
            collider: Default::default(),
        });

        // access the entity, add one more component
        let entity = new_spawn.id();
        let mut position = world.get_mut::<Position>(entity).unwrap();
        (*position).value = FixedPointV2::from_num(1.0, 1.0);
        println!("position: {:?}", *position);

    }
}