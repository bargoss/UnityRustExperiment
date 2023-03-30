use std::collections::HashMap;
use bevy_ecs::prelude::*;
use nalgebra::DimAdd;
use crate::arena_fight_game::components::*;
use crate::game_core::common::Id;
use crate::game_core::components::*;
use crate::game_core::math::*;
use crate::game_core::resources::*;
use crate::game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld;


pub fn unit_capture_node_system(
    mut capturing_unit_query: Query<(&mut Health, &Position, &BelongsToFaction, &NetId)>,
    mut target_node_query: Query<(&mut Node, &Position, &mut BelongsToFaction, &NetId)>,
    physics_world: Res<VerletPhysicsWorld>,
    id_entity_map: Res<IdEntityMap>,
    time: Res<Time>,
){
    let capture_per_second = FixedPoint::one();
    let capture_per_frame = time.fixed_delta_time * capture_per_second;
    let capture_range = FixedPoint::new(1.0);
    let mut nearby_bodies_query_buffer = Vec::new();

    let mut all_captures = HashMap::new();

    for (mut capturing_unit, capturing_unit_position, mut capturing_unit_faction, capturing_unit_net_id) in capturing_unit_query.iter_mut() {
        physics_world.overlap_circle(capturing_unit_position.value, capture_range, &mut nearby_bodies_query_buffer);

        // exclude self
        for body_id in nearby_bodies_query_buffer.iter() {
            if *body_id == capturing_unit_net_id.value.0 {
                continue;
            }
            let res = id_entity_map.get_mut_from_query(&mut target_node_query, Id::new(*body_id));
            if let Some((mut target_node, target_node_position, target_node_faction,target_node_net_id)) = res {

            }
        }
    }
}

fn progress_node_capture_progress(capture_amount: FixedPoint, capturing_faction: Faction, target_node: &mut Node) {
    if target_node.capture_progress_faction != capturing_faction {
        // decrease progress
        target_node.capture_progress -= capture_amount;
    }
    else {
        // increase progress
        target_node.capture_progress += capture_amount;
    }
}

deal with node capture race conditions

fn progress_node_capture_progress_faction(target_node: &mut Node, target_node_faction: BelongsToFaction) {
    if target_node.capture_progress >= FixedPoint::one() {
        // decrease progress
        target_node.capture_progress = FixedPoint::one();
        target_node.capture_progress_faction = target_node_faction.faction;
    }
    if target_node.capture_progress <= FixedPoint::zero() {
        target_node.capture_progress = FixedPoint::zero();
        target_node.capture_progress_faction = target_node_faction.faction;
    }
}