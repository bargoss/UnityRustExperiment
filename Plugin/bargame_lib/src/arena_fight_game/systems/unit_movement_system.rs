use bevy_ecs::prelude::*;
use nalgebra::distance;
use crate::arena_fight_game::components::*;
use crate::game_core::components::*;
use crate::game_core::resources::*;
use crate::game_core::verlet_physics::FixedPoint;

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
                let delta = target_position.value - position.value;
                let delta_mag = delta.magnitude();
                let dir_towards_target = delta / (delta_mag + FixedPoint::new(0.0001)); // prevent divide by zero

                let orbit_dir = dir_towards_target.perp();

                //let orbit_component = FixedPoint::inverse_lerp(orbit_distance*FixedPoint::new(0.5), orbit_distance, delta_mag).clamp01();
                //let follow_component = FixedPoint::new(1.0) - orbit_component;

                let mut follow_component = FixedPoint::new(1.0);
                let mut orbit_component = FixedPoint::new(0.0);


                if delta_mag < FixedPoint::new(2.5) {
                    follow_component = FixedPoint::new(0.0);
                    orbit_component = FixedPoint::new(1.0)
                }

                let movement_direction = dir_towards_target * follow_component + orbit_dir * orbit_component;

                character_movement.movement_direction = movement_direction;
            }
        }
    }
}