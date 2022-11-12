#mkdir -p ../Assets/Plugins/x86_64/
#mkdir -p ../Assets/Plugins/WebGL/

#cargo build --target x86_64-pc-windows-msvc --release
#cargo build --target wasm32-unknown-unknown --release


# editor debug
cargo rustc --target x86_64-pc-windows-msvc --release -- -C opt-level=3
cp target/x86_64-pc-windows-msvc/release/mandelbrot.dll ../


#../Assets/Plugins/x86_64/mandelbrot.dll
#../Assets/Plugins/WebGL/libmandelbrot.a

# delete these two paths the files are copied to
#rm -rf ../Assets/Plugins/x86_64/
#rm -rf ../Assets/Plugins/WebGL/













