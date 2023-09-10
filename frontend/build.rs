use std::fs::File;
use std::path::PathBuf;
///
/// Build Script
/// This is run as a pre-build step -- before the rust backend is compiled.
/// NOTE: Should be included in root's build script
///
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
    let dir = env!("CARGO_MANIFEST_DIR");

    let inputs = vec![PathBuf::from_iter([dir, "backend"])];
    let output = PathBuf::from_iter([dir, "frontend/src/types/rust.d.ts"]);

    tsync::generate_typescript_defs(inputs, output, false);
}

pub fn build_frontend() {
    // Only install frontend dependencies when building release
    // #[cfg(not(debug_assertions))]
    shell("cd frontend && npm install --frozen-lockfile");

    // Only build frontend when building a release
    #[cfg(not(debug_assertions))]
    shell("cd frontend && npm build");
}

// HACK: Dummy main to run with `cargo frontend` command
#[allow(dead_code)]
fn main() {}