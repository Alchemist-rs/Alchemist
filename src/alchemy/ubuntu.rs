use std::process::Command;
use std::collections::HashSet;

use db;

/// Installs Packages on Ubuntu Linux
///
/// #Examples
///
/// ```
/// let mut packages: HashSet<&str> = HashSet::new();
/// packages.insert("sudo");
/// packages.insert("postgresql");
/// ubuntu_install(packages);
/// ```
///
pub fn ubuntu_install(packages: HashSet<String>) {
    let ubuntu_packages = convert_to_ubuntu(packages);
    if !ubuntu_packages.is_empty() {
        apt_get(ubuntu_packages);
    }
}

///Convert package names from other distros to Ubuntu
fn convert_to_ubuntu(input_packages: HashSet<String>) -> HashSet<String> {
    let results = db::pack_query(input_packages);
    let mut pac_converted: HashSet<String> = HashSet::new();

    //Using the querys store into the HashSet the actual
    //Ubuntu package name for use later
    for i in results {
        //All querys will either be a string or '' in the db
        //allowing us to use is_empty()
        if !i.ubuntu.is_empty() {
            pac_converted.insert(i.ubuntu);
        }
        if !i.ubuntu_dev.is_empty() {
            pac_converted.insert(i.ubuntu_dev);
        }
    }

    pac_converted
}

//apt-get specific functions

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
            .spawn()
    {
        Ok(child) => child,
        Err(e)    => panic!("Failed to execute child: {}",e),
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
            .spawn()
    {
        Ok(child) => child,
        Err(e)    => panic!("Failed to execute child: {}",e),
    };
    let _unused = child.wait();
}
