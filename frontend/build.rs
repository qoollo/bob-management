///
/// Build Script
/// This is run as a pre-build step -- before the rust backend is compiled.
/// NOTE: Should be included in root's build script
///
use std::fs::File;
use std::path::PathBuf;
use std::{io::Write, process::Command};

/*
 * Note 1: this file was written for *nix systems -- it likely won't
 * work on windows!
 */

#[allow(dead_code)]
fn shell(command: &str) {
    // println!("build.rs => {}", command);

    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect(format!("Failed to run {cmd}", cmd = command).as_str());

    // println!("build.rs => {:?}", output.stdout);
    let mut file = File::create("build.log").expect("Couldn't create file...");
    file.write(b"build log\n\n\n\nSTDOUT:\n")
        .expect("Couldn't write to build log");
    file.write_all(&output.stdout)
        .expect("Couldn't write to build log");
    file.write(b"\n\n\n\nSTDERR:\n")
        .expect("Couldn't write to build log");
    file.write_all(&output.stderr)
        .expect("Couldn't write to build log");
}

pub fn build_types() {
    let mut inputs = vec![PathBuf::from(env!("CARGO_MANIFEST_DIR"))];
    let mut output = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    inputs[0].pop();

    inputs[0].push("backend");
    output.push("src/types/rust.d.ts");

    tsync::generate_typescript_defs(inputs, output, false);
}

pub fn build_frontend() {
    // Only install frontend dependencies when building release
    // #[cfg(not(debug_assertions))]
    shell("yarn");

    // Only build frontend when building a release
    // #[cfg(not(debug_assertions))]
    shell("yarn build");

    let mut dir = PathBuf::from(std::env::var("OUT_DIR").unwrap()); // OUT_DIR == <project_dir>/target/<traget_profile>/build/bob_management-<HASH>/out
    dir.pop();
    dir.pop();
    dir.pop();
    dir.pop();
    let mut project_dir = dir.clone();
    project_dir.pop();
    println!("cargo:warning=Moving /dist to: {dir:?}");
    println!("cargo:warning=PROJECT DIR: {project_dir:?}");
    println!(
        "cargo:warning=cp: {:?}",
        Command::new("cp")
            .args([
                "-rf",
                &format!("{}/frontend/dist", project_dir.to_string_lossy()),
                dir.to_str().unwrap()
            ])
            .output()
            .expect("Couldn't move frontend build artifacts")
    );
    // println!("cargo:rerun-if-changed=frontend");
    println!("cargo:rerun-if-changed=NULL");
}

#[allow(dead_code)]
fn main() {
    build_types();
    build_frontend();
}
