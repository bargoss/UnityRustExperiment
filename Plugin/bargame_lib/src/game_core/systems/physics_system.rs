//use bevy_ecs::prelude::ResMut;
//use bevy_ecs::system::{Query, Resource};
use bevy_ecs::prelude::*;
use crate::game_core::components::circle_collider::CircleCollider;
use crate::game_core::components::position::Position;
use crate::game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld;


// Query<(&Position, &CircleCollider)>

// also get the resource from bevy, of type VerletPhysicsWorld
fn push_all_bodies(query: Query<(&Position, &CircleCollider)>, mut physics_world: ResMut<VerletPhysicsWorld>) {
    physics_world.clear();
    // iterate over the query
    for (position, collider) in query.iter() {
        // add the body to the physics world

    }

}