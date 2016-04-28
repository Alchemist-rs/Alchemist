//External Crate Imports
extern crate shaper;
extern crate clap;

//Shaper Imports
use shaper::su;
use shaper::distro::common::{Distro,which_distro};
use shaper::distro::arch;

//Clap Imports
use clap::{App, Arg};

//Std Lib Imports
use std::process::exit;

fn main() {

    if !su::has_sudo() {
        println!("Sudo is not installed. Please install sudo (preferably) or run this as root");
        exit(0);
    }

    //Create Argument Flag Parser
    let args = App::new("Shaper")
                    .version("0.1.0")
                    .author("Michael Gattozzi <mgattozzi@gmail.com>")
                    .about("Unix Platform Agnostic Installer")
                    .arg(Arg::with_name("install")
                         .short("i")
                         .takes_value(true)
                         .multiple(true)
                         .help("Install the given programs"))
                    .arg(Arg::with_name("debug")
                         .short("d")
                         .long("debug")
                         .help("Turn debugging information on"))
                    .arg(Arg::with_name("verbose")
                         .short("v")
                         .long("verbose")
                         .help("Make output more verbose"))
                    .get_matches();

    //Create Argument Flag variables
    let mut debug = false;
    let mut verbose = false;

    //Toggle debug
    if args.is_present("debug") {
        debug = true;
    }

    //Toggle verbos
    if args.is_present("verbose") {
        verbose = true;
    }

    //Determine Distro of User
    let opt_dist = which_distro();
    if opt_dist.is_none() {
        println!("Your distribution is not supported");
        exit(0);
    }

    //Prepare parse arguments of what to install
    let mut packages: Vec<&str> = Vec::new();
    if let Some(p) = args.values_of("install") {
        for i in p {
            packages.push(i);
        }
    }

    match opt_dist.expect("None found, Distro was not handled") {
        Distro::Arch    => {
            arch::pac_install(packages);
        },
        Distro::Ubuntu  => println!("Ubuntu"),
        Distro::Mint    => println!("Mint"),
        Distro::Debian  => println!("Debian"),
        Distro::Gentoo  => println!("Gentoo"),
        Distro::Mac     => println!("Mac"),
        Distro::FreeBSD => println!("FreeBSD"),
        Distro::NetBSD  => println!("NetBSD"),
        Distro::OpenBSD => println!("OpenBSD"),
    }
}
