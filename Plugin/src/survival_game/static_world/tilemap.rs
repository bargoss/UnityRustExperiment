// a 2d tilemap
// has a type enum that can be dirt, stone, ore, grass
// has height
// has occupying terrain entity id which is a uint


use bevy_ecs::prelude::Component;
use quadtree_rs::Quadtree;

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

// create  tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tilemap() {
        use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree};

        // Instantiate a new quadtree which associates String values with u64 coordinates.
        let mut qt = Quadtree::<u64, String>::new(/*depth=*/4);

        // A depth of four means a square with width (and height) 2^4.
        assert_eq!(qt.width(), 16);

        // Associate the value "foo" with a rectangle of size 2x1, anchored at (0, 0).
        let region_a = AreaBuilder::default()
            .anchor(Point {x: 0, y: 0})
            .dimensions((2, 1))
            .build().unwrap();
        qt.insert(region_a, "foo".to_string());

        // Query over a region of size 2x2, anchored at (1, 0).
        let region_b = AreaBuilder::default()
            .anchor(Point {x: 1, y: 0})
            .dimensions((2, 2))
            .build().unwrap();
        let mut query = qt.query(region_b);

        // The query region (region_b) intersects the region "foo" is associated with (region_a), so the query iterator returns "foo" by reference.
        assert_eq!(query.next().unwrap().value_ref(), "foo");
    }
}