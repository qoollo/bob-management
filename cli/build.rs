use std::process::Command;
#[allow(clippy::unwrap_used)]
fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={git_hash}");

    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .unwrap();
    let git_branch = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_BRANCH={git_branch}");

    let mut command = Command::new("git");
    command.args(["describe", "--tags", "--abbrev=0"]);
    let output = command.output().unwrap();
    let git_tag = if command.status().unwrap().success() {
        String::from_utf8(output.stdout).unwrap()
    } else {
        String::from("NOTAG")
    };
    println!("cargo:rustc-env=GIT_TAG={git_tag}");

    println!("cargo:rustc-env=BUILD_TIME={:?}", chrono::Utc::now());
}
