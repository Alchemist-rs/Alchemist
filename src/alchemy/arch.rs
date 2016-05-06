use std::process::Command;
use std::collections::HashSet;
use std::fs;

use db;

/// Installs Packages on Arch Linux
///
/// #Examples
///
/// ```
/// let mut packages: HashSet<&str> = HashSet::new();
/// packages.push("sudo");
/// packages.push("postgresql");
/// arch_install(packages);
/// ```
///
pub fn arch_install(packages: HashSet<String>) {
    let (arch_packages, aur_packages) = convert_to_arch(packages);
    if !arch_packages.is_empty() {
        pacman(arch_packages);
    }
    if !aur_packages.is_empty() {
        aur(aur_packages);
    }
}

///Convert package names from other distros to Arch
fn convert_to_arch(input_packages: HashSet<String>) -> (HashSet<String>,HashSet<String>) {
    let results = db::pack_query(input_packages);
    let mut pac_converted: HashSet<String> = HashSet::new();
    let mut aur_converted: HashSet<String> = HashSet::new();
    //Using the querys store into the HashSet the actual
    //Arch package name for use later
    for i in results {
        //All querys will either be a string or '' in the db
        //allowing us to use is_empty()
        if !i.arch.is_empty() {
            pac_converted.insert(i.arch);
        }

        if !i.aur.is_empty() {
            aur_converted.insert(i.aur);
        }
    }

    (pac_converted,aur_converted)
}

//Pacman specific functions

/// Calls the pacman program to install packages
///
/// #Examples
///
/// ```
/// let mut packages: HashSet<String> = HashSet::new();
/// packages.push("sudo".to_owned());
/// pacman(packages);
/// ```
///
pub fn pacman(mut packages: HashSet<String>) {
    let mut child = match Command::new("pacman")
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

/// Calls the pacman program to refresh the package list
///
/// #Examples
///
/// ```
/// refresh_list();
/// ```
///
pub fn refresh_list() {
    let mut child = match Command::new("pacman")
            .arg("-Syy")
            .spawn()
    {
        Ok(child) => child,
        Err(e)    => panic!("Failed to execute child: {}",e),
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
    let mut child = match Command::new("pacman")
            .arg("-Syyu")
            .spawn()
    {
        Ok(child) => child,
        Err(e)    => panic!("Failed to execute child: {}",e),
    };
    let _unused = child.wait();
}

//AUR related functions and Data types

#[allow(dead_code)]
///Enum representing all the AUR installers available
enum AURHelper {
    ///User manually installs packages from the AUR
    NoHelp,
    ///Uses Aura to install packages from the AUR
    Aura,
    ///Uses Aurel to install packages from the AUR
    Aurel,
    ///Uses Aurutils to install packages from the AUR
    Aurutils,
    ///Uses Bauerbill to install packages from the AUR
    Bauerbill,
    ///Uses Burgaur to install packages from the AUR
    Burgaur,
    ///Uses Cower to install packages from the AUR
    Cower,
    ///Uses Pacaur to install packages from the AUR
    Pacaur,
    ///Uses Packer to install packages from the AUR
    Packer,
    ///Uses Pbget to install packages from the AUR
    Pbget,
    ///Uses PKGBUILDer to install packages from the AUR
    PKGBUILDer,
    ///Uses Prm to install packages from the AUR
    Prm,
    ///Uses Spinach to install packages from the AUR
    Spinach,
    ///Uses Trizen to install packages from the AUR
    Trizen,
    ///Uses Wrapaur to install packages from the AUR
    Wrapaur,
    ///Uses Yaah to install packages from the AUR
    Yaah,
    ///Uses Yaourt to install packages from the AUR
    Yaourt,
}

/// Installs or Lists AUR packages for the User
///
/// #Examples
/// ```
/// let mut packages: HashSet<String> = HashSet::new();
/// packages.push("google-chrome-unstable".to_owned());
/// aur(packages);
/// ```
///
pub fn aur(packages: HashSet<String>) {
    let helper = find_helper();
    match helper {
        AURHelper::Aura       => aura(packages),
        AURHelper::Pacaur     => pacaur(packages),
        AURHelper::Packer     => packer(packages),
        AURHelper::Yaourt     => yaourt(packages),
        AURHelper::NoHelp     => no_helper(packages),
        _ => unreachable!()
    }
}

/// Figures out which AUR Installer is used by the user
fn find_helper() -> AURHelper {
    //Maybe there's a better way to do this in the future?
    let aura   = fs::metadata("/usr/bin/aura");
    let pacaur = fs::metadata("/usr/bin/pacaur");
    let packer = fs::metadata("/usr/bin/packer");
    let yaourt = fs::metadata("/usr/bin/yaourt");

    //Depending on what file exists that AURHelper enum is
    //returned and used to install or upgrade packages.
    //Otherwise, the NoHelp exists and they have to install
    //packages manually
    if aura.is_ok() && aura.unwrap().is_file() {
        AURHelper::Aura
    } else if pacaur.is_ok() && pacaur.unwrap().is_file() {
        AURHelper::Pacaur
    } else if packer.is_ok() && packer.unwrap().is_file() {
        AURHelper::Packer
    } else if yaourt.is_ok() && yaourt.unwrap().is_file() {
        AURHelper::Yaourt
    } else {
        AURHelper::NoHelp
    }
}

///Installs packages from the AUR using Aura
fn aura(mut packages: HashSet<String>){
    let mut child = match Command::new("aura")
            .arg("-A")
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

///Installs packages from the AUR using Pacaur
fn pacaur(mut packages: HashSet<String>){
    let mut child = match Command::new("pacaur")
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

///Installs packages from the AUR using Packer
fn packer(mut packages: HashSet<String>){
    let mut child = match Command::new("packer")
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

///Installs packages from the AUR using Yaourt
fn yaourt(mut packages: HashSet<String>){
    let mut child = match Command::new("yaourt")
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

///Prints out package names to install manually from the AUR
fn no_helper(packages: HashSet<String>){
    println!("You have no aur helper installed.\nYou'll need to install the following packages manually:");
    for i in packages {
        println!("{}",i);
    }
}
