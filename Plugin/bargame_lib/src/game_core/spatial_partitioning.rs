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
impl<const GridMaxElementCount:usize> SpacialPartitioning<GridMaxElementCount>{
    pub fn new(grid_size: FixedPoint) -> SpacialPartitioning<GridMaxElementCount> {
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

    // get a list from outside, clear it, fill it with results
    pub fn overlap_box(&self, start_corner: FixedPointV2, end_corner: FixedPointV2, buffer: &mut Vec<u32>) {
        let (x0, y0) = self.get_grid(start_corner);
        let (x1, y1) = self.get_grid(end_corner);
        buffer.clear();
        for i in x0..x1 + 1 {
            for j in y0..y1 + 1 {
                if let Some(grid) = self.grids.get(&(i, j)) {
                    for i in 0..grid.len {
                        buffer.push(grid.arr[i]);
                    }
                }
            }
        }
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


// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector2;

    // let mut spacial_partitioning = SpacialPartitioning::<2>::new(FixedPoint::new(10.0));

    // test_add
    #[test]
    fn test_add() {
        let mut spacial_partitioning = SpacialPartitioning::<2>::new(FixedPoint::new(10.0));
        spacial_partitioning.add(1, FixedPointV2::new(0.0, 0.0));
        spacial_partitioning.add(2, FixedPointV2::new(0.0, 0.0));

        spacial_partitioning.add(2, FixedPointV2::new(9.9, 0.0));

        assert_eq!(spacial_partitioning.grids.len(), 1);
        assert_eq!(spacial_partitioning.grids.get(&(0, 0)).unwrap().len, 2);

        spacial_partitioning.add(3, FixedPointV2::new(10.0, 0.0));
        assert_eq!(spacial_partitioning.grids.len(), 2);
        assert_eq!(spacial_partitioning.grids.get(&(0, 0)).unwrap().len, 2);
        assert_eq!(spacial_partitioning.grids.get(&(1, 0)).unwrap().len, 1);
    }


    #[test]
    fn test_spacial_partitioning() {
        let mut partitioning: SpacialPartitioning<5> = SpacialPartitioning::new(FixedPoint::new(10.0));
        partitioning.add(1, FixedPointV2::new(5.0, 5.0));
        partitioning.add(2, FixedPointV2::new(25.0, 25.0));
        partitioning.add(3, FixedPointV2::new(-5.0, -5.0));
        partitioning.add(4, FixedPointV2::new(-25.0, -25.0));

        let mut buffer = Vec::new();
        partitioning.overlap_box(FixedPointV2::new(0.0, 0.0), FixedPointV2::new(11.0, 11.0), &mut buffer);
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer[0], 1);

        partitioning.overlap_box(FixedPointV2::new(-11.0, -11.0), FixedPointV2::new(11.0, 11.0), &mut buffer);
        assert_eq!(buffer.len(), 2);

        // sort the buffer
        buffer.sort();
        assert_eq!(buffer[0], 1);
        assert_eq!(buffer[1], 3);
    }
}

