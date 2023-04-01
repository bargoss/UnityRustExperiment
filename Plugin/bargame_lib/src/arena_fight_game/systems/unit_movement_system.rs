use bevy_ecs::prelude::*;
use crate::arena_fight_game::components::*;
use crate::game_core::components::*;
use crate::game_core::resources::*;

pub fn unit_movement_system(
    mut unit_query: Query<(&Unit, &mut CharacterMovement, &Position)>,
    position_query: Query<&Position>,
    id_entity_map: Res<IdEntityMap>,
) {
    for (unit, mut character_movement, position) in unit_query.iter_mut() {
        // let end_pos = match id_entity_map.get_from_query(&position_query, line_view.end) {
        match unit.following_entity {
            None => {continue}
            Some(following_net_id) => {
                let target_position = id_entity_map.get_from_query(&position_query, following_net_id.value).unwrap();
                let dir_normalized = (target_position.value - position.value).safe_normalize();
                character_movement.movement_direction = dir_normalized;
            }
        }
    }
}