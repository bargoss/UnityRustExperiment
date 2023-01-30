use std::default;
use std::collections::HashMap;
use bevy_math::Vec2;
use super::data_types::Vector2Int;



// use ico_math
//use ico_math::Vector2Int;

pub struct TileWorld{
    size_x : usize,
    size_y : usize,
    tile_occupations : Box<[TileOccupation]>,
}
pub enum TileWorldRaycastResult{
    HitOccupiedTile{pos : Vector2Int, tile : TileOccupation},
    HitNothing,
}

pub struct TileWorldRaycastParams{
    pub start : Vec2,
    pub end : Vec2,
}

pub fn raycast_to_grid_edge(position: Vec2, direction: Vec2) -> Vec2 {
    let position_x = position.x.floor();
    let position_y = position.y.floor();
    let t_x = if direction.x > 0.0 {
        (position_x + 1.0 - position.x) / direction.x
    } else if direction.x < 0.0 {
        (position_x - position.x) / direction.x
    } else {
        10000.0
    };
    let t_y = if direction.y > 0.0 {
        (position_y + 1.0 - position.y) / direction.y
    } else if direction.y < 0.0 {
        (position_y - position.y) / direction.y
    } else {
        10000.0
    };
    if t_x < t_y {
        //Vec2::new(position_x + (direction.x > 0.0) as i32 as f64, position.y + t_x * direction.y)
        Vec2{
            x: position_x + (direction.x > 0.0) as i32 as f32,
            y: position.y + t_x * direction.y
        }
    } else {
        //Vec2::new(position.x + t_y * direction.x, position_y + (direction.y > 0.0) as i32 as f64)
        Vec2{
            x: position.x + t_y * direction.x,
            y: position_y + (direction.y > 0.0) as i32 as f32
        }
    }
}

// finishes an epsilon outside the unit box border
pub fn step_by_grid(pos : Vec2, move_dir_normalized : Vec2) -> Vec2{
    let position_within_grid = Vec2{
        x: pos.x - pos.x.floor(),
        y: pos.y - pos.y.floor(),
    };
    let grid_pivot = Vec2{
        x: pos.x.floor(),
        y: pos.y.floor(),
    };
    let final_pos_within_grid = raycast_to_grid_edge(position_within_grid, move_dir_normalized);
    let final_pos = grid_pivot + final_pos_within_grid + move_dir_normalized * 0.0001;
    return final_pos;
}


// create some methods

impl TileWorld{
    pub fn new(size_x : usize, size_y : usize) -> TileWorld{
        TileWorld{
            size_x,
            size_y,
            // a boxed array of TileOccupation::Empty
            tile_occupations : vec![TileOccupation::Empty; size_x * size_y].into_boxed_slice(),

        }
    }

    pub fn get_index(&self, pos : Vector2Int) -> usize{
        (pos.y * self.size_x as i32 + pos.x) as usize
    }
    
    pub fn get_tile_pos(&self, pos: Vec2) -> Vector2Int{
        Vector2Int{
            x: pos.x.floor() as i32,
            y: pos.y.floor() as i32,
        }
    }
    
    pub fn get_tile(&self, pos : Vector2Int) -> TileOccupation{
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size_x as i32 || pos.y >= self.size_y as i32{
            return TileOccupation::OutOfBounds;
        }
        self.tile_occupations[self.get_index(pos)]
    }

    pub fn set_tiles(&mut self, pos : Vector2Int, size : Vector2Int, tile : TileOccupation){
        for dy in 0..size.y{
            for dx in 0..size.x{
                self.tile_occupations[self.get_index(Vector2Int{x: pos.x + dx, y: pos.y + dy})] = tile;
            }
        }
    }

    
    pub fn get_first_occupied_tile_in_region(&self, pos : Vector2Int, size : Vector2Int) -> Option<TileOccupation>{
        for y in 0..size.y{
            for x in 0..size.x{
                let tile = self.get_tile(Vector2Int{x, y});
                // if its not empty or our of bounds, return it
                match tile{
                    TileOccupation::Empty | TileOccupation::OutOfBounds=> {},
                    _ => return Some(tile),
                }
            }
        }
        None
    }

    
    // check all the grids that the ray passes through, return the first one that is occupied, return TileWorldRaycastResult::HitNothing if no grid is occupied
    // doesn't check the start grid
    pub fn raycast(&self, params : TileWorldRaycastParams) -> TileWorldRaycastResult{
        let start = params.start;
        let end = params.end;
        let normalized = (end - start).normalize();
        
        
        let end_tile = Vector2Int{
            x: end.x.floor() as i32,
            y: end.y.floor() as i32,
        };

        let mut current_pos = start;
        while self.get_tile_pos(current_pos) != end_tile{
            // move to next tile
            current_pos = step_by_grid(current_pos, normalized);
            // check tile
            let tile_pos = self.get_tile_pos(current_pos);
            let tile = self.get_tile(tile_pos);
            if tile != TileOccupation::Empty{
                return TileWorldRaycastResult::HitOccupiedTile {
                    pos : self.get_tile_pos(current_pos),
                    tile,
                };
            }
        }
        
        return TileWorldRaycastResult::HitNothing;
    }
    

}



/*
get_tile(Vec2Int pos) -> TileOccupation,
try_add_tile_entity(TileEntity entity) -> bool,
remove_tile_entity(TileEntity entity),
*/


/*
Empty: nothing is on the tile
TerrainBlocked: tile is blocked for some reason
EntityBlocked: tile is blocked by an entity it should also contain a reference to the polymorphic entity
*/
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileOccupation{
    Empty,    
    EntityBlocked {entity_id : usize},
    TerrainBlocked,
    OutOfBounds,
}

