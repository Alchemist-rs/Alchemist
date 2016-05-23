use std::process::Command;
use std::collections::HashSet;

use db;

/// Installs Packages on Debian Linux
///
/// #Examples
///
/// ```
/// let mut packages: HashSet<&str> = HashSet::new();
/// packages.push("sudo");
/// packages.push("postgresql");
/// debian_install(packages);
/// ```
///
pub fn debian_install(packages: HashSet<String>) {
    let debian_packages = convert_to_debian(packages);
    if !debian_packages.is_empty() {
        aptitude(debian_packages);
    }
}

/// Convert package names from other distros to Debian
fn convert_to_debian(input_packages: HashSet<String>) -> HashSet<String> {
    let results = db::pack_query(input_packages);
    let mut apt_converted: HashSet<String> = HashSet::new();

    // Using the querys store into the HashSet the actual
    // Debian package name for use later
    for i in results {
        // All querys will either be a string or '' in the db
        // allowing us to use is_empty()
        if !i.debian.is_empty() {
            apt_converted.insert(i.debian);
        }

    }

    apt_converted
}

// Apitude specific functions

/// Calls the aptitude program to install packages
///
/// #Examples
///
/// ```
/// let mut packages: HashSet<String> = HashSet::new();
/// packages.push("sudo".to_owned());
/// aptitude(packages);
/// ```
///
pub fn aptitude(mut packages: HashSet<String>) {
    let mut child = match Command::new("aptitude")
        .arg("install")
        .args(packages.drain()
            .collect::<Vec<String>>()
            .as_slice())
        .spawn() {
        Ok(child) => child,
        Err(e) => panic!("Failed to execute child: {}", e),
    };
    let _unused = child.wait();
}

/// Calls the aptitude program to refresh the package list
///
/// #Examples
///
/// ```
/// refresh_list();
/// ```
///
pub fn refresh_list() {
    let mut child = match Command::new("aptitude")
        .arg("update")
        .spawn() {
        Ok(child) => child,
        Err(e) => panic!("Failed to execute child: {}", e),
    };
    let _unused = child.wait();
}

/// Calls the aptitude program to upgrage all packages
///
/// #Examples
///
/// ```
/// refresh_list();
/// ```
///
pub fn upgrade_packages() {
    let mut child = match Command::new("aptitude")
        .arg("install")
        .spawn() {
        Ok(child) => child,
        Err(e) => panic!("Failed to execute child: {}", e),
    };
    let _unused = child.wait();
}
