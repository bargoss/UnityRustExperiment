#mkdir -p ../Assets/Plugins/x86_64/
#mkdir -p ../Assets/Plugins/WebGL/

#cargo build --target x86_64-pc-windows-msvc --release
#cargo build --target wasm32-unknown-unknown --release


# release
cargo rustc --target x86_64-pc-windows-msvc --release -- -C opt-level=3
rustup target add wasm32-unknown-unknown
cargo rustc --target wasm32-unknown-unknown --release -- -C opt-level=3
cp target/x86_64-pc-windows-msvc/release/mandelbrot.dll ../Assets/Plugins/x86_64/
cp target/wasm32-unknown-unknown/release/libmandelbrot.a ../Assets/Plugins/WebGL/
