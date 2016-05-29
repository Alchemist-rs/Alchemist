// Distro enum import
use super::distro::{Distro, which_distro};

// Imports needed to run each command
use std::process::Command;
// Std Lib Imports
use std::collections::HashSet;
use std::string::String;

use db;

/// Installs packages for what ever Distro is returned
/// from which_distro();
///
/// #Examples
///
/// ```
/// let mut packages: HashSet<&str> = HashSet::new();
/// packages.push("sudo");
/// packages.push("postgresql");
/// void_install(packages);
/// ```
///
pub fn distro_install(packages: HashSet<String>) {
    let distro_packages = convert_to_distro(packages);
    if !distro_packages.is_empty() {
        pac(distro_packages);
    }
}

/// Convert package names from other distros to the one being run by the user currently
fn convert_to_distro(input_packages: HashSet<String>) -> HashSet<String> {
    let results = db::pack_query(input_packages);
    let mut pac_converted: HashSet<String> = HashSet::new();

    // All querys will either be a string or '' in the db allowing us to
    // use is_empty()
    // Finds out what distro is used and inserts the proper conversions
    // into the HashSet to be returned from the function
    match which_distro().expect("None found, Distro was not handled") {
        Distro::Ubuntu => {
            for i in results {
                if !i.ubuntu.is_empty() {
                    pac_converted.insert(i.ubuntu);
                }
            }
        }
        Distro::Void => {
            for i in results {
                if !i.void.is_empty() {
                    pac_converted.insert(i.void);
                }
            }
        }
        Distro::Debian => {
            for i in results {
                if !i.debian.is_empty() {
                    pac_converted.insert(i.debian);
                }
            }
        }
        Distro::Mint => {
            for i in results {
                if !i.mint.is_empty() {
                    pac_converted.insert(i.mint);
                }
            }
        }
        Distro::FreeBSD => {
            for i in results {
                if !i.freebsd.is_empty() {
                    pac_converted.insert(i.freebsd);
                }
            }
        }
        Distro::Gentoo => println!("Gentoo"),
        Distro::Mac => println!("Mac"),
        Distro::NetBSD => println!("NetBSD"),
        Distro::OpenBSD => println!("OpenBSD"),
        Distro::Arch => println!("Arch"),
    }

    pac_converted
}

// Package Manager specific functions

/// Calls the package manager program to install packages
///
/// #Examples
///
/// ```
/// let mut packages: HashSet<String> = HashSet::new();
/// packages.push("sudo".to_owned());
/// pac(packages);
/// ```
///
pub fn pac(mut packages: HashSet<String>) {

    match which_distro().expect("None found, Distro was not handled") {
        Distro::Ubuntu => {
            let mut child = match Command::new("apt-get")
                .arg("install")
                .args(packages.drain().collect::<Vec<String>>().as_slice())
                .spawn() {
                Ok(child) => child,
                Err(e) => panic!("Failed to execute child: {}", e),
            };
            let _unused = child.wait();
        }
        Distro::Void => {
            let mut child = match Command::new("xbps-install")
                .arg("-Sy")
                .args(packages.drain().collect::<Vec<String>>().as_slice())
                .spawn() {
                Ok(child) => child,
                Err(e) => panic!("Failed to execute child: {}", e),
            };
            let _unused = child.wait();
        }
        Distro::Debian => {
            let mut child = match Command::new("apt-get")
                .arg("install")
                .args(packages.drain().collect::<Vec<String>>().as_slice())
                .spawn() {
                Ok(child) => child,
                Err(e) => panic!("Failed to execute child: {}", e),
            };
            let _unused = child.wait();
        }
        Distro::Mint => {
            let mut child = match Command::new("apt-get")
                .arg("install")
                .args(packages.drain().collect::<Vec<String>>().as_slice())
                .spawn() {
                Ok(child) => child,
                Err(e) => panic!("Failed to execute child: {}", e),
            };
            let _unused = child.wait();
        }
        Distro::FreeBSD => {
            let mut child = match Command::new("pkg")
                .arg("install")
                .args(packages.drain().collect::<Vec<String>>().as_slice())
                .spawn() {
                Ok(child) => child,
                Err(e) => panic!("Failed to execute child: {}", e),
            };
            let _unused = child.wait();
        }
        Distro::Gentoo => println!("Gentoo"),
        Distro::Mac => println!("Mac"),
        Distro::NetBSD => println!("NetBSD"),
        Distro::OpenBSD => println!("OpenBSD"),
        Distro::Arch => println!("Arch"),
    }
}

