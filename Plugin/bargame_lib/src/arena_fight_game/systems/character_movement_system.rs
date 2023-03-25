use bevy_ecs::prelude::{Query, Res};
use crate::arena_fight_game::arena_game::ArenaFightInput;
use crate::arena_fight_game::components::Character;
use crate::game_core::components::position::Position;
use crate::game_core::game_world::PlayerInputMap;

pub fn character_movement(
    query: Query<(&Position, &Character)>,
    player_id_to_input_map: &Res<PlayerInputMap<ArenaFightInput>>,
){

}