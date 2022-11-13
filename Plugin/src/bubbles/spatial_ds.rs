// GridSize: f32
// Dictionary<(x:i32 , y:i32), List<(T,Vec3)>>
// list = dict.Get((0,0));
// list.Add((T, Vec3))

use std::collections::HashMap;
use bevy_math::Vec3;

#[derive(Default, Copy, Clone)]
struct GridContent<T> {
    arr: [(T, Vec3); 10],
    len: usize,
}

// T implements Copy and Default
impl<T> GridContent<T> where T: Copy + Default,
{
    fn new() -> Self {
        Self {
            arr: Default::default(),
            len: 0,
        }
    }

    fn add(&mut self, item: T, pos: Vec3) {
        if self.len < self.arr.len() {
            self.arr[self.len] = (item, pos);
            self.len += 1;
        }
    }

    fn iter(&self) -> impl Iterator<Item = &(T, Vec3)> {
        self.arr.iter().take(self.len)
    }
}

pub struct LookUpGrids<T>{
    grid_size: f32,
    grids: HashMap<(i32, i32), GridContent<T>>,
}

impl<T> LookUpGrids<T> where T: Copy + Default{
    pub fn new(grid_size: f32) -> LookUpGrids<T> {
        LookUpGrids {
            grid_size,
            grids: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: T, position: Vec3) {
        // add new elements to grids
        let (x, y) = self.get_grid(position);
        let grid = self.grids.entry((x, y)).or_insert(GridContent::new());
    }

    // return iterator for grid neighbours
    pub fn get_neighbours(&self, position: Vec3) -> impl Iterator<Item = &(T,Vec3)> {
        let (x, y) = self.get_grid(position);
        let mut result = Vec::new();
        for i in -1..2 {
            for j in -1..2 {
                if let Some(grid) = self.grids.get(&(x + i, y + j)) {
                    for element in grid.arr.iter() {
                        result.push(element);
                    }
                }
            }
        }
        result.into_iter()
    }


    pub fn clear(&mut self) {
        self.grids.clear();
    }

    fn get_grid(&self, position: Vec3) -> (i32, i32) {
        let x = (position.x / self.grid_size).floor() as i32;
        let y = (position.y / self.grid_size).floor() as i32;
        (x, y)
    }
}


// tests

#[cfg(test)]
mod spatial_tests {
    use super::*;

    #[test]
    fn test() {

        let mut grids = LookUpGrids::new(2.0);
        for i in 0..10 {
            for j in 0..10 {
                grids.add((i + 10*j), Vec3::new((i as f32) * 0.5, (j as f32) * 0.5, 0.0));
            }
        }

        let mut count = 0;
        let mut neighbours = grids.get_neighbours(Vec3::new(0.0, 0.0, 0.0));

        let a = -0.0;
        for (_, neighbour_pos) in grids.get_neighbours(Vec3::new(a, a, 0.0)) {
            count += 1;
            println!("{:?}", neighbour_pos);
        }

        println!("count: {}", count);

        let a = 3;




    }

    #[test]
    fn test_simple() {
        let a = 3;
        let b = 3;
        let c = a+b;
        assert_eq!(c, 6);
    }
}


