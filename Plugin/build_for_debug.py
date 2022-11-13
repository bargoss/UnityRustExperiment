# cargo rustc --target x86_64-pc-windows-msvc --release -- -C opt-level=3
# cp target/x86_64-pc-windows-msvc/release/mandelbrot.dll ../
import os
import shutil

# run external command with python
os.system("cargo rustc --target x86_64-pc-windows-msvc --release -- -C opt-level=3")
shutil.copy("target/x86_64-pc-windows-msvc/release/mandelbrot.dll", "../")

# ../Assets/Plugins/x86_64/
# ../Assets/Plugins/WebGL/
# clear stuff in these folders
try:
    shutil.rmtree("../Assets/Plugins/x86_64/")
except:
    pass

# os.system("cp target/x86_64-pc-windows-msvc/release/mandelbrot.dll ../")
# in more pythonic way

# get input from user
input("Press Enter to continue...")
