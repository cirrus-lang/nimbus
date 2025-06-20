use std::env;

mod core {
    pub mod new_project;
}

fn print_help() {
    println!("Nimbus - The Cirrus Package Manager");
    println!();
    println!("Usage:");
    println!("  nimbus new <project_name>    Create a new Cirrus project");
    println!("  nimbus build                 Build the current project");
    println!("  nimbus run                   Build and run the project");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No command provided.");
        print_help();
        return;
    }

    let command = args[1].as_str();

    match command {
        "new" => {
            if args.len() < 3 {
                println!("Error: 'new' requires a project name.");
                println!("Usage: nimbus new <project_name>");
            } else {
                let project_name = &args[2];
                println!("Creating new project: {}", project_name);
                core::new_project::create_project(project_name).unwrap();
            }
        }
        "build" => {
            println!("Building project...");
        }
        "run" => {
            println!("Running project...");
        }
        _ => {
            println!("Unknown command: {}", command);
            print_help();
        }
    }
}
