use std::default;

use quadtree_rs::{Quadtree, area::AreaBuilder, point::Point};

pub struct TileMap{
    pub quadtree: Quadtree<i32,TileEntity>,
    pub terrain_tiles: Vec<TerrainTile>,
    side_len: usize,
}
impl TileMap{
    pub fn new(depth: usize) -> TileMap {
        let mut qt = Quadtree::<u64, TileEntity>::new(depth);
        let side_len = 2usize.pow(depth as u32);
    
        let mut terrain_tiles = Vec::with_capacity(side_len * side_len);

        TileMap{
            quadtree: qt,
            terrain_tiles,
            side_len,
        }
    }

    pub fn get_terrain(&self, x: i32, y: i32) -> TerrainTile {
        let index = (y * self.side_len as i32 + x) as usize;
        self.terrain_tiles[index]
    }

    pub fn set_terrain(&mut self, x: i32, y: i32, terrain: TerrainTile) {
        let index = (y * self.side_len as i32 + x) as usize;
        self.terrain_tiles[index] = terrain;
    }

    pub fn get_tile_entity(&self, x: i32, y: i32) -> Option<TileEntity> {
        let region = AreaBuilder::default()
            .anchor(Point {x, y})
            .dimensions((1, 1))
            .build().unwrap();
        let mut query = self.quadtree.query(region);
        let a = query.next();
        match a {
            Some(tile) => Some(tile.clone()),
            None => None,
        }
    

    }
}


#[derive(Clone, Copy)]
pub enum TerrainType{
    Dirt,
    Stone,
    Ore,
    Grass,
}
pub struct TileEntity{

}

#[derive(Clone, Copy)]
pub struct TerrainTile {
    terrain_type: TerrainType,
}


// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tilemap() {
        let mut tilemap = TileMap::new(100, 100);
        let tile = TileEntity{};
        tilemap.set_tile(0, 0, tile);
        assert_eq!(tilemap.get_tile(0, 0).height, 0);
    }
}