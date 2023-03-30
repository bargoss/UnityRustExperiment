use super::*;
use bevy_ecs::prelude::*;
use crate::arena_fight_game::arena_game::ArenaInput;
use crate::arena_fight_game::components::{CharacterMovement, PlayerCharacterControl};
use crate::game_core::game_world::PlayerInputMap;

pub fn player_input_system(
    mut player_input_map: ResMut<PlayerInputMap<ArenaInput>>,
    mut character_query: Query<(&mut CharacterMovement, &PlayerCharacterControl)>,
) {
    for (mut character, player_control) in character_query.iter_mut() {
        let input = player_input_map
            .get(&player_control.controlling_player_id)
            .unwrap_or_default();
        print!("input: {:?}", input);
        character.movement_direction = input.movement_direction;
    }
}