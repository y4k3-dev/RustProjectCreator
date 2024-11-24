use std::{env, fs, process};

const READMESTRING: &str = r#"
### Description
This is a new project created using a Rust program.

### Cargo commands
```bash
cargo run
```
```bash
cargo build
```
```bash
cargo build --release
```
```bash
cargo test
```
"#;

const TASKSSTRING: &str = r#"
{
    "version": "2.0.0",
    "tasks": [{
        "label": "Cargo Run",
        "command": "cargo",
        "args": ["run"],
        "type": "shell"
    },
    {
        "label": "Cargo Build",
        "command": "cargo",
        "args": ["build"],
        "type": "shell"
    },
    {
        "label": "Cargo Build Release",
        "command": "cargo",
        "args": ["build --release"],
        "type": "shell"
    },
    {
        "label": "Cargo Test",
        "command": "cargo",
        "args": ["test"],
        "type": "shell"
    }
]
}
"#;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Make sure 3 arguments are passed
    if args.len() != 3 {
        eprintln!("Usage: <file_path> <project_name>");
        process::exit(1);
    }

    let file_path = &args[1];
    let project_name = &args[2];

    // Create a directory for the project
    create_directory(file_path);

    // Create a new project using cargo
    create_rust_project(project_name, file_path);

    // Create a readme file
    create_readme(project_name, file_path);

    // Create a tasks.json file
    fs::write(
        format!("{}/{}/.vscode/tasks.json", file_path, project_name),
        TASKSSTRING,
    )
    .unwrap_or_else(|err| {
        eprintln!("Error creating tasks.json: {}", err);
        process::exit(1);
    });

    // Open the project in Visual Studio Code using cmd as the path is only set for the current session
    open_visual_studio_code(file_path, project_name);
}

fn open_visual_studio_code(file_path: &String, project_name: &String) {
    process::Command::new("cmd")
        .arg("/C")
        .arg(format!("code {}/{}", file_path, project_name))
        .output()
        .unwrap_or_else(|err| {
            eprintln!("Error opening project in Visual Studio Code: {}", err);
            process::exit(1);
        });
}

fn create_readme(project_name: &String, file_path: &String) {
    let read_me_text = format!("# {}\n{}", project_name, READMESTRING);

    fs::write(
        format!("{}/{}/README.md", file_path, project_name),
        read_me_text,
    )
    .unwrap_or_else(|err| {
        eprintln!("Error creating README.md: {}", err);
        process::exit(1);
    });
}

fn create_rust_project(project_name: &String, file_path: &String) {
    process::Command::new("cargo")
        .arg("new")
        .arg(project_name)
        .current_dir(file_path)
        .output()
        .unwrap_or_else(|err| {
            eprintln!("Error creating project: {}", err);
            process::exit(1);
        });
}

fn create_directory(file_path: &String) {
    fs::create_dir_all(file_path).unwrap_or_else(|err| {
        eprintln!("Error creating directory: {}", err);
        process::exit(1);
    });
}
