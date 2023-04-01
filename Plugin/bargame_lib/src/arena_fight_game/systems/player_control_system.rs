use bevy_ecs::prelude::*;
use crate::arena_fight_game::arena_game::ArenaInput;
use crate::arena_fight_game::components::*;
use crate::game_core::common::Id;
use crate::game_core::components::*;
use crate::game_core::game_world::PlayerInputMap;
use crate::game_core::math::*;
use crate::game_core::resources::IdEntityMap;
use crate::game_core::verlet_physics::verlet_physics_world::VerletPhysicsWorld;

pub fn player_to_faction(player_id: Id) -> Faction {
    match player_id.0 % 4 {
        0 => Faction::Red,
        1 => Faction::Blue,
        2 => Faction::Green,
        3 => Faction::Yellow,
        _ => unreachable!(),
    }
}

pub fn player_control_system(
    player_input_map: ResMut<PlayerInputMap<ArenaInput>>,
    node_query: Query<(&Node, &Position, &CircleCollider)>,
    mut unit_query: Query<(&mut Unit, &Faction)>,
    physics_world: Res<VerletPhysicsWorld>,
    id_entity_map: Res<IdEntityMap>,
) {
    let extra_control_radius = FixedPoint::new(0.5);

    let mut nearby_bodies_query_buffer = Vec::new();

    player_input_map.iter().for_each(|(player_id, input)| {
        if let Some(node_drag_drop_input) = input.node_drag_drop {
            let faction = player_to_faction(*player_id);

            //let source_node = node_query.get(node_drag_drop_input.source_node).unwrap();
            // get it or continue
            let (source_node_position,source_node_radius) = match id_entity_map.get_from_query(&node_query, node_drag_drop_input.source_node_net_id.value) {
                Some((_, position, collider)) => (position.value, collider.radius),
                None => return,
            };

            // just check that it exists
            if id_entity_map.get_from_query(&node_query, node_drag_drop_input.target_node_net_id.value).is_none() {
                return;
            }

            // physics_world.overlap_circle(attacking_unit_position.value, FixedPoint::new(0.5), &mut nearby_bodies_query_buffer);
            physics_world.overlap_circle(source_node_position, source_node_radius + extra_control_radius, &mut nearby_bodies_query_buffer);

            // exclude self
            for body_id in nearby_bodies_query_buffer.iter() {
                if *body_id == node_drag_drop_input.source_node_net_id.value.0 {
                    continue;
                }

                let (mut unit, unit_faction) = match id_entity_map.get_mut_from_query(&mut unit_query, Id::new(*body_id)) {
                    Some((unit, faction)) => (unit, faction),
                    None => continue,
                };

                if unit_faction == &faction {
                    unit.following_entity = Some(node_drag_drop_input.target_node_net_id)
                }
            }
        }
    });
}