use interoptopus::*;

#[ffi_function]
#[no_mangle]
pub extern "C" fn add(left: i64, right: i64) -> i64 {
    left + right
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
            .register(function!(add))
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
