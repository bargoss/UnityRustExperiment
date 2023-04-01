use bevy_ecs::prelude::*;
use crate::arena_fight_game::arena_game::ArenaInput;
use crate::arena_fight_game::components::{CharacterMovement, Faction, PlayerCharacterControl};
use crate::game_core::common::Id;
use crate::game_core::game_world::PlayerInputMap;

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
    mut player_input_map: ResMut<PlayerInputMap<ArenaInput>>,
    mut character_query: Query<(&mut CharacterMovement, &PlayerCharacterControl)>,
) {

}