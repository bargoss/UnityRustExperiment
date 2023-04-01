//use bevy_ecs::prelude::ResMut;
//use bevy_ecs::system::{Query, Resource};
use bevy_ecs::prelude::*;
use crate::game_core::components::beam::Beam;
use crate::game_core::components::circle_collider::CircleCollider;
use crate::game_core::components::impulse::Impulse;
use crate::game_core::components::net_id::NetId;
use crate::game_core::components::position::Position;
use crate::game_core::components::rigidbody::Rigidbody;
use crate::game_core::math::{FP, FP2};
use crate::game_core::resources::time::Time;
use crate::game_core::verlet_physics::verlet_beam::VerletBeam;
use crate::game_core::verlet_physics::verlet_object::VerletObject;
use crate::game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld;

pub fn process_impulses(
    mut body_query: Query<(&mut Rigidbody, &mut Impulse)>,
    _time: Res<Time>
) {
    for (mut rigidbody, mut impulse) in body_query.iter_mut() {
        let mass = rigidbody.mass;
        rigidbody.velocity += impulse.value / mass;
        //println!("velocity: {:?}", rigidbody.velocity);
        impulse.value = FP2::zero();
    }
}

pub fn push_all_bodies(
    body_query: Query<(&Position, &CircleCollider, Option<&Rigidbody>, &NetId)>,
    beam_query: Query<(&Beam, &NetId)>,
    mut physics_world: ResMut<VerletPhysicsWorld>,
    time: Res<Time>
) {
    physics_world.clear();
    // iterate over the query
    for (position, collider, rigidbody_opt, net_id) in body_query.iter() {
        // add the body to the physics world
        match rigidbody_opt {
            Some(rigidbody) => {
                physics_world.add_or_set_object(VerletObject{
                    acceleration: FP2::zero(),
                    position: position.value,
                    position_last: position.value - rigidbody.velocity*time.fixed_delta_time,
                    radius: collider.radius,
                    mass: rigidbody.mass,
                    is_static: false,
                }, net_id.value);
            },
            None => {
                physics_world.add_or_set_object(VerletObject{
                    acceleration: FP2::zero(),
                    position: position.value,
                    position_last: position.value,
                    radius: collider.radius,
                    mass: FP::one(),
                    is_static: true,
                }, net_id.value);
            }
        }
    }

    for (beam, net_id) in beam_query.iter() {
        physics_world.add_or_set_beam(VerletBeam{
            verlet_object_id_a: beam.a,
            verlet_object_id_b: beam.b,
            length: beam.length,
        }, net_id.value);
    }
}

pub fn run_physics_step(mut physics_world: ResMut<VerletPhysicsWorld>, time: Res<Time>){
    //&mut self, dt: FixedPoint, iteration_id_buffer: &mut Vec<u32>, overlap_circle_buffer: &mut Vec<u32>
    let mut buffer_a = Vec::new();
    let mut buffer_b = Vec::new();
    physics_world.update(time.fixed_delta_time,&mut buffer_a, &mut buffer_b);
}

pub fn pull_bodies(
    mut body_query: Query<(&mut Position, &CircleCollider, Option<&mut Rigidbody>, &NetId)>,
    physics_world: Res<VerletPhysicsWorld>,
    time: Res<Time>
){
    for (mut position, _collider, mut rigidbody_opt, net_id) in body_query.iter_mut() {
        if let Some(entry) = physics_world.get_object(net_id.value) {
            match rigidbody_opt {
                Some(mut rigidbody) => {
                    position.value = entry.position;
                    rigidbody.velocity = (entry.position - entry.position_last)/time.fixed_delta_time;
                },
                None => {}
            }
        }
    }
}