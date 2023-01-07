// a 2d tilemap
// has a type enum that can be dirt, stone, ore, grass
// has height
// has occupying terrain entity id which is a uint



use bevy_ecs::prelude::Component;

#[derive(Clone)]
pub enum TileType{
    Dirt,
    Stone,
    Ore,
    Grass,
}



#[derive(Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub height: u32,
    pub timemap_collider_id: Option<TileMapColliderId>,
}

#[derive(Clone, Debug)]
pub struct TileMapColliderId(pub u32);


pub struct TileMap {
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
}

impl TileMap {
    pub fn new(width: u32, height: u32) -> TileMap {
        let mut tiles = Vec::new();
        for _ in 0..width * height {
            tiles.push(Tile{
                height: 0,
                tile_type: TileType::Dirt,
                timemap_collider_id: None,
            });
        }
        TileMap {
            tiles,
            width,
            height,
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> &Tile {
        &self.tiles[(y * self.width + x) as usize]
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile: Tile) {
        self.tiles[(y * self.width + x) as usize] = tile;
    }
}
