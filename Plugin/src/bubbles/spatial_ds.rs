// GridSize: f32
// Dictionary<(x:i32 , y:i32), List<(T,Vec3)>>
// list = dict.Get((0,0));
// list.Add((T, Vec3))

use std::collections::HashMap;
use bevy_math::Vec3;

#[derive(Default, Copy, Clone, Debug)]
struct GridContent<T> {
    arr: [T; 20],
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

    fn add(&mut self, item: T) {
        if self.len < self.arr.len() {
            self.arr[self.len] = item;
            self.len += 1;
        }
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.arr.iter().take(self.len)
    }
}

#[derive(Debug)]
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
        grid.add(item);
    }

    // return iterator for grid neighbours
    pub fn get_neighbours(&self, position: Vec3) -> impl Iterator<Item = &T> {
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


    pub fn get_all_neighbours(&self) -> impl Iterator<Item = (T, T)> {
        let neighbour_deltas = [(1,0), (1,1), (0,1), (-1,1)];
        let mut result = Vec::<(T, T)>::new();
        for my_grid_key in self.grids.keys() {
            // my grid:
            let my_grid = self.grids.get(my_grid_key).unwrap();
            for i in 0..my_grid.len {
                //for j in 0..i{
                for j in 0..i {
                    result.push((my_grid.arr[i], my_grid.arr[j]));
                }
            }

            for(i,j) in neighbour_deltas{
                let neighbor_grid_key = &(my_grid_key.0 + i, my_grid_key.1 + j);
                if let Some(neighbour_grid) = self.grids.get(neighbor_grid_key) {
                    for my_element in self.grids.get(&my_grid_key).unwrap().iter() {
                        for neighbour_element in neighbour_grid.iter() {
                            result.push((my_element.clone(), neighbour_element.clone()));
                        }
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
    fn print_all_neighborsfsdfds(){
        let grids = create_test_structure();

        println!("dasdas start");
        for (a, b) in grids.get_all_neighbours() {
            println!("{:?} {:?} dasdas", a, b);
        }
        println!("dasdas end");
    }

    #[test]
    fn test() {
        let grids = create_test_structure();

        let mut count = 0;
        let mut neighbours = grids.get_neighbours(Vec3::new(0.0, 0.0, 0.0));

        let a = -0.0;
        for neighbor in grids.get_neighbours(Vec3::new(a, a, 0.0)) {
            count += 1;
            println!("{:?}", neighbor);
        }

        println!("count: {}", count);

        let a = 3;




    }

    fn create_test_structure() -> LookUpGrids<i32> {
        let mut grids = LookUpGrids::new(2.0);
        let a = 2.0;
        for i in 0..20 {
            for j in 0..20 {
                grids.add((i + 20 * j), Vec3::new((i as f32) * a, (j as f32) * a, 0.0));
            }
        }
        grids
    }

    #[test]
    fn test_simple() {
        let a = 3;
        let b = 3;
        let c = a+b;
        assert_eq!(c, 6);
    }
}


