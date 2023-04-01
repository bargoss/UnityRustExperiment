use bevy_ecs::prelude::*;
use crate::game_core::components::position::Position;
use crate::game_core::math::FP;
use crate::game_core::resources::id_entity_map::IdEntityMap;
use crate::game_core::resources::time::Time;
use crate::game_core::view_components::beam_view::LineView;
use crate::game_core::view_resources::view_snapshot_interpolator::BufferedViewSnapshotInterpolator;
use crate::game_core::view_resources::view_snapshots::line_snapshot::LineSnapshot;

pub fn line_view_system(
    line_view_query: Query<&LineView>,
    position_query: Query<(Entity, &Position)>,
    id_entity_map: Res<IdEntityMap>,
    mut line_snapshots: ResMut<BufferedViewSnapshotInterpolator<LineSnapshot>>,
    time: Res<Time>,
) {
    for line_view in line_view_query.iter() {
        let start_pos = match id_entity_map.get_from_query(&position_query, line_view.start) {
            Some((_, interpolated_position)) => interpolated_position.value,
            None => continue,
        };

        let end_pos = match id_entity_map.get_from_query(&position_query, line_view.end) {
            Some((_, interpolated_position)) => interpolated_position.value,
            None => continue,
        };

        let time = FP::new(time.tick as f64) * time.fixed_delta_time;
        // pub fn push(&mut self, view_custom_id: Id, time: f32, value: T) {
        line_snapshots.push(
            line_view.view_custom_id, time, LineSnapshot {
                start: start_pos.into(),
                end: end_pos.into(),
                width: line_view.thickness.into(),
            }
        );
    }
}

