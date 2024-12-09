use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let shader_dir_path = Path::new("src/renderer/sampler/metal/shader");
    let kernel_path = shader_dir_path.join("kernel.metal");

    let out_dir_path = Path::new("src/renderer/sampler/metal");
    let air_path = out_dir_path.join("shader.air");
    let metallib_path = out_dir_path.join("shader.metallib");

    // Compile the Metal shader to an AIR file
    let status = Command::new("xcrun")
        .args(&["-sdk", "macosx", "metal", kernel_path.to_str().unwrap(), 
                "-I", shader_dir_path.to_str().unwrap(), 
                "-c", "-o", air_path.to_str().unwrap()])
        .status()
        .expect("Failed to execute xcrun metal command");
    if !status.success() {
        panic!("Metal compilation failed");
    }

    // Compile the AIR file to a metallib
    let status = Command::new("xcrun")
        .args(&["-sdk", "macosx", "metallib", air_path.to_str().unwrap(), 
                "-o", metallib_path.to_str().unwrap()])
        .status()
        .expect("Failed to execute xcrun metallib command");
    if !status.success() {
        panic!("Metallib compilation failed");
    }

    // Remove the AIR file
    let status = Command::new("rm")
        .args(&["-f", air_path.to_str().unwrap()])
        .status()
        .expect("Failed to remove the AIR file");
    if !status.success() {
        panic!("Removing the AIR file failed");
    }
    Command::new("rm")
        .current_dir(out_dir_path.to_str().unwrap())
        .args(&["-f", "*.tmp"])
        .status()
        .expect("Failed to remove the AIR file");

    for entry in fs::read_dir(shader_dir_path.to_str().unwrap()).expect("Failed to read shader directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.is_file() {
            println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
        }
    }
    println!("cargo:rerun-if-changed={}", kernel_path.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", metallib_path.to_str().unwrap());
}