use super::math::FixedPoint;
use super::math::FixedPointExt;
use std::collections::HashMap;
use crate::game_core::math::FixedPointV2;
use nalgebra;

#[derive(Copy, Clone, Debug)]
struct GridContent<const GridElementCount: usize> {
    arr: [u32; GridElementCount],
    len: usize,
}
impl<const GridElementCount: usize> GridContent<GridElementCount>
{
    fn new() -> Self {
        Self {
            arr: [0; GridElementCount],
            len: 0,
        }
    }

    fn add(&mut self, item: u32) {
        if self.len < self.arr.len() {
            self.arr[self.len] = item;
            self.len += 1;
        }
    }

    fn iter(&self) -> impl Iterator<Item = &u32> {
        self.arr.iter().take(self.len)
    }
}
pub struct SpacialPartitioning<const N:usize>{
    grid_size: FixedPoint,
    grids: HashMap<(i32, i32), GridContent<N>>,
}
impl<const N:usize> SpacialPartitioning<N>{
    pub fn new(grid_size: FixedPoint) -> SpacialPartitioning<N> {
        SpacialPartitioning {
            grid_size,
            grids: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: u32, position: FixedPointV2) {
        // add new elements to grids
        let (x, y) = self.get_grid(position);
        let grid = self.grids.entry((x, y)).or_insert(GridContent::new());
        grid.add(item);
    }

    // return iterator for grid neighbours
    pub fn get_neighbours(&self, position: FixedPointV2) -> impl Iterator<Item = u32> {
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


    pub fn get_all_neighbours(&self, buffer : &mut Vec::<(u32, u32)>) {
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

    pub fn get_grid(&self, position: FixedPointV2) -> (i32, i32) {
        let x = (position.0.x / self.grid_size.0).floor_to_i32();
        let y = (position.0.y / self.grid_size.0).floor_to_i32();

        (x,y)
    }
}