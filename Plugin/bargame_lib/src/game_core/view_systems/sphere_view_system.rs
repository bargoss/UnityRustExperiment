use bevy_ecs::prelude::*;
use crate::game_core::common::Vector2;
use crate::game_core::view_components::interpolated_position::InterpolatedPosition;
use crate::game_core::view_components::sphere_view::SphereView;
use crate::game_core::view_resources::view_snapshots::{LineSnapshots, SphereSnapshot, SphereSnapshots};


// fn run_physics_step(
// mut physics_world: ResMut<VerletPhysicsWorld>,
// time: Res<Time>
// ){
pub fn sphere_view_system(
    sphere_views: Query<(Entity, &SphereView, &InterpolatedPosition)>,
    mut sphere_snapshots: ResMut<SphereSnapshots>
) {
    todo!("sphere_view_system");
    //sphere_snapshots.spheres.clear();
    //for (entity, sphere_view, interpolated_position) in sphere_views.iter() {
    //    sphere_snapshots.spheres.push(SphereSnapshot{
    //        position: interpolated_position.value.into(),
    //        radius: sphere_view.radius.into(),
    //    });
    //}
}