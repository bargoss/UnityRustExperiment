use bevy_ecs::prelude::World;
use bevy_math::Vec3;
use crate::bubbles::{BubblePushPoints, Game, PositionFloatBuffer, WorldParams};

mod bubbles;

//native array struct for interop with size and array
#[repr(C)]
pub struct NativeArrayFloat{
    pub size: usize,
    pub value: [f32; 1500]
}




pub static mut OUTPUT: [i32; 3000] = [0; 3000];

#[no_mangle]
pub extern "C" fn get_int_array_ptr() -> *const i32 {
    unsafe{
        OUTPUT[0] = 1;
        OUTPUT[1] = 2;
        OUTPUT[2] = 3;
        OUTPUT[3] = 4;
        OUTPUT[4] = 5;
        OUTPUT[5] = 6;
        OUTPUT[6] = 7;
        OUTPUT[7] = 8;
        OUTPUT[8] = 9;
        OUTPUT[9] = 10;
        OUTPUT[10] = 11;
        OUTPUT[11] = 12;
        OUTPUT[12] = 13;
        OUTPUT[13] = 14;
        OUTPUT[14] = 15;
        OUTPUT[15] = 16;
        OUTPUT[16] = 17;
        OUTPUT[17] = 18;
        OUTPUT[18] = 19;
        OUTPUT[19] = 20;
        OUTPUT[20] = 21;
        OUTPUT[21] = 22;
        OUTPUT[22] = 23;
        OUTPUT[23] = 24;
        OUTPUT[24] = 25;
        OUTPUT[25] = 26;
        OUTPUT[26] = 27;
        OUTPUT[27] = 28;
        OUTPUT[28] = 29;
        OUTPUT[29] = 30;
        OUTPUT[30] = 31;
        OUTPUT[31] = 32;
        OUTPUT[32] = 33;
        OUTPUT[33] = 34;
        OUTPUT[34] = 35;
        OUTPUT[35] = 36;
        OUTPUT[36] = 37;
        OUTPUT[37] = 38;
        OUTPUT[38] = 39;
        OUTPUT[39] = 40;
        OUTPUT[40] = 41;
        OUTPUT[41] = 42;
        OUTPUT[42] = 43;
        OUTPUT[43] = 44;
        OUTPUT[44] = 45;
        OUTPUT.as_ptr()
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// expose it with extern c
#[no_mangle]
pub extern "C" fn add_extern(a: usize, b: usize) -> usize {
    add(a, b)
}





#[no_mangle]
pub extern "C" fn get_float_array_value(array_id:i32, index:i32) -> f32 {
    let mut float_array = [0.0; 5];
    float_array[0] = 1.1;
    float_array[1] = 2.1;
    float_array[2] = 3.1;
    float_array[3] = 4.1;
    float_array[4] = 5.1;
    float_array[index as usize]
}



#[no_mangle]
pub extern "C" fn get_int_array_value(array_id:i32, index:i32) -> i32 {
    let mut int_array = [0; 5];
    int_array[0] = 1;
    int_array[1] = 2;
    int_array[2] = 3;
    int_array[3] = 4;
    int_array[4] = 5;
    int_array[index as usize]
}

#[no_mangle]
pub extern "C" fn create_game2(bubble_count : usize) {
    //create a vec<float> with 10 elements that go like 0 1 2 3 4 5 6 7 8 9
    let mut vec = Vec::new();
    for i in 0..100000 {
        vec.push(i as i32);
    }
}


#[no_mangle]
pub extern "C" fn create_game(bubble_count : usize) {
    let game = Box::new(Game::new(WorldParams{bubble_count}));

    //std::mem::forget(&game);
    //Box::into_raw(game)
}

#[no_mangle]
pub extern "C" fn update_game(game: *mut Game) {
    let game = unsafe { &mut *game };
    game.update();
}

// give f32 array to c#
#[no_mangle]
pub extern "C" fn get_bubble_positions(game: *mut Game) -> *const f32 {
    let game = unsafe { &mut *game };
    let resource = game.world.get_resource::<PositionFloatBuffer>().unwrap();

    resource.value.as_ptr()
}

#[no_mangle]
pub extern "C" fn apply_bubble_push(game: *mut Game, x: f32, y: f32, z:f32) {
    let game = unsafe { &mut *game };
    let mut resource = game.world.get_resource_mut::<BubblePushPoints>().unwrap();
    resource.points.push(Vec3::new(x, y, z));
}

// feed in a float array
#[no_mangle]
pub extern "C" fn set_push_position(game: *mut Game, x: f32, y: f32, z: f32) {
    let game = unsafe { &mut *game };

    let mut push_points = Vec::new();
    push_points.push(Vec3::new(x, y, z));

    game.set_push_points(push_points);
}






#[cfg(test)]
mod tests {
    use super::*;

    //execute action, measure time
    fn time_it<F>(action: F) -> u128
    where
        F: FnOnce(),
    {
        let start = std::time::Instant::now();
        action();
        start.elapsed().as_millis()
    }

    #[test]
    fn get_int_array_ptr_test() {
        let arr_ptr = get_int_array_ptr();
        // print the int value of this
        println!("arr_ptr: {}", arr_ptr as usize);

    }

    // ignored test
    #[test]
    fn interop_tests() {
        //let game = create_game(20);
        //{
        //    update_game(game);
        //}
//
        //// update game 100 times and measure time
//
        //let mut total_duration = 0;
        //let iterations = 10;
        //for _ in 0..iterations {
        //    let elapsed = time_it(|| {
        //        update_game(game);
        //    });
        //    total_duration += elapsed;
        //}
        //let average_duration = total_duration / iterations;
        //println!("average duration: {}", average_duration);
//
//
        //let positions = get_bubble_positions(game);
        //let raw_address_value = positions as usize;
        //let positions = unsafe { std::slice::from_raw_parts(positions, 10) };
        //let pos0 = positions[0];
        //let pos1 = positions[1];
        //let pos2 = positions[2];
//
        //let pos3 = positions[3];
        //let pos4 = positions[4];
        //let pos5 = positions[5];
//
        //let pos6 = positions[6];
        //let pos7 = positions[7];
        //let pos8 = positions[8];
//
        //assert_eq!(pos0.abs() > 0.001, true);
        //assert_eq!(pos1.abs() > 0.001, true);
        //assert_eq!(pos2.abs() == 0.0, true);
//
        //assert_eq!(pos3.abs() > 0.001, true);
        //assert_eq!(pos4.abs() > 0.001, true);
        //assert_eq!(pos5.abs() == 0.0, true);
//
        //assert_eq!(pos6.abs() > 0.001, true);
        //assert_eq!(pos7.abs() > 0.001, true);
        //assert_eq!(pos8.abs() == 0.0, true);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    /*
    #[test]
    fn real_test() {
        let mut game = bubbles::Game::new(WorldParams{bubble_count: 500});
        game.update();
        let iter = game.get_positions_iter();

        // tolist
        let mut list = Vec::new();
        for i in iter {
            list.push(i.clone());
        }

        game.update();
        let iter = game.get_positions_iter();
        let mut list2 = Vec::new();
        for i in iter {
            list2.push(i.clone());
        }

        let mut distance_deltas = Vec::new();
        for i in 0..list.len() {
            // abs of list[i]-list2[i]
            let delta_abs = (list[i] - list2[i]).abs();
            distance_deltas.push(delta_abs);
        }

        let average_distance = distance_deltas.iter().sum::<f32>() / distance_deltas.len() as f32;

        // assert average_distance > 0.001
        assert!(average_distance > 0.001);
    }

     */

}
