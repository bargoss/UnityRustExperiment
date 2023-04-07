use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::io::Write;

fn main() {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("example_txt_in_project_folder.txt")
        .unwrap();

    // Compile protobuf for C#
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let protoc_executable = PathBuf::from(&manifest_dir).join("protoc.exe");
    env::set_var("PROTOC", protoc_executable.clone());

    let proto_dir = PathBuf::from(&manifest_dir).join("proto");

    let csharp_out_dir = PathBuf::from(&manifest_dir).join("csharp\\Generated");

    let mut binding = Command::new(&protoc_executable.clone());
    let command = binding
        .arg(format!("--proto_path={}", proto_dir.display()))
        .arg(format!("--csharp_out={}", csharp_out_dir.display()))
        .arg(proto_dir.join("example.proto"));

    let status = command.status().expect("Failed to execute protoc");

    writeln!(file, "status: {:?}", command).unwrap();

    // Compile protobuf for Rust
    let rust_out_dir = PathBuf::from(&manifest_dir).join("src/protos");

    if !rust_out_dir.exists() {
        std::fs::create_dir_all(&rust_out_dir).unwrap();
    }

    prost_build::Config::new()
        .out_dir(&rust_out_dir)
        .compile_protos(&[&proto_dir.join("example.proto")], &[&proto_dir])
        .unwrap();
}