/*
TileEntity: a polymorphic trait that can be implemented by any entity that can be placed on a tile
*/

pub struct TileEntity{
    pub pos : Vector2Int,
    pub size : Vector2Int,
    pub id : usize,
    pub tile_entity : Box<dyn TileEntityBehaviour>,
}

pub trait TileEntityBehaviour {}

// wall entity, it 1x1 size and it has health






#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug)]
    pub struct Wall{
        pos : Vector2Int,
    }
    
    impl TileEntityBehaviour for Wall{}

    #[derive(Clone, Copy, Debug)]
    pub struct Barracks{}
    
    impl TileEntityBehaviour for Barracks{}
    

    #[test]
    fn test_tileworld() {
        let mut tileworld = TileWorld::new(10, 10);

        // test get_tile
        assert_eq!(tileworld.get_tile(Vector2Int{x : 0, y : 0}), TileOccupation::Empty);

        // test set_tiles
        tileworld.set_tiles(Vector2Int{x : 0, y : 0}, Vector2Int{x : 2, y : 2}, TileOccupation::TerrainBlocked);
        assert_eq!(tileworld.get_tile(Vector2Int{x : 0, y : 0}), TileOccupation::TerrainBlocked);
        assert_eq!(tileworld.get_tile(Vector2Int{x : 1, y : 0}), TileOccupation::TerrainBlocked);
        assert_eq!(tileworld.get_tile(Vector2Int{x : 0, y : 1}), TileOccupation::TerrainBlocked);
        assert_eq!(tileworld.get_tile(Vector2Int{x : 1, y : 1}), TileOccupation::TerrainBlocked);
        assert_eq!(tileworld.get_tile(Vector2Int{x : 2, y : 0}), TileOccupation::Empty);
        assert_eq!(tileworld.get_tile(Vector2Int{x : 0, y : 2}), TileOccupation::Empty);
        assert_eq!(tileworld.get_tile(Vector2Int{x : 2, y : 2}), TileOccupation::Empty);

        // test get_tile_entity_by_id
        let wall = Wall{pos : Vector2Int{x : 4, y : 4}};
        let mut tile_entity = TileEntity{
            pos : Vector2Int{x : 4, y : 4},
            size : Vector2Int{x : 1, y : 1},
            id : 10,
            tile_entity : Box::new(wall),
        };
        tileworld.set_tiles(Vector2Int{x : 4, y : 4}, Vector2Int{x : 1, y : 1}, TileOccupation::EntityBlocked { entity_id: (10) });
        
        // test get_first_tile_entity_id_in_region
        let occupation = tileworld.get_first_occupied_tile_in_region(Vector2Int{x : 4, y : 4}, Vector2Int{x : 1, y : 1});
        match occupation{
            Some(tile_occupation) => assert_eq!(tile_occupation, TileOccupation::EntityBlocked { entity_id: (10) }),
            None => assert!(false),
        }
    }
    
    #[test]
    fn test_tile_world_raycast(){
        let mut tileworld = TileWorld::new(10, 10);

        // test set_tiles
        tileworld.set_tiles(Vector2Int{x : 4, y : 4}, Vector2Int{x : 2, y : 2}, TileOccupation::TerrainBlocked);

        // test raycast
        let raycast_params = TileWorldRaycastParams{
            start : Vec2{x : 0.5, y : 0.5},
            end : Vec2{x : 9.0, y : 9.0},
        };
        let raycast_result = tileworld.raycast(raycast_params);
        match raycast_result{
            TileWorldRaycastResult::HitOccupiedTile{pos, tile} => {
                assert_eq!(pos, Vector2Int{x : 4, y : 4});
                assert_eq!(tile, TileOccupation::TerrainBlocked);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_raycast() {
        let position = Vec2{x: 0.5, y: 0.5};
        let direction = Vec2{x: -1.0, y: -1.1}.normalize();

        let result = raycast_to_grid_edge(position, direction);

        let a = 3;
    }

    #[test]
    fn test_step_by_grid(){
        let direction = Vec2{x: -1.0, y: -1.0}.normalize();

        let step_0 = Vec2{x: 0.25, y: 0.0001};
        let step_0_grid = Vec2{x: step_0.x.floor(), y: step_0.y.floor()};
        println!("step_0_grid: {:?}", step_0_grid);
        
        let step_1 = step_by_grid(step_0, direction);
        let step_1_grid = Vec2{x: step_1.x.floor(), y: step_1.y.floor()};
        println!("step_1_grid: {:?}", step_1_grid);
        
        let step_2 = step_by_grid(step_1, direction);
        let step_2_grid = Vec2{x: step_2.x.floor(), y: step_2.y.floor()};
        println!("step_2_grid: {:?}", step_2_grid);
        
        let step_3 = step_by_grid(step_2, direction);
        let step_3_grid = Vec2{x: step_3.x.floor(), y: step_3.y.floor()};
        println!("step_3_grid: {:?}", step_3_grid);

        let step_4 = step_by_grid(step_3, direction);
        let step_4_grid = Vec2{x: step_4.x.floor(), y: step_4.y.floor()};
        println!("step_4_grid: {:?}", step_4_grid);

        let step_5 = step_by_grid(step_4, direction);
        let step_5_grid = Vec2{x: step_5.x.floor(), y: step_5.y.floor()};
        println!("step_5_grid: {:?}", step_5_grid);

        let step_6 = step_by_grid(step_5, direction);
        let step_6_grid = Vec2{x: step_6.x.floor(), y: step_6.y.floor()};
        println!("step_6_grid: {:?}", step_6_grid);

        let a = 3;
    }
}