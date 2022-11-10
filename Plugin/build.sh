#mkdir -p ../Assets/Plugins/x86_64/
#mkdir -p ../Assets/Plugins/WebGL/
cargo build --target x86_64-pc-windows-msvc --release
cp target/x86_64-pc-windows-msvc/release/mandelbrot.dll ../Assets/Plugins/x86_64/
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/libmandelbrot.a ../Assets/Plugins/WebGL/








