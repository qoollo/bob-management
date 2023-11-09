///
/// Build Script
/// This is run as a pre-build step -- before the rust backend is compiled.
/// NOTE: Should be included in root's build script
///
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{fs, io};
use std::{io::Write, process::Command};

#[cfg(target_family = "windows")]
const SHELL: &str = "cmd";
#[cfg(target_family = "unix")]
const SHELL: &str = "sh";
#[cfg(target_family = "windows")]
const SHELL_ARG: &str = "/C";
#[cfg(target_family = "unix")]
const SHELL_ARG: &str = "-c";

const FRONTEND_DIR: &str = "frontend";

/*
 * Note 1: this file was written for *nix systems -- it likely won't
 * work on windows!
 */

#[allow(dead_code)]
fn shell(command: impl AsRef<OsStr> + Display) {
    // println!("build.rs => {}", command);

    let output = Command::new(SHELL)
        .arg(SHELL_ARG)
        .arg(&command)
        .output()
        .unwrap_or_else(|_| panic!("Failed to run {cmd}", cmd = command));

    // println!("build.rs => {:?}", output.stdout);
    let mut file = File::create("build.log").expect("Couldn't create file...");
    file.write_all(b"build log\n\n\n\nSTDOUT:\n")
        .expect("Couldn't write to build log");
    file.write_all(&output.stdout)
        .expect("Couldn't write to build log");
    file.write_all(b"\n\n\n\nSTDERR:\n")
        .expect("Couldn't write to build log");
    file.write_all(&output.stderr)
        .expect("Couldn't write to build log");

    assert!(
        output.status.success(),
        "{command} couldn't build frontend. Exit code: {:?}",
        output.status.code().expect("The process was terminated")
    );
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
}

pub fn move_frontend() {
    let mut target = PathBuf::from(std::env::var("OUT_DIR").unwrap()); // OUT_DIR == <project_dir>/target/<target_profile>/build/bob_management-<HASH>/out
    target.pop();
    target.pop();
    target.pop();
    target.push(FRONTEND_DIR);

    let mut project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project_dir.push(FRONTEND_DIR);

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("build.log")
        .expect("Couldn't open log file...");
    file.write_all(format!("PROJECT DIR: {project_dir:?}\n").as_bytes())
        .expect("Couldn't write to build log");

    file.write_all(
        format!("Moving /{FRONTEND_DIR} from {project_dir:?} to: {target:?}\n").as_bytes(),
    )
    .expect("Couldn't write to build log");

    copy_dir_all(project_dir, target).expect("Couldn't move frontend build artifacts");
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn main() {
    build_types();
    build_frontend();
    move_frontend();
    println!("cargo:rerun-if-changed=./");
}
