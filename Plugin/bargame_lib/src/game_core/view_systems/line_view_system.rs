use bevy_ecs::prelude::*;
use crate::game_core::components::position::Position;
use crate::game_core::resources::id_entity_map::IdEntityMap;
use crate::game_core::view_components::beam_view::LineView;
use crate::game_core::view_resources::view_snapshot_interpolator::BufferedViewSnapshotInterpolator;
use crate::game_core::view_resources::view_snapshots::LineSnapshot::LineSnapshot;
use crate::game_core::view_resources::view_time::ViewTime;

pub fn line_view_system(
    line_view_query: Query<&LineView>,
    position_query: Query<(Entity, &Position)>,
    id_entity_map: Res<IdEntityMap>,
    mut line_snapshots: ResMut<BufferedViewSnapshotInterpolator<LineSnapshot>>,
    view_time: Res<ViewTime>,
) {
    for line_view in line_view_query.iter() {
        let start_pos = match id_entity_map.get_from_query(&position_query, line_view.start) {
            Ok((_, interpolated_position)) => interpolated_position.value,
            Err(_) => continue,
        };

        let end_pos = match id_entity_map.get_from_query(&position_query, line_view.end) {
            Ok((_, interpolated_position)) => interpolated_position.value,
            Err(_) => continue,
        };

        line_snapshots.push(
            LineSnapshot {
                start: start_pos.into(),
                end: end_pos.into(),
                width: line_view.thickness.into(),
            },
            view_time.time as f32,
        );
    }
}

