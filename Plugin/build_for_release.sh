#mkdir -p ../Assets/Plugins/x86_64/
#mkdir -p ../Assets/Plugins/WebGL/

#cargo build --target x86_64-pc-windows-msvc --release
#cargo build --target wasm32-unknown-unknown --release


# release
cargo rustc --target x86_64-pc-windows-msvc --release -- -C opt-level=3
rustup target add wasm32-wasi
cargo rustc --target wasm32-wasi --release -- -C opt-level=3
cp target/x86_64-pc-windows-msvc/release/mandelbrot.dll ../Assets/Plugins/x86_64
cp target/wasm32-wasi/release/libmandelbrot.a ../Assets/Plugins/WebGL

#aarch64-linux-android
rustup target add aarch64-linux-android
cargo rustc --target aarch64-linux-android --release -- -C opt-level=3
# alternative:
cargo build --target aarch64-linux-android --release
# why do i get linker 'cc' not found?
# it's because i need to install the android ndk
# how do i install it in windows?
# https://developer.android.com/ndk/downloads



cp target/aarch64-linux-android/release/libmandelbrot.so ../Assets/Plugins/Android

