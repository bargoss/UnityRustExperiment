use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let proto_dir = PathBuf::from(&manifest_dir).join("proto");

    compile_protobuf_for_csharp(&manifest_dir, &proto_dir);
    compile_protobuf_for_rust(&manifest_dir, &proto_dir);
}

fn compile_protobuf_for_csharp(manifest_dir: &str, proto_dir: &PathBuf) {
    let protoc_executable = PathBuf::from(manifest_dir).join("protoc.exe");
    env::set_var("PROTOC", &protoc_executable);

    let csharp_out_dir = PathBuf::from(manifest_dir).join("csharp\\Generated");

    Command::new(&protoc_executable)
        .arg(format!("--proto_path={}", proto_dir.display()))
        .arg(format!("--csharp_out={}", csharp_out_dir.display()))
        .arg(proto_dir.join("example.proto"))
        .status()
        .expect("Failed to execute protoc");
}

fn compile_protobuf_for_rust(manifest_dir: &str, proto_dir: &PathBuf) {
    let rust_out_dir = PathBuf::from(manifest_dir).join("src/protos");
    std::fs::create_dir_all(&rust_out_dir).unwrap_or_default();

    prost_build::Config::new()
        .out_dir(&rust_out_dir)
        .compile_protos(&[&proto_dir.join("example.proto")], &[&proto_dir])
        .unwrap();
}