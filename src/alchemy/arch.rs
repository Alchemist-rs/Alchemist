use std::process::Command;
use db;
use std::fs;

/// Installs Packages on Arch Linux
///
/// #Examples
///
/// ```
/// let mut packages: Vec<&str> = Vec::new();
/// packages.push("sudo");
/// packages.push("postgresql");
/// arch_install(packages);
/// ```
///
pub fn arch_install(packages: Vec<&str>) {
    let (arch_packages,aur_packages) = convert_to_arch(packages);
    if !arch_packages.is_empty() {
        pacman(arch_packages);
    }
    if !aur_packages.is_empty() {
        aur(aur_packages);
    }
}

///Convert package names from other distros to Arch
fn convert_to_arch(input_packages: Vec<&str>) -> (Vec<String>,Vec<String>) {
    let results = db::pack_query(input_packages);
    let mut pac_converted: Vec<String> = Vec::new();
    let mut aur_converted: Vec<String> = Vec::new();

    //Using the querys store into the vectors the actual
    //Arch package name for use later
    for i in results {
        //All querys will either be a string or '' in the db
        //allowing us to use is_empty()
        if !i.arch.is_empty() {
            pac_converted.push(i.arch);
        }

        if !i.aur.is_empty() {
            aur_converted.push(i.aur);
        }
    }

    //Remove duplicates from both vectors
    if !pac_converted.is_empty() {
        pac_converted.sort();
        pac_converted.dedup();
    }
    if !aur_converted.is_empty() {
        aur_converted.sort();
        aur_converted.dedup();
    }

    (pac_converted,aur_converted)
}

//Pacman specific functions

/// Calls the pacman program to install packages
///
/// #Examples
///
/// ```
/// let mut packages: Vec<String> = Vec::new();
/// packages.push("sudo".to_owned());
/// pacman(packages);
/// ```
///
pub fn pacman(packages: Vec<String>) {
    let mut child = match Command::new("pacman")
            .arg("-S")
            .args(packages.as_slice())
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

///Enum representing all the AUR installers available
enum AURHelper {
    ///User manually installs packages from the AUR
    NoHelp,
    ///Users uses Yaourt to install packages from the AUR
    Yaourt,
    ///Users uses Aura to install packages from the AUR
    Aura,
    ///Users uses Pacuar to install packages from the AUR
    Pacuar,
}

/// Installs or Lists AUR packages for the User
///
/// #Examples
/// ```
/// let mut packages: Vec<String> = Vec::new();
/// packages.push("google-chrome-unstable".to_owned());
/// aur(packages);
/// ```
///
pub fn aur(packages: Vec<String>) {
    let helper = find_helper();
    match helper {
        AURHelper::NoHelp => no_helper(packages),
        AURHelper::Yaourt => yaourt(packages),
        AURHelper::Aura   => aura(packages),
        AURHelper::Pacuar => pacuar(packages)
    }
}

/// Figures out which AUR Installer is used by the user
fn find_helper() -> AURHelper {
    let pacuar = fs::metadata("/usr/bin/pacuar")
    let aura   = fs::metadata("/usr/bin/aura");
    let yaourt = fs::metadata("/usr/bin/yaourt");

    if yaourt.is_ok() && yaourt.unwrap().is_file() {
        //If the file exists they use Yaourt so use that to
        //install any packages needed from the AUR
        AURHelper::Yaourt
    }
    else if aura.is_ok() && aura.unwrap().is_file() {
        //If the file exists they use Aura so use that to
        //install any packages needed from the AUR
        AURHelper::Aura
    }
    else if pacuar.is_ok() && pacuar.unwrap().is_file() {
    } else {
        //They don't have one available
        AURHelper::NoHelp
    }
}

///Prints out package names to install manually from the AUR
fn no_helper(packages: Vec<String>){
    println!("You have no aur helper installed.\nYou'll need to install the following packages manually:");
    for i in packages {
        println!("{}",i);
    }
}

///Installs packages from the AUR using Yaourt
fn yaourt(packages: Vec<String>){
    let mut child = match Command::new("yaourt")
            .args(packages.as_slice())
            .spawn()
    {
        Ok(child) => child,
        Err(e)    => panic!("Failed to execute child: {}",e),
    };
    let _unused = child.wait();
}

///Installs packages from the AUR using Auara
fn aura(packages: Vec<String>){
    let mut child = match Command::new("aura")
            .arg("-A")
            .args(packages.as_slice())
            .spawn()
    {
        Ok(child) => child,
        Err(e)    => panic!("Failed to execute child: {}",e),
    };
    let _unused = child.wait();
}
///Installs packages from the AUR using Pacuar
fn pacuar(packages: Vac<String>){
    let mut child = match Command::new("pacuar")
            .arg("-S")
            .args(packages.as_slice())
            .spawn()
    {
        Ok(child) => child,
        Err(e)    => panic!("Failed to execute child: {}",e),
    };
    let _unused = child.wait();
}
