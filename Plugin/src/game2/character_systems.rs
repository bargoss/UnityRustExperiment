use super::terrain::TileWorld;
use super::data_types::Vector2Int;
use super::data_types::Vec2FFloat;


use bevy_ecs::{self, bundle::Bundle, component::Component, entity::Entity, prelude::{Commands, Query, Res, ResMut}};
//use bevy_ecs::schedule::Stage;
//use bevy_ecs::system::CommandQueue;
//use bevy_ecs::world::World;
use bevy_math::Vec3;

#[derive(Component, Clone ,Debug, Default)]
pub struct Position  {
    pub value: Vec3
}
#[derive(Component, Clone ,Debug, Default)]
pub struct Velocity {
    pub value: Vec3
}
#[derive(Component, Clone ,Debug, Default)]
pub struct Collider {
    pub value: Vec3
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