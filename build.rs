#[allow(unused_imports)]
use std::{path::PathBuf, process::Command};

#[allow(dead_code)]
fn main() {
    #[cfg(feature = "frontend")]
    {
        let mut dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let project = PathBuf::from(std::env::var_os("PWD").unwrap());
        dir.pop();
        dir.pop();
        dir.pop();
        println!("cargo:warning=Moving /dist to: {dir:?}");
        println!("cargo:warning=PROJECT DIR: {project:?}");
        println!(
            "cargo:warning=cp: {:?}",
            Command::new("cp")
                .args([
                    "-rf",
                    &format!("{}/frontend/dist", project.to_string_lossy()),
                    dir.to_str().unwrap()
                ])
                .output()
                .expect("Couldn't move frontend build artifacts")
        );
        println!("cargo:rerun-if-changed=frontend");
    }
    println!("cargo:rerun-if-changed=NULL");
}
