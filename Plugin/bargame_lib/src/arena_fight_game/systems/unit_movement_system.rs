use bevy_ecs::prelude::*;
use crate::arena_fight_game::components::*;
use crate::game_core::components::*;
use crate::game_core::math::*;
use crate::game_core::resources::*;

pub fn unit_movement_system(
    mut unit_query: Query<(&Unit, &Rigidbody, &mut Impulse, &Position)>,
    time: Res<Time>,
) {
    let damping: FixedPoint = FixedPoint::new(0.05);
    for (unit, rigidbody, mut impulse, position) in unit_query.iter_mut() {
        let target_movement_position = unit.target_movement_position;
        let movement_dir = (target_movement_position - position.value).normalize();
        let movement_impulse = movement_dir * rigidbody.mass;
        let dampen_impulse = -rigidbody.velocity * rigidbody.mass * damping;
        let total = time.fixed_delta_time * (movement_impulse + dampen_impulse);
        impulse.value += total;
    }
}