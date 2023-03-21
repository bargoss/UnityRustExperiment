use std::io::Read;
use bevy_ecs::prelude::*;
use bevy_ecs::query::{Fetch, QueryEntityError, QueryItem, ROQueryItem, WorldQuery, WorldQueryGats};
use crate::game_core::components::position::Position;
use crate::game_core::components::rigidbody::Rigidbody;
use crate::game_core::resources::id_entity_map::IdEntityMap;
use crate::game_core::view_components::beam_view::LineView;
use crate::game_core::view_resources::view_snapshot_interpolator::BufferedViewSnapshotInterpolator;
use crate::game_core::view_resources::view_snapshots::LineSnapshot::LineSnapshot;
use crate::game_core::view_resources::view_time::ViewTime;

/*
pub fn my_system(
    line_view_query: Query<(Entity, &LineView)>,
    position_query: Query<(Entity, &Position)>,
    id_entity_map: Res<IdEntityMap>,
    mut line_snapshots: BufferedViewSnapshotInterpolator<LineSnapshot>,
    view_time: Res<ViewTime>
){
    for (entity, line_view) in line_view_query.iter() {
        let start_position = match id_entity_map.get_query_result(line_view.start_id, position_query) {
            Some(result) => {
                result.1.value
            }
            None => continue,
        };
        let end_position = match id_entity_map.get_query_result(line_view.end_id, position_query) {
            Some(result) => {
                result.1.value
            }
            None => continue,
        };
    }
}
 */

/*
pub fn my_system(
    position_query: Query<(Entity, &Position)>,
    rigidbody_query: Query<(Entity, &Rigidbody)>,
){
    position_query.iter().for_each(|(entity, position)| {
        if let Ok((_, rigidbody)) = rigidbody_query.get(entity) {
            println!("Entity {:?} has position {:?} and velocity {:?}", entity, position, rigidbody);
        }
    });
}
 */

pub fn get_component<'w, Q: WorldQuery + WorldQuery<ReadOnly = Q>>(
    query: &'w Query<'w, 'w, Q>,
    entity: Entity,
) -> Result<<<Q as WorldQueryGats<'w>>::Fetch as Fetch<'w>>::Item, QueryEntityError> {
    query.get(entity)
}

pub fn my_system(
    position_query: Query<(Entity, &Position)>,
    rigidbody_query: Query<(Entity, &Rigidbody)>,
) {
    position_query.iter().for_each(|(entity, position)| {
        if let Ok((_, rigidbody)) = get_component(&rigidbody_query, entity) {
            println!("Entity {:?} has position {:?} and rigidbody {:?}", entity, position, rigidbody);
        }
    });
}

pub fn line_view_system(
    line_view_query: Query<(Entity, &LineView)>,
    position_query: Query<(Entity, &Position)>,
    id_entity_map: Res<IdEntityMap>,
    mut line_snapshots: BufferedViewSnapshotInterpolator<LineSnapshot>,
    view_time: Res<ViewTime>
) {
    for (entity, line_view) in line_view_query.iter() {
        // id_entity_map.get(line_view.start)
        let start_pos = match id_entity_map.get(line_view.start) {
            Some(entity) => {
                match position_query.get(entity) {
                    Ok(interpolated_position_entity) => {
                        interpolated_position_entity.1.value
                    },
                    Err(_) => continue,
                }
            }
            None => continue,
        };
        let end_pos = match id_entity_map.get(line_view.end) {
            Some(entity) => {
                match position_query.get(entity) {
                    Ok(interpolated_position_entity) => {
                        interpolated_position_entity.1.value
                    },
                    Err(_) => continue,
                }
            }
            None => continue,
        };

        let view_time = view_time.time;

        line_snapshots.push(LineSnapshot{
            start : start_pos.into(),
            end : end_pos.into(),
            width : line_view.thickness.into()
        }, view_time as f32);
    }
}
