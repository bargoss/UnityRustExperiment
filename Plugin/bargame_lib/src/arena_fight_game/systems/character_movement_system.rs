

use bevy_ecs::prelude::{Query};

use crate::arena_fight_game::components::CharacterMovement;
use crate::game_core::components::impulse::Impulse;

use crate::game_core::components::rigidbody::Rigidbody;

use crate::game_core::math::FP;

pub fn character_movement_system(
    mut character_query: Query<(&CharacterMovement, &Rigidbody, &mut Impulse)>,
) {
    let damping = FP::new(0.075);
    let force_multiplier = FP::new(0.1);
    for (character, rigidbody, mut impulse) in character_query.iter_mut() {
        let movement_dir = character.movement_direction;
        let movement_impulse = movement_dir * rigidbody.mass * force_multiplier;
        let dampen_impulse = -rigidbody.velocity * rigidbody.mass * damping;
        impulse.value += movement_impulse;
        impulse.value += dampen_impulse;
    }
}
