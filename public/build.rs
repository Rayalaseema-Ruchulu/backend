use std::{env, process::{Command, Stdio}};

fn main() {
    Command::new("flutter")
        .args(["build", "web", "--wasm", "--release"])
        .current_dir("./frontend/")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute flutter web build command!");

    if env::var("NON_ROOT_CWD").is_ok() {
        println!("Moving to expected location");
        
        Command::new("mkdir")
            .args(["-p", "../backend/public/frontend/build"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .status()
            .expect("Failed to execute temp directory creation!");

        Command::new("mv")
            .args(["./frontend/build/web", "../backend/public/frontend/build"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .status()
            .expect("Failed to execute temporary move!");
    }
}
