use bevy_ecs::prelude::*;
use crate::arena_fight_game::components::*;
use crate::game_core::common::Id;
use crate::game_core::components::*;
use crate::game_core::math::*;
use crate::game_core::resources::*;
use crate::game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld;

pub fn unit_cohesion_system(
    unit_query: Query<(&NetId ,&Unit, &Position, &Rigidbody, &BelongsToFaction, Entity)>,
    position_query: Query<&Position>,
    mut impulse_query: Query<&mut Impulse>,
    physics_world: Res<VerletPhysicsWorld>,
    id_entity_map: Res<IdEntityMap>,
    time: Res<Time>,
) {
    let mut nearby_bodies_buffer = Vec::new();

    for (net_id, unit, unit_position, unit_rigidbody, unit_faction, unit_entity) in unit_query.iter() {
        physics_world.overlap_circle(unit_position.value, FP::new(2.5), &mut nearby_bodies_buffer);
        let mut force = FP2::zero();

        nearby_bodies_buffer.sort_by(|a, b| {
            let a_pos = match id_entity_map.get_from_query(&position_query, *a) {
                Some(pos) => pos.value,
                None => return std::cmp::Ordering::Equal,
            };
            let b_pos = match id_entity_map.get_from_query(&position_query, *b) {
                Some(pos) => pos.value,
                None => return std::cmp::Ordering::Equal,
            };
            let a_dist = (a_pos - unit_position.value).magnitude_squared();
            let b_dist = (b_pos - unit_position.value).magnitude_squared();
            a_dist.partial_cmp(&b_dist).unwrap()
        });

        // iterate taking first two
        nearby_bodies_buffer.iter().take(2).for_each(|body_id| {
            let body_pos = match id_entity_map.get_from_query(&position_query, *body_id) {
                Some(pos) => pos.value,
                None => return,
            };
            force += calculate_cohesion(unit_position.value, body_pos, FP::new(10.5), FP::new(2.5));
        });

        let mut impulse = impulse_query.get_mut(unit_entity).unwrap();
        impulse.value += force * time.fixed_delta_time;
    }
}

pub fn calculate_cohesion(pos_a : FP2 , pos_b : FP2, multiplier : FP, max_distance : FP) -> FP2 {
    let distance = (pos_a - pos_b).magnitude();
    if distance == FP::zero() {
        return FP2::zero();
    }
    if distance > max_distance {
        return FP2::zero();
    }
    let distance_multiplier = multiplier * (max_distance - distance) / max_distance;
    (pos_b - pos_a).normalize() * distance_multiplier
}



/*
pub fn unit_cohesion_system2(
    unit_query: Query<(&NetId ,&Unit, &Position, &Rigidbody, &BelongsToFaction, Entity)>,
    mut impulse_query: Query<&mut Impulse>,
    physics_world: Res<VerletPhysicsWorld>,
    id_entity_map: Res<IdEntityMap>,
    time: Res<Time>,
){
    let mut nearby_bodies_buffer = Vec::new();

    for (net_id, unit, unit_position, unit_rigidbody, unit_faction, unit_entity) in unit_query.iter() {
        physics_world.overlap_circle(unit_position.value, FP::new(0.5), &mut nearby_bodies_buffer);

        let mut cohesion = FP2::zero();

        for body_id in nearby_bodies_buffer.iter() {
            if *body_id == net_id.value.0 {
                continue;
            }

            let res = id_entity_map.get_from_query(&unit_query, Id::new(*body_id));
            if let Some((_net_id, _unit, target_unit_position, _target_unit_rigidbody, target_unit_faction, target_unity_entity)) = res {
                if unit_faction.faction == target_unit_faction.faction {
                    cohesion += target_unit_position.value - unit_position.value;
                }
            }
        }

        if !nearby_bodies_buffer.is_empty() {
            cohesion /= nearby_bodies_buffer.len() as FP;
        }

        if cohesion.magnitude_squared() > FP::new(0.0001) {
            let impulse = cohesion.normalize() * FP::new(0.1);
            //if let Ok(mut impulse) = impulse_query.get_mut(net_id.value) {
            //    impulse.value += impulse;
            //}
            if let Ok(mut impulse) = impulse_query.get_mut(unit_entity) {
                impulse.value += impulse;
            }
        }
    }
}
 */