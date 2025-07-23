use std::process::{Command, Stdio};

fn main() {
    let web_build = Command::new("flutter")
        .args(["build", "web", "--wasm", "--release"])
        .current_dir("./frontend/")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute flutter web build command!");
}
