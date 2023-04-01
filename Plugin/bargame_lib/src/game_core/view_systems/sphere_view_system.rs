use bevy_ecs::prelude::*;
use crate::game_core::components::position::Position;
use crate::game_core::resources::time::Time;
use crate::game_core::verlet_physics::FixedPoint;
use crate::game_core::view_components::sphere_view::SphereView;
use crate::game_core::view_resources::view_snapshot_interpolator::BufferedViewSnapshotInterpolator;
use crate::game_core::view_resources::view_snapshots::sphere_snapshot::SphereSnapshot;

pub fn sphere_view_system(
    sphere_views: Query<(&SphereView, &Position)>,
    mut sphere_snapshots: ResMut<BufferedViewSnapshotInterpolator<SphereSnapshot>>,
    time: Res<Time>
) {
    for (sphere_view, position) in sphere_views.iter() {
        let time = FixedPoint::new(time.tick as f64) * time.fixed_delta_time;
        let position = position.value;
        let radius = sphere_view.radius;

        // put them in one line
        //println!("view_time: {}, view_position: {}, radius: {}", time, position, radius);

        sphere_snapshots.push(sphere_view.view_custom_id, time, SphereSnapshot{
            position : position.into(),
            radius : sphere_view.radius.into(),
            color: [1.0, 1.0, 1.0, 1.0]
        });
    }
}