import os
import shutil

# cargo rustc --target x86_64-pc-windows-msvc --release -- -C opt-level=3
# cargo rustc --target wasm32-unknown-unknown --release -- -C opt-level=3
# cp target/x86_64-pc-windows-msvc/release/mandelbrot.dll ../Assets/Plugins/x86_64/
# cp target/wasm32-unknown-unknown/release/libmandelbrot.a ../Assets/Plugins/WebGL/
# run external command with python
os.system("cargo rustc --target wasm32-unknown-unknown --release -- -C opt-level=3")
shutil.copy("target/wasm32-unknown-unknown/release/libmandelbrot.a", "../Assets/Plugins/WebGL/")

# "C:\Program Files\Unity\Hub\Editor\2019.4.11f1\Editor\Unity.exe" -quit -batchmode -projectPath "C:\Users\Neil\Dev\shrinewars\ShrineWars" -executeMethod BuildScript.BuildAll

# define unity.exe path
unity_path = "C:/Program Files/Unity/Hub/Editor/2021.3.4f1/Editor/Unity.exe"
unity_path = "\"" + unity_path + "\""

# current path
current_path = os.getcwd()
# define project path from the root
project_path = current_path + "/../"
project_path = "\"" + project_path + "\""

# define build command
build_command = unity_path + " -quit -batchmode -projectPath " + project_path + " -executeMethod WebGLBuilder.Build"
print("build_command\n", build_command)

# run build command
os.system(build_command)




#os.system("cp target/x86_64-pc-windows-msvc/release/mandelbrot.dll ../")
# in more pythonic way

print("DoneDoneDoneDoneDoneDoneDoneDoneDoneDoneDoneDoneDoneDoneDoneDoneDoneDoneDoneDoneDoneDoneDoneDone")




