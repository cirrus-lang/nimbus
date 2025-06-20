use std::fs;
use std::path::Path;
use std::process::Command;

#[cfg(target_os = "windows")]
const EXECUTABLE_NAME: &str = "main.exe";
#[cfg(not(target_os = "windows"))]
const EXECUTABLE_NAME: &str = "main";

pub fn build_project() {
    let source_file = Path::new("src").join("main.crs");

    if !source_file.exists() {
        println!("Error: Source file '{}' not found.", source_file.display());
        return;
    }

    let status = Command::new("cirrus")
        .arg(source_file.to_str().unwrap())
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("Compiled successfully.");
        }
        Ok(s) => {
            println!("Compiler exited with status code: {}", s.code().unwrap_or(-1));
            return;
        }
        Err(e) => {
            println!("Failed to run cirrus compiler: {}", e);
            return;
        }
    }
    let target_dir = Path::new("target");
    if !target_dir.exists() {
        if let Err(e) = fs::create_dir(&target_dir) {
            println!("Failed to create 'target/' directory: {}", e);
            return;
        }
    }

    let executable = Path::new(EXECUTABLE_NAME);
    let destination = target_dir.join(EXECUTABLE_NAME);

    if executable.exists() {
        if let Err(e) = fs::rename(&executable, &destination) {
            println!("Failed to move executable to target/: {}", e);
        } else {
            println!("Executable moved to {}", destination.display());
        }
    } else {
        println!(
            "Warning: Expected executable '{}' not found.",
            executable.display()
        );
    }
}
