use std::process::Command;
use std::collections::HashSet;
use std::fs;

use db;

/// Installs Packages on FreeBSD
///
/// #Examples
///
/// ```
/// let mut packages: HashSet<&str> = HashSet::new();
/// packages.push("sudo");
/// packages.push("postgresql");
/// freebsd_install(packages);
/// ```
///
pub fn freebsd_install(packages: HashSet<String>) {
    let freebsd_packages = convert_to_freebsd(packages);
    if !freebsd_packages.is_empty() {
        pkg(freebsd_packages);
    }
}

/// Convert package names from other distros to FreeBSD
fn convert_to_freebsd(input_packages: HashSet<String>) -> HashSet<String> {
    let results = db::pack_query(input_packages);
    let mut pkg_converted: HashSet<String> = HashSet::new();

    // Using the querys store into the HashSet the actual
    // FreeBSD package name for use later
    for i in results {
        // All querys will either be a string or '' in the db
        // allowing us to use is_empty()
        if !i.freebsd.is_empty() {
            pkg_converted.insert(i.freebsd);
        }
    }

    pkg_converted
}

// Pkg specific functions

/// Calls the pkg program to install packages
///
/// #Examples
///
/// ```
/// let mut packages: HashSet<String> = HashSet::new();
/// packages.push("sudo".to_owned());
/// pkg(packages);
/// ```
///
pub fn pkg(mut packages: HashSet<String>) {
    let mut child = match Command::new("pkg")
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

/// Calls the pkg program to refresh the package list
///
/// #Examples
///
/// ```
/// refresh_list();
/// ```
///
pub fn refresh_list() {
    let mut child = match Command::new("pkg")
        .arg("update")
        .spawn() {
        Ok(child) => child,
        Err(e) => panic!("Failed to execute child: {}", e),
    };
    let _unused = child.wait();
}

/// Calls the pacman program to upgrage all packages
///
/// #Examples
///
/// ```
/// refresh_list();
/// ```
///
pub fn upgrade_packages() {
    let mut child = match Command::new("pkg")
        .arg("upgrade")
        .spawn() {
        Ok(child) => child,
        Err(e) => panic!("Failed to execute child: {}", e),
    };
    let _unused = child.wait();
}
