use bevy_ecs::prelude::{Query, Res};
use crate::game2::components::position::WorldPosition;
use crate::game2::components::unit::Unit;
use super::super::data_types::*;
use super::super::terrain::*;

pub fn handle_unit_movement_by_velocity(mut query: Query<(&mut WorldPosition, &mut Unit)>, tile_world: Res<TileWorld>){
    for (mut position, mut unit) in query.iter_mut() {
        // raycast with unit velocity
        let raycast_result = tile_world.raycast(TileWorldRaycastParams{
            start: position.pos,
            end: position.pos + unit.velocity,
        });

        // if raycast result is not None, then we hit something
        match raycast_result {
            TileWorldRaycastResult::HitOccupiedTile { .. } => {
                // stop and back off a bit and don't move
                unit.velocity = Vec2FFloat::zero();
            }
            TileWorldRaycastResult::HitNothing => {
                position.pos += unit.velocity;
            }
        }
    }
}




// tests
#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::*;
    use crate::game2::components::position::WorldPosition;
    use crate::game2::components::unit::Unit;
    use super::super::super::data_types::*;
    use super::super::super::terrain::*;

    fn bevy_ecs_test(system: impl bevy_ecs::schedule::IntoSystemDescriptor<()>) {
        let mut world = World::new();
        let mut init_schedule = Schedule::default();
        let mut init_stage = SystemStage::single_threaded();
        init_schedule.add_stage("init", init_stage);

        let mut update_schedule = Schedule::default();
        let mut update_stage = SystemStage::single_threaded();
        update_stage.add_system(system);
        update_schedule.add_stage("update", update_stage);

        init_schedule.run_once(&mut world);
        update_schedule.run_once(&mut world);
    }

    #[test]
    fn test_handle_unit_movement_by_velocity() {
        let mut world = World::new();

        let mut tile_world = TileWorld::new(10, 10);

        let mut init_schedule = Schedule::default();
        let mut init_stage = SystemStage::single_threaded();
        init_stage.add_system(handle_unit_movement_by_velocity);
        init_schedule.add_stage("init", init_stage);

        let mut update_schedule = Schedule::default();


    }
}