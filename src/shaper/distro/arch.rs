use std::process::Command;
use super::super::db;
use std::fs;

pub fn arch_install(packages: Vec<&str>) {
    let (arch_packages,aur_packages) = convert_to_arch(packages);
    if !arch_packages.is_empty() {
        pacman(arch_packages);
    }
    if !aur_packages.is_empty() {
        aur(aur_packages);
    }
}

fn convert_to_arch(input_packages: Vec<&str>) -> (Vec<String>,Vec<String>) {
    let results = db::pack_query(input_packages);
    let mut pac_converted: Vec<String> = Vec::new();
    let mut aur_converted: Vec<String> = Vec::new();
    for i in results {
        //All querys will either be a string or '' in the db
        if !i.arch.is_empty() {
            pac_converted.push(i.arch);
        }

        if !i.aur.is_empty() {
            aur_converted.push(i.aur);
        }
    }

    (pac_converted,aur_converted)
}

//Pacman specific functions
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
enum AURHelper {
    NoHelp,
    Yaourt,
    Aura,
}

fn aur(packages: Vec<String>) {
    let helper = find_helper();
    match helper {
        AURHelper::NoHelp => no_helper(packages),
        AURHelper::Yaourt => yaourt(packages),
        AURHelper::Aura   => aura(packages)
    }
}

fn find_helper() -> AURHelper {
    let aura   = fs::metadata("/usr/bin/aura");
    let yaourt = fs::metadata("/usr/bin/yaourt");
    if yaourt.is_ok() && yaourt.unwrap().is_file() {
        return AURHelper::Yaourt;
    }
    else if aura.is_ok() && aura.unwrap().is_file() {
        return AURHelper::Aura;
    } else {
        AURHelper::NoHelp
    }
}

fn no_helper(packages: Vec<String>){
    println!("You have no aur helper installed.\nYou'll need to install the following packages manually:");
    for i in packages {
        println!("{}",i);
    }
}

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


