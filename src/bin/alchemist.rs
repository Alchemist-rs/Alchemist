// External Crate Imports
extern crate alchemy;
extern crate clap;
extern crate diesel;

// Alchemy Imports
use alchemy::su;
use alchemy::distro::{Distro, which_distro};
use alchemy::{arch, transmute};

// Clap Imports
use clap::{App, Arg};

// Std Lib Imports
use std::process::exit;
use std::collections::HashSet;

fn main() {

    if !su::has_sudo() {
        println!("Sudo is not installed. Please install sudo (preferably) or run this as root");
        exit(0);
    }

    // Create Argument Flag Parser
    let args = App::new("Shaper")
        .version("0.1.0")
        .author("Michael Gattozzi <mgattozzi@gmail.com>")
        .about("Unix Platform Agnostic Installer")
        .arg(Arg::with_name("install")
            .short("i")
            .takes_value(true)
            .multiple(true)
            .help("Install the given programs"))
        .arg(Arg::with_name("refresh")
            .short("r")
            .long("refresh")
            .help("Refresh package list with newest version")
            .takes_value(false)
            .conflicts_with("upgrade"))
        .arg(Arg::with_name("upgrade")
            .short("u")
            .long("upgrade")
            .help("Refresh & upgrade packages to newest version")
            .takes_value(false)
            .conflicts_with("refresh"))
        .get_matches();


    // Determine Distro of User
    let opt_dist = which_distro();
    if opt_dist.is_none() {
        println!("Your distribution is not supported");
        exit(0);
    }

    // Prepare parse arguments of what to install
    let mut package_inputs: HashSet<String> = HashSet::new();
    if let Some(p) = args.values_of("install") {
        for i in p {
            package_inputs.insert(i.to_string());
        }
        if package_inputs.contains("pb") {
            println!("Looks like you're trying to turn lead into gold.");
            println!("That's not how this program works.");
            exit(0);
        }
    }

    match opt_dist.expect("None found, Distro was not handled") {
        Distro::Arch => {
            if args.values_of("refresh").is_some() {
                arch::refresh_list();
            } else if args.values_of("upgrade").is_some() {
                arch::upgrade_packages();
            }
            arch::arch_install(package_inputs);
        }
        Distro::Debian => {
            if args.values_of("refresh").is_some() {
                transmute::refresh_list();
            } else if args.values_of("upgrade").is_some() {
                transmute::upgrade_packages();
            }
            transmute::distro_install(package_inputs);
        }
        Distro::Mint => {
            if args.values_of("refresh").is_some() {
                transmute::refresh_list();
            } else if args.values_of("upgrade").is_some() {
                transmute::upgrade_packages();
            }
            transmute::distro_install(package_inputs);
        }
        Distro::Ubuntu => {
            if args.values_of("refresh").is_some() {
                transmute::refresh_list();
            } else if args.values_of("upgrade").is_some() {
                transmute::upgrade_packages();
            }
            transmute::distro_install(package_inputs);
        }
        Distro::Void => {
            if args.values_of("refresh").is_some() {
                transmute::refresh_list();
            } else if args.values_of("upgrade").is_some() {
                transmute::upgrade_packages();
            }
            transmute::distro_install(package_inputs);
        }
        Distro::FreeBSD => {
            if args.values_of("refresh").is_some() {
                transmute::refresh_list();
            } else if args.values_of("upgrade").is_some() {
                transmute::upgrade_packages();
            }
            transmute::distro_install(package_inputs);
        }
        Distro::Gentoo => println!("Gentoo"),
        Distro::Mac => println!("Mac"),
        Distro::NetBSD => println!("NetBSD"),
        Distro::OpenBSD => println!("OpenBSD"),
    }
}
