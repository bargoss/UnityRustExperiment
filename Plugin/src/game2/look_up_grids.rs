use std::collections::HashMap;
use crate::game2::data_types::FFloat;
use super::data_types::Vec2FFloat;

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
    grid_size: FFloat,
    grids: HashMap<(i32, i32), GridContent<T>>,
}

impl<T> LookUpGrids<T> where T: Copy + Default{
    pub fn new(grid_size: FFloat) -> LookUpGrids<T> {
        LookUpGrids {
            grid_size,
            grids: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: T, position: Vec2FFloat) {
        // add new elements to grids
        let (x, y) = self.get_grid(position);
        let grid = self.grids.entry((x, y)).or_insert(GridContent::new());
        grid.add(item);
    }

    // return iterator for grid neighbours
    pub fn get_neighbours(&self, position: Vec2FFloat) -> impl Iterator<Item = T> {
        let (x, y) = self.get_grid(position);
        let mut result = Vec::new();
        for i in -1..2 {
            for j in -1..2 {
                if let Some(grid) = self.grids.get(&(x + i, y + j)) {
                    for i in 0..grid.len {
                        result.push(grid.arr[i]);
                    }
                }
            }
        }
        result.into_iter()
    }


    pub fn get_all_neighbours(&self, buffer : &mut Vec::<(T, T)>) {
        let neighbour_deltas = [(1,0), (1,1), (0,1), (-1,1)];
        buffer.clear();
        for my_grid_key in self.grids.keys() {
            // my grid:
            let my_grid = self.grids.get(my_grid_key).unwrap();
            for i in 0..my_grid.len {
                //for j in 0..i{
                for j in 0..i {
                    buffer.push((my_grid.arr[i], my_grid.arr[j]));
                }
            }

            for(i,j) in neighbour_deltas{
                let neighbor_grid_key = &(my_grid_key.0 + i, my_grid_key.1 + j);
                if let Some(neighbour_grid) = self.grids.get(neighbor_grid_key) {
                    for my_element in self.grids.get(&my_grid_key).unwrap().iter() {
                        for neighbour_element in neighbour_grid.iter() {
                            buffer.push((my_element.clone(), neighbour_element.clone()));
                        }
                    }
                }
            }
        }
    }


    pub fn clear(&mut self) {
        self.grids.clear();
    }

    pub fn get_grid(&self, position: Vec2FFloat) -> (i32, i32) {
        let x = i32::from((position.x / self.grid_size).floor());
        let y = i32::from((position.y / self.grid_size).floor());
        (x, y)
    }
}


// tests

#[cfg(test)]
mod spatial_tests {
    use fixed::types::I24F8;
    use super::*;

    // ignore
    #[ignore]
    #[test]
    fn print_all_neighborsfsdfds(){
        let grids = create_test_structure();
        let mut buffer = Vec::new();
        grids.get_all_neighbours(&mut buffer);
        println!("dasdas start");
        for (a, b) in buffer.iter() {
            println!("{:?} {:?} dasdas", a, b);
        }
        println!("dasdas end");
    }

    #[test]
    fn test() {
        let grids = create_test_structure();

        let mut count = 0;
        let mut neighbours = grids.get_neighbours(Vec2FFloat::new(0.0, 0.0));

        let a = -0.0;
        for neighbor in grids.get_neighbours(Vec2FFloat::new(a, a)) {
            count += 1;
            println!("{:?}", neighbor);
        }

        println!("count: {}", count);

        let a = 3;




    }

    #[test]
    fn test2() {
        let mut grids = LookUpGrids::new(FFloat::new(2.0));
        grids.add(4 as i32, Vec2FFloat::new(0.5, 0.5));

        // assert 0 grids.get_neighbours(Vec2FFloat::new(0.0, 0.0)).count();
        assert_eq!(grids.get_neighbours(Vec2FFloat::new(4.5, 4.5)).count(), 0);

        grids.add(5 as i32, Vec2FFloat::new(4.5, 4.5));
        //iterate and print all neighbours

        let neighbors= grids.get_neighbours(Vec2FFloat::new(4.5, 4.5)).collect::<Vec<_>>();
        // assert count is 1
        assert_eq!(neighbors.len(), 1);

        // assert 5 is in the list
        assert_eq!(neighbors[0], 5);

        // add one more close to 5
        grids.add(6 as i32, Vec2FFloat::new(4.5, 4.7));

        // assert count is 2
        assert_eq!(grids.get_neighbours(Vec2FFloat::new(4.5, 4.5)).count(), 2);

        // assert 5 and 6 are somewhere in the list
        let neighbors= grids.get_neighbours(Vec2FFloat::new(4.5, 4.5)).collect::<Vec<_>>();
        assert!(neighbors.contains(&5));
        assert!(neighbors.contains(&6));

        let mut neighbour_pairs = Vec::<(i32, i32)>::new();
        let all_neighbor_pairs = grids.get_all_neighbours(&mut neighbour_pairs);

        // loop and print
        for (a, b) in neighbour_pairs.iter() {
            println!("neighbour pair: {:?} {:?}", a, b);
        }

        // assert count is 1
        assert_eq!(neighbour_pairs.len(), 1);

        // assert first pair is either 5,6 or 6,5
        assert!(neighbour_pairs[0] == (5, 6) || neighbour_pairs[0] == (6, 5));





    }

    fn create_test_structure() -> LookUpGrids<i32> {
        let mut grids = LookUpGrids::new(FFloat::new(2.0));
        let a = 2.0;
        for i in 0..20 {
            for j in 0..20 {
                grids.add((i + 20 * j), Vec2FFloat::new((i as f32) * a, (j as f32) * a));
            }
        }
        grids
    }

}

