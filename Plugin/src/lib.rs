use bevy_math::Vec3;
use crate::bubbles::{BubblePushPoints, Game, PositionFloatBuffer, WorldParams};
mod bubbles;
use interoptopus::{ffi_type, function, Interop, Inventory, InventoryBuilder};
use interoptopus::util::NamespaceMappings;
use interoptopus_backend_csharp::{Config, Generator, Unsafe};
use interoptopus_backend_csharp::overloads::Unity;
use interoptopus::ffi_function;


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

#[ffi_type]
#[repr(C)]
pub struct GameExt {
    ptr: *const u8
}


#[ffi_function]
#[no_mangle]
pub extern "C" fn create_game(bubble_count : i32) -> GameExt {
    let game = Box::new(Game::new(WorldParams{bubble_count: bubble_count as usize}));
    let ptr = Box::into_raw(game);
    GameExt{ptr: ptr as *const u8}
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn update_game(game: GameExt) {
    let game = unsafe { &mut *(game.ptr as *mut Game) };
    game.update();
}

// give f32 array to c#
#[ffi_function]
#[no_mangle]
pub extern "C" fn get_bubble_positions(game: GameExt) -> *const f32 {
    let game = unsafe { &mut *(game.ptr as *mut Game) };
    let resource = game.world.get_resource::<PositionFloatBuffer>().unwrap();
    resource.value.as_ptr()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn apply_bubble_push(game: GameExt, x: f32, y: f32, z:f32) {
    let game = unsafe { &mut *(game.ptr as *mut Game) };
    let mut resource = game.world.get_resource_mut::<BubblePushPoints>().unwrap();
    resource.points.push(Vec3::new(x, y, z));
}

// feed in a float array
#[ffi_function]
#[no_mangle]
pub extern "C" fn set_push_position(game: GameExt, x: f32, y: f32, z: f32) {
    let game = unsafe { &mut *(game.ptr as *mut Game) };

    let mut push_points = Vec::new();
    push_points.push(Vec3::new(x, y, z));

    game.set_push_points(push_points);
}




#[cfg(test)]
mod tests {
    use std::fs;
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
    fn generate_bindings(){

        let my_inventory = InventoryBuilder::new()
            .register(function!(create_game))
            .register(function!(update_game))
            .register(function!(get_bubble_positions))
            .register(function!(apply_bubble_push))
            .register(function!(set_push_position))
            .inventory();
        let config = Config {
            dll_name: "mandelbrot".to_string(),
            namespace_mappings: NamespaceMappings::new("Bubbles"),
            use_unsafe: Unsafe::UnsafeKeyword,
            ..Config::default()
        };
        Generator::new(config, my_inventory)
            .add_overload_writer(Unity::new())
            //.add_overload_writer(DotNet::new())
            .write_file("Wrapper.cs");

        // move the build artifacts to unity plugins folder
        // path to here
        let path = std::env::current_dir().unwrap();
        let path = path.to_str().unwrap();
        // print path

        let path_to_built_dll = format!("{}\\target\\x86_64-pc-windows-msvc\\release\\mandelbrot.dll", path);
        println!("path_to_built_dll: {}", path_to_built_dll);
        let path_to_unity_plugin_folder = format!("{}\\..\\Assets\\Plugins\\x86_64", path);
        println!("path_to_unity_plugin_folder: {}", path_to_unity_plugin_folder);
        let path_to_wrapper = format!("{}\\Wrapper.cs", path);

        //delete everything in the unity plugin folder with try catch for each element
        let paths = fs::read_dir(path_to_unity_plugin_folder.clone()).unwrap();
        paths.for_each(|path| {
            let path = path.unwrap().path();
            let path = path.to_str().unwrap();
            println!("deleting: {}", path);
            fs::remove_file(path).unwrap_or(());
        });

        // copy the dll to the unity plugin folder
        fs::copy(path_to_built_dll, format!("{}\\mandelbrot.dll", path_to_unity_plugin_folder)).unwrap();
        // copy the wrapper to the unity plugin folder
        fs::copy(path_to_wrapper, format!("{}\\Wrapper.cs", path_to_unity_plugin_folder)).unwrap();












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
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
