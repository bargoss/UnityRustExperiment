use bevy_ecs::prelude::{Query, Res};
use crate::arena_fight_game::arena_game::ArenaFightInput;
use crate::arena_fight_game::components::Character;
use crate::game_core::components::impulse::Impulse;
use crate::game_core::components::position::Position;
use crate::game_core::components::rigidbody::Rigidbody;
use crate::game_core::game_world::PlayerInputMap;

pub fn character_movement(
    mut character_query: Query<(&Character, &Rigidbody, &mut Impulse)>,
) {
    for (character, rigidbody, mut impulse) in character_query.iter_mut() {
        let mut impulse = impulse;
        let movement_dir = character.movement_direction;
        impulse.value += movement_dir * rigidbody.mass;
    }
}
