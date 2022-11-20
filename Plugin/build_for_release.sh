#mkdir -p ../Assets/Plugins/x86_64/
#mkdir -p ../Assets/Plugins/WebGL/

#cargo build --target x86_64-pc-windows-msvc --release
#cargo build --target wasm32-unknown-unknown --release


# release
cargo rustc --target x86_64-pc-windows-msvc --release -- -C opt-level=3



#cargo build --release --target wasm32-unknown-unknown
#wasm-bindgen --out-dir ./out/ --target web ./target/

rustup target add wasm32-wasi
cargo rustc --target wasm32-wasi --release -- -C opt-level=3


cp target/x86_64-pc-windows-msvc/release/mandelbrot.dll ../Assets/Plugins/x86_64
cp target/wasm32-wasi/release/libmandelbrot.a ../Assets/Plugins/WebGL
cp target/wasm32-wasi/release/libmandelbrot.d ../Assets/Plugins/WebGL

cp target/wasm32-wasi/release/mandelbrot.wasm ../Assets/Plugins/WebGL
cp target/wasm32-wasi/release/mandelbrot.d ../Assets/Plugins/WebGL
