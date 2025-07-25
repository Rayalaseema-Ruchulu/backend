use std::env;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::process::{Command, Stdio};

/// Runs a flutter command with the given arguments in the `./frontend` directory.
///
/// Panics with a helpful error message if the command fails.
fn run_flutter_command<I, S>(args: I, error_message: &str)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr> + Debug,
{
    // Try to find flutter via FLUTTER_HOME, or fall back to just `flutter`
    // and hope it's in the PATH.
    let flutter_cmd = env::var("FLUTTER_HOME")
        .map(|p| format!("{}/bin/flutter", p))
        .unwrap_or_else(|_| "flutter".to_string());

    let args_vec: Vec<S> = args.into_iter().collect();

    println!(
        "cargo:warning=Running command: {} {:?}",
        flutter_cmd, args_vec
    );

    let status = Command::new(&flutter_cmd)
        .args(&args_vec)
        .current_dir("./frontend/")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(exit_status) => {
            if !exit_status.success() {
                panic!("{} failed with status: {}", error_message, exit_status);
            }
        }
        Err(e) => {
            panic!(
                "{} failed. Is flutter in your PATH or is FLUTTER_HOME set? Error: {}",
                error_message, e
            );
        }
    }
}

fn main() {
    // Rerun if this build script changes
    println!("cargo:rerun-if-changed=build.rs");
    // Rerun if frontend dependencies change
    println!("cargo:rerun-if-changed=frontend/pubspec.yaml");
    // Rerun if frontend source code changes. This is a directory, which is supported.
    println!("cargo:rerun-if-changed=frontend/lib");

    println!("cargo:warning=Running flutter pub get...");
    run_flutter_command(["pub", "get"], "Failed to download dependencies");

    println!("cargo:warning=Building flutter web app...");

    if cfg!(debug_assertions) {
        println!("cargo:warning=Debug mode");

        run_flutter_command(
            [
                "build",
                "web",
                "--wasm",
                "--debug",
                "--pwa-strategy=none",
                "--no-web-resources-cdn",
                "--source-maps",
            ],
            "Failed to execute flutter web build command!",
        );
    } else {
        println!("cargo:warning=Release mode");

        run_flutter_command(
            [
                "build",
                "web",
                "--wasm",
                "--release",
                "--pwa-strategy=none",
                "--no-web-resources-cdn",
                "--source-maps",
                "--dart2js-optimization",
                "O4",
            ],
            "Failed to execute flutter web build command!",
        );
    }
}
