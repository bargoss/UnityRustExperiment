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


// Define a callback function type
pub type MyCallback = extern "C" fn(i32);

// Function that accepts the callback function as a parameter
#[ffi_function]
pub extern "C" fn rust_function_with_callback(callback: MyCallback) {
    let result = 42; // Some result value
    callback(result); // Call the callback function
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;
    use interoptopus::{function, Interop, InventoryBuilder, callback};
    use interoptopus::util::NamespaceMappings;
    use interoptopus_backend_csharp::{Config, Generator, Unsafe};
    use interoptopus_backend_csharp::overloads::Unity;

    use interoptopus::Symbol;

    fn generate_bindings_with_functions(functions: Vec<Symbol>) {
        let postfix = rand::random::<u32>();
        let dll_name = format!("game_{}", postfix);

        let mut inventory_builder = InventoryBuilder::new();

        for function in functions {
            inventory_builder = inventory_builder.register(function);
        }

        let my_inventory = inventory_builder.inventory();
        let config = Config {
            dll_name: dll_name.to_string(),
            namespace_mappings: NamespaceMappings::new("Bubbles"),
            use_unsafe: Unsafe::UnsafeKeyword,
            ..Config::default()
        };
        Generator::new(config, my_inventory)
            .add_overload_writer(Unity::new())
            .write_file("Wrapper.cs");

        let mut file = fs::File::open("Wrapper.cs").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let stringToReplace = format!("public const string NativeLib = \"{}\";", dll_name);
        let stringToReplaceWith = format!("#if UNITY_EDITOR\npublic const string NativeLib = \"{}\";\n#else\npublic const string NativeLib = \"{}\";\n#endif", dll_name, "game");
        let contents = contents.replace(&stringToReplace, &stringToReplaceWith);
        fs::write("Wrapper.cs", contents).unwrap();

        let path = std::env::current_dir().unwrap();
        let path = path.to_str().unwrap();
        println!("path: {}", path);

        let path_to_built_dll = format!("{}\\..\\target\\debug\\bargame_lib.dll", path);
        println!("path_to_built_dll: {}", path_to_built_dll);
        let path_to_unity_plugin_folder = format!("{}\\..\\..\\Assets\\Plugins\\x86_64", path);
        println!("path_to_unity_plugin_folder: {}", path_to_unity_plugin_folder);
        let path_to_wrapper = format!("{}\\Wrapper.cs", path);

        let paths = fs::read_dir(path_to_unity_plugin_folder.clone()).unwrap();
        paths.for_each(|path| {
            let path = path.unwrap().path();
            let path = path.to_str().unwrap();
            println!("deleting: {}", path);
            fs::remove_file(path).unwrap_or(());
        });

        fs::copy(path_to_built_dll, format!("{}\\{}.dll", path_to_unity_plugin_folder, dll_name)).unwrap();
        fs::copy(path_to_wrapper, format!("{}\\Wrapper.cs", path_to_unity_plugin_folder)).unwrap();
    }

    // Modify the generate_bindings function to call the new function
    #[test]
    fn generate_bindings() {
        let functions: Vec<Symbol> = vec![
            function!(add_extern),
            function!(get_example_array),
            function!(get_example_list),
            function!(allocate_native_array),
            function!(deallocate_native_array),
            function!(rust_function_with_callback),
        ];

        generate_bindings_with_functions(functions);
    }
}