#mkdir -p ../Assets/Plugins/x86_64/
#mkdir -p ../Assets/Plugins/WebGL/
#mkdir -p ../Assets/Plugins/ARMv7/
#mkdir -p ../Assets/Plugins/ARM64/

#cargo build --target x86_64-pc-windows-msvc --release
#cargo build --target wasm32-unknown-unknown --release


# release

#rustup target add wasm32-unknown-unknown
#cargo rustc --target wasm32-unknown-unknown --release -- -C opt-level=3

cargo rustc --target x86_64-pc-windows-msvc --release -- -C opt-level=3
cargo install -f cross
cross build --target armv7-unknown-linux-gnueabihf --release
cross build --target aarch64-linux-android --release

cp target/x86_64-pc-windows-msvc/release/mandelbrot.dll ../Assets/Plugins/x86_64/
cp target/armv7-unknown-linux-gnueabihf/release/libmandelbrot.a ../Assets/Plugins/ARMv7/
cp target/aarch64-linux-android/release/libmandelbrot.a ../Assets/Plugins/ARM64/
cp target/armv7-unknown-linux-gnueabihf/release/libmandelbrot.so ../Assets/Plugins/ARMv7/

#cp target/wasm32-unknown-unknown/release/libmandelbrot.a ../Assets/Plugins/WebGL/