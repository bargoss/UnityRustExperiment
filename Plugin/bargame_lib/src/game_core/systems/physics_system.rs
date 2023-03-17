//use bevy_ecs::prelude::ResMut;
//use bevy_ecs::system::{Query, Resource};
use bevy_ecs::prelude::*;
use crate::game_core::components::circle_collider::CircleCollider;
use crate::game_core::components::net_id::NetId;
use crate::game_core::components::position::Position;
use crate::game_core::components::rigidbody::Rigidbody;
use crate::game_core::math::FixedPointV2;
use crate::game_core::resources::time::Time;
use crate::game_core::verlet_physics::verlet_object::VerletObject;
use crate::game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld;


// Query<(&Position, &CircleCollider)>

// also get the resource from bevy, of type VerletPhysicsWorld
fn push_all_bodies(
    query: Query<(&Position, &CircleCollider, &Rigidbody, &NetId)>,
    mut physics_world: ResMut<VerletPhysicsWorld>,
    time: Res<Time>
) {
    physics_world.clear();
    // iterate over the query
    for (position, collider, rigidbody, net_id) in query.iter() {
        // add the body to the physics world
        physics_world.add_or_set_object(VerletObject{
            acceleration: FixedPointV2::zero(),
            position: position.value,
            position_last: position.value - rigidbody.velocity*time.fixed_delta_time,
            radius: collider.radius,
            mass: rigidbody.mass,
            is_static: false,
        }, net_id.value);

    }

}