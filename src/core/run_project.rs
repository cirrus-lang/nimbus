use std::process::Command;
use std::path::Path;

#[cfg(target_os = "windows")]
const EXECUTABLE_PATH: &str = "target/main.exe";
#[cfg(not(target_os = "windows"))]
const EXECUTABLE_PATH: &str = "target/main";

use crate::core::build_project::build_project;

pub fn run_project() {
    build_project();

    let exec_path = Path::new(EXECUTABLE_PATH);
    if !exec_path.exists() {
        println!("Error: Executable not found at {}", exec_path.display());
        return;
    }

    println!("Running project...\n");

    let status = Command::new(exec_path)
        .status();

    match status {
        Ok(s) => {
            if !s.success() {
                println!("\nProgram exited with status code: {}", s.code().unwrap_or(-1));
            }
        }
        Err(e) => {
            println!("Failed to execute binary: {}", e);
        }
    }
}
