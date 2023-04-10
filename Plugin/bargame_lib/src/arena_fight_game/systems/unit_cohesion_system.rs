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
        physics_world.overlap_circle(unit_position.value, FP::new(0.5), &mut nearby_bodies_buffer);

        let mut force = FP2::zero();

        // sort by sqr distance
        //nearby_bodies_buffer.sort_by(|a, b| {
        //    let a_pos = position_query.get(id_entity_map.get( a).unwrap()).unwrap().value;
        //    let b_pos = position_query.get(id_entity_map.get( b).unwrap()).unwrap().value;
        //    let a_dist = (a_pos - unit_position.value).sqr_length();
        //    let b_dist = (b_pos - unit_position.value).sqr_length();
        //    a_dist.partial_cmp(&b_dist).unwrap()
        //});
    }
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