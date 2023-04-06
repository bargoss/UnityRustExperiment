use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::io::Write;

fn create_txt_experiment(){
    // create a text file in the local directory
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let file_name = "example_txt_in_project_folder.txt";
    let file_path = PathBuf::from(project_dir).join(file_name);
    std::fs::write(file_path, "Hello, world!").unwrap();
}

fn protostuff(){

    // create a text that I can append to
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("example_txt_in_project_folder.txt")
        .unwrap();

    // Compile protobuf for Rust
    prost_build::compile_protos(&["proto/example.proto"], &["proto/"]).unwrap();

    // Compile protobuf for C#
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // append to the text file
    writeln!(file, "manifest_dir: {}", manifest_dir).unwrap();

    let protoc_executable = PathBuf::from(manifest_dir).join("protoc.exe");
    // append to the text file
    writeln!(file, "protoc_executable: {}", protoc_executable.display()).unwrap();

    let status = Command::new(protoc_executable)
        .arg("--proto_path=proto")
        .arg("--csharp_out=csharp/Generated")
        .arg("proto/example.proto")
        .status()
        .expect("Failed to execute protoc");

    // append to the text file
    writeln!(file, "status: {}", status).unwrap();

    assert!(status.success(), "Failed to generate C# structures");
}

fn main() {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("example_txt_in_project_folder.txt")
        .unwrap();

    // Compile protobuf for C#
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    writeln!(file, "manifest_dir: {}", manifest_dir).unwrap();

    let protoc_executable = PathBuf::from(&manifest_dir).join("protoc.exe");
    writeln!(file, "protoc_executable: {}", protoc_executable.display()).unwrap();

    let proto_dir = PathBuf::from(&manifest_dir).join("proto");
    writeln!(file, "proto_dir: {}", proto_dir.display()).unwrap();

    let csharp_out_dir = PathBuf::from(&manifest_dir).join("csharp/Generated");
    writeln!(file, "csharp_out_dir: {}", csharp_out_dir.display()).unwrap();

    let status = Command::new(protoc_executable)
        .arg(format!("--proto_path={}", proto_dir.display()))
        .arg(format!("--csharp_out={}", csharp_out_dir.display()))
        .arg(proto_dir.join("example.proto"))
        .status()
        .expect("Failed to execute protoc");

    assert!(status.success(), "Failed to generate C# structures");
}