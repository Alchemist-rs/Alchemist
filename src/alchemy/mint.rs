use std::process::Command;
use std::collections::HashSet;

use db;

/// Installs Packages on Mint Linux
///
/// #Examples
///
/// ```
/// let mut packages: HashSet<&str> = HashSet::new();
/// packages.insert("sudo");
/// packages.insert("postgresql");
/// Mint_install(packages);
/// ```
///
pub fn mint_install(packages: HashSet<String>) {
    let mint_packages = convert_to_mint(packages);
    if !mint_packages.is_empty() {
        apt_get(mint_packages);
    }
}

/// Convert package names from other distros to Mint
fn convert_to_mint(input_packages: HashSet<String>) -> HashSet<String> {
    let results = db::pack_query(input_packages);
    let mut apt_converted: HashSet<String> = HashSet::new();

    // Using the querys store into the HashSet the actual
    // Mint package name for use later
    for i in results {
        // All querys will either be a string or '' in the db
        // allowing us to use is_empty()
        if !i.mint.is_empty() {
            apt_converted.insert(i.mint);
        }
    }

    apt_converted
}

// apt-get specific functions

/// Calls the apt-get program to install packages
///
/// #Examples
///
/// ```
/// let mut packages: HashSet<String> = HashSet::new();
/// packages.insert("sudo".to_owned());
/// apt-get(packages);
/// ```
///
pub fn apt_get(mut packages: HashSet<String>) {
    let mut child = match Command::new("apt-get")
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

/// Calls the apt-get program to refresh the package list
///
/// #Examples
///
/// ```
/// refresh_list();
/// ```
///
pub fn refresh_list() {
    let mut child = match Command::new("apt-get")
        .arg("update")
        .spawn() {
        Ok(child) => child,
        Err(e) => panic!("Failed to execute child: {}", e),
    };
    let _unused = child.wait();
}

/// Calls the apt-get program to upgrage all packages
///
/// #Examples
///
/// ```
/// refresh_list();
/// ```
///
pub fn upgrade_packages() {
    let mut child = match Command::new("apt-get")
        .arg("upgrade")
        .spawn() {
        Ok(child) => child,
        Err(e) => panic!("Failed to execute child: {}", e),
    };
    let _unused = child.wait();
}
