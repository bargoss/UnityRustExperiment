use bevy_ecs::prelude::*;
use crate::game_core::components::position::Position;
use crate::game_core::view_components::sphere_view::SphereView;
use crate::game_core::view_resources::view_snapshot_interpolator::BufferedViewSnapshotInterpolator;
use crate::game_core::view_resources::view_snapshots::SphereSnapshot::SphereSnapshot;
use crate::game_core::view_resources::view_time::ViewTime;

pub fn sphere_view_system(
    sphere_views: Query<(&SphereView, &Position)>,
    mut sphere_snapshots: ResMut<BufferedViewSnapshotInterpolator<SphereSnapshot>>,
    view_time: Res<ViewTime>
) {
    for (sphere_view, interpolated_position) in sphere_views.iter() {
        let view_time = view_time.time;
        let position = interpolated_position.value;

        sphere_snapshots.push(SphereSnapshot{
            position : position.into(),
            radius : sphere_view.radius.into()
        }, view_time as f32);
    }
}