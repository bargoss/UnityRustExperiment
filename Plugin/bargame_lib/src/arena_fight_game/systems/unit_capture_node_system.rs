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
    mut capturing_unit_query: Query<(&mut Health, &Position, &BelongsToFaction, &NetId, &CircleCollider)>,
    mut node_query: Query<(Entity, &mut Node, &Position, &mut BelongsToFaction, &NetId, &CircleCollider)>,
    physics_world: Res<VerletPhysicsWorld>,
    id_entity_map: Res<IdEntityMap>,
    time: Res<Time>,
){
    let capture_per_second = FixedPoint::new(0.05);
    let capture_per_frame = time.fixed_delta_time * capture_per_second;
    let capture_range = FixedPoint::new(1.0);

    let unit_health_loss_per_second = FixedPoint::new(0.5);
    let unit_health_loss_per_frame = time.fixed_delta_time * unit_health_loss_per_second;

    // Collect entities and their NetIds into a vector
    let mut entities_and_net_ids: Vec<(Entity, NetId)> = node_query.iter().map(|(entity, _, _, _, net_id, _)| (entity, *net_id)).collect();

    // Sort the vector by NetId
    entities_and_net_ids.sort_by_key(|(_, net_id)| *net_id);


    let mut nearby_bodies_query_buffer = Vec::new();

    // Iterate through the sorted entities
    for (entity, _) in entities_and_net_ids {
        // Get the node components for each entity
        if let Ok((_, mut node, node_position, mut node_faction, node_net_id, node_collider)) = node_query.get_mut(entity) {
            // Get the entities that are overlapping the node
            physics_world.overlap_circle(node_position.value, node_collider.radius + FixedPoint::new(0.1), &mut nearby_bodies_query_buffer);

            // Iterate through the entities that are overlapping the node
            for body_id in nearby_bodies_query_buffer.iter() {
                // Get the capturing unit components
                if let Some((mut capturing_unit, capturing_unit_position, capturing_unit_faction, capturing_unit_net_id, capturing_unit_collider)) = id_entity_map.get_mut_from_query(&mut capturing_unit_query, Id::new(*body_id)) {
                    // Check if the capturing unit is on the same faction as the node
                    if capturing_unit_faction.faction != node_faction.faction {
                        // Check if the capturing unit is within range of the node
                        let distance = (capturing_unit_position.value - node_position.value).magnitude_squared();
                        if distance < capture_range * capture_range {
                            // Decrease the capturing unit's health
                            capturing_unit.health -= unit_health_loss_per_frame;
                            // Decrease the node's capture progress
                            progress_node_capture_progress(capture_per_frame, capturing_unit_faction.faction, &mut node, &mut node_faction);
                        }
                    }
                }
            }
        }
    }
}

fn progress_node_capture_progress(capture_amount: FixedPoint, capturing_faction: Faction, target_node: &mut Node, belongs_to_faction: &mut BelongsToFaction) {
    if target_node.capture_progress_faction != capturing_faction {
        // decrease progress
        target_node.capture_progress -= capture_amount;
        if target_node.capture_progress < FixedPoint::zero() {
            target_node.capture_progress = FixedPoint::zero();
            target_node.capture_progress_faction = capturing_faction;
        }
    }
    else{
        // increase progress
        target_node.capture_progress += capture_amount;
        if target_node.capture_progress > FixedPoint::one() {
            target_node.capture_progress = FixedPoint::one();
            target_node.capture_progress_faction = capturing_faction;
            belongs_to_faction.faction = capturing_faction;
        }
    }
}
