use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let proto_dir = PathBuf::from(&manifest_dir).join("proto");

    let proto_files = get_proto_files(&proto_dir);

    for proto_file in &proto_files {
        compile_protobuf_for_csharp(&manifest_dir, &proto_dir, &proto_file);
        compile_protobuf_for_rust(&manifest_dir, &proto_dir, &proto_file);
    }
}

fn get_proto_files(proto_dir: &PathBuf) -> Vec<PathBuf> {
    fs::read_dir(proto_dir)
        .expect("Failed to read proto directory")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().unwrap_or_default() == "proto")
        .map(|entry| entry.path())
        .collect()
}

fn compile_protobuf_for_csharp(manifest_dir: &str, proto_dir: &PathBuf, proto_file: &PathBuf) {
    let protoc_executable = PathBuf::from(manifest_dir).join("protoc.exe");
    env::set_var("PROTOC", &protoc_executable);

    let csharp_out_dir = PathBuf::from(manifest_dir).join("csharp\\Generated");

    Command::new(&protoc_executable)
        .arg(format!("--proto_path={}", proto_dir.display()))
        .arg(format!("--csharp_out={}", csharp_out_dir.display()))
        .arg(proto_file)
        .status()
        .expect("Failed to execute protoc");
}

fn compile_protobuf_for_rust(manifest_dir: &str, proto_dir: &PathBuf, proto_file: &PathBuf) {
    let rust_out_dir = PathBuf::from(manifest_dir).join("src/protos");
    std::fs::create_dir_all(&rust_out_dir).unwrap_or_default();

    prost_build::Config::new()
        .out_dir(&rust_out_dir)
        .compile_protos(&[proto_file], &[&proto_dir])
        .unwrap();
}