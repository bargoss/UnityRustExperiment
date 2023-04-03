use bevy_ecs::entity::Entity;
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
    use crate::game2::terrain::TileOccupation::TerrainBlocked;
    use super::super::super::data_types::*;
    use super::super::super::terrain::*;
    use super::super::super::test_utils::*;



    #[test]
    fn test_handle_unit_movement_by_velocity() {

        let mut world = World::new();
        let mut tile_world = TileWorld::new(10, 10);
        tile_world.set_tiles(Vector2Int{x: 4, y: 0}, Vector2Int{x: 5, y: 9}, TileOccupation::TerrainBlocked);

        world.insert_resource(tile_world);


        // add an entity
        let entity = world.spawn()
            .insert(Unit{velocity: Vec2FFloat::new(0.15, 0.0)})
            .insert(WorldPosition{pos: Vec2FFloat::new(1.5, 4.5)})
            .id();

        run_system_once(&mut world, handle_unit_movement_by_velocity);

        // check unit position, make sure it has moved
        let position = world.get::<WorldPosition>(entity).unwrap();
        println!("position: {:?}", position.pos);
        assert_eq!(position.pos, Vec2FFloat::new(1.65, 4.5));

        // keep moving until velocity is zero
        for _ in 0..100 {
            run_system_once(&mut world, handle_unit_movement_by_velocity);
        }

        // check unit position, make sure it has stopped
        let position = world.get::<WorldPosition>(entity).unwrap();
        println!("position after hitting the wall: {:?}", position.pos);

        // make sure velocity is zero
        let unit = world.get::<Unit>(entity).unwrap();
        println!("unit velocity after hitting the wall: {:?}", unit.velocity);
        assert_eq!(unit.velocity, Vec2FFloat::zero());

        // assert that final position x is somewhere between 3.5 and 4.5
        assert!(position.pos.x >= FFloat::new(3.5) && position.pos.x <= FFloat::new(4.5));
    }
}