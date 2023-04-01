
use bevy_ecs::prelude::*;


use crate::arena_fight_game::components::*;
use crate::game_core::common::Id;
use crate::game_core::components::*;
use crate::game_core::math::*;
use crate::game_core::resources::*;
use crate::game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld;


pub fn unit_capture_node_system(
    mut capturing_unit_query: Query<(Entity, &mut Health, &Position, &NetId, &CircleCollider)>,
    mut node_query: Query<(Entity, &mut Node, &Position, &NetId, &CircleCollider)>,
    mut belongs_to_faction_query: Query<(Entity, &mut BelongsToFaction)>,
    physics_world: Res<VerletPhysicsWorld>,
    id_entity_map: Res<IdEntityMap>,
    time: Res<Time>,
) {
    let capture_per_second = FP::new(0.05);
    let capture_per_frame = time.fixed_delta_time * capture_per_second;
    let capture_range = FP::new(1.0);

    let unit_health_loss_per_second = FP::new(0.5);
    let unit_health_loss_per_frame = time.fixed_delta_time * unit_health_loss_per_second;

    let mut nearby_bodies_query_buffer = Vec::new();

    // iterate node_query
    for (node_entity, mut node, node_position, _node_net_id, node_collider) in node_query.iter_mut() {
        physics_world.overlap_circle(node_position.value, capture_range + node_collider.radius, &mut nearby_bodies_query_buffer);

        // iterate nearby_bodies_query_buffer
        for body_id in nearby_bodies_query_buffer.iter() {
            match id_entity_map.get(Id::new(*body_id)) {
                Some(unit_entity) => {
                    if let Ok(captuing_unit) = capturing_unit_query.get_mut(unit_entity) {
                        // unit is capturing node
                        let node_belongs_to_faction = belongs_to_faction_query.get(node_entity).unwrap().1;
                        let capturing_unit_belongs_to_faction = belongs_to_faction_query.get(unit_entity).unwrap().1;

                        if
                            node_belongs_to_faction.faction == capturing_unit_belongs_to_faction.faction &&
                            node.capture_progress_faction == capturing_unit_belongs_to_faction.faction &&
                            node.capture_progress > FP::one() - capture_per_frame * 2
                        {
                            continue;
                        }

                        let node_clone = node.clone();
                        let (updated_node_belongs_to_faction, updated_node) = progress_node_capture_progress(capture_per_frame, capturing_unit_belongs_to_faction.faction, &node_clone, node_belongs_to_faction);
                        *node = updated_node;
                        let mut node_belongs_to_faction = belongs_to_faction_query.get_mut(node_entity).unwrap().1;
                        *node_belongs_to_faction = updated_node_belongs_to_faction;

                        let mut capturing_unit_health = captuing_unit.1;
                        capturing_unit_health.health -= unit_health_loss_per_frame;
                    }
                }
                None => {continue}
            }
        }



    }



}

fn progress_node_capture_progress(capture_amount: FP, capturing_faction: Faction, target_node: &Node, belongs_to_faction: &BelongsToFaction) -> (BelongsToFaction, Node)
{
    let mut updated_node = target_node.clone();
    let mut updated_belongs_to_faction = belongs_to_faction.clone();

    if updated_node.capture_progress_faction != capturing_faction {
        updated_node.capture_progress -= capture_amount;
        if updated_node.capture_progress < FP::zero() {
            updated_node.capture_progress = FP::zero();
            updated_node.capture_progress_faction = capturing_faction;
        }
    }
    else{
        // increase progress
        updated_node.capture_progress += capture_amount;
        if updated_node.capture_progress > FP::one() {
            updated_node.capture_progress = FP::one();
            updated_node.capture_progress_faction = capturing_faction;
            updated_belongs_to_faction.faction = capturing_faction;
        }
    }

    (updated_belongs_to_faction, updated_node)
}