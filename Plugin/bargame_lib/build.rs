use std::fs;
use std::path::Path;
use std::process::Command;
use flatc_rust::{Args, run};

fn main() {
    let schema_path = Path::new("src/schema/schema.fbs");
    let out_path = Path::new("src/schema/generated");

    // Check if the output directory exists, if not, create it
    if !out_path.exists() {
        fs::create_dir_all(&out_path).expect("Failed to create output directory");
    }

    // Determine the platform-specific flatc executable path
    let flatc_executable = Path::new("../flatc/Windows.flatc.binary/flatc.exe");

    //Compile the FlatBuffers schema
    let status = Command::new(flatc_executable)
        .arg("-r")
        .arg("--rust")
        .arg("-o")
        .arg(out_path)
        .arg(schema_path)
        .status()
        .expect("Failed to execute flatc");
    if !status.success() {
        panic!("Failed to compile FlatBuffers schema");
    }
}
