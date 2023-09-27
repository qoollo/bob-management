use std::process::{Command, ExitStatus};

fn main() {
    option_env!("BOBGUI_GIT_HASH")
        .is_none()
        .then(|| set_env("BOBGUI_GIT_HASH", "git", &["rev-parse", "HEAD"]));

    option_env!("BOBGUI_BUILD_BRANCH_TAG").is_none().then(|| {
        if !set_env(
            "BOBGUI_BUILD_BRANCH_TAG",
            "git",
            &["describe", "--tags", "--abbrev=0"],
        )
        .success()
        {
            set_env(
                "BOBGUI_BUILD_BRANCH_TAG",
                "git",
                &["rev-parse", "--abbrev-ref", "HEAD"],
            );
        }
    });

    println!("cargo:rerun-if-changed=../backend");
}

#[allow(clippy::unwrap_used)]
fn set_env(env_var: &str, cmd: &str, args: &[&str]) -> ExitStatus {
    let mut command = Command::new(cmd);
    command.args(args);
    let output = command.output().unwrap();
    if command.status().unwrap().success() {
        println!(
            "cargo:rustc-env={env_var}={}",
            String::from_utf8(output.stdout).unwrap()
        );
    };

    command.status().unwrap()
}
