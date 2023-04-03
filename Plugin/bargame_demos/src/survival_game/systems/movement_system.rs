use bevy_ecs::system::{Query, Res};

use crate::{bubbles::spatial_ds::LookUpGrids};
use crate::survival_game::{components::Pawn, static_world::tilemap::TileMap};

/// .
pub fn _handle_pawn_movement(
    mut _pawn_query: Query<&mut Pawn>,
    _lookup_grids: Res<LookUpGrids<u32>>,
    _terrain: Res<TileMap>
    //mut read_query: Query<(&Bubble, &Position)>,
    //mut write_query : Query<(&Bubble, &Position, &mut Velocity)>,
    //lookup_grids: Res<LookUpGrids<u32>>,
    //mut buffer: ResMut<Vec<(u32, u32)>>, // for neighbor pair ids
){
    //for (position, mut velocity) in query.iter_mut() {
    for mut _pawn in _pawn_query.iter_mut() {
        let movement = _pawn.movement;
        _pawn.position += movement;
    }
    
}
