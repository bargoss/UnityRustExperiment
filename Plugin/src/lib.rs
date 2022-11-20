use crate::bubbles::Game;

mod bubbles;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// expose it with extern c
#[no_mangle]
pub extern "C" fn add_extern(a: usize, b: usize) -> usize {
    add(a, b)
}


// return a zero pointer
#[no_mangle]
pub extern "C" fn ptr_test() -> *const usize {
    0 as *const usize
}

// return a pointer to this array:
#[no_mangle]
pub extern "C" fn get_float_array() -> *const f32 {
    // create a vec with 5 elements
    let mut vec = [0.0; 5];
    vec[0] = 1.0;
    vec[1] = 2.0;
    vec[2] = 3.0;
    vec[3] = 4.0;
    vec[4] = 5.0;

    // make the compiler forget about the vec so it doesnt drop it
    let ptr = vec.as_ptr();
    std::mem::forget(vec);
    ptr
}

// get_float_array c# code that returns a pointer to the array:
//[DllImport("bubbles.dll", CallingConvention = CallingConvention.Cdecl)]
//public static extern IntPtr get_float_array();






// takes array id and index
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

// c# side:
// [DllImport("bevy_rust_test.dll", CallingConvention = CallingConvention.Cdecl)]
// public static extern IntPtr get_vec3_array();
//
// public static Vector3[] GetVec3Array()
// {
//     IntPtr ptr = get_vec3_array();
//     Vector3[] vec3Array = new Vector3[5];
//     Marshal.Copy(ptr, vec3Array, 0, 5);
//     return vec3Array;
// }

// pub struct Game{
//     pub world: bevy_ecs::prelude::World,
//     pub start_schedule: Schedule,
//     pub update_schedule: Schedule,
// }

// extern function that creates a Game
#[no_mangle]
pub extern "C" fn create_game() -> *mut Game {
    //let game = Box::new(Game::new());
    //Box::into_raw(game)

    // do it but dont drop the "Game"
    let game = Box::new(Game::new());
    std::mem::forget(&game);
    Box::into_raw(game)
}

#[no_mangle]
pub extern "C" fn start_game(game: *mut Game) {
    let game = unsafe { &mut *game };
    game.start();
}

#[no_mangle]
pub extern "C" fn update_game(game: *mut Game) {
    let game = unsafe { &mut *game };
    game.update();
}

//return game.positions_floats pointer
#[no_mangle]
pub extern "C" fn get_bubble_positions(game: *mut Game) -> *const f32 {
    let game = unsafe { &mut *game };
    let resource = game.world.get_resource::<Vec<f32>>().unwrap();

    // tell the copiler not to drop this array
    //std::mem::forget(resource);

    resource.as_ptr()
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
    fn interop_tests() {
        let a = 3;
        let game = create_game();
        {
            start_game(game);
        }
        {
            update_game(game);
        }

        // update game 100 times and measure time

        let mut total_duration = 0;
        let iterations = 1;
        for _ in 0..iterations {
            let elapsed = time_it(|| {
                update_game(game);
            });
            total_duration += elapsed;
        }
        let average_duration = total_duration / iterations;
        println!("average duration: {}", average_duration);


        let positions = get_bubble_positions(game);
        let positions = unsafe { std::slice::from_raw_parts(positions, 10) };
        let pos0 = positions[0];
        let pos1 = positions[1];
        let pos2 = positions[2];

        let pos3 = positions[3];
        let pos4 = positions[4];
        let pos5 = positions[5];

        let pos6 = positions[6];
        let pos7 = positions[7];
        let pos8 = positions[8];

        assert_eq!(pos0.abs() > 0.001, true);
        assert_eq!(pos1.abs() > 0.001, true);
        assert_eq!(pos2.abs() == 0.0, true);

        assert_eq!(pos3.abs() > 0.001, true);
        assert_eq!(pos4.abs() > 0.001, true);
        assert_eq!(pos5.abs() == 0.0, true);

        assert_eq!(pos6.abs() > 0.001, true);
        assert_eq!(pos7.abs() > 0.001, true);
        assert_eq!(pos8.abs() == 0.0, true);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn real_test() {
        let mut game = bubbles::Game::new();
        game.start();
        game.update();
        let mut iter = game.get_positions_iter();

        // tolist
        let mut list = Vec::new();
        for i in iter {
            list.push(i.clone());
        }

        game.update();
        let mut iter = game.get_positions_iter();
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

        let sdad = 3;
    }

}
