use std::process::Command;
use std::collections::HashSet;

use db;

/// Installs Packages on Void Linux
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
pub fn void_install(packages: HashSet<String>) {
    let void_packages = convert_to_void(packages);
    if !void_packages.is_empty() {
        xbps(void_packages);
    }
}

///Convert package names from other distros to void
fn convert_to_void(input_packages: HashSet<String>) -> HashSet<String> {
    let results = db::pack_query(input_packages);
    let mut xbps_converted: HashSet<String> = HashSet::new();

    //Using the querys store into the HashSet the actual
    //void package name for use later
    for i in results {
        //All querys will either be a string or '' in the db
        //allowing us to use is_empty()
        if !i.void.is_empty() {
            xbps_converted.insert(i.void);
        }
    }

    xbps_converted
}

//xbps specific functions

/// Calls the xbps program to install packages
///
/// #Examples
///
/// ```
/// let mut packages: HashSet<String> = HashSet::new();
/// packages.push("sudo".to_owned());
/// xbps(packages);
/// ```
///
pub fn xbps(mut packages: HashSet<String>) {
    let mut child = match Command::new("xbps-install")
            .arg("-S")
            .args(packages
                  .drain()
                  .collect::<Vec<String>>()
                  .as_slice())
            .spawn()
    {
        Ok(child) => child,
        Err(e)    => panic!("Failed to execute child: {}",e),
    };
    let _unused = child.wait();
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
    let mut child = match Command::new("xbps-install")
            .arg("-Sy")
            .spawn()
    {
        Ok(child) => child,
        Err(e)    => panic!("Failed to execute child: {}",e),
    };
    let _unused = child.wait();
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
    let mut child = match Command::new("xbps-install")
            .arg("-Syu")
            .spawn()
    {
        Ok(child) => child,
        Err(e)    => panic!("Failed to execute child: {}",e),
    };
    let _unused = child.wait();
}
