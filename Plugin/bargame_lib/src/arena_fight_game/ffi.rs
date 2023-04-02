use interoptopus::*;


#[ffi_type]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GameExt {
    pub ptr: *const u8
}

//#[ffi_function]
//#[no_mangle]
//pub extern "C" fn create_game() -> GameExt {
//    let game = Box::new(Game::new(WorldParams{}));
//    let ptr = Box::into_raw(game);
//    GameExt{ptr: ptr as *const u8}
//}

#[ffi_type]
#[repr(C)]
pub struct NativeArrayFloat{
    pub size: u32,
    //pub value: [f32; 16]
    pub value: [f32; 16]
}

#[ffi_type]
#[repr(C)]
pub struct NativeListFloat{
    pub size: u32,
    pub data: *const f32,
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn add_extern(left: i32, right: i32) -> i32 {
    left + right
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn get_example_array() -> NativeArrayFloat {
    let mut array = NativeArrayFloat{
        size: 1500,
        value: [0.0; 16]
    };
    for i in 0..16 {
        array.value[i] = i as f32;
    }
    array
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn get_example_list() -> NativeListFloat {
    let mut array = NativeListFloat{
        size: 10,
        data: std::ptr::null()
    };
    array
}

#[ffi_type]
#[repr(C)]
pub struct NativeArray<T: 'static> {
    pub size: u32,
    pub data: *const T,
}


use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::os::raw::c_void;

#[ffi_function]
#[no_mangle]
pub unsafe extern "C" fn allocate_native_array(size: u32, elem_size: u32) -> *mut c_void {
    let total_size = (size * elem_size) as usize;
    let array_layout = Layout::from_size_align(total_size, std::mem::align_of::<u8>()).unwrap();
    alloc_zeroed(array_layout) as *mut c_void
}

#[ffi_function]
#[no_mangle]
pub unsafe extern "C" fn deallocate_native_array(data: *mut c_void, size: u32, elem_size: u32) {
    let total_size = (size * elem_size) as usize;
    let array_layout = Layout::from_size_align(total_size, std::mem::align_of::<u8>()).unwrap();
    dealloc(data as *mut u8, array_layout);
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;
    use interoptopus::{function, Interop, InventoryBuilder};
    use interoptopus::util::NamespaceMappings;
    use interoptopus_backend_csharp::{Config, Generator, Unsafe};
    use interoptopus_backend_csharp::overloads::Unity;

    #[test]
    fn generate_bindings(){
        let postfix = rand::random::<u32>();
        let dll_name = format!("game_{}", postfix);

        let my_inventory = InventoryBuilder::new()
            .register(function!(add_extern))
            .register(function!(get_example_array))
            .register(function!(get_example_list))
            .register(function!(allocate_native_array))
            .register(function!(deallocate_native_array))
            .inventory();
        let config = Config {
            // add postfix
            dll_name: dll_name.to_string(),
            namespace_mappings: NamespaceMappings::new("Bubbles"),
            use_unsafe: Unsafe::UnsafeKeyword,
            ..Config::default()
        };
        Generator::new(config, my_inventory)
            .add_overload_writer(Unity::new())
            //.add_overload_writer(DotNet::new())
            .write_file("Wrapper.cs");

        /*
        change the

        public const string DllName = "game";

        to:

        #if UNITY_EDITOR
        public const string DllName = "game_1234";
        #else
        public const string DllName = "mandelbrot";
        #endif
         */

        let mut file = fs::File::open("Wrapper.cs").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let stringToReplace = format!("public const string NativeLib = \"{}\";", dll_name);
        let stringToReplaceWith = format!("#if UNITY_EDITOR\npublic const string NativeLib = \"{}\";\n#else\npublic const string NativeLib = \"{}\";\n#endif", dll_name, "game");
        let contents = contents.replace(&stringToReplace, &stringToReplaceWith);
        fs::write("Wrapper.cs", contents).unwrap();



        // move the build artifacts to unity plugins folder
        // path to here
        let path = std::env::current_dir().unwrap();
        let path = path.to_str().unwrap();
        // print path

        let path_to_built_dll = format!("{}\\target\\debug\\bargame_lib.dll", path);
        println!("path_to_built_dll: {}", path_to_built_dll);
        let path_to_unity_plugin_folder = format!("{}\\..\\..\\Assets\\Plugins\\x86_64", path);
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

        // copy the dll to the unity plugin folder, add postfix to dll name
        fs::copy(path_to_built_dll, format!("{}\\{}.dll", path_to_unity_plugin_folder, dll_name)).unwrap();

        // copy the wrapper to the unity plugin folder
        fs::copy(path_to_wrapper, format!("{}\\Wrapper.cs", path_to_unity_plugin_folder)).unwrap();








    }
}

pub struct SphereSnapshot{
    pub center: [f32; 3],
    pub radius: f32,
    pub color: [f32; 3],
}
pub struct LineSnapshot{
    pub start: [f32; 3],
    pub end: [f32; 3],
    pub color: [f32; 3],
    pub thickness: f32,
}
pub struct CharacterSnapshot{
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub health: f32,
    pub velocity: [f32; 3],
    pub is_on_ground: bool,
    pub shooting: bool,
}
pub struct GameSnapshot{
    pub spheres: Vec<SphereSnapshot>,
    pub lines: Vec<LineSnapshot>,
    pub characters: Vec<CharacterSnapshot>,
}