/// Calls the xbps-install program to refresh the package list
///
/// #Examples
///
/// ```
/// refresh_list();
/// ```
///
pub fn refresh_list() {
    match which_distro().expect("None found, Distro was not handled") {
        Distro::Ubuntu => {
            let mut child = match Command::new("apt-get")
                .arg("update")
                .spawn() {
                Ok(child) => child,
                Err(e) => panic!("Failed to execute child: {}", e),
            };
            let _unused = child.wait();
        }
        Distro::Void => {
            let mut child = match Command::new("xbps-install")
                .arg("-Sy")
                .spawn() {
                Ok(child) => child,
                Err(e) => panic!("Failed to execute child: {}", e),
            };
            let _unused = child.wait();
        }
        Distro::Debian => {
            let mut child = match Command::new("apt-get")
                .arg("update")
                .spawn() {
                Ok(child) => child,
                Err(e) => panic!("Failed to execute child: {}", e),
            };
            let _unused = child.wait();
        }
        Distro::Mint => {
            let mut child = match Command::new("apt-get")
                .arg("update")
                .spawn() {
                Ok(child) => child,
                Err(e) => panic!("Failed to execute child: {}", e),
            };
            let _unused = child.wait();
        }
        Distro::FreeBSD => {
            let mut child = match Command::new("pkg")
                .arg("update")
                .spawn() {
                Ok(child) => child,
                Err(e) => panic!("Failed to execute child: {}", e),
            };
            let _unused = child.wait();
        }
        Distro::Gentoo => println!("Gentoo"),
        Distro::Mac => println!("Mac"),
        Distro::NetBSD => println!("NetBSD"),
        Distro::OpenBSD => println!("OpenBSD"),
        Distro::Arch => println!("Arch"),
    }
}

/// Calls the xbps-install program to upgrage all packages
///
/// #Examples
///
/// ```
/// refresh_list();
/// ```
///
pub fn upgrade_packages() {

    match which_distro().expect("None found, Distro was not handled") {
        Distro::Ubuntu => {
            let mut child = match Command::new("apt-get")
                .arg("upgrade")
                .spawn() {
                Ok(child) => child,
                Err(e) => panic!("Failed to execute child: {}", e),
            };
            let _unused = child.wait();
        }
        Distro::Void => {
            let mut child = match Command::new("xbps-install")
                .arg("-Syu")
                .spawn() {
                Ok(child) => child,
                Err(e) => panic!("Failed to execute child: {}", e),
            };
            let _unused = child.wait();
        }
        Distro::Debian => {
            let mut child = match Command::new("apt-get")
                .arg("upgrade")
                .spawn() {
                Ok(child) => child,
                Err(e) => panic!("Failed to execute child: {}", e),
            };
            let _unused = child.wait();
        }
        Distro::Mint => {
            let mut child = match Command::new("apt-get")
                .arg("upgrade")
                .spawn() {
                Ok(child) => child,
                Err(e) => panic!("Failed to execute child: {}", e),
            };
            let _unused = child.wait();
        }
        Distro::FreeBSD => {
            let mut child = match Command::new("pkg")
                .arg("upgrade")
                .spawn() {
                Ok(child) => child,
                Err(e) => panic!("Failed to execute child: {}", e),
            };
            let _unused = child.wait();
        }
        Distro::Gentoo => println!("Gentoo"),
        Distro::Mac => println!("Mac"),
        Distro::NetBSD => println!("NetBSD"),
        Distro::OpenBSD => println!("OpenBSD"),
        Distro::Arch => println!("Arch"),
    }
}
