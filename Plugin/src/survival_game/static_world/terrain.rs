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

// polymorphic entity called "Building"
// Wall, Turret, Tree, CraftingStation are all "Buildings"

#[derive(Component, Clone ,Debug)]
pub struct Building {
    pub id: u32,
    pub health: u32,
    pub max_health: u32,
}








pub struct Terrain {
    tiles: Vec<TerrainTile>,
    width: u32,
    height: u32,
}

#[derive(Clone)]
pub struct TerrainTile {
    pub tile_type: TileType,
    pub height: u32,
    pub occupying_entity_id: Option<u32>,
}


impl Terrain {
    pub fn new(width: u32, height: u32) -> Terrain {
        let mut tiles = Vec::new();
        for _ in 0..width * height {
            tiles.push(TerrainTile{
                height: 0,
                tile_type: TileType::Dirt,
                occupying_entity_id: None,
            });
        }
        Terrain {
            tiles,
            width,
            height,
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> &TerrainTile {
        &self.tiles[(y * self.width + x) as usize]
    }    
}
