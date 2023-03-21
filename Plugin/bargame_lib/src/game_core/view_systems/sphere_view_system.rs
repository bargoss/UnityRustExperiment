use bevy_ecs::prelude::*;
use crate::game_core::components::position::Position;
use crate::game_core::resources::time::Time;
use crate::game_core::view_components::sphere_view::SphereView;
use crate::game_core::view_resources::view_snapshot_interpolator::BufferedViewSnapshotInterpolator;
use crate::game_core::view_resources::view_snapshots::SphereSnapshot::SphereSnapshot;

pub fn sphere_view_system(
    sphere_views: Query<(&SphereView, &Position)>,
    mut sphere_snapshots: ResMut<BufferedViewSnapshotInterpolator<SphereSnapshot>>,
    time: Res<Time>
) {
    for (sphere_view, interpolated_position) in sphere_views.iter() {
        let time = (time.tick as f32) * time.fixed_delta_time.to_f32();
        let position = interpolated_position.value;

        // put them in one line
        println!("view_time: {}, view_position: {}", time, position);

        sphere_snapshots.push(sphere_view.view_custom_id, time, SphereSnapshot{
            position : position.into(),
            radius : sphere_view.radius.into()
        });
    }
}