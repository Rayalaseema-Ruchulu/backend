use std::{
    env,
    process::{Command, Stdio},
};

fn main() {
    let _ = Command::new("flutter")
        .args(["build", "web", "--wasm", "--release"])
        .current_dir("./frontend/")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()
        .expect("Failed to execute flutter web build command!");

    if env::var("NON_ROOT_CWD").is_ok() {
        Command::new("mkdir")
            .args(["-p", "../frontend/build/"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .status()
            .expect("Failed to execute temp directory creation!");

        Command::new("mv")
            .args(["./frontend/build/web", "../frontend/build/"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .status()
            .expect("Failed to execute temporary move!");
    }
}
