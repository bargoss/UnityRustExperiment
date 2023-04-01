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
    mut unit_query: Query<(&mut Unit, &BelongsToFaction)>,
    physics_world: Res<VerletPhysicsWorld>,
    id_entity_map: Res<IdEntityMap>,
) {
    let extra_control_radius = FP::new(6.0);

    let mut nearby_bodies_query_buffer = Vec::new();

    for (player_id, input) in player_input_map.iter() {
        if let Some(node_drag_drop_input) = input.select_and_set_destination {
            let faction = player_to_faction(*player_id);

            if id_entity_map.get_from_query(&node_query, node_drag_drop_input.target_node_net_id.value).is_none() {
                return;
            }

            physics_world.overlap_circle(node_drag_drop_input.position, node_drag_drop_input.radius, &mut nearby_bodies_query_buffer);

            for body_id in nearby_bodies_query_buffer.iter() {
                let (mut unit, unit_faction) = match id_entity_map.get_mut_from_query(&mut unit_query, Id::new(*body_id)) {
                    Some((unit, faction)) => (unit, faction.faction),
                    None => continue,
                };

                if unit_faction == faction {
                    unit.following_entity = Some(node_drag_drop_input.target_node_net_id)
                }
            }
        }
    }
}