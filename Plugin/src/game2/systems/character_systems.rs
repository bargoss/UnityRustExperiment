use bevy_ecs::prelude::{Query, Res};
use crate::game2::components::position::WorldPosition;
use crate::game2::components::unit::Unit;
use super::super::data_types::*;
use super::super::terrain::*;

pub fn handle_unit_tile_map_collisions(mut query: Query<(&WorldPosition, &mut Unit)>, tile_world: Res<TileWorld>){
    for (position, mut unit) in query.iter_mut() {
        let mut unit_velocity = &mut unit.velocity;
        let unit_position = position.pos;






    }
}




//pub fn handle_character_movement(mut query: Query<(&Position, &mut Velocity)>, mut tile_world: ResMut<TileWorld>){
//    for (position, mut velocity) in query.iter_mut() {
//        for push_point in push_points.points.iter() {
//            let delta_to_push_point = position.value - *push_point;
//            let sqr_distance = delta_to_push_point.length_squared();
//            let effect_radius = 15.0;
//            if sqr_distance < effect_radius * effect_radius {
//                let direction = delta_to_push_point.normalize();
//                velocity.value += direction * 0.2 * DELTA_TIME;
//            }
//        }
//    }
//
//    // clear the push points
//    push_points.points.clear();
//}
//