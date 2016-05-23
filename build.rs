use std::process::Command;

fn main() {
    // Run which in order to check for availability
    let which = String::from_utf8(Command::new("which")
            .arg("diesel")
            .output()
            .unwrap()
            .stderr)
        .unwrap();

    // If diesel_cli isn't installed install it for the user
    if which.contains("no diesel in") || which.is_empty() {
        let mut install_diesel = Command::new("cargo")
            .arg("install")
            .arg("diesel_cli")
            .arg("--no-default-features")
            .arg("--features=sqlite")
            .spawn()
            .unwrap_or_else(|e| panic!("Failed to execute: {}", e));
        install_diesel.wait()
            .unwrap_or_else(|e| panic!("Failed to execute: {}", e));
    }

    // Run any migrations available that haven't been run before building
    let _dump = Command::new("diesel")
        .arg("migration")
        .arg("run")
        .output()
        .unwrap_or_else(|e| panic!("Failed to execute: {}", e));
}
