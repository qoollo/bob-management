use std::{path::PathBuf, process::Command};

pub fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");

    Command::new("cargo")
        .args(["watch", "-x", "run -- --default", "-w", "backend"])
        .current_dir(PathBuf::from(dir))
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
}
