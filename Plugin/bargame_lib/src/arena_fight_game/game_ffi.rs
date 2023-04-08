use interoptopus::*;
use crate::arena_fight_game::arena_game::ArenaFightGame;
use crate::game_core::math::FP;

#[ffi_type]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec3Ext {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[ffi_type]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ArenaGameInput{
    pub player_id: u32,
}

#[ffi_type]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ArenaFightGameExt {
    pub ptr: *const u8,
    pub inputs: [ArenaGameInput; 16]
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn create_game() -> ArenaFightGameExt {
    let game = ArenaFightGame::default();
    let ptr = Box::into_raw(Box::new(game));
    ArenaFightGameExt{ptr: ptr as *const u8, inputs: [ArenaGameInput{player_id: 0}; 16]}
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn advance_tick(game: ArenaFightGameExt) {
    let game = unsafe { &mut *(game.ptr as *mut ArenaFightGame) };
    game.advance_tick(Default::default());
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn register_views(game: ArenaFightGameExt) {
    let game = unsafe { &mut *(game.ptr as *mut ArenaFightGame) };
    game.register_views();
}

#[ffi_type]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SphereRenderParams{
    pub position: Vec3Ext,
    pub radius: f32,
    pub color: Vec3Ext,
}

// Define a callback function type
pub type SphereRenderAction = extern "C" fn(SphereRenderParams);

#[ffi_function]
#[no_mangle]
pub extern "C" fn render(game: ArenaFightGameExt, viewing_time: f32, sphere_render_action: SphereRenderAction) {
    let game = unsafe { &mut *(game.ptr as *mut ArenaFightGame) };
    game.render(
        FP::from_num(viewing_time as f64),
        |sphere_view_params| {
            sphere_render_action(SphereRenderParams{
                position: Vec3Ext{x: sphere_view_params.position.x().to_f32(), y: sphere_view_params.position.y().to_f32(), z: sphere_view_params.position.z().to_f32()},
                radius: sphere_view_params.radius.to_f32(),
                color: Vec3Ext{x: sphere_view_params.color[0], y: sphere_view_params.color[1], z: sphere_view_params.color[2]}
            });
        },
        |line_view_params| {}
    );
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

    #[test]
    fn generate_bindings(){
        let postfix = rand::random::<u32>();
        let dll_name = format!("game_{}", postfix);

        let my_inventory = InventoryBuilder::new()
            .register(function!(create_game))
            .register(function!(advance_tick))
            .register(function!(register_views))
            .register(function!(render))
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
        println!("path: {}", path);

        let path_to_built_dll = format!("{}\\..\\target\\debug\\bargame_lib.dll", path);
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