use std::{
    env,
    fs::remove_dir_all,
    path::Path,
    process::{Command, Stdio},
};

fn main() {
    // Always rerun
    println!("cargo:rerun-if-changed=NULL");
    println!("Running build script");

    Command::new("flutter")
        .args(["build", "web", "--wasm", "--release"])
        .current_dir("./frontend/")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute flutter web build command!");

    if env::var("NON_ROOT_CWD").is_ok() {
        println!("Moving to expected location");

        if Path::new("../backend/public/frontend/build/web").exists() {
            remove_dir_all("../backend/public/frontend/build/web").unwrap();
        }

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
