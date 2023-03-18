use bevy_ecs::prelude::*;
use crate::game_core::common::Vector2;
use crate::game_core::resources::id_entity_map::IdEntityMap;
use crate::game_core::view_components::beam_view::LineView;
use crate::game_core::view_components::interpolated_position::InterpolatedPosition;
use crate::game_core::view_components::sphere_view::SphereView;
use crate::game_core::view_resources::view_snapshots::{LineSnapshot, LineSnapshots, SphereSnapshot, SphereSnapshots};


//pub fn sphere_view_system(
//    sphere_views: Query<(Entity, &SphereView, &InterpolatedPosition)>,
//    mut sphere_snapshots: ResMut<SphereSnapshots>
//) {
//    sphere_snapshots.spheres.clear();
//    for (entity, sphere_view, interpolated_position) in sphere_views.iter() {
//        sphere_snapshots.spheres.push(SphereSnapshot{
//            position: interpolated_position.value.into(),
//            radius: sphere_view.radius.into(),
//        });
//    }
//}


pub fn line_view_system(
    beam_entities: Query<(Entity, &LineView)>,
    interpolated_position_entities: Query<(Entity, &InterpolatedPosition)>,
    id_entity_map: Res<IdEntityMap>,
    mut line_snapshots: ResMut<LineSnapshots>
) {
    line_snapshots.lines.clear();
    for (entity, line_view) in beam_entities.iter() {
        todo!("need to figure out a good net_id mapping system");

        //let start = match interpolated_position_entities.get(line_view.start) {
        //    Ok(interpolated_position) => interpolated_position.value,
        //    Err(_) => continue,
        //};
        //let end = match interpolated_position_entities.get(line_view.end) {
        //    Ok(interpolated_position) => interpolated_position.value,
        //    Err(_) => continue,
        //};
        //line_snapshots.lines.push(LineSnapshot{
        //    start: start.into(),
        //    end: end.into(),
        //    width: line_view.width.into(),
        //});
    }
}