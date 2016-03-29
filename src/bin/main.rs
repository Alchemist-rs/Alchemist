//External Crate Imports
extern crate shaper;
extern crate clap;

//Shaper Imports
use shaper::distro::common::{Distro,which_distro};

//Clap Imports
use clap::{App, Arg};

//Std Lib Imports
use std::process::exit;

fn main() {

    //Create Argument Flag Parser
    let args = App::new("Shaper")
                    .version("0.1.0")
                    .author("Michael Gattozzi <mgattozzi@gmail.com>")
                    .about("Unix Platform Agnostic Installer")
                    .arg(Arg::with_name("debug")
                         .short("d")
                         .long("debug")
                         .help("Turn debugging information on"))
                    .arg(Arg::with_name("verbose")
                         .short("v")
                         .long("verbose")
                         .help("Make output more verbose"))
                    .get_matches();

    //Set Argument Flag variables
    let mut debug = false;
    let mut verbose = false;

    if let Some(d) = args.value_of("debug") {
        debug = true;
    }

    if let Some(v) = args.value_of("verbose") {
        verbose = true;
    }

    //Determine Distro of User
    let opt_dist = which_distro();
    if opt_dist.is_none() {
        println!("Your distribution is not supported");
        exit(0);
    }

    match opt_dist.expect("None Distro was not handled") {
        Distro::Arch    => println!("Arch Linux"),
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
