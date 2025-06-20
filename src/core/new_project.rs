use std::fs;
use std::io::{self};
use std::path::Path;

pub fn create_project(project_name: &str) -> io::Result<()> {
    let base_path = Path::new(project_name);

    if base_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Directory of the project '{}' already exists.", project_name),
        ));
    }

    fs::create_dir(&base_path)?;

    create_gitignore(&base_path)?;
    create_nimbus_toml(&base_path, project_name)?;

    let src_path = base_path.join("src");
    fs::create_dir(&src_path)?;

    create_main_file(&src_path)?;

    println!("Project '{}' succesfully created!", project_name);

    Ok(())
}

fn create_gitignore(base_path: &Path) -> io::Result<()> {
    let gitignore_content = "# Cirrus target folder\ntarget/\n";
    fs::write(base_path.join(".gitignore"), gitignore_content)
}

fn create_nimbus_toml(base_path: &Path, project_name: &str) -> io::Result<()> {
    let nimbus_content = format!(
        r#"[project]
name = "{}"
version = "0.1.0"
"#,
        project_name
    );
    fs::write(base_path.join("nimbus.toml"), nimbus_content)
}

fn create_main_file(src_path: &Path) -> io::Result<()> {
    let main_code = r#"void main() {
    println("Hello, World!");
}
"#;
    fs::write(src_path.join("main.crs"), main_code)
}