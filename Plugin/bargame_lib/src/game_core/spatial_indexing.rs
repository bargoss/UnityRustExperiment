use super::math::FP;
use std::collections::HashMap;
use crate::game_core::math::FP2;


#[derive(Copy, Clone, Debug)]
pub struct GridContent<const GRID_ELEMENT_COUNT: usize> {
    arr: [u32; GRID_ELEMENT_COUNT],
    len: usize,
}
impl<const GRID_ELEMENT_COUNT: usize> GridContent<GRID_ELEMENT_COUNT>
{
    fn new() -> Self {
        Self {
            arr: [0; GRID_ELEMENT_COUNT],
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

#[derive(Copy, Clone, Debug)]
pub struct GridBoundingBox{
    pub start_corner: (i32, i32),
    pub end_corner: (i32, i32)
}

pub struct SpacialPartitioning<const N:usize>{
    grid_size: FP,
    grids: HashMap<(i32, i32), GridContent<N>>,
    object_grid_bounding_boxes: HashMap<u32, GridBoundingBox>,
}
impl<const GRID_ELEMENT_COUNT:usize> SpacialPartitioning<GRID_ELEMENT_COUNT>{
    pub fn new(grid_size: FP) -> SpacialPartitioning<GRID_ELEMENT_COUNT> {
        SpacialPartitioning {
            grid_size,
            grids: HashMap::new(),
            object_grid_bounding_boxes: HashMap::new(),
        }
    }

    pub fn get_object(&self, item: u32) -> Option<&GridBoundingBox> {
        self.object_grid_bounding_boxes.get(&item)
    }

    pub fn add_point(&mut self, item: u32, position: FP2) {
        self.add_box(item, position, position);
    }
    pub fn add_box(&mut self, item: u32, start_corner: FP2, end_corner: FP2) {
        if let Some(_grid_bounding_box) = self.object_grid_bounding_boxes.get(&item) {
            // remove item from old grids
            self.remove_with_id(item);
        }

        let (x0, y0) = self.get_grid(start_corner);
        let (x1, y1) = self.get_grid(end_corner);
        for i in x0..x1 + 1 {
            for j in y0..y1 + 1 {
                let grid = self.grids.entry((i, j)).or_insert(GridContent::new());
                grid.add(item);
            }
        }
    }

    pub fn remove_with_id(&mut self, item: u32) {
        if let Some(grid_bounding_box) = self.object_grid_bounding_boxes.remove(&item) {
            for i in grid_bounding_box.start_corner.0..grid_bounding_box.end_corner.0 + 1 {
                for j in grid_bounding_box.start_corner.1..grid_bounding_box.end_corner.1 + 1 {
                    if let Some(grid) = self.grids.get_mut(&(i, j)) {
                        // remove item from grid
                        for i in 0..grid.len {
                            if grid.arr[i] == item {
                                grid.arr[i] = grid.arr[grid.len - 1];
                                grid.len -= 1;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn add_circle(&mut self, item: u32, center: FP2, radius: FP) {
        // add_box from here
        let half_size = FP2::new(radius, radius);
        let box_start_corner = center - half_size;
        let box_end_corner = center + half_size;

        self.add_box(item, box_start_corner, box_end_corner);
    }

    // return iterator for grid neighbours
    pub fn get_neighbours(&self, position: FP2) -> impl Iterator<Item = u32> {
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

    // collects into buffer
    pub fn overlap_circle(&self, center: FP2, radius: FP, buffer: &mut Vec<u32>) {
        let half_size = FP2::new(radius, radius);
        let box_start_corner = center - half_size;
        let box_end_corner = center + half_size;

        self.overlap_box(box_start_corner, box_end_corner, buffer);
    }

    // get a list from outside, clear it, fill it with results
    pub fn overlap_box(&self, start_corner: FP2, end_corner: FP2, buffer: &mut Vec<u32>) {
        let (x0, y0) = self.get_grid(start_corner);
        let (x1, y1) = self.get_grid(end_corner);
        buffer.clear();
        for i in x0..x1 + 1 {
            for j in y0..y1 + 1 {
                if let Some(grid) = self.grids.get(&(i, j)) {
                    for i in 0..grid.len {
                        if !buffer.contains(&grid.arr[i]) {
                            buffer.push(grid.arr[i]);
                        }
                    }
                }
            }
        }
        buffer.sort(); // for deterministic iterations
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
        self.object_grid_bounding_boxes.clear();
    }

    pub fn get_grid(&self, position: FP2) -> (i32, i32) {
        let _div_x = position.x() / self.grid_size;
        let x = (position.x() / self.grid_size).floor_to_i32();
        let y = (position.y() / self.grid_size).floor_to_i32();

        (x,y)
    }
}


// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    // let mut spacial_partitioning = SpacialPartitioning::<2>::new(FixedPoint::new(10.0));

    // test_add
    #[test]
    fn test_add() {
        let mut spacial_partitioning = SpacialPartitioning::<2>::new(FP::new(10.0));
        spacial_partitioning.add_point(1, FP2::from_num(0.0, 0.0));
        spacial_partitioning.add_point(2, FP2::from_num(0.0, 0.0));

        spacial_partitioning.add_point(2, FP2::from_num(9.9, 0.0));

        assert_eq!(spacial_partitioning.grids.len(), 1);
        assert_eq!(spacial_partitioning.grids.get(&(0, 0)).unwrap().len, 2);

        spacial_partitioning.add_point(3, FP2::from_num(10.0, 0.0));
        assert_eq!(spacial_partitioning.grids.len(), 2);
        assert_eq!(spacial_partitioning.grids.get(&(0, 0)).unwrap().len, 2);
        assert_eq!(spacial_partitioning.grids.get(&(1, 0)).unwrap().len, 1);
    }


    #[test]
    fn test_spacial_partitioning() {
        let mut partitioning: SpacialPartitioning<5> = SpacialPartitioning::new(FP::new(10.0));
        partitioning.add_point(1, FP2::from_num(5.0, 5.0));
        partitioning.add_point(2, FP2::from_num(25.0, 25.0));
        partitioning.add_point(3, FP2::from_num(-5.0, -5.0));
        partitioning.add_point(4, FP2::from_num(-25.0, -25.0));

        let mut buffer = Vec::new();
        partitioning.overlap_box(FP2::from_num(0.0, 0.0), FP2::from_num(11.0, 11.0), &mut buffer);
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer[0], 1);

        partitioning.overlap_box(FP2::from_num(-11.0, -11.0), FP2::from_num(11.0, 11.0), &mut buffer);
        assert_eq!(buffer.len(), 2);

        // sort the buffer
        buffer.sort();
        assert_eq!(buffer[0], 1);
        assert_eq!(buffer[1], 3);
    }

    #[test]
    fn test_add_box(){
            let mut partitioning: SpacialPartitioning<5> = SpacialPartitioning::new(FP::new(10.0));
        partitioning.add_box(1, FP2::from_num(9.0, 23.0), FP2::from_num(21.0, 25.0));


        let mut buffer = Vec::new();
        partitioning.overlap_box(FP2::from_num(0.0, 0.0), FP2::from_num(11.0, 11.0), &mut buffer);
        assert_eq!(buffer.len(), 0);

        partitioning.overlap_box(FP2::from_num(0.0, 0.0), FP2::from_num(21.0, 25.0), &mut buffer);
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer[0], 1);

    }

    // test adding with same key
    #[test]
    fn test_add_same_key_different_position() {
        let mut spacial_partitioning = SpacialPartitioning::<2>::new(FP::new(10.0));
        spacial_partitioning.add_point(1, FP2::from_num(0.0, 0.0));
        spacial_partitioning.add_point(2, FP2::from_num(0.0, 0.0));

        spacial_partitioning.add_point(2, FP2::from_num(9.9, 0.0));

        assert_eq!(spacial_partitioning.grids.len(), 1);
        assert_eq!(spacial_partitioning.grids.get(&(0, 0)).unwrap().len, 2);

        spacial_partitioning.add_point(3, FP2::from_num(10.0, 0.0));
        assert_eq!(spacial_partitioning.grids.len(), 2);
        assert_eq!(spacial_partitioning.grids.get(&(0, 0)).unwrap().len, 2);
        assert_eq!(spacial_partitioning.grids.get(&(1, 0)).unwrap().len, 1);


        let mut my_list = Vec::new();
        my_list.push(1);
        my_list.push(2);

        println!("my_list: {:?}", my_list);

        println!("my_list: {:?}", my_list);
    }
}

