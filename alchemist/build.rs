use std::path::Path;
use std::process::Command;

fn main() {
    // If cross is installed its probs being built on travis, don't do anything
    if Command::new("which").arg("cross").output().unwrap().status.success() {
        println!("Cross is found on the sytem, not worrying about diesel/databases as this is \
                  probably being run on Travis");
        return;
    } else {
        // If the alchemist.db isn't existent, make one.
        if Path::new("./alchemist.db").exists() == false {
            println!("database not found, making one...");
            Command::new("diesel").arg("setup");
        }
        // Run any migrations available that haven't been run before building
        println!("Running Migrations...");
        Command::new("diesel")
            .arg("migration")
            .arg("run")
            .output()
            .unwrap_or_else(|e| panic!("Failed to execute: {}", e));

    }
}
